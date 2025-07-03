#include "App.h"
#include <Windows.h>
#include <cassert>
#include <combaseapi.h>
#include <d3d12.h>
#include <dxgi.h>
#include <dxgiformat.h>
#include <dxgitype.h>
#include <synchapi.h>

namespace {
const auto ClassName = TEXT("Hello, D3D12");

template <class T>
void Release(T *&p) {
    if (p != nullptr) {
        p->Release();
        p = nullptr;
    }
}
} // namespace

App::App(uint32_t const width, uint32_t const height)
    : hinst_(nullptr), hwnd_(nullptr), width_(width), height_(height) {}

App::~App() {}

void App::run() {
    if (init_app()) {
        main_loop();
    }
    term_app();
}

bool App::init_app() {
    return init_window() && init_d3d();
}

bool App::init_window() {
    auto const hinst = GetModuleHandle(nullptr);
    if (hinst == nullptr) {
        return false;
    }

    WNDCLASSEX wc = {};
    wc.cbSize = sizeof(WNDCLASSEX);
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = window_proc;
    wc.hIcon = LoadIcon(hinst, IDI_APPLICATION);
    wc.hCursor = LoadCursor(hinst, IDC_ARROW);
    wc.hbrBackground = GetSysColorBrush(COLOR_BACKGROUND);
    wc.lpszMenuName = nullptr;
    wc.lpszClassName = ClassName;
    wc.hIconSm = LoadIcon(hinst, IDI_APPLICATION);

    if (!RegisterClassEx(&wc)) {
        return false;
    }

    hinst_ = hinst;

    RECT rc = {};
    rc.right = static_cast<LONG>(width_);
    rc.bottom = static_cast<LONG>(height_);

    auto const style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU;
    AdjustWindowRect(&rc, style, FALSE);

    hwnd_ = CreateWindowEx(
        0,
        ClassName,
        TEXT("Sample"),
        style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        rc.right - rc.left,
        rc.bottom - rc.top,
        nullptr,
        nullptr,
        hinst_,
        nullptr);

    if (hwnd_ == nullptr) {
        return false;
    }

    ShowWindow(hwnd_, SW_SHOWNORMAL);

    UpdateWindow(hwnd_);

    return true;
}

void App::main_loop() {
    MSG msg = {};
    while (msg.message != WM_QUIT) {
        if (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE) == TRUE) {
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        } else {
            render();
        }
    }
}

void App::term_app() {
    term_d3d();
    term_window();
}

void App::term_window() {
    if (hinst_ != nullptr) {
        UnregisterClass(ClassName, hinst_);
    }
    hinst_ = nullptr;
    hwnd_ = nullptr;
}

bool App::init_d3d() {
    // Create device
    {
        auto const hr = D3D12CreateDevice(
            nullptr,                 // Video adaptor
            D3D_FEATURE_LEVEL_11_0,  // Minimal feature level
            IID_PPV_ARGS(&device_)); // Macro equivalent to `__uuidof(&device_), (void **)device_`
        if (FAILED(hr)) {
            return false;
        }
    }

    // Create command queue to submit draw commands to GPU
    {
        D3D12_COMMAND_QUEUE_DESC desc{};
        desc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT; // Commands that GPU executes
        desc.Priority = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL;
        desc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
        desc.NodeMask = 0; // Note: Assumes only single GPU is available

        auto const hr = device_->CreateCommandQueue(&desc, IID_PPV_ARGS(&queue_));
        if (FAILED(hr)) {
            return false;
        }
    }

    // Create swap chain to swap frame buffers (double buffer)
    {
        // Factory object to access DXGI
        // Note: DXGI = DirectX Graphics Infrastructure
        IDXGIFactory4 *factory = nullptr;
        auto hr = CreateDXGIFactory1(IID_PPV_ARGS(&factory));
        if (FAILED(hr)) {
            return false;
        }

        DXGI_SWAP_CHAIN_DESC desc{};
        desc.BufferDesc.Width = width_;
        desc.BufferDesc.Height = height_;
        desc.BufferDesc.RefreshRate.Numerator = 60; // Refresh rate is 60/1
        desc.BufferDesc.RefreshRate.Denominator = 1;
        desc.BufferDesc.ScanlineOrdering = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
        desc.BufferDesc.Scaling = DXGI_MODE_SCALING_UNSPECIFIED;
        desc.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
        desc.SampleDesc.Count = 1; // Number of multi-sampling per pixel
        desc.SampleDesc.Quality = 0;
        desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
        desc.BufferCount = FRAME_COUNT;
        desc.OutputWindow = hwnd_;
        desc.Windowed = TRUE;
        desc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
        desc.Flags = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH; // Switch display mode with `IDXGISwapChain::ResizeTarget()`

        IDXGISwapChain *swap_chain = nullptr;
        hr = factory->CreateSwapChain(queue_, &desc, &swap_chain);
        if (FAILED(hr)) {
            Release(factory);
            return false;
        }

        hr = swap_chain->QueryInterface(IID_PPV_ARGS(&swap_chain_));
        if (FAILED(hr)) {
            Release(factory);
            Release(swap_chain);
            return false;
        }

        frame_index_ = swap_chain_->GetCurrentBackBufferIndex();

        Release(factory);
        Release(swap_chain);
    }

    // Create command allocator
    {
        for (auto i = 0; i < FRAME_COUNT; i++) {
            auto const hr = device_->CreateCommandAllocator(
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                IID_PPV_ARGS(&cmd_alloc_[i]));
            if (FAILED(hr)) {
                return false;
            }
        }
    }

    // Create command list
    {
        auto const hr = device_->CreateCommandList(
            0,
            D3D12_COMMAND_LIST_TYPE_DIRECT,
            cmd_alloc_[frame_index_],
            nullptr,
            IID_PPV_ARGS(&cmd_list_));
        if (FAILED(hr)) {
            return false;
        }
    }

    // Create render target view (resource view object)
    {
        D3D12_DESCRIPTOR_HEAP_DESC desc{};
        desc.NumDescriptors = FRAME_COUNT;
        desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
        desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
        desc.NodeMask = 0;

        auto const hr = device_->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&heap_rtv_));
        if (FAILED(hr)) {
            return false;
        }

        auto handle = heap_rtv_->GetCPUDescriptorHandleForHeapStart();
        auto const inc_size = device_->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);

        for (auto i = 0; i < FRAME_COUNT; i++) {
            auto const hr = swap_chain_->GetBuffer(i, IID_PPV_ARGS(&color_buffer_[i]));
            if (FAILED(hr)) {
                return false;
            }

            D3D12_RENDER_TARGET_VIEW_DESC desc{};
            desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB;
            desc.ViewDimension = D3D12_RTV_DIMENSION_TEXTURE2D;
            desc.Texture2D.MipSlice = 0; // Mipmap level (0 = one mipmap)
            desc.Texture2D.PlaneSlice = 0;

            device_->CreateRenderTargetView(color_buffer_[i], &desc, handle);

            handle_rtv_[i] = handle;
            handle.ptr += inc_size;
        }
    }

    // Create fence
    {
        for (auto i = 0; i < FRAME_COUNT; i++) {
            fence_counter_[i] = 0;
        }

        auto const hr = device_->CreateFence(
            fence_counter_[frame_index_],
            D3D12_FENCE_FLAG_NONE,
            IID_PPV_ARGS(&fence_));
        if (FAILED(hr)) {
            return false;
        }

        fence_counter_[frame_index_]++;

        fence_event_ = CreateEvent(/* attributes */ nullptr, /* manual reset */ FALSE, /* initial state */ FALSE, /* name */ nullptr);
        if (fence_event_ == nullptr) {
            return false;
        }
    }

    cmd_list_->Close();

    return true;
}

void App::render() {
    cmd_alloc_[frame_index_]->Reset();
    cmd_list_->Reset(cmd_alloc_[frame_index_], /*pipeline state*/ nullptr);

    D3D12_RESOURCE_BARRIER barrier{};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    barrier.Flags = D3D12_RESOURCE_BARRIER_FLAG_NONE;
    barrier.Transition.pResource = color_buffer_[frame_index_];
    barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    // Transition: Present -> Render (write)
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_PRESENT;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET;

    cmd_list_->ResourceBarrier(1, &barrier);

    cmd_list_->OMSetRenderTargets(1, &handle_rtv_[frame_index_], FALSE, /*depth stencil view*/ nullptr);

    float const clear_color[] = {0.25f, 0.25f, 0.25f, 1.0f};

    cmd_list_->ClearRenderTargetView(handle_rtv_[frame_index_], clear_color, 0, /*target rect*/ nullptr);

    // TODO: Render polygons

    // Transition: Render (write) -> Present
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_PRESENT;

    cmd_list_->ResourceBarrier(1, &barrier);

    cmd_list_->Close();

    ID3D12CommandList *cmd_lists[] = {cmd_list_};
    queue_->ExecuteCommandLists(1, cmd_lists);

    present(1);
}

void App::present(uint32_t const interval) {
    // Render the front buffer to the screen and swap frame buffers
    // Passing `1` to `interval` means sync after the first vsync
    swap_chain_->Present(interval, 0);

    // Set the `counter` value to the fence when the commands in the command list are completed
    auto const counter = fence_counter_[frame_index_];
    queue_->Signal(fence_, counter);

    frame_index_ = swap_chain_->GetCurrentBackBufferIndex();

    // Wait until the next frame is prepared (= the fence value is set to `counter`)
    if (fence_->GetCompletedValue() < counter) {
        fence_->SetEventOnCompletion(counter, fence_event_);
        WaitForSingleObjectEx(fence_event_, INFINITE, FALSE);
    }

    // Next fence counter
    fence_counter_[frame_index_]++;
}

void App::wait_gpu() {
    assert(queue_ != nullptr);
    assert(fence_ != nullptr);
    assert(fence_event_ != nullptr);

    auto const counter = fence_counter_[frame_index_];
    queue_->Signal(fence_, counter);
    fence_->SetEventOnCompletion(counter, fence_event_);
    WaitForSingleObjectEx(fence_event_, INFINITE, FALSE);

    fence_counter_[frame_index_]++;
}

void App::term_d3d() {
    wait_gpu();

    if (fence_event_ != nullptr) {
        CloseHandle(fence_event_);
        fence_event_ = nullptr;
    }

    Release(fence_);
    Release(heap_rtv_);
    for (auto i = 0; i < FRAME_COUNT; i++) {
        Release(color_buffer_[i]);
    }
    Release(cmd_list_);
    for (auto i = 0; i < FRAME_COUNT; i++) {
        Release(cmd_alloc_[i]);
    }
    Release(swap_chain_);
    Release(queue_);
    Release(device_);
}

LRESULT CALLBACK App::window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    switch (msg) {
    case WM_DESTROY:
        PostQuitMessage(0);
        break;
    default:
        break;
    }
    return DefWindowProc(hwnd, msg, wp, lp);
}

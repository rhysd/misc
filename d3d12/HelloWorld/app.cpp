#include "App.h"
#include <DDSTextureLoader.h>
#include <ResourceUploadBatch.h>
#include <VertexTypes.h>
#include <cassert>
#include <cstring>
#include <d3d12.h>

namespace {
const auto ClassName = TEXT("Hello, D3D12");
} // namespace

App::App(uint32_t const width, uint32_t const height)
    : hinst_(nullptr), hwnd_(nullptr), width_(width), height_(height), device_(nullptr), queue_(nullptr), swap_chain_(nullptr), depth_buffer_(nullptr), cmd_list_(nullptr), heap_rtv_(nullptr), fence_(nullptr), heap_cbv_srv_uav_(nullptr), heap_dsv_(nullptr), vb_(nullptr), ib_(nullptr), root_signature_(nullptr), pipeline_state_(nullptr), frame_index_(0), fence_event_(nullptr), rotate_angle_(0.0f) {
    for (auto i = 0; i < FRAME_COUNT; i++) {
        color_buffer_[i] = nullptr;
        cmd_alloc_[i] = nullptr;
        fence_counter_[i] = 0;
    }
}

App::~App() {}

void App::run() {
    assert(init_app());
    if (true) {
        main_loop();
    }
    term_app();
}

bool App::init_app() {
    return init_window() && init_d3d() && on_init();
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
    on_term();
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
#if defined(DEBUG) || defined(_DEBUG)
    {
        // Enable debug layer
        ComPtr<ID3D12Debug> debug;
        auto const hr = D3D12GetDebugInterface(IID_PPV_ARGS(debug.GetAddressOf()));
        if (FAILED(hr)) {
            return false;
        }
        debug->EnableDebugLayer();
    }
#endif

    // Create device
    {
        auto const hr = D3D12CreateDevice(
            nullptr,                               // Video adaptor
            D3D_FEATURE_LEVEL_11_0,                // Minimal feature level
            IID_PPV_ARGS(device_.GetAddressOf())); // Macro equivalent to `__uuidof(&device_), (void **)device_`
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

        auto const hr = device_->CreateCommandQueue(&desc, IID_PPV_ARGS(queue_.GetAddressOf()));
        if (FAILED(hr)) {
            return false;
        }
    }

    // Create swap chain to swap frame buffers (double buffer)
    {
        // Factory object to access DXGI
        // Note: DXGI = DirectX Graphics Infrastructure
        ComPtr<IDXGIFactory4> factory = nullptr;
        auto hr = CreateDXGIFactory1(IID_PPV_ARGS(factory.GetAddressOf()));
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

        ComPtr<IDXGISwapChain> swap_chain = nullptr;
        hr = factory->CreateSwapChain(queue_.Get(), &desc, swap_chain.GetAddressOf());
        if (FAILED(hr)) {
            return false;
        }

        hr = swap_chain->QueryInterface(IID_PPV_ARGS(swap_chain_.GetAddressOf()));
        if (FAILED(hr)) {
            return false;
        }

        frame_index_ = swap_chain_->GetCurrentBackBufferIndex();
    }

    // Create command allocator
    {
        for (auto i = 0; i < FRAME_COUNT; i++) {
            auto const hr = device_->CreateCommandAllocator(
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                IID_PPV_ARGS(cmd_alloc_[i].GetAddressOf()));
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
            cmd_alloc_[frame_index_].Get(),
            nullptr,
            IID_PPV_ARGS(cmd_list_.GetAddressOf()));
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

        auto const hr = device_->CreateDescriptorHeap(&desc, IID_PPV_ARGS(heap_rtv_.GetAddressOf()));
        if (FAILED(hr)) {
            return false;
        }

        auto handle = heap_rtv_->GetCPUDescriptorHandleForHeapStart();
        auto const inc_size = device_->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);

        for (auto i = 0; i < FRAME_COUNT; i++) {
            auto const hr = swap_chain_->GetBuffer(i, IID_PPV_ARGS(color_buffer_[i].GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }

            D3D12_RENDER_TARGET_VIEW_DESC desc{};
            desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB;
            desc.ViewDimension = D3D12_RTV_DIMENSION_TEXTURE2D;
            desc.Texture2D.MipSlice = 0; // Mipmap level (0 = one mipmap)
            desc.Texture2D.PlaneSlice = 0;

            device_->CreateRenderTargetView(color_buffer_[i].Get(), &desc, handle);

            handle_rtv_[i] = handle;
            handle.ptr += inc_size;
        }
    }

    // Create depth stencil buffer
    {
        D3D12_HEAP_PROPERTIES prop{};
        prop.Type = D3D12_HEAP_TYPE_DEFAULT;
        prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
        prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
        prop.CreationNodeMask = 1;
        prop.VisibleNodeMask = 1;

        D3D12_RESOURCE_DESC desc{};
        desc.Dimension = D3D12_RESOURCE_DIMENSION_TEXTURE2D;
        desc.Alignment = 0;
        desc.Width = width_;
        desc.Height = height_;
        desc.DepthOrArraySize = 1;
        desc.MipLevels = 1;
        desc.Format = DXGI_FORMAT_D32_FLOAT; // 'D' of 'D32' means depth
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Layout = D3D12_TEXTURE_LAYOUT_UNKNOWN;
        desc.Flags = D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL;

        // The value set when the depth stencil buffer is cleared
        D3D12_CLEAR_VALUE clear_value;
        clear_value.Format = DXGI_FORMAT_D32_FLOAT;
        clear_value.DepthStencil.Depth = 1.0;
        clear_value.DepthStencil.Stencil = 0;

        // Create resource
        {
            auto const hr = device_->CreateCommittedResource(
                &prop,
                D3D12_HEAP_FLAG_NONE,
                &desc,
                D3D12_RESOURCE_STATE_DEPTH_WRITE,
                &clear_value,
                IID_PPV_ARGS(depth_buffer_.GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }
        }

        // Create descriptor heap
        {
            D3D12_DESCRIPTOR_HEAP_DESC desc{};
            desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_DSV;
            desc.NumDescriptors = 1;
            desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE; // Vertex shader refers the constant buffers
            desc.NodeMask = 0;

            auto const hr = device_->CreateDescriptorHeap(
                &desc,
                IID_PPV_ARGS(heap_dsv_.GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }
        }

        // Assign the CPU descriptor handle to the depth stencil view
        {
            auto const handle = heap_dsv_->GetCPUDescriptorHandleForHeapStart();

            D3D12_DEPTH_STENCIL_VIEW_DESC desc{};
            desc.Format = DXGI_FORMAT_D32_FLOAT;
            desc.ViewDimension = D3D12_DSV_DIMENSION_TEXTURE2D;
            desc.Texture2D.MipSlice = 0;
            desc.Flags = D3D12_DSV_FLAG_NONE;

            device_->CreateDepthStencilView(depth_buffer_.Get(), &desc, handle);
            handle_dsv_ = handle;
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
            IID_PPV_ARGS(fence_.GetAddressOf()));
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
    // Update state
    {
        rotate_angle_ += 0.025f;
        cbv_[NUM_INSTANCES * frame_index_ + 0].buffer->World = DirectX::XMMatrixRotationY(rotate_angle_);
    }

    // Clear command buffer
    cmd_alloc_[frame_index_]->Reset();
    cmd_list_->Reset(cmd_alloc_[frame_index_].Get(), /*pipeline state*/ nullptr);

    D3D12_RESOURCE_BARRIER barrier{};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    barrier.Flags = D3D12_RESOURCE_BARRIER_FLAG_NONE;
    barrier.Transition.pResource = color_buffer_[frame_index_].Get();
    barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    // Transition: Present -> Render (write)
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_PRESENT;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET;

    cmd_list_->ResourceBarrier(1, &barrier);

    cmd_list_->OMSetRenderTargets(1, &handle_rtv_[frame_index_], FALSE, &handle_dsv_);

    float const clear_color[] = {0.25f, 0.25f, 0.25f, 1.0f};

    cmd_list_->ClearRenderTargetView(handle_rtv_[frame_index_], clear_color, 0, /*target rect*/ nullptr);
    cmd_list_->ClearDepthStencilView(handle_dsv_, D3D12_CLEAR_FLAG_DEPTH, /*depth*/ 1.0f, /*stencil*/ 0, /*number of rects*/ 0, /*rects*/ nullptr);

    // Draw polygons
    {
        cmd_list_->SetGraphicsRootSignature(root_signature_.Get());
        cmd_list_->SetDescriptorHeaps(1, heap_cbv_srv_uav_.GetAddressOf());
        cmd_list_->SetGraphicsRootDescriptorTable(1, texture_.handle_gpu);
        cmd_list_->SetPipelineState(pipeline_state_.Get());

        cmd_list_->IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        cmd_list_->IASetVertexBuffers(/*slot*/ 0, /*number of buffers*/ 1, &vbv_);
        cmd_list_->IASetIndexBuffer(&ibv_);
        cmd_list_->RSSetViewports(1, &viewport_);
        cmd_list_->RSSetScissorRects(1, &scissor_);

        for (auto i = 0; i < NUM_INSTANCES; i++) {
            cmd_list_->SetGraphicsRootConstantBufferView(0, cbv_[NUM_INSTANCES * frame_index_ + i].desc.BufferLocation);
            cmd_list_->DrawIndexedInstanced(
                /*number of vertices per instance*/ 6,
                /*number of instances*/ 1,
                /*start index of indices*/ 0,
                /*start index of vertices*/ 0,
                /*start index of instances*/ 0);
        }
    }

    // Transition: Render (write) -> Present
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_PRESENT;

    cmd_list_->ResourceBarrier(1, &barrier);

    cmd_list_->Close();

    ID3D12CommandList *cmd_lists[] = {cmd_list_.Get()};
    queue_->ExecuteCommandLists(1, cmd_lists);

    present(1);
}

void App::present(uint32_t const interval) {
    // Render the front buffer to the screen and swap frame buffers
    // Passing `1` to `interval` means sync after the first vsync
    swap_chain_->Present(interval, 0);

    // Set the `counter` value to the fence when the commands in the command list are completed
    auto const counter = fence_counter_[frame_index_];
    queue_->Signal(fence_.Get(), counter);

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
    queue_->Signal(fence_.Get(), counter);
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

    fence_.Reset();
    heap_rtv_.Reset();
    for (auto i = 0; i < FRAME_COUNT; i++) {
        color_buffer_[i].Reset();
    }
    cmd_list_.Reset();
    for (auto i = 0; i < FRAME_COUNT; i++) {
        cmd_alloc_[i].Reset();
    }
    swap_chain_.Reset();
    queue_.Reset();
    device_.Reset();
}

bool App::on_init() {
    // Create the vertex buffer
    {
        DirectX::VertexPositionTexture vertices[] = {
            DirectX::VertexPositionTexture(DirectX::XMFLOAT3(-1.0f, 1.0f, 0.0f), DirectX::XMFLOAT2(0.0f, 0.0f)),
            DirectX::VertexPositionTexture(DirectX::XMFLOAT3(1.0f, 1.0f, 0.0f), DirectX::XMFLOAT2(1.0f, 0.0f)),
            DirectX::VertexPositionTexture(DirectX::XMFLOAT3(1.0f, -1.0f, 0.0f), DirectX::XMFLOAT2(1.0f, 1.0f)),
            DirectX::VertexPositionTexture(DirectX::XMFLOAT3(-1.0f, -1.0f, 0.0f), DirectX::XMFLOAT2(0.0f, 1.0f)),
        };

        D3D12_HEAP_PROPERTIES prop{};
        prop.Type = D3D12_HEAP_TYPE_UPLOAD; // Write once from CPU and read once from GPU
        prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
        prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
        prop.CreationNodeMask = 1; // Assume single GPU
        prop.VisibleNodeMask = 1;

        D3D12_RESOURCE_DESC desc{};
        desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
        desc.Alignment = 0;
        desc.Width = sizeof(vertices);
        desc.Height = 1;           // Fixed to 1
        desc.DepthOrArraySize = 1; // Fixed to 1
        desc.MipLevels = 1;        // Fixed to 1
        desc.Format = DXGI_FORMAT_UNKNOWN;
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
        desc.Flags = D3D12_RESOURCE_FLAG_NONE;

        // Create resource
        {
            auto const hr = device_->CreateCommittedResource(
                &prop,
                D3D12_HEAP_FLAG_NONE,
                &desc,
                D3D12_RESOURCE_STATE_GENERIC_READ, // For D3D12_HEAP_TYPE_UPLOAD
                nullptr,
                IID_PPV_ARGS(vb_.GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }
        }

        // Map vertex data
        {
            void *dest = nullptr;
            auto const hr = vb_->Map(/*first subresource*/ 0, /*entire buffer*/ nullptr, &dest);
            if (FAILED(hr)) {
                return false;
            }
            memcpy(dest, vertices, sizeof(vertices));
            vb_->Unmap(0, nullptr);
        }

        // Configure vertex buffer view
        vbv_.BufferLocation = vb_->GetGPUVirtualAddress();
        vbv_.SizeInBytes = static_cast<UINT>(sizeof(vertices));
        vbv_.StrideInBytes = static_cast<UINT>(sizeof(DirectX::VertexPositionTexture));
    }

    // Create the index buffer
    {
        // 0   1
        // ┌───┐
        // │   │
        // └───┘
        // 3   2
        uint32_t const indices[] = {0, 1, 2, 0, 2, 3};

        D3D12_HEAP_PROPERTIES prop{};
        prop.Type = D3D12_HEAP_TYPE_UPLOAD;
        prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
        prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
        prop.CreationNodeMask = 1;
        prop.VisibleNodeMask = 1;

        D3D12_RESOURCE_DESC desc{};
        desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
        desc.Alignment = 0;
        desc.Width = sizeof(indices);
        desc.Height = 1;
        desc.DepthOrArraySize = 1;
        desc.MipLevels = 1;
        desc.Format = DXGI_FORMAT_UNKNOWN;
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
        desc.Flags = D3D12_RESOURCE_FLAG_NONE;

        // Create resource
        {
            auto const hr = device_->CreateCommittedResource(
                &prop,
                D3D12_HEAP_FLAG_NONE,
                &desc,
                D3D12_RESOURCE_STATE_GENERIC_READ, // For D3D12_HEAP_TYPE_UPLOAD
                nullptr,
                IID_PPV_ARGS(ib_.GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }
        }

        // Map index data
        {
            void *dest = nullptr;
            auto const hr = ib_->Map(0, nullptr, &dest);
            if (FAILED(hr)) {
                return false;
            }
            memcpy(dest, indices, sizeof(indices));
            ib_->Unmap(0, nullptr);
        }

        ibv_.BufferLocation = ib_->GetGPUVirtualAddress();
        ibv_.Format = DXGI_FORMAT_R32_UINT;
        ibv_.SizeInBytes = sizeof(indices);
    }

    // Create descriptor heap for constant buffer and shader resource
    {
        D3D12_DESCRIPTOR_HEAP_DESC desc{};
        desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;
        desc.NumDescriptors = NUM_INSTANCES * FRAME_COUNT + 1;  // `+ 1` for shader resource (texture)
        desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE; // Vertex shader refers the constant buffers
        desc.NodeMask = 0;

        auto const hr = device_->CreateDescriptorHeap(
            &desc,
            IID_PPV_ARGS(heap_cbv_srv_uav_.GetAddressOf()));
        if (FAILED(hr)) {
            return false;
        }
    }

    // Create constant buffer for `Transform` constant
    {
        D3D12_HEAP_PROPERTIES prop{};
        prop.Type = D3D12_HEAP_TYPE_UPLOAD;
        prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
        prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
        prop.CreationNodeMask = 1;
        prop.VisibleNodeMask = 1;

        D3D12_RESOURCE_DESC desc{};
        desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
        desc.Alignment = 0;
        desc.Width = sizeof(Transform);
        desc.Height = 1;
        desc.DepthOrArraySize = 1;
        desc.MipLevels = 1;
        desc.Format = DXGI_FORMAT_UNKNOWN;
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
        desc.Flags = D3D12_RESOURCE_FLAG_NONE;

        auto const increment_size = device_->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);

        // We use double buffers for constants because constants are updated by CPU. While GPU is processing
        // the constants, CPU should not modify them. Ensure this constraint
        for (auto i = 0; i < NUM_INSTANCES * FRAME_COUNT; i++) {
            auto const hr = device_->CreateCommittedResource(
                &prop,
                D3D12_HEAP_FLAG_NONE,
                &desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                nullptr,
                IID_PPV_ARGS(cb_[i].GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }

            auto handle_cpu = heap_cbv_srv_uav_->GetCPUDescriptorHandleForHeapStart();
            auto handle_gpu = heap_cbv_srv_uav_->GetGPUDescriptorHandleForHeapStart();
            handle_cpu.ptr += increment_size * i;
            handle_gpu.ptr += increment_size * i;

            cbv_[i].handle_cpu = handle_cpu;
            cbv_[i].handle_gpu = handle_gpu;
            cbv_[i].desc.BufferLocation = cb_[i]->GetGPUVirtualAddress();
            cbv_[i].desc.SizeInBytes = sizeof(Transform);

            device_->CreateConstantBufferView(&cbv_[i].desc, handle_cpu);

            // Create mapping
            {
                auto const hr = cb_[i]->Map(0, nullptr, reinterpret_cast<void **>(&cbv_[i].buffer));
                if (FAILED(hr)) {
                    return false;
                }
            }

            auto const eye_pos = DirectX::XMVectorSet(0.0f, 0.0f, 5.0f, 0.0f);
            auto const target_pos = DirectX::XMVectorZero();
            auto const upward = DirectX::XMVectorSet(0.0f, 1.0f, 0.0f, 0.0f);
            auto const fov_y = DirectX::XMConvertToRadians(37.5f);
            auto const aspect = static_cast<float>(width_) / static_cast<float>(height_);

            // Set constant value
            cbv_[i].buffer->World = DirectX::XMMatrixIdentity();
            cbv_[i].buffer->View = DirectX::XMMatrixLookAtRH(eye_pos, target_pos, upward);
            cbv_[i].buffer->Proj = DirectX::XMMatrixPerspectiveFovRH(fov_y, aspect, 1.0f, 1000.0f);

            // Note: `Unmap` is called at `App::on_term`
        }
    }

    // Create root signature. Root signature is an object to determine layout of resources
    {
        D3D12_ROOT_PARAMETER params[2];

        // Parameter for constant buffer view
        params[0].ParameterType = D3D12_ROOT_PARAMETER_TYPE_CBV;
        params[0].Descriptor.ShaderRegister = 0;                     // Specify b0
        params[0].Descriptor.RegisterSpace = 0;                      // Unused
        params[0].ShaderVisibility = D3D12_SHADER_VISIBILITY_VERTEX; // Accessed by the vertex shader only

        // Parameter for textures. Textures are registered with descriptors table. In this case only one texture is used.
        D3D12_DESCRIPTOR_RANGE range{};
        range.RangeType = D3D12_DESCRIPTOR_RANGE_TYPE_SRV;
        range.NumDescriptors = 1;
        range.BaseShaderRegister = 0;
        range.RegisterSpace = 0;
        range.OffsetInDescriptorsFromTableStart = 0;
        params[1].ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
        params[1].DescriptorTable.NumDescriptorRanges = 1;
        params[1].DescriptorTable.pDescriptorRanges = &range;
        params[1].ShaderVisibility = D3D12_SHADER_VISIBILITY_PIXEL;

        // Static texture sampler that configures texture addressing mode, texture filtering, etc.
        D3D12_STATIC_SAMPLER_DESC sampler{};
        sampler.Filter = D3D12_FILTER_MIN_MAG_MIP_LINEAR;     // Filtering method
        sampler.AddressU = D3D12_TEXTURE_ADDRESS_MODE_CLAMP;  // Texture addressing mode for U coord
        sampler.AddressV = D3D12_TEXTURE_ADDRESS_MODE_CLAMP;  // Texture addressing mode for V coord
        sampler.AddressW = D3D12_TEXTURE_ADDRESS_MODE_CLAMP;  // Texture addressing mode for W coord
        sampler.MipLODBias = D3D12_DEFAULT_MIP_LOD_BIAS;      // Offset from the calculated mip map level
        sampler.MaxAnisotropy = 1;                            // Unused
        sampler.ComparisonFunc = D3D12_COMPARISON_FUNC_NEVER; // Unused
        sampler.BorderColor = D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK;
        sampler.MinLOD = 0;        // Minimum level of mip map
        sampler.RegisterSpace = 0; // Indicates s0
        sampler.ShaderVisibility = D3D12_SHADER_VISIBILITY_PIXEL;

        D3D12_ROOT_SIGNATURE_DESC desc{};
        desc.NumParameters = 2;
        desc.pParameters = params;
        desc.NumStaticSamplers = 1;
        desc.pStaticSamplers = &sampler;
        desc.Flags =
            // Only vertex/pixel shaders access the root signature
            D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT |
            D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS |
            D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS |
            D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS;

        ComPtr<ID3DBlob> blob = nullptr;
        ComPtr<ID3DBlob> error_blob = nullptr;

        {
            auto const hr = D3D12SerializeRootSignature(
                &desc,
                D3D_ROOT_SIGNATURE_VERSION_1_0,
                blob.GetAddressOf(),
                error_blob.GetAddressOf());
            if (FAILED(hr)) {
                return false;
            }
        }

        {
            auto const hr = device_->CreateRootSignature(
                /*nodeMask*/ 0, // Use single GPU
                blob->GetBufferPointer(),
                blob->GetBufferSize(),
                IID_PPV_ARGS(root_signature_.GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }
        }
    }

    // Create graphics pipeline state
    {
        D3D12_INPUT_ELEMENT_DESC elems[2];

        // Layout for POSITION vertex input
        elems[0].SemanticName = "POSITION";
        elems[0].SemanticIndex = 0;
        elems[0].Format = DXGI_FORMAT_R32G32B32_FLOAT;             // float3
        elems[0].InputSlot = 0;                                    // We use single vertex buffer
        elems[0].AlignedByteOffset = D3D12_APPEND_ALIGNED_ELEMENT; // Elements are contiguous and no padding between them
        elems[0].InputSlotClass = D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA;
        elems[0].InstanceDataStepRate = 0; // Fixed to 0 because of D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA

        // Layout for TEXCOORD vertex input
        elems[1].SemanticName = "TEXCOORD";
        elems[1].SemanticIndex = 0;
        elems[1].Format = DXGI_FORMAT_R32G32_FLOAT; // float2
        elems[1].InputSlot = 0;
        elems[1].AlignedByteOffset = D3D12_APPEND_ALIGNED_ELEMENT;
        elems[1].InputSlotClass = D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA;
        elems[1].InstanceDataStepRate = 0;

        D3D12_RASTERIZER_DESC rasterizer_state;
        rasterizer_state.FillMode = D3D12_FILL_MODE_SOLID;
        rasterizer_state.CullMode = D3D12_CULL_MODE_NONE;
        rasterizer_state.FrontCounterClockwise = FALSE;
        rasterizer_state.DepthBias = D3D12_DEFAULT_DEPTH_BIAS;
        rasterizer_state.DepthBiasClamp = D3D12_DEFAULT_DEPTH_BIAS_CLAMP;
        rasterizer_state.SlopeScaledDepthBias = D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS;
        rasterizer_state.DepthClipEnable = FALSE;
        rasterizer_state.MultisampleEnable = FALSE;
        rasterizer_state.AntialiasedLineEnable = FALSE;
        rasterizer_state.ForcedSampleCount = 0;
        rasterizer_state.ConservativeRaster = D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF;

        // Configuration of blending for render target. We don't use blending
        D3D12_RENDER_TARGET_BLEND_DESC rt_blend{
            /*blend enable*/ FALSE,
            /*logic op enable*/ FALSE,
            // Blend. Simply src overwrites dest (src * 1 + dest * 0)
            /*src*/ D3D12_BLEND_ONE, /*dest*/ D3D12_BLEND_ZERO, /*op*/ D3D12_BLEND_OP_ADD,
            // Blend alpha. Simply src overwrites dest (src * 1 + dest * 0)
            /*src*/ D3D12_BLEND_ONE, /*dest*/ D3D12_BLEND_ZERO, /*op*/ D3D12_BLEND_OP_ADD,
            /*logic op*/ D3D12_LOGIC_OP_NOOP,
            D3D12_COLOR_WRITE_ENABLE_ALL};

        // Configure blend state
        D3D12_BLEND_DESC blend_state;
        blend_state.AlphaToCoverageEnable = FALSE;
        blend_state.IndependentBlendEnable = FALSE;
        for (auto i = 0; i < D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT; i++) {
            blend_state.RenderTarget[i] = rt_blend;
        }

        D3D12_DEPTH_STENCIL_DESC depth_stencil_state{};
        depth_stencil_state.DepthEnable = TRUE;
        depth_stencil_state.DepthWriteMask = D3D12_DEPTH_WRITE_MASK_ALL;
        depth_stencil_state.DepthFunc = D3D12_COMPARISON_FUNC_LESS_EQUAL;
        depth_stencil_state.StencilEnable = FALSE;

        // Read shader files
        ComPtr<ID3DBlob> vs_blob = nullptr;
        {
            auto const hr = D3DReadFileToBlob(L"tex_vs.cso", vs_blob.GetAddressOf());
            if (FAILED(hr)) {
                return false;
            }
        }
        ComPtr<ID3DBlob> ps_blob = nullptr;
        {
            auto const hr = D3DReadFileToBlob(L"tex_ps.cso", ps_blob.GetAddressOf());
            if (FAILED(hr)) {
                return false;
            }
        }

        // Configure pipeline state
        {
            D3D12_GRAPHICS_PIPELINE_STATE_DESC desc{};
            desc.InputLayout = {elems, _countof(elems)};
            desc.pRootSignature = root_signature_.Get();
            desc.VS = {vs_blob->GetBufferPointer(), vs_blob->GetBufferSize()};
            desc.PS = {ps_blob->GetBufferPointer(), ps_blob->GetBufferSize()};
            desc.RasterizerState = rasterizer_state;
            desc.BlendState = blend_state;
            desc.DepthStencilState = depth_stencil_state;
            desc.SampleMask = UINT_MAX;
            desc.PrimitiveTopologyType = D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE;
            desc.NumRenderTargets = 1;
            desc.RTVFormats[0] = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB;
            desc.DSVFormat = DXGI_FORMAT_D32_FLOAT;
            desc.SampleDesc.Count = 1;
            desc.SampleDesc.Quality = 0;

            auto const hr = device_->CreateGraphicsPipelineState(
                &desc,
                IID_PPV_ARGS(pipeline_state_.GetAddressOf()));
            if (FAILED(hr)) {
                return false;
            }
        }
    }

    // Load texture. We create a single shader resource used by both frames because it is read-only.
    {
        // Create batch to upload the texture data to GPU
        DirectX::ResourceUploadBatch batch(device_.Get());
        batch.Begin();
        auto const hr = DirectX::CreateDDSTextureFromFile(
            device_.Get(),
            batch,
            L"ferris.dds",
            texture_.resource.GetAddressOf(),
            /* generate mipmap*/ true);
        if (FAILED(hr)) {
            return false;
        }
        batch.End(queue_.Get()).wait(); // `End` returns a future. Wait the command batch finishes

        auto const increment_size = device_->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);
        texture_.handle_cpu = heap_cbv_srv_uav_->GetCPUDescriptorHandleForHeapStart();
        texture_.handle_gpu = heap_cbv_srv_uav_->GetGPUDescriptorHandleForHeapStart();
        texture_.handle_cpu.ptr += increment_size * 2;
        texture_.handle_gpu.ptr += increment_size * 2;

        auto const tex_desc = texture_.resource->GetDesc();

        D3D12_SHADER_RESOURCE_VIEW_DESC desc{};
        desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2D;
        desc.Format = tex_desc.Format;
        desc.Shader4ComponentMapping = D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING; // RGBA
        desc.Texture2D.MostDetailedMip = 0;
        desc.Texture2D.MipLevels = tex_desc.MipLevels;
        desc.Texture2D.PlaneSlice = 0;
        desc.Texture2D.ResourceMinLODClamp = 0.0f;

        device_->CreateShaderResourceView(texture_.resource.Get(), &desc, texture_.handle_cpu);
    }

    // Configure viewport and scissor rect
    {
        viewport_.TopLeftX = 0;
        viewport_.TopLeftY = 0;
        viewport_.Width = static_cast<float>(width_);
        viewport_.Height = static_cast<float>(height_);
        viewport_.MinDepth = 0.0f;
        viewport_.MaxDepth = 1.0f;

        scissor_.top = 0;
        scissor_.right = width_;
        scissor_.bottom = height_;
        scissor_.left = 0;
    }

    return true;
}

void App::on_term() {
    for (auto i = 0; i < NUM_INSTANCES * FRAME_COUNT; i++) {
        if (cb_[i].Get() != nullptr) {
            cb_[i]->Unmap(0, nullptr);
            std::memset(&cbv_[i], 0, sizeof(cbv_[i]));
        }
        cb_[i].Reset();
    }
    vb_.Reset();
    pipeline_state_.Reset();
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

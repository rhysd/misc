#include "App.h"
#include <Windows.h>

namespace {
const auto ClassName = TEXT("Hello, D3D12");
}

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
    return init_window();
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
        }
    }
}

void App::term_app() {
    term_window();
}

void App::term_window() {
    if (hinst_ != nullptr) {
        UnregisterClass(ClassName, hinst_);
    }
    hinst_ = nullptr;
    hwnd_ = nullptr;
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

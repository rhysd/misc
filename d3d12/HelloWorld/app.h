#pragma once
#pragma comment(lib, "d3d12.lib")
#pragma comment(lib, "dxgi.lib")

#include <Windows.h>
#include <cstdint>
#include <d3d12.h>
#include <dxgi1_4.h>

class App {
  public:
    App(uint32_t const width, uint32_t const height);
    ~App();
    void run();

  private:
    bool init_app();
    void term_app();
    bool init_window();
    void term_window();
    void main_loop();
    bool init_d3d();
    void term_d3d();
    void render();
    void wait_gpu();
    void present(uint32_t const interval);

    static LRESULT CALLBACK window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp);
    static const uint32_t FRAME_COUNT = 2; // Number of frame buffers

    HINSTANCE hinst_;
    HWND hwnd_;
    uint32_t width_;
    uint32_t height_;
    ID3D12Device *device_;        // Physical device
    ID3D12CommandQueue *queue_;   // Command queue to submit draw commands to GPU
    IDXGISwapChain3 *swap_chain_; // Swap frame buffers (double buffer)
    ID3D12Resource *color_buffer_[FRAME_COUNT];
    ID3D12CommandAllocator *cmd_alloc_[FRAME_COUNT];
    ID3D12GraphicsCommandList *cmd_list_;
    ID3D12DescriptorHeap *heap_rtv_;
    ID3D12Fence *fence_; // Fence between CPU and GPU
    HANDLE fence_event_;
    uint64_t fence_counter_[FRAME_COUNT];
    uint32_t frame_index_;
    D3D12_CPU_DESCRIPTOR_HANDLE handle_rtv_[FRAME_COUNT];
};

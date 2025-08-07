#pragma once
#pragma comment(lib, "d3d12.lib")
#pragma comment(lib, "dxgi.lib")
#pragma comment(lib, "d3dcompiler.lib")

#include <DirectXMath.h>
#include <Windows.h>
#include <cstdint>
#include <d3d12.h>
#include <d3dcompiler.h>
#include <dxgi1_4.h>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

// D3D requires 256 bytes alignment to constant buffers
struct alignas(256) Transform {
    DirectX::XMMATRIX World;
    DirectX::XMMATRIX View;
    DirectX::XMMATRIX Proj;
};

template <class T>
struct ConstantBufferView {
    D3D12_CONSTANT_BUFFER_VIEW_DESC desc;
    D3D12_CPU_DESCRIPTOR_HANDLE handle_cpu;
    D3D12_GPU_DESCRIPTOR_HANDLE handle_gpu;
    T *buffer;
};

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
    bool on_init();
    void on_term();

    static LRESULT CALLBACK window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp);
    static const uint32_t FRAME_COUNT = 2;   // Number of frame buffers
    static const uint32_t NUM_INSTANCES = 2; // Render two instances (= two draw calls)

    HINSTANCE hinst_;
    HWND hwnd_;
    uint32_t width_;
    uint32_t height_;
    ComPtr<ID3D12Device> device_;        // Physical device
    ComPtr<ID3D12CommandQueue> queue_;   // Command queue to submit draw commands to GPU
    ComPtr<IDXGISwapChain3> swap_chain_; // Swap frame buffers (double buffer)
    ComPtr<ID3D12Resource> color_buffer_[FRAME_COUNT];
    ComPtr<ID3D12Resource> depth_buffer_;
    ComPtr<ID3D12CommandAllocator> cmd_alloc_[FRAME_COUNT];
    ComPtr<ID3D12GraphicsCommandList> cmd_list_;
    ComPtr<ID3D12DescriptorHeap> heap_rtv_;                  // Heap descriptor for render target view
    ComPtr<ID3D12Fence> fence_;                              // Fence between CPU and GPU
    ComPtr<ID3D12DescriptorHeap> heap_cbv_;                  // Heap descriptor for constant buffer view
    ComPtr<ID3D12DescriptorHeap> heap_dsv_;                  // Heap descriptor for depth stencil buffer
    ComPtr<ID3D12Resource> vb_;                              // Vertex buffer
    ComPtr<ID3D12Resource> ib_;                              // Index buffer
    ComPtr<ID3D12Resource> cb_[NUM_INSTANCES * FRAME_COUNT]; // Constant buffers
    ComPtr<ID3D12RootSignature> root_signature_;
    ComPtr<ID3D12PipelineState> pipeline_state_;
    HANDLE fence_event_;
    uint64_t fence_counter_[FRAME_COUNT];
    uint32_t frame_index_;
    D3D12_CPU_DESCRIPTOR_HANDLE handle_rtv_[FRAME_COUNT];
    D3D12_CPU_DESCRIPTOR_HANDLE handle_dsv_; // We need only single depth stencil buffer because the buffer is only accessed by GPU. We don't need double buffers
    D3D12_VERTEX_BUFFER_VIEW vbv_;
    D3D12_INDEX_BUFFER_VIEW ibv_;
    D3D12_VIEWPORT viewport_;
    D3D12_RECT scissor_;
    ConstantBufferView<Transform> cbv_[NUM_INSTANCES * FRAME_COUNT]; // View of constant buffer for World-View-Projection transform
    float rotate_angle_;
};

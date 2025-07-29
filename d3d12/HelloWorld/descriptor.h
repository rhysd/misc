#pragma once

#include "pool.h"
#include <d3d12.h>
#include <optional>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

struct Descriptor {
    D3D12_CPU_DESCRIPTOR_HANDLE handle_cpu;
    D3D12_GPU_DESCRIPTOR_HANDLE handle_gpu;

    bool has_cpu() const {
        return handle_cpu.ptr != 0;
    }
    bool has_gpu() const {
        return handle_gpu.ptr != 0;
    }
};

class DescriptorPool final {
    Pool<Descriptor> pool_;
    ComPtr<ID3D12DescriptorHeap> heap_;
    uint32_t descriptor_size_;

    DescriptorPool(size_t size) : pool_(size), heap_(nullptr) {}

  public:
    DescriptorPool(DescriptorPool &&other) = default;
    DescriptorPool(DescriptorPool &other) = delete;
    DescriptorPool &operator=(DescriptorPool &other) = delete;

    static std::optional<DescriptorPool> create(
        ID3D12Device *device,
        D3D12_DESCRIPTOR_HEAP_DESC const *desc);

    Descriptor *alloc();
    void dealloc(Descriptor *);
};

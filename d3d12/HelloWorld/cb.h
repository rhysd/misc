#pragma once

#include "descriptor.h"
#include <d3d12.h>
#include <memory>
#include <optional>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class ConstantBuffer final {
    ComPtr<ID3D12Resource> cb_;
    Descriptor *handle_;
    std::shared_ptr<DescriptorPool> pool_;
    D3D12_CONSTANT_BUFFER_VIEW_DESC desc_;
    void *mapped_;

    explicit ConstantBuffer(std::shared_ptr<DescriptorPool> pool) : cb_(nullptr), handle_(nullptr), pool_(pool), mapped_(nullptr) {}

  public:
    static std::optional<ConstantBuffer> create(
        ID3D12Device *device,
        std::shared_ptr<DescriptorPool> pool,
        size_t size);
    D3D12_CPU_DESCRIPTOR_HANDLE get_handle_cpu() const;
    D3D12_GPU_DESCRIPTOR_HANDLE get_handle_gpu() const;
    D3D12_GPU_VIRTUAL_ADDRESS get_address() const;
    void *get_mapped_ptr() const;
    ~ConstantBuffer();
    ConstantBuffer(ConstantBuffer &&other) = default;
    ConstantBuffer(ConstantBuffer &other) = delete;
    ConstantBuffer &operator=(ConstantBuffer &other) = delete;
};

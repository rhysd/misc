#pragma once

#include "descriptor.h"
#include <ResourceUploadBatch.h>
#include <d3d12.h>
#include <memory>
#include <optional>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class Texture final {
    ComPtr<ID3D12Resource> tex_;
    Descriptor *handle_;
    std::shared_ptr<DescriptorPool> pool_;

    explicit Texture(std::shared_ptr<DescriptorPool> pool) : tex_(nullptr), handle_(nullptr), pool_(pool) {}
    D3D12_SHADER_RESOURCE_VIEW_DESC view_desc(bool const is_cube);

  public:
    static std::optional<Texture> create(
        ID3D12Device *device,
        std::shared_ptr<DescriptorPool> pool,
        wchar_t const *filepath,
        DirectX::ResourceUploadBatch &batch);
    D3D12_CPU_DESCRIPTOR_HANDLE get_handle_cpu() const;
    D3D12_GPU_DESCRIPTOR_HANDLE get_handle_gpu() const;
    Texture(Texture &&other) = default;
    Texture &operator=(Texture &&) = default;
    Texture(Texture const &) = delete;
    Texture &operator=(Texture &) = delete;
    ~Texture();
};

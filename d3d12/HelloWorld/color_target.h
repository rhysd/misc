#pragma once

#include "descriptor.h"
#include <d3d12.h>
#include <dxgi1_6.h>
#include <memory>
#include <optional>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class ColorTarget final {
    ComPtr<ID3D12Resource> res_;
    Descriptor *handle_rtv_;
    std::shared_ptr<DescriptorPool> pool_;
    D3D12_RENDER_TARGET_VIEW_DESC view_desc_;

    ColorTarget(std::shared_ptr<DescriptorPool> pool, DXGI_FORMAT const format);

  public:
    static std::optional<ColorTarget> create(
        ID3D12Device *device,
        std::shared_ptr<DescriptorPool> pool_rtv,
        uint32_t const width,
        uint32_t const height,
        DXGI_FORMAT format);
    static std::optional<ColorTarget> create_from_back_buffer(
        ID3D12Device *device,
        std::shared_ptr<DescriptorPool> pool_rtv,
        uint32_t const index,
        IDXGISwapChain *swap_chain);
    ~ColorTarget();
    ColorTarget(ColorTarget &&other) = default;
    ColorTarget &operator=(ColorTarget &&other) = default;
    ColorTarget(ColorTarget &other) = delete;
    ColorTarget &operator=(ColorTarget &other) = delete;
    Descriptor *handle_rtv() const;
    ID3D12Resource *resource() const;
    D3D12_RESOURCE_DESC resource_desc() const;
    D3D12_RENDER_TARGET_VIEW_DESC view_desc() const;
};

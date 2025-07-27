#pragma once

#include "descriptor.h"
#include <d3d12.h>
#include <memory>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class DepthTarget {
    ComPtr<ID3D12Resource> res_;
    Descriptor *handle_dsv_;
    std::shared_ptr<DescriptorPool> pool_;
    D3D12_DEPTH_STENCIL_VIEW_DESC view_desc_;

    explicit DepthTarget(std::shared_ptr<DescriptorPool> pool) : res_(nullptr), handle_dsv_(nullptr), pool_(pool), view_desc_() {}

  public:
    static std::optional<DepthTarget> create(
        ID3D12Device *device,
        std::shared_ptr<DescriptorPool> pool_dsv,
        uint32_t const width,
        uint32_t const height,
        DXGI_FORMAT format);
    ~DepthTarget();
    Descriptor *handle_dsv() const;
    ID3D12Resource *resource() const;
    D3D12_RESOURCE_DESC resource_desc() const;
    D3D12_DEPTH_STENCIL_VIEW_DESC view_desc() const;
};

#include "depth_target.h"
#include <cassert>

std::optional<DepthTarget> DepthTarget::create(ID3D12Device *device, std::shared_ptr<DescriptorPool> pool_dsv, uint32_t const width, uint32_t const height, DXGI_FORMAT format) {
    assert(device != nullptr && pool_dsv != nullptr && width > 0 && height > 0);

    DepthTarget target(std::move(pool_dsv));

    target.handle_dsv_ = target.pool_->alloc();
    if (target.handle_dsv_ == nullptr) {
        return std::nullopt;
    }

    D3D12_HEAP_PROPERTIES prop{};
    prop.Type = D3D12_HEAP_TYPE_DEFAULT;
    prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
    prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
    prop.CreationNodeMask = 1;
    prop.VisibleNodeMask = 1;

    D3D12_RESOURCE_DESC desc{};
    desc.Dimension = D3D12_RESOURCE_DIMENSION_TEXTURE2D;
    desc.Alignment = 0;
    desc.Width = UINT64(width);
    desc.Height = height;
    desc.DepthOrArraySize = 1;
    desc.MipLevels = 1;
    desc.Format = format;
    desc.SampleDesc.Count = 1;
    desc.SampleDesc.Quality = 0;
    desc.Layout = D3D12_TEXTURE_LAYOUT_UNKNOWN;
    desc.Flags = D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL;

    // The value set when the depth stencil buffer is cleared
    D3D12_CLEAR_VALUE clear_value;
    clear_value.Format = format;
    clear_value.DepthStencil.Depth = 1.0f;
    clear_value.DepthStencil.Stencil = 0;

    auto const hr = device->CreateCommittedResource(
        &prop,
        D3D12_HEAP_FLAG_NONE,
        &desc,
        D3D12_RESOURCE_STATE_DEPTH_WRITE,
        &clear_value,
        IID_PPV_ARGS(target.res_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    target.view_desc_.ViewDimension = D3D12_DSV_DIMENSION_TEXTURE2D;
    target.view_desc_.Format = format;
    target.view_desc_.Texture2D.MipSlice = 0;
    target.view_desc_.Flags = D3D12_DSV_FLAG_NONE;

    device->CreateDepthStencilView(
        target.res_.Get(),
        &target.view_desc_,
        target.handle_dsv_->handle_cpu);

    return target;
}

DepthTarget::~DepthTarget() {
    res_.Reset();
    if (pool_ != nullptr) {
        pool_->dealloc(handle_dsv_);
        pool_.reset();
    }
}

Descriptor *DepthTarget::handle_dsv() const {
    return handle_dsv_;
}

ID3D12Resource *DepthTarget::resource() const {
    return res_.Get();
}

D3D12_RESOURCE_DESC DepthTarget::resource_desc() const {
    if (res_ == nullptr) {
        return D3D12_RESOURCE_DESC();
    }
    return res_->GetDesc();
}

D3D12_DEPTH_STENCIL_VIEW_DESC DepthTarget::view_desc() const {
    return view_desc_;
}

#include "color_target.h"
#include <cassert>

ColorTarget::ColorTarget(std::shared_ptr<DescriptorPool> pool, DXGI_FORMAT const format)
    : res_(nullptr),
      handle_rtv_(nullptr),
      pool_(pool),
      view_desc_() {
    view_desc_.ViewDimension = D3D12_RTV_DIMENSION_TEXTURE2D;
    view_desc_.Format = format;
    view_desc_.Texture2D.MipSlice = 0; // Mipmap level (0 = one mipmap)
    view_desc_.Texture2D.PlaneSlice = 0;
}

std::optional<ColorTarget> ColorTarget::create(ID3D12Device *device, std::shared_ptr<DescriptorPool> pool_rtv, uint32_t const width, uint32_t const height, DXGI_FORMAT format) {
    assert(device != nullptr && pool_rtv != nullptr && width > 0 && height > 0);

    ColorTarget target(std::move(pool_rtv), format);

    target.handle_rtv_ = target.pool_->alloc();
    if (target.handle_rtv_ == nullptr) {
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
    desc.Flags = D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET;

    // The value set when the depth stencil buffer is cleared
    D3D12_CLEAR_VALUE clear_value;
    clear_value.Format = format;
    for (int i = 0; i < 4; i++) {
        clear_value.Color[i] = 1.0f;
    }

    auto const hr = device->CreateCommittedResource(
        &prop,
        D3D12_HEAP_FLAG_NONE,
        &desc,
        D3D12_RESOURCE_STATE_RENDER_TARGET,
        &clear_value,
        IID_PPV_ARGS(target.res_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    device->CreateRenderTargetView(
        target.res_.Get(),
        &target.view_desc_,
        target.handle_rtv_->handle_cpu);

    return target;
}

std::optional<ColorTarget> ColorTarget::create_from_back_buffer(ID3D12Device *device, std::shared_ptr<DescriptorPool> pool_rtv, uint32_t const index, IDXGISwapChain *swap_chain) {
    assert(device != nullptr && pool_rtv != nullptr && swap_chain != nullptr);

    DXGI_SWAP_CHAIN_DESC desc;
    auto hr = swap_chain->GetDesc(&desc);
    if (FAILED(hr)) {
        return std::nullopt;
    }

    ColorTarget target(std::move(pool_rtv), desc.BufferDesc.Format);

    target.handle_rtv_ = target.pool_->alloc();
    if (target.handle_rtv_ == nullptr) {
        return std::nullopt;
    }

    hr = swap_chain->GetBuffer(index, IID_PPV_ARGS(target.res_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    device->CreateRenderTargetView(
        target.res_.Get(),
        &target.view_desc_,
        target.handle_rtv_->handle_cpu);

    return target;
}

ColorTarget::~ColorTarget() {
    res_.Reset();
    if (pool_ != nullptr) {
        pool_->dealloc(handle_rtv_);
        pool_.reset();
    }
}

Descriptor *ColorTarget::handle_rtv() const {
    return handle_rtv_;
}

ID3D12Resource *ColorTarget::resource() const {
    return res_.Get();
}

D3D12_RESOURCE_DESC ColorTarget::resource_desc() const {
    if (res_ == nullptr) {
        return D3D12_RESOURCE_DESC();
    }
    return res_->GetDesc();
}

D3D12_RENDER_TARGET_VIEW_DESC ColorTarget::view_desc() const {
    return view_desc_;
}

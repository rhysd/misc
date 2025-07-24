#include "ib.h"

std::optional<IndexBuffer> IndexBuffer::create(ID3D12Device *device, size_t const size, uint32_t const *init) {
    auto const bytes = sizeof(uint32_t) * size;
    IndexBuffer ib;

    D3D12_HEAP_PROPERTIES prop{};
    prop.Type = D3D12_HEAP_TYPE_UPLOAD;
    prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
    prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
    prop.CreationNodeMask = 1;
    prop.VisibleNodeMask = 1;

    D3D12_RESOURCE_DESC desc{};
    desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    desc.Alignment = 0;
    desc.Width = bytes;
    desc.Height = 1;
    desc.DepthOrArraySize = 1;
    desc.MipLevels = 1;
    desc.Format = DXGI_FORMAT_UNKNOWN;
    desc.SampleDesc.Count = 1;
    desc.SampleDesc.Quality = 0;
    desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
    desc.Flags = D3D12_RESOURCE_FLAG_NONE;

    auto const hr = device->CreateCommittedResource(
        &prop,
        D3D12_HEAP_FLAG_NONE,
        &desc,
        D3D12_RESOURCE_STATE_GENERIC_READ, // For D3D12_HEAP_TYPE_UPLOAD
        nullptr,
        IID_PPV_ARGS(ib.res_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    ib.view_.BufferLocation = ib.res_->GetGPUVirtualAddress();
    ib.view_.Format = DXGI_FORMAT_R32_UINT;
    ib.view_.SizeInBytes = static_cast<UINT>(bytes);

    if (init != nullptr) {
        auto const ptr = ib.map();
        if (ptr == nullptr) {
            return std::nullopt;
        }
        memcpy(ptr, init, size);
        ib.unmap();
    }

    return ib;
}

IndexBuffer::~IndexBuffer() {
    res_.Reset();
    memset(&view_, 0, sizeof(view_));
}

uint32_t *IndexBuffer::map() {
    void *dest;
    auto const hr = res_->Map(0, nullptr, &dest);
    return FAILED(hr) ? nullptr : reinterpret_cast<uint32_t *>(dest);
}

void IndexBuffer::unmap() {
    res_->Unmap(0, nullptr);
}

D3D12_INDEX_BUFFER_VIEW IndexBuffer::view() const {
    return view_;
}

#include "vb.h"
#include <cassert>

std::optional<VertexBuffer> VertexBuffer::create_impl(ID3D12Device *device, size_t const bytes, size_t const stride, void const *init) {
    assert(device != nullptr && bytes > 0);

    VertexBuffer vb;

    D3D12_HEAP_PROPERTIES prop{};
    prop.Type = D3D12_HEAP_TYPE_UPLOAD; // Write once from CPU and read once from GPU
    prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
    prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
    prop.CreationNodeMask = 1; // Assume single GPU
    prop.VisibleNodeMask = 1;

    D3D12_RESOURCE_DESC desc{};
    desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    desc.Alignment = 0;
    desc.Width = bytes;
    desc.Height = 1;           // Fixed to 1
    desc.DepthOrArraySize = 1; // Fixed to 1
    desc.MipLevels = 1;        // Fixed to 1
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
        IID_PPV_ARGS(vb.res_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    // Configure vertex buffer view
    vb.view_.BufferLocation = vb.res_->GetGPUVirtualAddress();
    vb.view_.SizeInBytes = static_cast<UINT>(bytes);
    vb.view_.StrideInBytes = static_cast<UINT>(stride);

    // Write the initial data to the buffer
    if (init != nullptr) {
        void *ptr = vb.map_impl();
        if (ptr == nullptr) {
            return std::nullopt;
        }
        memcpy(ptr, init, bytes);
        vb.unmap();
    }

    return vb;
}

VertexBuffer::~VertexBuffer() {
    res_.Reset();
    memset(&view_, 0, sizeof(view_));
}

void *VertexBuffer::map_impl() {
    void *ptr;
    auto const hr = res_->Map(0, nullptr, &ptr);
    return FAILED(hr) ? nullptr : ptr;
}

void VertexBuffer::unmap() {
    res_->Unmap(0, nullptr);
}

D3D12_VERTEX_BUFFER_VIEW VertexBuffer::view() const {
    return view_;
}

#include "constantbuffer.h"
#include <cassert>

std::optional<ConstantBuffer> ConstantBuffer::create(
    ID3D12Device *device,
    std::shared_ptr<DescriptorPool> pool,
    size_t size) {
    assert(device != nullptr && pool != nullptr && size > 0);

    ConstantBuffer ret(std::move(pool));

    // Adjust size to fit to the alignment
    size_t const align = D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT;
    size = (size + (align - 1)) & ~(align - 1);

    D3D12_HEAP_PROPERTIES prop{};
    prop.Type = D3D12_HEAP_TYPE_UPLOAD;
    prop.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
    prop.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
    prop.CreationNodeMask = 1;
    prop.VisibleNodeMask = 1;

    D3D12_RESOURCE_DESC desc{};
    desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    desc.Alignment = 0;
    desc.Width = size;
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
        D3D12_RESOURCE_STATE_GENERIC_READ,
        nullptr,
        IID_PPV_ARGS(ret.cb_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    {
        auto const hr = ret.cb_->Map(0, nullptr, &ret.mapped_);
        if (FAILED(hr)) {
            return std::nullopt;
        }
    }

    ret.desc_.BufferLocation = ret.cb_->GetGPUVirtualAddress();
    ret.desc_.SizeInBytes = UINT(size);
    ret.handle_ = ret.pool_->alloc();

    device->CreateConstantBufferView(&ret.desc_, ret.handle_->handle_cpu);

    return ret;
}

ConstantBuffer::~ConstantBuffer() {
    if (cb_ != nullptr) {
        cb_->Unmap(0, nullptr);
        cb_.Reset();
    }
    if (pool_ != nullptr) {
        pool_->dealloc(handle_);
        handle_ = nullptr;
        pool_.reset();
    }
    mapped_ = nullptr;
}

D3D12_CPU_DESCRIPTOR_HANDLE ConstantBuffer::get_handle_cpu() const {
    if (handle_ != nullptr) {
        return handle_->handle_cpu;
    }
    return D3D12_CPU_DESCRIPTOR_HANDLE();
}

D3D12_GPU_DESCRIPTOR_HANDLE ConstantBuffer::get_handle_gpu() const {
    if (handle_ != nullptr) {
        return handle_->handle_gpu;
    }
    return D3D12_GPU_DESCRIPTOR_HANDLE();
}

D3D12_GPU_VIRTUAL_ADDRESS ConstantBuffer::get_address() const {
    return desc_.BufferLocation;
}

void *ConstantBuffer::get_mapped_ptr() const {
    return mapped_;
}

#include "descriptor.h"
#include <cassert>

std::optional<DescriptorPool> DescriptorPool::create(ID3D12Device *device, D3D12_DESCRIPTOR_HEAP_DESC const *desc) {
    assert(device != nullptr && desc != nullptr);

    DescriptorPool self(desc->NumDescriptors);
    auto const hr = device->CreateDescriptorHeap(desc, IID_PPV_ARGS(self.heap_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    self.descriptor_size_ = device->GetDescriptorHandleIncrementSize(desc->Type);

    return {std::move(self)};
}

Descriptor *DescriptorPool::alloc() {
    return pool_.alloc_with([this](Descriptor *descriptor, size_t index) {
        descriptor->handle_cpu = heap_->GetCPUDescriptorHandleForHeapStart();
        descriptor->handle_cpu.ptr += descriptor_size_ * index;

        descriptor->handle_gpu = heap_->GetGPUDescriptorHandleForHeapStart();
        descriptor->handle_gpu.ptr += descriptor_size_ * index;
    });
}

void DescriptorPool::dealloc(Descriptor *const ptr) {
    assert(ptr != nullptr);
    pool_.dealloc(ptr);
}

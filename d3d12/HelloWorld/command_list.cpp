#include "command_list.h"
#include <cassert>

ID3D12CommandAllocator *CommandList::current_alloc() {
    return allocs_[index_].Get();
}

std::optional<CommandList> CommandList::create(ID3D12Device *device, D3D12_COMMAND_LIST_TYPE const type, uint32_t const count) {
    assert(device != nullptr && count > 0);

    CommandList cl;

    for (uint32_t i = 0; i < count; i++) {
        ComPtr<ID3D12CommandAllocator> alloc;
        auto const hr = device->CreateCommandAllocator(type, IID_PPV_ARGS(alloc.GetAddressOf()));
        if (FAILED(hr)) {
            return std::nullopt;
        }
        cl.allocs_.push_back(alloc);
    }

    auto hr = device->CreateCommandList(
        /*node mask*/ 1,
        type,
        cl.current_alloc(),
        /*initial state*/ nullptr,
        IID_PPV_ARGS(cl.cmd_list_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }
    hr = cl.cmd_list_->Close();
    if (FAILED(hr)) {
        return std::nullopt;
    }

    return cl;
}

ID3D12GraphicsCommandList *CommandList::start() {
    auto hr = current_alloc()->Reset();
    if (FAILED(hr)) {
        return nullptr;
    }

    hr = cmd_list_->Reset(current_alloc(), /*pipeline state*/ nullptr);
    if (FAILED(hr)) {
        return nullptr;
    }

    index_ = (index_ + 1) % static_cast<uint32_t>(allocs_.size());
    return cmd_list_.Get();
}

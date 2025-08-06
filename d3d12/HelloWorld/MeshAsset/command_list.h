#pragma once

#include <d3d12.h>
#include <optional>
#include <vector>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class CommandList {
    ComPtr<ID3D12GraphicsCommandList> cmd_list_;
    std::vector<ComPtr<ID3D12CommandAllocator>> allocs_;
    uint32_t index_;

    CommandList() : cmd_list_(nullptr), allocs_(), index_(0) {}
    ID3D12CommandAllocator *current_alloc();

  public:
    static std::optional<CommandList> create(
        ID3D12Device *device,
        D3D12_COMMAND_LIST_TYPE const type,
        uint32_t const count);
    CommandList(CommandList &&other) = default;
    CommandList &operator=(CommandList &&other) = default;
    CommandList(CommandList &other) = delete;
    CommandList &operator=(CommandList &other) = delete;
    ID3D12GraphicsCommandList *start();
};

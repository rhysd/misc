#pragma once

#include <Windows.h>
#include <d3d12.h>
#include <optional>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class Fence final {
    ComPtr<ID3D12Fence> fence_;
    HANDLE event_;
    UINT counter_;

    Fence() : fence_(nullptr), event_(), counter_(0) {}

  public:
    static std::optional<Fence> create(ID3D12Device *device);
    ~Fence();
    Fence(Fence &&other) = default;
    Fence(Fence &other) = delete;
    Fence &operator=(Fence &other) = delete;
    bool wait(ID3D12CommandQueue *queue, UINT const timeout = INFINITE);
    bool sync(ID3D12CommandQueue *queue);
};

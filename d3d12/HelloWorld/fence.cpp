#include "fence.h"
#include <cassert>
#include <synchapi.h>

std::optional<Fence> Fence::create(ID3D12Device *device) {
    assert(device != nullptr);

    Fence fence;

    fence.event_ = CreateEvent(
        /* attributes */ nullptr,
        /* manual reset */ FALSE,
        /* initial state */ FALSE,
        /* name */ nullptr);
    if (fence.event_ == nullptr) {
        return std::nullopt;
    }

    auto const hr = device->CreateFence(
        fence.counter_,
        D3D12_FENCE_FLAG_NONE,
        IID_PPV_ARGS(fence.fence_.GetAddressOf()));
    if (FAILED(hr)) {
        return std::nullopt;
    }

    fence.counter_++;

    return fence;
}

Fence::~Fence() {
    if (event_ != nullptr) {
        CloseHandle(event_);
        event_ = nullptr;
    }
    fence_.Reset();
    counter_ = 0;
}

bool Fence::wait(ID3D12CommandQueue *queue, UINT const timeout) {
    assert(queue != nullptr);

    auto const value = counter_;
    auto const hr = queue->Signal(fence_.Get(), value);
    if (FAILED(hr)) {
        return false;
    }

    counter_++;

    // Wait until the next frame is prepared (= the fence value is set to `value`)
    if (fence_->GetCompletedValue() < value) {
        auto const hr = fence_->SetEventOnCompletion(value, event_);
        if (FAILED(hr)) {
            return false;
        }
        if (WaitForSingleObjectEx(event_, timeout, FALSE) != WAIT_OBJECT_0) {
            return false;
        }
    }

    return true;
}

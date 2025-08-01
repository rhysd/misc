#pragma once

#include <d3d12.h>
#include <optional>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class IndexBuffer final {
    ComPtr<ID3D12Resource> res_;
    D3D12_INDEX_BUFFER_VIEW view_;

    IndexBuffer() : res_(nullptr) {
        memset(&view_, 0, sizeof(view_));
    }

  public:
    static std::optional<IndexBuffer> create(
        ID3D12Device *device,
        size_t const size,
        uint32_t const *init = nullptr);
    ~IndexBuffer();
    IndexBuffer(IndexBuffer &&other) = default;
    IndexBuffer &operator=(IndexBuffer &&other) = default;
    IndexBuffer(IndexBuffer &other) = delete;
    IndexBuffer &operator=(IndexBuffer &other) = delete;
    uint32_t *map();
    void unmap();
    D3D12_INDEX_BUFFER_VIEW view() const;
};

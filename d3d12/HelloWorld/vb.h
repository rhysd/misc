#pragma once

#include <d3d12.h>
#include <optional>
#include <wrl/client.h>

using Microsoft::WRL::ComPtr;

class VertexBuffer final {
    ComPtr<ID3D12Resource> res_;
    D3D12_VERTEX_BUFFER_VIEW view_;

    VertexBuffer() : res_(nullptr) {
        memset(&view_, 0, sizeof(view_));
    }
    void *map_impl();

    static std::optional<VertexBuffer> create_impl(
        ID3D12Device *device,
        size_t const bytes,
        size_t const stride,
        void const *init);

  public:
    template <class T>
    static std::optional<VertexBuffer> create(
        ID3D12Device *device,
        size_t const size,
        T const *init = nullptr) {
        return VertexBuffer::create_impl(
            device,
            sizeof(T) * size,
            sizeof(T),
            reinterpret_cast<void const *>(init));
    }
    VertexBuffer(VertexBuffer &&other) = default;
    VertexBuffer &operator=(VertexBuffer &&other) = default;
    VertexBuffer(VertexBuffer &other) = delete;
    VertexBuffer &operator=(VertexBuffer &other) = delete;
    ~VertexBuffer();
    template <class T>
    T *map() {
        return reinterpret_cast<T *>(map_impl());
    }
    void unmap();
    D3D12_VERTEX_BUFFER_VIEW view() const;
};

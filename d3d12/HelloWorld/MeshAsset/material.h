#pragma once

#include "cb.h"
#include "descriptor.h"
#include "texture.h"
#include <ResourceUploadBatch.h>
#include <d3d12.h>
#include <memory>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

class Material final {
  public:
    enum class TextureUsage : uint8_t {
        Diffuse = 0,
        Specular = 1,
        Shiness = 2,
        Normal = 3,
    };

    static std::optional<Material> create(ID3D12Device *device, std::shared_ptr<DescriptorPool> pool, size_t const buf_size, size_t const count);
    Material(Material &&other) = default;
    Material &operator=(Material &&other) = default;
    Material(Material &other) = delete;
    Material &operator=(Material &other) = delete;
    bool set_texture_at(size_t const index, TextureUsage const usage, std::wstring const &path, DirectX::ResourceUploadBatch &batch);
    size_t count() const;
    template <class T>
    T *buffer_at(size_t const index) const {
        return reinterpret_cast<T *>(buffer_at_impl(index));
    }
    D3D12_GPU_VIRTUAL_ADDRESS buffer_address_at(size_t const index) const;
    D3D12_GPU_DESCRIPTOR_HANDLE texture_handle_at(size_t const index, TextureUsage const usage) const;

  private:
    static size_t const NUM_USAGES = 4;

    struct TextureResource {
        std::optional<ConstantBuffer> cb;
        D3D12_GPU_DESCRIPTOR_HANDLE handle[NUM_USAGES];
        explicit TextureResource(std::optional<ConstantBuffer> cb) : cb(std::move(cb)) {
            for (int i = 0; i < NUM_USAGES; i++) {
                handle[i].ptr = 0;
            }
        }
        TextureResource() : TextureResource(std::nullopt) {}
    };

    Material(ID3D12Device *device, std::shared_ptr<DescriptorPool> pool) : device_(device), pool_(pool), res_(), cache_() {}
    void *buffer_at_impl(size_t const index) const;

    std::vector<TextureResource> res_;
    std::unordered_map<std::wstring, Texture> cache_;
    ID3D12Device *device_;
    std::shared_ptr<DescriptorPool> pool_;
};

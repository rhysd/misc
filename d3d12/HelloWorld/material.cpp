#include "material.h"
#include <cassert>

std::optional<Material> Material::create(ID3D12Device *device, std::shared_ptr<DescriptorPool> pool, size_t const buf_size, size_t const count) {
    assert(device != nullptr && pool != nullptr && count > 0);

    Material mat(device, std::move(pool));

    mat.res_.reserve(count);

    auto const size = buf_size * count;
    if (size > 0) {
        for (size_t i = 0; i < count; i++) {
            auto cb = ConstantBuffer::create(mat.device_, mat.pool_, buf_size);
            if (!cb) {
                return std::nullopt;
            }
            D3D12_GPU_DESCRIPTOR_HANDLE handle;
            handle.ptr = 0;
            mat.res_.emplace_back(cb, handle, TextureUsage::Unknown);
        }
    } else {
        for (size_t i = 0; i < count; i++) {
            D3D12_GPU_DESCRIPTOR_HANDLE handle;
            mat.res_.emplace_back(std::nullopt, handle, TextureUsage::Unknown);
        }
    }

    return mat;
}

bool Material::set_texture_at(size_t const index, TextureUsage usage, std::wstring const &path, DirectX::ResourceUploadBatch &batch) {
    assert(index < count());

    if (cache_.find(path) != cache_.end()) {
        assert(res_[index].usage == TextureUsage::Unknown);
        res_[index].usage = usage;
        return true;
    }

    auto tex = Texture::create(device_, pool_, path.c_str(), batch);
    if (!tex) {
        return false;
    }

    res_[index].handle = tex->get_handle_gpu();
    cache_.emplace(path, std::move(*tex));
    return true;
}

size_t Material::count() const {
    return res_.size();
}

void *Material::buffer_at_impl(size_t const index) const {
    assert(index < count());
    return res_[index].cb->get_mapped_ptr();
}

D3D12_GPU_VIRTUAL_ADDRESS Material::buffer_address_at(size_t const index) const {
    assert(index < count());
    return res_[index].cb->get_address();
}

D3D12_GPU_DESCRIPTOR_HANDLE Material::texture_handle_at(size_t const index) const {
    assert(index < count());
    return res_[index].handle;
}

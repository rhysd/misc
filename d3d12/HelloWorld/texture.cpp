#include "texture.h"
#include <DDSTextureLoader.h>
#include <cassert>

std::optional<Texture> Texture::create(
    ID3D12Device *device,
    std::shared_ptr<DescriptorPool> pool,
    wchar_t const *filepath,
    DirectX::ResourceUploadBatch &batch) {
    assert(device != nullptr && pool != nullptr && filepath != nullptr);

    Texture ret(std::move(pool));

    ret.handle_ = ret.pool_->alloc();
    if (ret.handle_ == nullptr) {
        return std::nullopt;
    }

    bool is_cube;
    auto const hr = DirectX::CreateDDSTextureFromFile(
        device,
        batch,
        filepath,
        ret.tex_.GetAddressOf(),
        /* generate mipmap*/ true,
        0,
        nullptr,
        &is_cube);
    if (FAILED(hr)) {
        return std::nullopt;
    }

    auto desc = ret.view_desc(is_cube);
    device->CreateShaderResourceView(ret.tex_.Get(), &desc, ret.handle_->handle_cpu);

    return ret;
}

D3D12_SHADER_RESOURCE_VIEW_DESC Texture::view_desc(bool const is_cube) {
    auto const desc = tex_->GetDesc();

    D3D12_SHADER_RESOURCE_VIEW_DESC view_desc{};

    view_desc.Format = desc.Format;
    view_desc.Shader4ComponentMapping = D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING; // RGBA

    // TODO: Currently only 2D texture is supported
    if (is_cube) {
        if (desc.DepthOrArraySize > 6) {
            view_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURECUBEARRAY;

            view_desc.TextureCubeArray.MostDetailedMip = 0;
            view_desc.TextureCubeArray.MipLevels = desc.MipLevels;
            view_desc.TextureCubeArray.First2DArrayFace = 0;
            view_desc.TextureCubeArray.NumCubes = (desc.DepthOrArraySize / 6);
            view_desc.TextureCubeArray.ResourceMinLODClamp = 0.0f;
        } else {
            view_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURECUBE;

            view_desc.TextureCube.MostDetailedMip = 0;
            view_desc.TextureCube.MipLevels = desc.MipLevels;
            view_desc.TextureCube.ResourceMinLODClamp = 0.0f;
        }
    } else {
        if (desc.DepthOrArraySize > 1) {
            if (desc.MipLevels > 1) {
                view_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY;

                view_desc.Texture2DMSArray.FirstArraySlice = 0;
                view_desc.Texture2DMSArray.ArraySize = desc.DepthOrArraySize;
            } else {
                view_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2DARRAY;

                view_desc.Texture2DArray.MostDetailedMip = 0;
                view_desc.Texture2DArray.MipLevels = desc.MipLevels;
                view_desc.Texture2DArray.FirstArraySlice = 0;
                view_desc.Texture2DArray.ArraySize = desc.DepthOrArraySize;
                view_desc.Texture2DArray.PlaneSlice = 0;
                view_desc.Texture2DArray.ResourceMinLODClamp = 0.0f;
            }
        } else {
            if (desc.MipLevels > 1) {
                view_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2DMS;
            } else {
                view_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2D;

                view_desc.Texture2D.MostDetailedMip = 0;
                view_desc.Texture2D.MipLevels = desc.MipLevels;
                view_desc.Texture2D.PlaneSlice = 0;
                view_desc.Texture2D.ResourceMinLODClamp = 0.0f;
            }
        }
    }

    return view_desc;
}

Texture::~Texture() {
    tex_.Reset();
    if (handle_ != nullptr && pool_ != nullptr) {
        pool_->dealloc(handle_);
    }
}

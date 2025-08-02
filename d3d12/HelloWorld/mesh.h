#pragma once

#include "ib.h"
#include "vb.h"
#include <DirectXMath.h>
#include <d3d12.h>
#include <optional>
#include <string>
#include <vector>

struct MeshVertex {
    DirectX::XMFLOAT3 position;
    DirectX::XMFLOAT3 normal;
    DirectX::XMFLOAT2 tex_coord;
    DirectX::XMFLOAT3 tangent;

    static const D3D12_INPUT_LAYOUT_DESC INPUT_LAYOUT;

  private:
    static const int INPUT_ELEMENT_COUNT = 4;
    static const D3D12_INPUT_ELEMENT_DESC INPUT_ELEMENTS[INPUT_ELEMENT_COUNT];
};

struct MaterialAsset {
    DirectX::XMFLOAT3 diffuse;
    DirectX::XMFLOAT3 specular;
    float alpha;
    float shininess;
    std::string diffuse_map;
};

struct MeshAsset {
    std::vector<MeshVertex> vertices;
    std::vector<uint32_t> indices;
    uint32_t material_id;
};

bool load_mesh(wchar_t const *filepath, std::vector<MeshAsset> &meshes, std::vector<MaterialAsset> &materials);

class Mesh final {
    VertexBuffer vb_;
    IndexBuffer ib_;
    uint32_t material_id_;
    uint32_t index_count_;

    Mesh(VertexBuffer vb, IndexBuffer ib)
        : vb_(std::move(vb)),
          ib_(std::move(ib)),
          material_id_(),
          index_count_() {}

  public:
    static std::optional<Mesh> create(ID3D12Device *device, MeshAsset const &asset);
    Mesh(Mesh &&other) = default;
    Mesh &operator=(Mesh &&other) = default;
    Mesh(Mesh &other) = delete;
    Mesh &operator=(Mesh &other) = delete;
    uint32_t material_id() const;
    void draw(ID3D12GraphicsCommandList *const cmd_list);
};

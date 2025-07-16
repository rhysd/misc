#include "assimp/material.h"
#include <DirectXMath.h>
#define NOMINMAX

#include "mesh.h"
#include <assimp/Importer.hpp>
#include <assimp/postprocess.h>
#include <assimp/scene.h>
#include <cassert>
#include <codecvt>
#include <optional>

namespace {

std::optional<std::string> to_utf8(wchar_t const *src) {
    auto const length = WideCharToMultiByte(CP_UTF8, 0U, src, -1, nullptr, 0, nullptr, nullptr);
    if (length <= 0) {
        return std::nullopt;
    }
    std::vector<char> buf(length);
    WideCharToMultiByte(CP_UTF8, 0U, src, -1, buf.data(), length, nullptr, nullptr);
    return std::string{buf.data(), buf.size()};
}

class MeshLoader {
  public:
    MeshLoader() = default;
    ~MeshLoader() = default;

    bool load(wchar_t const *filepath, std::vector<Mesh> &meshes, std::vector<Material> &materials);

  private:
    void parse_mesh(Mesh &dst, aiMesh const *src);
    void parse_material(Material &dst, aiMaterial const *src);
};

bool MeshLoader::load(wchar_t const *filepath, std::vector<Mesh> &meshes, std::vector<Material> &materials) {
    auto const path = to_utf8(filepath);
    if (!path) {
        return false;
    }

    Assimp::Importer importer;
    int const flags =
        aiProcess_Triangulate |
        aiProcess_PreTransformVertices |
        aiProcess_CalcTangentSpace |
        aiProcess_GenSmoothNormals |
        aiProcess_GenUVCoords |
        aiProcess_RemoveRedundantMaterials |
        aiProcess_OptimizeMeshes;

    auto const scene = importer.ReadFile(*path, flags);
    if (scene == nullptr) {
        return false;
    }

    meshes.clear();
    meshes.resize(scene->mNumMeshes);
    for (auto i = 0; i < meshes.size(); i++) {
        parse_mesh(meshes[i], scene->mMeshes[i]);
    }

    materials.clear();
    materials.resize(scene->mNumMaterials);
    for (auto i = 0; i < materials.size(); i++) {
        parse_material(materials[i], scene->mMaterials[i]);
    }

    return true;
}

void MeshLoader::parse_mesh(Mesh &dst, aiMesh const *src) {
    aiVector3D zero(0.0f, 0.0f, 0.0f);

    dst.material_id = src->mMaterialIndex;

    dst.vertices.resize(src->mNumVertices);
    for (auto i = 0u; i < src->mNumVertices; i++) {
        auto const position = &src->mVertices[i];
        auto const normal = &src->mNormals[i];
        auto const tex_coord = src->HasTextureCoords(0) ? &src->mTextureCoords[0][i] : &zero;
        auto const tangent = src->HasTangentsAndBitangents() ? &src->mTangents[i] : &zero;

        MeshVertex vert;
        vert.position = DirectX::XMFLOAT3(position->x, position->y, position->z);
        vert.normal = DirectX::XMFLOAT3(normal->x, normal->y, normal->z);
        vert.tex_coord = DirectX::XMFLOAT2(tex_coord->x, tex_coord->y);
        vert.tangent = DirectX::XMFLOAT3(tangent->x, tangent->y, tangent->z);

        dst.vertices[i] = vert;
    }

    dst.indices.resize(src->mNumFaces * 3);
    for (auto i = 0u; i < src->mNumFaces; i++) {
        auto const &face = src->mFaces[i];
        assert(face.mNumIndices == 3); // This must pass because of `aiProcess_Triangulate`
        dst.indices[i * 3 + 0] = face.mIndices[0];
        dst.indices[i * 3 + 1] = face.mIndices[1];
        dst.indices[i * 3 + 2] = face.mIndices[2];
    }
}

void MeshLoader::parse_material(Material &dst, aiMaterial const *src) {
    aiColor3D color;

    if (src->Get(AI_MATKEY_COLOR_DIFFUSE, color) == AI_SUCCESS) {
        dst.diffuse.x = color.r;
        dst.diffuse.y = color.g;
        dst.diffuse.z = color.b;
    } else {
        dst.diffuse = DirectX::XMFLOAT3(0.5f, 0.5f, 0.5f);
    }

    if (src->Get(AI_MATKEY_COLOR_SPECULAR, color) == AI_SUCCESS) {
        dst.specular.x = color.r;
        dst.specular.y = color.g;
        dst.specular.z = color.b;
    } else {
        dst.specular = DirectX::XMFLOAT3(0.0f, 0.0f, 0.0f);
    }

    if (src->Get(AI_MATKEY_SHININESS, dst.shininess) != AI_SUCCESS) {
        dst.shininess = 0.0f;
    }

    aiString path;
    if (src->Get(AI_MATKEY_TEXTURE_DIFFUSE(0), path) == AI_SUCCESS) {
        dst.diffuse_map = std::string(path.C_Str());
    } else {
        dst.diffuse_map.clear();
    }
}

} // namespace

bool load_mesh(wchar_t const *filepath, std::vector<Mesh> &meshes, std::vector<Material> &materials) {
    return MeshLoader{}.load(filepath, meshes, materials);
}

const D3D12_INPUT_ELEMENT_DESC MeshVertex::INPUT_ELEMENTS[] = {
    {
        /*SemanticName*/ "POSITION",
        /*SematicIndex*/ 0,
        /*Format: float3*/ DXGI_FORMAT_R32G32B32_FLOAT,
        /*InputSlot: We use single vertex buffer*/ 0,
        /*AlignedByteOffset: Elements are contiguous and no padding between them*/ D3D12_APPEND_ALIGNED_ELEMENT,
        /*InputSlotClass*/ D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
        /*InstanceDataStepRate: Fixed to 0 because of D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA*/ 0,
    },
    {"NORMAL", 0, DXGI_FORMAT_R32G32B32_FLOAT, 0, D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0},
    {"TEXCOORD", 0, /*float2*/ DXGI_FORMAT_R32G32_FLOAT, 0, D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0},
    {"TANGENT", 0, DXGI_FORMAT_R32G32B32_FLOAT, 0, D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0},
};

const D3D12_INPUT_LAYOUT_DESC MeshVertex::INPUT_LAYOUT{MeshVertex::INPUT_ELEMENTS, MeshVertex::INPUT_ELEMENT_COUNT};

static_assert(sizeof(MeshVertex) == 44);

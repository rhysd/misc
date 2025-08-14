#include <DirectXMath.h>

#define NOMINMAX

#include <assimp/Importer.hpp>
#include <assimp/material.h>
#include <assimp/postprocess.h>
#include <assimp/scene.h>

#include "mesh.h"
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

MeshAsset parse_mesh(aiMesh const *src) {
    MeshAsset ret;
    aiVector3D const zero(0.0f, 0.0f, 0.0f);

    ret.material_id = src->mMaterialIndex;

    auto const has_texture_coords = src->HasTextureCoords(0);
    auto const has_tangents = src->HasTangentsAndBitangents();

    ret.vertices.reserve(src->mNumVertices);
    for (auto i = 0u; i < src->mNumVertices; i++) {
        auto const position = &src->mVertices[i];
        auto const normal = &src->mNormals[i];
        auto const tex_coord = has_texture_coords ? &src->mTextureCoords[0][i] : &zero;
        auto const tangent = has_tangents ? &src->mTangents[i] : &zero;

        MeshVertex vert;
        vert.position = DirectX::XMFLOAT3(position->x, position->y, position->z);
        vert.normal = DirectX::XMFLOAT3(normal->x, normal->y, normal->z);
        vert.tex_coord = DirectX::XMFLOAT2(tex_coord->x, tex_coord->y);
        vert.tangent = DirectX::XMFLOAT3(tangent->x, tangent->y, tangent->z);

        ret.vertices.push_back(vert);
    }

    ret.indices.reserve(src->mNumFaces * 3);
    for (auto i = 0u; i < src->mNumFaces; i++) {
        auto const &face = src->mFaces[i];
        assert(face.mNumIndices == 3); // This must pass because of `aiProcess_Triangulate`
        for (auto i = 0; i < 3; i++) {
            ret.indices.push_back(face.mIndices[i]);
        }
    }

    return ret;
}

MaterialAsset parse_material(aiMaterial const *src) {
    MaterialAsset ret;
    aiColor3D color;

    if (src->Get(AI_MATKEY_COLOR_DIFFUSE, color) == AI_SUCCESS) {
        ret.diffuse.x = color.r;
        ret.diffuse.y = color.g;
        ret.diffuse.z = color.b;
    } else {
        ret.diffuse = DirectX::XMFLOAT3(0.5f, 0.5f, 0.5f);
    }

    if (src->Get(AI_MATKEY_COLOR_SPECULAR, color) == AI_SUCCESS) {
        ret.specular.x = color.r;
        ret.specular.y = color.g;
        ret.specular.z = color.b;
    } else {
        ret.specular = DirectX::XMFLOAT3(0.0f, 0.0f, 0.0f);
    }

    if (src->Get(AI_MATKEY_SHININESS, ret.shininess) != AI_SUCCESS) {
        ret.shininess = 0.0f;
    }

    aiString path;
    if (src->Get(AI_MATKEY_TEXTURE_DIFFUSE(0), path) == AI_SUCCESS) {
        ret.diffuse_map = std::string(path.C_Str());
    } else {
        ret.diffuse_map.clear();
    }

    return ret;
}

} // namespace

void load_mesh(wchar_t const *filepath, std::vector<MeshAsset> &meshes, std::vector<MaterialAsset> &materials) {
    auto const path = to_utf8(filepath);
    assert(path);

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
    assert(scene != nullptr);

    meshes.clear();
    meshes.reserve(scene->mNumMeshes);
    for (auto i = 0u; i < scene->mNumMeshes; i++) {
        meshes.push_back(parse_mesh(scene->mMeshes[i]));
    }

    materials.clear();
    materials.reserve(scene->mNumMaterials);
    for (auto i = 0u; i < scene->mNumMaterials; i++) {
        materials.push_back(parse_material(scene->mMaterials[i]));
    }
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

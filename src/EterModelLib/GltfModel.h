#pragma once

#include "GltfTypes.h"
#include <d3d9.h>
#include <string>
#include <vector>
#include <unordered_map>
#include <utility>

namespace EterModelLib
{

class CGltfModel
{
public:
    CGltfModel();
    virtual ~CGltfModel();

    CGltfModel(const CGltfModel&) = delete;
    CGltfModel& operator=(const CGltfModel&) = delete;

    CGltfModel(CGltfModel&& other) noexcept
        : m_bLoaded(other.m_bLoaded)
        , m_modelData(std::move(other.m_modelData))
        , m_boneIndexMap(std::move(other.m_boneIndexMap))
        , m_animIndexMap(std::move(other.m_animIndexMap))
        , m_pVertexBuffer(std::exchange(other.m_pVertexBuffer, nullptr))
        , m_pIndexBuffer(std::exchange(other.m_pIndexBuffer, nullptr))
    {
        other.m_bLoaded = false;
    }

    CGltfModel& operator=(CGltfModel&& other) noexcept
    {
        if (this != &other)
        {
            ReleaseGpuBuffers();
            m_bLoaded = std::exchange(other.m_bLoaded, false);
            m_modelData = std::move(other.m_modelData);
            m_boneIndexMap = std::move(other.m_boneIndexMap);
            m_animIndexMap = std::move(other.m_animIndexMap);
            m_pVertexBuffer = std::exchange(other.m_pVertexBuffer, nullptr);
            m_pIndexBuffer = std::exchange(other.m_pIndexBuffer, nullptr);
        }
        return *this;
    }

    // Ladowanie z pliku oraz z pamieci
    bool LoadFromFile(const std::string& filename);
    bool LoadFromMemory(const void* data, size_t size, const std::string& modelName = "");

    // Czyszczenie
    void Clear();
    bool IsLoaded() const { return m_bLoaded || !m_modelData.vertices.empty() || !m_modelData.submeshes.empty() || !m_modelData.skin.joints.empty(); }
    void SetLoaded(bool loaded = true);

    // Nazwa modelu
    const std::string& GetName() const { return m_modelData.name; }

    // Dostep do podsiatek (Submeshes)
    size_t GetSubmeshCount() const { return m_modelData.submeshes.size(); }
    const GltfSubmesh* GetSubmesh(size_t index) const;
    const std::vector<GltfSubmesh>& GetSubmeshes() const { return m_modelData.submeshes; }

    // Dostep do kosci i szkieletu (Joints / Skin)
    size_t GetBoneCount() const { return m_modelData.skin.joints.size(); }
    const GltfJoint* GetBone(size_t index) const;
    int FindBoneIndex(const std::string& name) const;
    const GltfSkin& GetSkin() const { return m_modelData.skin; }

    // Dostep do animacji (Motions / Animations)
    size_t GetAnimationCount() const { return m_modelData.motions.size(); }
    const GltfMotionData* GetAnimation(size_t index) const;
    const GltfMotionData* FindAnimation(const std::string& name) const;
    int FindAnimationIndex(const std::string& name) const;
    const std::vector<GltfMotionData>& GetAnimations() const { return m_modelData.motions; }

    // Dostep do geometrii wierzcholkow i indeksow
    size_t GetVertexCount() const { return m_modelData.vertices.size(); }
    size_t GetIndexCount() const { return m_modelData.indices.size(); }
    const std::vector<GltfVertex>& GetVertices() const { return m_modelData.vertices; }
    const std::vector<unsigned int>& GetIndices() const { return m_modelData.indices; }

    // Pelna struktura modelu
    const GltfModelData& GetModelData() const { return m_modelData; }
    GltfModelData& GetModelData() { return m_modelData; }
    void SetModelData(const GltfModelData& modelData);

    // Statyczne bufory sprzetowe GPU (wspoldzielone przez wszystkie instancje CGltfModelInstance)
    bool EnsureGpuBuffers(LPDIRECT3DDEVICE9 pDevice) const;
    void ReleaseGpuBuffers() const;
    LPDIRECT3DVERTEXBUFFER9 GetVertexBuffer() const { return m_pVertexBuffer; }
    LPDIRECT3DINDEXBUFFER9 GetIndexBuffer() const { return m_pIndexBuffer; }
    bool HasGpuBuffers() const { return m_pVertexBuffer != nullptr && m_pIndexBuffer != nullptr; }

    void RebuildLookupTables();

private:
    bool m_bLoaded;
    GltfModelData m_modelData;
    std::unordered_map<std::string, int> m_boneIndexMap;
    std::unordered_map<std::string, int> m_animIndexMap;
    mutable LPDIRECT3DVERTEXBUFFER9 m_pVertexBuffer = nullptr;
    mutable LPDIRECT3DINDEXBUFFER9 m_pIndexBuffer = nullptr;
};

} // namespace EterModelLib

using CGltfModel = EterModelLib::CGltfModel;

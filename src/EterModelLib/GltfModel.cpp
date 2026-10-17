#include "GltfModel.h"
#include "GltfLoader.h"
#include "EterBase/Debug.h"

namespace EterModelLib
{

CGltfModel::CGltfModel()
    : m_bLoaded(false)
{
}

CGltfModel::~CGltfModel()
{
    Clear();
}

void CGltfModel::Clear()
{
    ReleaseGpuBuffers();
    m_bLoaded = false;
    m_modelData.name.clear();
    m_modelData.vertices.clear();
    m_modelData.indices.clear();
    m_modelData.submeshes.clear();
    m_modelData.materials.clear();
    m_modelData.skin.name.clear();
    m_modelData.skin.joints.clear();
    m_modelData.motions.clear();
}

bool CGltfModel::LoadFromFile(const std::string& filename)
{
    Clear();
    GltfLoader loader;
    if (!loader.LoadFromFile(filename, m_modelData))
    {
        TraceErrorOnce("CGltfModel::LoadFromFile: Failed to load glTF model from file '%s'", filename.c_str());
        return false;
    }

    if (m_modelData.name.empty())
    {
        m_modelData.name = filename;
    }

    if (m_modelData.vertices.empty() || m_modelData.indices.empty())
    {
        TraceErrorOnce("CGltfModel::LoadFromFile: Model '%s' loaded with empty geometry (vertices=%zu, indices=%zu)",
            m_modelData.name.c_str(), m_modelData.vertices.size(), m_modelData.indices.size());
    }

    m_bLoaded = true;
    return true;
}

bool CGltfModel::LoadFromMemory(const void* data, size_t size, const std::string& modelName)
{
    Clear();
    GltfLoader loader;
    if (!loader.LoadFromMemory(data, size, m_modelData))
    {
        TraceErrorOnce("CGltfModel::LoadFromMemory: Failed to load glTF model '%s' (data=%p, size=%zu)",
            modelName.c_str(), data, size);
        return false;
    }

    if (!modelName.empty())
    {
        m_modelData.name = modelName;
    }

    m_bLoaded = true;
    return true;
}

const GltfSubmesh* CGltfModel::GetSubmesh(size_t index) const
{
    if (index < m_modelData.submeshes.size())
    {
        return &m_modelData.submeshes[index];
    }
    return nullptr;
}

const GltfJoint* CGltfModel::GetBone(size_t index) const
{
    if (index < m_modelData.skin.joints.size())
    {
        return &m_modelData.skin.joints[index];
    }
    return nullptr;
}

int CGltfModel::FindBoneIndex(const std::string& name) const
{
    for (size_t i = 0; i < m_modelData.skin.joints.size(); ++i)
    {
        if (m_modelData.skin.joints[i].name == name)
        {
            return static_cast<int>(i);
        }
    }
    return -1;
}

const GltfMotionData* CGltfModel::GetAnimation(size_t index) const
{
    if (index < m_modelData.motions.size())
    {
        return &m_modelData.motions[index];
    }
    return nullptr;
}

int CGltfModel::FindAnimationIndex(const std::string& name) const
{
    for (size_t i = 0; i < m_modelData.motions.size(); ++i)
    {
        if (m_modelData.motions[i].name == name)
        {
            return static_cast<int>(i);
        }
    }
    return -1;
}

const GltfMotionData* CGltfModel::FindAnimation(const std::string& name) const
{
    int index = FindAnimationIndex(name);
    if (index >= 0)
    {
        return &m_modelData.motions[index];
    }
    return nullptr;
}

void CGltfModel::ReleaseGpuBuffers() const
{
    if (m_pVertexBuffer)
    {
        m_pVertexBuffer->Release();
        m_pVertexBuffer = nullptr;
    }
    if (m_pIndexBuffer)
    {
        m_pIndexBuffer->Release();
        m_pIndexBuffer = nullptr;
    }
}

bool CGltfModel::EnsureGpuBuffers(LPDIRECT3DDEVICE9 pDevice) const
{
    if (m_pVertexBuffer && m_pIndexBuffer)
        return true;

    if (!pDevice || m_modelData.vertices.empty() || m_modelData.indices.empty())
    {
        TRACE_ERROR_ONCE_KEYED(
            reinterpret_cast<uintptr_t>(this),
            "CGltfModel::EnsureGpuBuffers: Invalid state for model '{}' (pDevice={:#x}, vertices={}, indices={})",
            m_modelData.name,
            reinterpret_cast<uintptr_t>(pDevice),
            m_modelData.vertices.size(),
            m_modelData.indices.size());
        return false;
    }

    ReleaseGpuBuffers();

    struct GpuPackedGltfVertex
    {
        float position[3];
        float normal[3];
        float uv0[2];
        float uv1[2];
        float jointIndices[4];
        float jointWeights[4];
    };
    static_assert(sizeof(GpuPackedGltfVertex) == sizeof(GltfVertex), "Stride must be 72 bytes");

    const UINT vbSize = static_cast<UINT>(m_modelData.vertices.size() * sizeof(GpuPackedGltfVertex));
    const UINT ibSize = static_cast<UINT>(m_modelData.indices.size() * sizeof(unsigned int));

    HRESULT hrVB = pDevice->CreateVertexBuffer(
        vbSize, D3DUSAGE_WRITEONLY, 0, D3DPOOL_DEFAULT, &m_pVertexBuffer, nullptr);
    if (FAILED(hrVB))
    {
        TraceError("CGltfModel::EnsureGpuBuffers: CreateVertexBuffer failed hr=0x%08X size=%u", hrVB, vbSize);
        ReleaseGpuBuffers();
        return false;
    }

    GpuPackedGltfVertex* pLockedVB = nullptr;
    if (FAILED(m_pVertexBuffer->Lock(0, vbSize, reinterpret_cast<void**>(&pLockedVB), 0)))
    {
        TraceError("CGltfModel::EnsureGpuBuffers: Lock VB failed");
        ReleaseGpuBuffers();
        return false;
    }

    for (size_t i = 0; i < m_modelData.vertices.size(); ++i)
    {
        const GltfVertex& src = m_modelData.vertices[i];
        GpuPackedGltfVertex& dst = pLockedVB[i];
        dst.position[0]     = src.position.x;
        dst.position[1]     = src.position.y;
        dst.position[2]     = src.position.z;
        dst.normal[0]       = src.normal.x;
        dst.normal[1]       = src.normal.y;
        dst.normal[2]       = src.normal.z;
        dst.uv0[0]          = src.uv0.x;
        dst.uv0[1]          = src.uv0.y;
        dst.uv1[0]          = src.uv1.x;
        dst.uv1[1]          = src.uv1.y;
        dst.jointIndices[0] = static_cast<float>(src.jointIndices.x);
        dst.jointIndices[1] = static_cast<float>(src.jointIndices.y);
        dst.jointIndices[2] = static_cast<float>(src.jointIndices.z);
        dst.jointIndices[3] = static_cast<float>(src.jointIndices.w);
        dst.jointWeights[0] = src.jointWeights.x;
        dst.jointWeights[1] = src.jointWeights.y;
        dst.jointWeights[2] = src.jointWeights.z;
        dst.jointWeights[3] = src.jointWeights.w;
    }
    m_pVertexBuffer->Unlock();

    HRESULT hrIB = pDevice->CreateIndexBuffer(
        ibSize, D3DUSAGE_WRITEONLY, D3DFMT_INDEX32, D3DPOOL_DEFAULT, &m_pIndexBuffer, nullptr);
    if (FAILED(hrIB))
    {
        TraceError("CGltfModel::EnsureGpuBuffers: CreateIndexBuffer failed hr=0x%08X size=%u", hrIB, ibSize);
        ReleaseGpuBuffers();
        return false;
    }

    void* pLockedIB = nullptr;
    if (FAILED(m_pIndexBuffer->Lock(0, ibSize, &pLockedIB, 0)))
    {
        TraceError("CGltfModel::EnsureGpuBuffers: Lock IB failed");
        ReleaseGpuBuffers();
        return false;
    }
    std::memcpy(pLockedIB, m_modelData.indices.data(), ibSize);
    m_pIndexBuffer->Unlock();

    return true;
}

} // namespace EterModelLib

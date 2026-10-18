#include "GltfModelInstance.h"
#include <algorithm>
#include <cmath>
#include <cstring>

namespace EterModelLib
{

CGltfModelInstance::CGltfModelInstance()
    : m_pModel(nullptr)
    , m_pLinkedModelInstance(nullptr)
    , m_pParentInstance(nullptr)
    , m_parentBoneIndex(-1)
    , m_bHasParentBoneMatrix(false)
    , m_currentAnimIndex(-1)
    , m_pExternalMotion(nullptr)
    , m_currentTime(0.0f)
    , m_prevTime(0.0f)
    , m_duration(0.0f)
    , m_isLoop(true)
    , m_isFinished(false)
    , m_playSpeed(1.0f)
    , m_bSkinningDirty(true)
    , m_deformCount(0)
{
    m_parentBoneMatrix = Matrix4x4::Identity();
}

CGltfModelInstance::CGltfModelInstance(const CGltfModel* pModel)
    : CGltfModelInstance()
{
    SetModel(pModel);
}

void CGltfModelInstance::RegisterLinkedChild(CGltfModelInstance* pChild) const
{
    if (!pChild) return;
    if (std::find(m_linkedChildren.begin(), m_linkedChildren.end(), pChild) == m_linkedChildren.end())
    {
        m_linkedChildren.push_back(pChild);
    }
}

void CGltfModelInstance::UnregisterLinkedChild(CGltfModelInstance* pChild) const
{
    if (!pChild) return;
    auto it = std::find(m_linkedChildren.begin(), m_linkedChildren.end(), pChild);
    if (it != m_linkedChildren.end())
    {
        m_linkedChildren.erase(it);
    }
}

void CGltfModelInstance::RegisterAttachedChild(CGltfModelInstance* pChild) const
{
    if (!pChild) return;
    if (std::find(m_attachedChildren.begin(), m_attachedChildren.end(), pChild) == m_attachedChildren.end())
    {
        m_attachedChildren.push_back(pChild);
    }
}

void CGltfModelInstance::UnregisterAttachedChild(CGltfModelInstance* pChild) const
{
    if (!pChild) return;
    auto it = std::find(m_attachedChildren.begin(), m_attachedChildren.end(), pChild);
    if (it != m_attachedChildren.end())
    {
        m_attachedChildren.erase(it);
    }
}

CGltfModelInstance::~CGltfModelInstance()
{
    if (m_pLinkedModelInstance)
    {
        m_pLinkedModelInstance->UnregisterLinkedChild(this);
        m_pLinkedModelInstance = nullptr;
    }

    for (auto* pChild : m_linkedChildren)
    {
        if (pChild)
        {
            pChild->m_pLinkedModelInstance = nullptr;
        }
    }
    m_linkedChildren.clear();

    if (m_pParentInstance)
    {
        m_pParentInstance->UnregisterAttachedChild(this);
        m_pParentInstance = nullptr;
    }

    for (auto* pChild : m_attachedChildren)
    {
        if (pChild)
        {
            pChild->m_pParentInstance = nullptr;
            pChild->m_parentBoneIndex = -1;
            pChild->m_parentBoneName.clear();
        }
    }
    m_attachedChildren.clear();
}

void CGltfModelInstance::LinkSkeleton(const CGltfModelInstance* pParentInstance)
{
    if (m_pLinkedModelInstance == pParentInstance)
        return;

    if (m_pLinkedModelInstance)
    {
        m_pLinkedModelInstance->UnregisterLinkedChild(this);
    }

    m_pLinkedModelInstance = pParentInstance;

    if (m_pLinkedModelInstance)
    {
        m_pLinkedModelInstance->RegisterLinkedChild(this);
    }

    m_bSkinningDirty = true;
    if (m_pModel && m_pModel->IsLoaded())
    {
        UpdateTransforms(m_currentTime);
    }
}

void CGltfModelInstance::SetParentModelInstance(const CGltfModelInstance* pParentInstance, const char* szBoneName)
{
    if (m_pParentInstance && m_pParentInstance != pParentInstance)
    {
        m_pParentInstance->UnregisterAttachedChild(this);
    }

    m_pParentInstance = pParentInstance;
    if (m_pParentInstance)
    {
        m_pParentInstance->RegisterAttachedChild(this);
    }

    m_parentBoneName = (szBoneName ? szBoneName : "");
    m_parentBoneIndex = -1;
    if (pParentInstance && pParentInstance->GetModel())
    {
        m_parentBoneIndex = pParentInstance->GetModel()->FindBoneIndex(m_parentBoneName);
    }
    m_bHasParentBoneMatrix = false;
    m_bSkinningDirty = true;
    if (m_pModel && m_pModel->IsLoaded())
    {
        UpdateTransforms(m_currentTime);
    }
}

void CGltfModelInstance::SetParentModelInstance(const CGltfModelInstance* pParentInstance, int boneIndex)
{
    if (m_pParentInstance && m_pParentInstance != pParentInstance)
    {
        m_pParentInstance->UnregisterAttachedChild(this);
    }

    m_pParentInstance = pParentInstance;
    if (m_pParentInstance)
    {
        m_pParentInstance->RegisterAttachedChild(this);
    }

    m_parentBoneIndex = boneIndex;
    m_parentBoneName.clear();
    m_bHasParentBoneMatrix = false;
    m_bSkinningDirty = true;
    if (m_pModel && m_pModel->IsLoaded())
    {
        UpdateTransforms(m_currentTime);
    }
}

void CGltfModelInstance::SetParentBoneMatrix(const Matrix4x4* pParentMatrix)
{
    if (pParentMatrix)
    {
        m_parentBoneMatrix = *pParentMatrix;
        m_bHasParentBoneMatrix = true;
    }
    else
    {
        m_bHasParentBoneMatrix = false;
    }
    m_bSkinningDirty = true;
    if (m_pModel && m_pModel->IsLoaded())
    {
        UpdateTransforms(m_currentTime);
    }
}

const Matrix4x4* CGltfModelInstance::GetParentBoneMatrix() const
{
    if (m_pParentInstance)
    {
        if (m_parentBoneIndex >= 0)
        {
            return m_pParentInstance->GetBoneMatrixPointer(m_parentBoneIndex);
        }
        if (!m_parentBoneName.empty())
        {
            return m_pParentInstance->GetBoneMatrixPointer(m_parentBoneName.c_str());
        }
    }

    if (m_bHasParentBoneMatrix)
    {
        return &m_parentBoneMatrix;
    }

    return nullptr;
}

void CGltfModelInstance::SetModel(const CGltfModel* pModel)
{
    m_pModel = pModel;
    m_currentAnimIndex = -1;
    m_pExternalMotion = nullptr;
    m_currentTime = 0.0f;
    m_prevTime = 0.0f;
    m_duration = 0.0f;
    m_isFinished = false;
    m_bSkinningDirty = true;
    m_deformCount = 0;

    if (m_pModel && m_pModel->IsLoaded())
    {
        size_t boneCount = m_pModel->GetBoneCount();
        m_localTransforms.assign(boneCount, Matrix4x4::Identity());
        m_worldTransforms.assign(boneCount, Matrix4x4::Identity());
        m_skinningMatrices.assign(boneCount, Matrix4x4::Identity());

        m_workTranslations.resize(boneCount);
        m_workRotations.resize(boneCount);
        m_workScales.resize(boneCount);
        m_workParentIndices.resize(boneCount);
        m_workInvBindMatrices.resize(boneCount);
        m_workNodeState.resize(boneCount);

        m_deformedVertices.clear();

        UpdateTransforms(0.0f);
    }
    else
    {
        m_localTransforms.clear();
        m_worldTransforms.clear();
        m_skinningMatrices.clear();
        m_workTranslations.clear();
        m_workRotations.clear();
        m_workScales.clear();
        m_workParentIndices.clear();
        m_workInvBindMatrices.clear();
        m_workNodeState.clear();
        m_deformedVertices.clear();
    }
}

const GltfMotionData* CGltfModelInstance::GetCurrentAnimation() const
{
    if (m_pExternalMotion)
    {
        return m_pExternalMotion;
    }
    if (!m_pModel || m_currentAnimIndex < 0)
    {
        return nullptr;
    }
    return m_pModel->GetAnimation(static_cast<size_t>(m_currentAnimIndex));
}

bool CGltfModelInstance::SetAnimation(int animIndex, bool loop)
{
    if (!m_pModel || !m_pModel->IsLoaded())
    {
        return false;
    }

    if (animIndex < 0 || static_cast<size_t>(animIndex) >= m_pModel->GetAnimationCount())
    {
        return false;
    }

    const GltfMotionData* pMotion = m_pModel->GetAnimation(static_cast<size_t>(animIndex));
    if (!pMotion)
    {
        return false;
    }

    m_pExternalMotion = nullptr;
    m_currentAnimIndex = animIndex;
    m_isLoop = loop;
    m_currentTime = 0.0f;
    m_prevTime = 0.0f;
    m_duration = pMotion->duration;
    m_isFinished = false;

    UpdateTransforms(0.0f);
    return true;
}

bool CGltfModelInstance::SetAnimation(const std::string& animName, bool loop)
{
    if (!m_pModel || !m_pModel->IsLoaded())
    {
        return false;
    }

    int animIndex = m_pModel->FindAnimationIndex(animName);
    if (animIndex < 0)
    {
        return false;
    }

    return SetAnimation(animIndex, loop);
}

bool CGltfModelInstance::SetAnimation(const GltfMotionData* pMotion, bool loop)
{
    if (!pMotion)
    {
        return false;
    }

    m_pExternalMotion = pMotion;
    m_currentAnimIndex = -1;
    m_isLoop = loop;
    m_currentTime = 0.0f;
    m_prevTime = 0.0f;
    m_duration = pMotion->duration;
    m_isFinished = false;

    UpdateTransforms(0.0f);
    return true;
}

bool CGltfModelInstance::HaveBlendThing() const
{
    if (!m_pModel || !m_pModel->IsLoaded())
        return false;

    const auto& materials = m_pModel->GetModelData().materials;
    for (const auto& mat : materials)
    {
        if (!mat.opacityTexture.empty())
            return true;
        if (mat.name.size() >= 5 && !_strnicmp(mat.name.c_str(), "blend", 5))
            return true;
    }
    return false;
}

void CGltfModelInstance::SetCurrentTime(float time)
{
    m_prevTime = m_currentTime;
    m_currentTime = time;

    if (m_duration > 0.0f)
    {
        if (m_currentTime >= m_duration)
        {
            if (m_isLoop)
            {
                m_currentTime = std::fmod(m_currentTime, m_duration);
            }
            else
            {
                m_currentTime = m_duration;
                m_isFinished = true;
            }
        }
        else if (m_currentTime < 0.0f)
        {
            if (m_isLoop)
            {
                float rem = std::fmod(m_currentTime, m_duration);
                if (rem < 0.0f) rem += m_duration;
                m_currentTime = rem;
            }
            else
            {
                m_currentTime = 0.0f;
            }
        }
        else
        {
            m_isFinished = false;
        }
    }

    UpdateTransforms(m_currentTime);
}

void CGltfModelInstance::Update(float deltaTime)
{
    if (!m_pModel || !m_pModel->IsLoaded())
    {
        return;
    }

    m_prevTime = m_currentTime;

    const GltfMotionData* pMotion = GetCurrentAnimation();
    if (pMotion && m_duration > 0.0f)
    {
        m_currentTime += deltaTime * m_playSpeed;

        if (m_currentTime >= m_duration)
        {
            if (m_isLoop)
            {
                m_currentTime = std::fmod(m_currentTime, m_duration);
            }
            else
            {
                m_currentTime = m_duration;
                m_isFinished = true;
            }
        }
        else if (m_currentTime < 0.0f)
        {
            if (m_isLoop)
            {
                float rem = std::fmod(m_currentTime, m_duration);
                if (rem < 0.0f) rem += m_duration;
                m_currentTime = rem;
            }
            else
            {
                m_currentTime = 0.0f;
            }
        }
    }

    m_bSkinningDirty = true;
    for (auto* pChild : m_linkedChildren)
    {
        if (pChild)
        {
            pChild->m_bSkinningDirty = true;
        }
    }

    UpdateTransforms(m_currentTime);
}

void CGltfModelInstance::UpdateTransforms(float time)
{
    if (!m_pModel || !m_pModel->IsLoaded())
    {
        return;
    }

    size_t boneCount = m_pModel->GetBoneCount();
    if (boneCount == 0)
    {
        if (m_pLinkedModelInstance)
        {
            m_worldTransforms = m_pLinkedModelInstance->GetWorldTransforms();
            m_skinningMatrices = m_pLinkedModelInstance->GetSkinningMatrices();
        }
        else
        {
            m_localTransforms.clear();
            m_worldTransforms.clear();
            m_skinningMatrices.clear();
        }
        return;
    }

    if (m_localTransforms.size() != boneCount)
        m_localTransforms.assign(boneCount, Matrix4x4::Identity());
    if (m_worldTransforms.size() != boneCount)
        m_worldTransforms.assign(boneCount, Matrix4x4::Identity());
    if (m_skinningMatrices.size() != boneCount)
        m_skinningMatrices.assign(boneCount, Matrix4x4::Identity());

    if (m_workTranslations.size() != boneCount) m_workTranslations.resize(boneCount);
    if (m_workRotations.size() != boneCount) m_workRotations.resize(boneCount);
    if (m_workScales.size() != boneCount) m_workScales.resize(boneCount);
    if (m_workParentIndices.size() != boneCount) m_workParentIndices.resize(boneCount);
    if (m_workInvBindMatrices.size() != boneCount) m_workInvBindMatrices.resize(boneCount);
    if (m_workNodeState.size() != boneCount) m_workNodeState.resize(boneCount);

    auto& translations = m_workTranslations;
    auto& rotations = m_workRotations;
    auto& scales = m_workScales;
    auto& parentIndices = m_workParentIndices;
    auto& invBindMatrices = m_workInvBindMatrices;
    auto& state = m_workNodeState;

    const auto& joints = m_pModel->GetModelData().skin.joints;
    for (size_t j = 0; j < boneCount && j < joints.size(); ++j)
    {
        translations[j] = Vector3(joints[j].localTranslation.x, joints[j].localTranslation.y, joints[j].localTranslation.z);
        rotations[j] = Quaternion(joints[j].localRotation.x, joints[j].localRotation.y, joints[j].localRotation.z, joints[j].localRotation.w);
        scales[j] = Vector3(joints[j].localScale.x, joints[j].localScale.y, joints[j].localScale.z);
    }

    const GltfMotionData* pMotion = GetCurrentAnimation();
    if (pMotion)
    {
        for (const auto& channel : pMotion->channels)
        {
            if (channel.times.empty() || channel.values.empty())
                continue;

            int jointIdx = (m_pExternalMotion == nullptr) ? channel.jointIndex : -1;
            if (jointIdx < 0 && !channel.targetNodeName.empty())
                jointIdx = m_pModel->FindBoneIndex(channel.targetNodeName);
            if (jointIdx < 0)
                jointIdx = channel.jointIndex;

            if (jointIdx < 0 || static_cast<size_t>(jointIdx) >= boneCount)
                continue;

            auto applyConstantValue = [&](const float4& val) {
                if (channel.pathType == GltfAnimationPathType::Translation)
                    translations[jointIdx] = Vector3(val.x, val.y, val.z);
                else if (channel.pathType == GltfAnimationPathType::Rotation)
                {
                    rotations[jointIdx] = Quaternion(val.x, val.y, val.z, val.w);
                    rotations[jointIdx].Normalize();
                }
                else if (channel.pathType == GltfAnimationPathType::Scale)
                    scales[jointIdx] = Vector3(val.x, val.y, val.z);
            };

            if (channel.times.size() == 1 || time <= channel.times.front())
            {
                applyConstantValue(channel.values.front());
            }
            else if (time >= channel.times.back())
            {
                applyConstantValue(channel.values.back());
            }
            else
            {
                auto it = std::upper_bound(channel.times.begin(), channel.times.end(), time);
                size_t idx1 = static_cast<size_t>(std::distance(channel.times.begin(), it));
                size_t idx0 = idx1 - 1;

                if (idx1 >= channel.values.size())
                {
                    applyConstantValue(channel.values.back());
                    continue;
                }

                if (channel.interpolation == GltfInterpolationType::Step)
                {
                    applyConstantValue(channel.values[idx0]);
                    continue;
                }

                const float t0 = channel.times[idx0];
                const float t1 = channel.times[idx1];
                const float factor = (t1 > t0) ? std::clamp((time - t0) / (t1 - t0), 0.0f, 1.0f) : 0.0f;

                const auto& v0 = channel.values[idx0];
                const auto& v1 = channel.values[idx1];

                if (channel.pathType == GltfAnimationPathType::Translation)
                    translations[jointIdx] = Vector3::Lerp({v0.x, v0.y, v0.z}, {v1.x, v1.y, v1.z}, factor);
                else if (channel.pathType == GltfAnimationPathType::Rotation)
                    rotations[jointIdx] = Quaternion::Slerp({v0.x, v0.y, v0.z, v0.w}, {v1.x, v1.y, v1.z, v1.w}, factor);
                else if (channel.pathType == GltfAnimationPathType::Scale)
                    scales[jointIdx] = Vector3::Lerp({v0.x, v0.y, v0.z}, {v1.x, v1.y, v1.z}, factor);
            }
        }
    }

    for (size_t j = 0; j < boneCount; ++j)
    {
        m_localTransforms[j] = Matrix4x4::TRS(translations[j], rotations[j], scales[j]);
    }

    const auto& skin = m_pModel->GetSkin();
    for (size_t j = 0; j < boneCount; ++j)
    {
        parentIndices[j] = skin.joints[j].parentIndex;
    }

    EvaluateHierarchy(parentIndices, m_localTransforms, m_worldTransforms, state);

    if (m_pLinkedModelInstance)
    {
        for (size_t j = 0; j < boneCount; ++j)
        {
            if (!skin.joints[j].name.empty())
            {
                if (const Matrix4x4* pParentMat = m_pLinkedModelInstance->GetBoneMatrixPointer(skin.joints[j].name.c_str()))
                {
                    m_worldTransforms[j] = *pParentMat;
                    continue;
                }
            }
            int p = parentIndices[j];
            if (p >= 0 && static_cast<size_t>(p) < boneCount && static_cast<size_t>(p) != j)
                m_worldTransforms[j] = m_localTransforms[j] * m_worldTransforms[p];
        }
    }

    if (const Matrix4x4* pAttachMat = GetParentBoneMatrix())
    {
        for (size_t j = 0; j < boneCount; ++j)
            m_worldTransforms[j] = m_worldTransforms[j] * (*pAttachMat);
    }

    for (size_t j = 0; j < boneCount; ++j)
    {
        std::memcpy(invBindMatrices[j].m, skin.joints[j].inverseBindMatrix.m, sizeof(invBindMatrices[j].m));
    }

    ComputeSkinningMatrices(m_worldTransforms, invBindMatrices, m_skinningMatrices);
    m_bSkinningDirty = true;
}

void CGltfModelInstance::Deform(bool bForce)
{
    DeformVertices(bForce);
}

void CGltfModelInstance::DeformVertices(bool bForce)
{
    if (!m_pModel || !m_pModel->IsLoaded())
    {
        return;
    }

    if (m_pLinkedModelInstance || GetParentBoneMatrix())
    {
        UpdateTransforms(m_currentTime);
    }

    if (!bForce && !m_bSkinningDirty)
    {
        return;
    }

    const auto& srcVertices = m_pModel->GetVertices();
    if (srcVertices.empty())
    {
        m_deformedVertices.clear();
        m_bSkinningDirty = false;
        return;
    }

    if (m_deformedVertices.size() != srcVertices.size())
    {
        if (m_deformedVertices.capacity() < srcVertices.size())
        {
            m_deformedVertices.reserve(srcVertices.size());
        }
        m_deformedVertices.assign(srcVertices.begin(), srcVertices.end());
    }

    if (m_skinningMatrices.empty())
    {
        const Matrix4x4* pAttachMat = GetParentBoneMatrix();
        if (pAttachMat)
        {
            const auto& mat = pAttachMat->m;
            size_t vertexCount = srcVertices.size();
            for (size_t v = 0; v < vertexCount; ++v)
            {
                const GltfVertex& src = srcVertices[v];
                GltfVertex& dst = m_deformedVertices[v];

                float px = src.position.x * mat[0][0] + src.position.y * mat[1][0] + src.position.z * mat[2][0] + mat[3][0];
                float py = src.position.x * mat[0][1] + src.position.y * mat[1][1] + src.position.z * mat[2][1] + mat[3][1];
                float pz = src.position.x * mat[0][2] + src.position.y * mat[1][2] + src.position.z * mat[2][2] + mat[3][2];
                dst.position = { px, py, pz };

                float nx = src.normal.x * mat[0][0] + src.normal.y * mat[1][0] + src.normal.z * mat[2][0];
                float ny = src.normal.x * mat[0][1] + src.normal.y * mat[1][1] + src.normal.z * mat[2][1];
                float nz = src.normal.x * mat[0][2] + src.normal.y * mat[1][2] + src.normal.z * mat[2][2];
                float nLen = std::sqrt(nx * nx + ny * ny + nz * nz);
                if (nLen > 0.000001f)
                    dst.normal = { nx / nLen, ny / nLen, nz / nLen };
                else
                    dst.normal = src.normal;
            }
            m_bSkinningDirty = false;
            ++m_deformCount;
            return;
        }

        m_deformedVertices = srcVertices;
        m_bSkinningDirty = false;
        ++m_deformCount;
        return;
    }

    size_t vertexCount = srcVertices.size();
    for (size_t v = 0; v < vertexCount; ++v)
    {
        const GltfVertex& src = srcVertices[v];
        GltfVertex& dst = m_deformedVertices[v];

        const unsigned int joints[4] = {
            src.jointIndices.x,
            src.jointIndices.y,
            src.jointIndices.z,
            src.jointIndices.w
        };
        const float weights[4] = {
            src.jointWeights.x,
            src.jointWeights.y,
            src.jointWeights.z,
            src.jointWeights.w
        };

        float totalWeight = weights[0] + weights[1] + weights[2] + weights[3];
        if (totalWeight < 0.0001f)
        {
            dst.position = src.position;
            dst.normal = src.normal;
            continue;
        }

        float invTotalWeight = 1.0f / totalWeight;
        float outX = 0.0f, outY = 0.0f, outZ = 0.0f;
        float normX = 0.0f, normY = 0.0f, normZ = 0.0f;
        float processedWeight = 0.0f;

        for (int i = 0; i < 4; ++i)
        {
            float w = weights[i] * invTotalWeight;
            if (w <= 0.0f)
            {
                continue;
            }

            unsigned int jIdx = joints[i];
            if (jIdx >= m_skinningMatrices.size())
            {
                continue;
            }

            processedWeight += w;
            const auto& mat = m_skinningMatrices[jIdx].m;

            // Pozycja z uwzglednieniem translacji wierszowej: [px, py, pz, 1] * M
            float px = src.position.x * mat[0][0] + src.position.y * mat[1][0] + src.position.z * mat[2][0] + mat[3][0];
            float py = src.position.x * mat[0][1] + src.position.y * mat[1][1] + src.position.z * mat[2][1] + mat[3][1];
            float pz = src.position.x * mat[0][2] + src.position.y * mat[1][2] + src.position.z * mat[2][2] + mat[3][2];

            outX += px * w;
            outY += py * w;
            outZ += pz * w;

            // Normalna bez translacji: [nx, ny, nz, 0] * M
            float nx = src.normal.x * mat[0][0] + src.normal.y * mat[1][0] + src.normal.z * mat[2][0];
            float ny = src.normal.x * mat[0][1] + src.normal.y * mat[1][1] + src.normal.z * mat[2][1];
            float nz = src.normal.x * mat[0][2] + src.normal.y * mat[1][2] + src.normal.z * mat[2][2];

            normX += nx * w;
            normY += ny * w;
            normZ += nz * w;
        }

        // Fallback: jesli zadna kosc nie byla poprawna, zachowaj wierzcholek zrodlowy (zamiast spike do (0,0,0))
        if (processedWeight < 0.0001f)
        {
            dst.position = src.position;
            dst.normal = src.normal;
            continue;
        }
        else if (processedWeight < 0.999f)
        {
            float renormalize = 1.0f / processedWeight;
            outX *= renormalize;
            outY *= renormalize;
            outZ *= renormalize;
            normX *= renormalize;
            normY *= renormalize;
            normZ *= renormalize;
        }

        dst.position = { outX, outY, outZ };

        float normLen = std::sqrt(normX * normX + normY * normY + normZ * normZ);
        if (normLen > 0.000001f)
        {
            dst.normal = { normX / normLen, normY / normLen, normZ / normLen };
        }
        else
        {
            dst.normal = src.normal;
        }
    }

    m_bSkinningDirty = false;
    ++m_deformCount;
}

void CGltfModelInstance::CheckEvents(float prevTime, float currTime, std::vector<GltfEvent>& outEvents) const
{
    if (!m_pModel || !m_pModel->IsLoaded() || prevTime == currTime)
    {
        return;
    }

    const GltfMotionData* pMotion = GetCurrentAnimation();
    if (!pMotion || pMotion->events.empty())
    {
        return;
    }

    if (prevTime < currTime)
    {
        for (const auto& evt : pMotion->events)
        {
            if ((evt.time > prevTime && evt.time <= currTime) ||
                (prevTime == 0.0f && evt.time == 0.0f))
            {
                outEvents.push_back(evt);
            }
        }
    }
    else if (m_playSpeed > 0.0f)
    {
        // Nastapil przeskok / zapetlenie animacji (prevTime > currTime)
        float duration = m_duration > 0.0f ? m_duration : pMotion->duration;
        for (const auto& evt : pMotion->events)
        {
            if (evt.time > prevTime && evt.time <= duration)
            {
                outEvents.push_back(evt);
            }
        }
        for (const auto& evt : pMotion->events)
        {
            if (evt.time >= 0.0f && evt.time <= currTime)
            {
                outEvents.push_back(evt);
            }
        }
    }
}

void CGltfModelInstance::CheckEvents(std::vector<GltfEvent>& outEvents) const
{
    CheckEvents(m_prevTime, m_currentTime, outEvents);
}

const Matrix4x4* CGltfModelInstance::GetBoneMatrixPointer(int boneIndex) const
{
    if (boneIndex < 0 || static_cast<size_t>(boneIndex) >= m_worldTransforms.size())
    {
        return nullptr;
    }
    return &m_worldTransforms[boneIndex];
}

const Matrix4x4* CGltfModelInstance::GetBoneMatrixPointer(const char* szBoneName) const
{
    if (!szBoneName)
    {
        return nullptr;
    }

    if (m_pModel && m_pModel->IsLoaded())
    {
        int boneIdx = m_pModel->FindBoneIndex(szBoneName);
        if (boneIdx >= 0)
        {
            return GetBoneMatrixPointer(boneIdx);
        }
    }

    if (m_pLinkedModelInstance && m_pLinkedModelInstance != this)
    {
        if (m_pLinkedModelInstance->GetModel() && m_pLinkedModelInstance->GetModel()->IsLoaded())
        {
            int pBoneIdx = m_pLinkedModelInstance->GetModel()->FindBoneIndex(szBoneName);
            if (pBoneIdx >= 0)
            {
                return m_pLinkedModelInstance->GetBoneMatrixPointer(pBoneIdx);
            }
        }
    }

    return nullptr;
}

bool CGltfModelInstance::GetBoneWorldTransform(size_t boneIndex, Matrix4x4& outTransform) const
{
    const Matrix4x4* pMat = GetBoneMatrixPointer(static_cast<int>(boneIndex));
    if (!pMat)
    {
        return false;
    }
    outTransform = *pMat;
    return true;
}

bool CGltfModelInstance::GetBoneWorldTransform(const std::string& boneName, Matrix4x4& outTransform) const
{
    const Matrix4x4* pMat = GetBoneMatrixPointer(boneName.c_str());
    if (!pMat)
    {
        return false;
    }
    outTransform = *pMat;
    return true;
}

} // namespace EterModelLib

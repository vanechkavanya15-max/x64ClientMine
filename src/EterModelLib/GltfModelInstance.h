#pragma once

#include "GltfModel.h"
#include "SkeletalMath.h"
#include <vector>
#include <string>
#include <type_traits>

namespace EterModelLib
{

// Bezpieczne rzutowanie wskaznikow materialow (checked material cast z obsluga nullptr)
template <typename TTarget, typename TSource>
inline TTarget* CheckedMaterialCast(TSource* pSource)
{
    if (!pSource)
    {
        return nullptr;
    }
    if constexpr (std::is_polymorphic_v<TSource>)
    {
        return dynamic_cast<TTarget*>(pSource);
    }
    else
    {
        return static_cast<TTarget*>(pSource);
    }
}

template <typename TTarget, typename TSource>
inline const TTarget* CheckedMaterialCast(const TSource* pSource)
{
    if (!pSource)
    {
        return nullptr;
    }
    if constexpr (std::is_polymorphic_v<TSource>)
    {
        return dynamic_cast<const TTarget*>(pSource);
    }
    else
    {
        return static_cast<const TTarget*>(pSource);
    }
}

class CGltfModelInstance
{
public:
    CGltfModelInstance();
    explicit CGltfModelInstance(const CGltfModel* pModel);
    virtual ~CGltfModelInstance();

    CGltfModelInstance(const CGltfModelInstance&) = delete;
    CGltfModelInstance& operator=(const CGltfModelInstance&) = delete;
    CGltfModelInstance(CGltfModelInstance&&) noexcept = default;
    CGltfModelInstance& operator=(CGltfModelInstance&&) noexcept = default;

    // Przypisanie i odczyt modelu zrodlowego
    void SetModel(const CGltfModel* pModel);
    const CGltfModel* GetModel() const { return m_pModel; }

    // Wspoldzielony szkielet (Hair Link / Linked Skeleton)
    void LinkSkeleton(const CGltfModelInstance* pParentInstance);
    const CGltfModelInstance* GetLinkedSkeleton() const { return m_pLinkedModelInstance; }
    void SetLinkedModelInstance(const CGltfModelInstance* pParentInstance) { LinkSkeleton(pParentInstance); }
    const CGltfModelInstance* GetLinkedModelInstance() const { return m_pLinkedModelInstance; }

    // Zarzadzanie aktywna animacja
    bool SetAnimation(int animIndex, bool loop = true);
    bool SetAnimation(const std::string& animName, bool loop = true);
    bool SetAnimation(const GltfMotionData* pMotion, bool loop = true);
    bool PlayMotion(int animIndex, bool loop = true) { return SetAnimation(animIndex, loop); }
    bool PlayMotion(const std::string& animName, bool loop = true) { return SetAnimation(animName, loop); }
    bool PlayMotion(const GltfMotionData* pMotion, bool loop = true) { return SetAnimation(pMotion, loop); }
    int GetCurrentAnimationIndex() const { return m_currentAnimIndex; }
    const GltfMotionData* GetCurrentAnimation() const;
    bool HaveBlendThing() const;

    // Sterowanie czasem animacji
    float GetCurrentTime() const { return m_currentTime; }
    void SetCurrentTime(float time);
    float GetPrevTime() const { return m_prevTime; }
    float GetDuration() const { return m_duration; }
    bool IsLoop() const { return m_isLoop; }
    void SetLoop(bool loop) { m_isLoop = loop; }
    bool IsFinished() const { return m_isFinished; }
    float GetPlaySpeed() const { return m_playSpeed; }
    void SetPlaySpeed(float speed) { m_playSpeed = speed; }

    // Glowna metoda aktualizacji klatek animacji
    // (LERP pozycji/skali, SLERP kwaternionow ze SkeletalMath, EvaluateHierarchy, ComputeSkinningMatrices)
    void Update(float deltaTime);

    // CPU Skinning wierzcholkow na podstawie macierzy skinningu i wag kosci
    void Deform(bool bForce = false);
    void DeformVertices(bool bForce = false);
    const std::vector<GltfVertex>& GetDeformedVertices() const
    {
        if (m_bSkinningDirty || (m_deformedVertices.empty() && m_pModel && !m_pModel->GetVertices().empty()))
            const_cast<CGltfModelInstance*>(this)->DeformVertices(false);
        return m_deformedVertices;
    }
    std::vector<GltfVertex>& GetDeformedVerticesRef()
    {
        if (m_bSkinningDirty || (m_deformedVertices.empty() && m_pModel && !m_pModel->GetVertices().empty()))
            DeformVertices(false);
        return m_deformedVertices;
    }

    bool IsSkinningDirty() const { return m_bSkinningDirty; }
    void MarkSkinningDirty() { m_bSkinningDirty = true; }
    size_t GetDeformCount() const { return m_deformCount; }

    // Bezpieczne pobranie materialu (checked material cast z obsluga nullptr)
    template <typename T = GltfMaterial>
    const T* GetCheckedMaterial(size_t index) const
    {
        if (!m_pModel)
            return nullptr;
        const auto& materials = m_pModel->GetModelData().materials;
        if (index >= materials.size())
            return nullptr;
        return CheckedMaterialCast<T>(&materials[index]);
    }

    // Wykrywanie eventow walki w oknie czasowym
    void CheckEvents(float prevTime, float currTime, std::vector<GltfEvent>& outEvents) const;
    void CheckEvents(std::vector<GltfEvent>& outEvents) const;

    // Macierze transformacji kosci
    const std::vector<Matrix4x4>& GetLocalTransforms() const { return m_localTransforms; }
    const std::vector<Matrix4x4>& GetWorldTransforms() const { return m_worldTransforms; }
    const std::vector<Matrix4x4>& GetSkinningMatrices() const { return m_skinningMatrices; }

    // Odczyt macierzy transformacji kosci
    const Matrix4x4* GetBoneMatrixPointer(int boneIndex) const;
    const Matrix4x4* GetBoneMatrixPointer(const char* szBoneName) const;

    // Odczyt macierzy swiatowej kosci (np. do doczepiania broni, efektow czasteczkowych)
    bool GetBoneWorldTransform(size_t boneIndex, Matrix4x4& outTransform) const;
    bool GetBoneWorldTransform(const std::string& boneName, Matrix4x4& outTransform) const;

    void RegisterLinkedChild(CGltfModelInstance* pChild) const;
    void UnregisterLinkedChild(CGltfModelInstance* pChild) const;

    void RegisterAttachedChild(CGltfModelInstance* pChild) const;
    void UnregisterAttachedChild(CGltfModelInstance* pChild) const;

    // Gniazda kosci (Bone Attachment / Sockets)
    void SetParentModelInstance(const CGltfModelInstance* pParentInstance, const char* szBoneName);
    void SetParentModelInstance(const CGltfModelInstance* pParentInstance, int boneIndex);
    void SetParentBoneMatrix(const Matrix4x4* pParentMatrix);
    const Matrix4x4* GetParentBoneMatrix() const;
    const CGltfModelInstance* GetParentModelInstance() const { return m_pParentInstance; }
    int GetParentBoneIndex() const { return m_parentBoneIndex; }
    const std::string& GetParentBoneName() const { return m_parentBoneName; }

private:
    void UpdateTransforms(float time);

private:
    const CGltfModel* m_pModel;
    const CGltfModelInstance* m_pLinkedModelInstance;
    mutable std::vector<CGltfModelInstance*> m_linkedChildren;

    // Gniazdo kosci (Bone Attachment / Socket)
    const CGltfModelInstance* m_pParentInstance;
    mutable std::vector<CGltfModelInstance*> m_attachedChildren;
    std::string m_parentBoneName;
    int m_parentBoneIndex;
    Matrix4x4 m_parentBoneMatrix;
    bool m_bHasParentBoneMatrix;

    int m_currentAnimIndex;
    const GltfMotionData* m_pExternalMotion;
    float m_currentTime;
    float m_prevTime;
    float m_duration;
    bool m_isLoop;
    bool m_isFinished;
    float m_playSpeed;

    std::vector<Matrix4x4> m_localTransforms;
    std::vector<Matrix4x4> m_worldTransforms;
    std::vector<Matrix4x4> m_skinningMatrices;

    // Dedykowany bufor deformacji wierzcholkow
    std::vector<GltfVertex> m_deformedVertices;
    bool m_bSkinningDirty;
    size_t m_deformCount;

    // Bufory robocze CPU skinningu i transformacji kosci (zero alokacji na stercie per klatka)
    std::vector<Vector3> m_workTranslations;
    std::vector<Quaternion> m_workRotations;
    std::vector<Vector3> m_workScales;
    std::vector<int32_t> m_workParentIndices;
    std::vector<Matrix4x4> m_workInvBindMatrices;
    std::vector<uint8_t> m_workNodeState;
};

} // namespace EterModelLib

using CGltfModelInstance = EterModelLib::CGltfModelInstance;

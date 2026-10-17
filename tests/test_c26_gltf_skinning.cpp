#include <cassert>
#include <iostream>
#include <vector>
#include <cmath>
#include <string>
#include <DirectXMath.h>

#include "EterModelLib/GltfModel.h"
#include "EterModelLib/GltfModelInstance.h"
#include "EterModelLib/GltfTypes.h"
#include "EterModelLib/SkeletalMath.h"

using namespace EterModelLib;

// 1. Static assert verification
static_assert(sizeof(Matrix4x4) == 64, "Matrix4x4 size mismatch");
static_assert(sizeof(DirectX::XMFLOAT4X4) == 64, "XMFLOAT4X4 size mismatch");

static bool ApproxEqual(float a, float b, float epsilon = 0.005f)
{
    return std::abs(a - b) < epsilon;
}

static bool ApproxEqualVec3(const float3& a, float x, float y, float z, float epsilon = 0.005f)
{
    return ApproxEqual(a.x, x, epsilon) &&
           ApproxEqual(a.y, y, epsilon) &&
           ApproxEqual(a.z, z, epsilon);
}

// ------------------------------------------------------------------------------------------------
// Test 1: Brak alokacji na stercie w kazdej klatce (stabilnosc wskaznika bufora deformacji)
// ------------------------------------------------------------------------------------------------
void TestDeformedBufferZeroAllocation()
{
    std::cout << "[RUN] TestDeformedBufferZeroAllocation..." << std::endl;

    CGltfModel model;
    GltfModelData& data = model.GetModelData();
    data.name = "TestZeroAllocModel";

    // 2 kosci: Root i Arm
    GltfJoint jointRoot;
    jointRoot.name = "Root";
    jointRoot.parentIndex = -1;
    jointRoot.inverseBindMatrix = { 1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1 };

    GltfJoint jointArm;
    jointArm.name = "Arm";
    jointArm.parentIndex = 0;
    jointArm.inverseBindMatrix = { 1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1 };

    data.skin.joints.push_back(jointRoot);
    data.skin.joints.push_back(jointArm);

    // 10 wierzcholkow
    for (int i = 0; i < 10; ++i)
    {
        GltfVertex v;
        v.position = { static_cast<float>(i * 2), 0.0f, 0.0f };
        v.normal = { 0.0f, 1.0f, 0.0f };
        v.uv0 = { 0.0f, 0.0f };
        v.uv1 = { 0.0f, 0.0f };
        v.jointIndices = { 1, 0, 0, 0 };
        v.jointWeights = { 1.0f, 0.0f, 0.0f, 0.0f };
        data.vertices.push_back(v);
        data.indices.push_back(i);
    }

    // Prosta animacja rotacji kosci Arm wokol osi Z
    GltfMotionData motion;
    motion.name = "swing";
    motion.duration = 1.0f;

    GltfAnimationChannel chRot;
    chRot.jointIndex = 1;
    chRot.targetNodeName = "Arm";
    chRot.pathType = GltfAnimationPathType::Rotation;
    chRot.times = { 0.0f, 0.5f, 1.0f };
    // 0s: 0 deg, 0.5s: 90 deg wokół Z (0, 0, sin(45), cos(45)), 1s: 180 deg (0, 0, 1, 0)
    chRot.values = {
        { 0.0f, 0.0f, 0.0f, 1.0f },
        { 0.0f, 0.0f, 0.7071068f, 0.7071068f },
        { 0.0f, 0.0f, 1.0f, 0.0f }
    };
    motion.channels.push_back(chRot);
    data.motions.push_back(motion);

    model.SetLoaded(true);

    CGltfModelInstance instance(&model);
    assert(instance.GetDeformedVertices().size() == 10);
    assert(instance.GetDeformedVertices().capacity() >= 10);

    const GltfVertex* pInitialBuffer = instance.GetDeformedVertices().data();
    assert(pInitialBuffer != nullptr);

    instance.PlayMotion("swing", true);

    // Symulacja 120 klatek animacji i deformacji wierzcholkow
    for (int frame = 0; frame < 120; ++frame)
    {
        instance.Update(0.01666f);
        instance.Deform();

        // Wskaznik na bufor w kazdej klatce MUSI byc identyczny (brak jakichkolwiek re-alokacji)
        assert(instance.GetDeformedVertices().data() == pInitialBuffer);
    }

    std::cout << "[PASS] TestDeformedBufferZeroAllocation passed." << std::endl;
}

// ------------------------------------------------------------------------------------------------
// Test 2: Dwa Deform w klatce = jeden skinning (flaga dirty / zapobieganie duplikatom obliczen)
// ------------------------------------------------------------------------------------------------
void TestTwoDeformInFrameOneSkinning()
{
    std::cout << "[RUN] TestTwoDeformInFrameOneSkinning..." << std::endl;

    CGltfModel model;
    GltfModelData& data = model.GetModelData();
    data.name = "TestSkinningDirtyModel";

    GltfJoint joint;
    joint.name = "Root";
    joint.parentIndex = -1;
    joint.inverseBindMatrix = { 1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1 };
    data.skin.joints.push_back(joint);

    GltfVertex v;
    v.position = { 5.0f, 0.0f, 0.0f };
    v.normal = { 0.0f, 1.0f, 0.0f };
    v.uv0 = { 0.0f, 0.0f };
    v.uv1 = { 0.0f, 0.0f };
    v.jointIndices = { 0, 0, 0, 0 };
    v.jointWeights = { 1.0f, 0.0f, 0.0f, 0.0f };
    data.vertices.push_back(v);

    model.SetLoaded(true);

    CGltfModelInstance instance(&model);
    assert(instance.IsSkinningDirty());

    // Pierwsze Deform wykonuje skinning
    instance.Deform();
    assert(!instance.IsSkinningDirty());
    size_t countAfterFirstDeform = instance.GetDeformCount();
    assert(countAfterFirstDeform == 1);

    // Drugie wywolanie Deform w tej samej klatce bez Update
    instance.Deform();
    assert(!instance.IsSkinningDirty());
    assert(instance.GetDeformCount() == 1); // Licznik NIE wzrosl! Obliczenia zostaly pominiete.

    // Trzecie wywolanie DeformVertices
    instance.DeformVertices();
    assert(instance.GetDeformCount() == 1); // Nadal 1!

    // Aktualizacja klatki animacji
    instance.Update(0.016f);
    assert(instance.IsSkinningDirty()); // Oznaczone jako dirty

    // Kolejne Deform wykonuje skinning dokladnie raz
    instance.Deform();
    assert(!instance.IsSkinningDirty());
    assert(instance.GetDeformCount() == 2);

    // Ponowne wywolanie Deform w tej samej klatce nie przelicza ponownie
    instance.Deform();
    assert(instance.GetDeformCount() == 2);

    std::cout << "[PASS] TestTwoDeformInFrameOneSkinning passed." << std::endl;
}

// ------------------------------------------------------------------------------------------------
// Test 3: Poprawnosc obliczen skinningu CPU (rotacja wierzcholkow i normalnych)
// ------------------------------------------------------------------------------------------------
void TestSkinningCorrectness()
{
    std::cout << "[RUN] TestSkinningCorrectness..." << std::endl;

    CGltfModel model;
    GltfModelData& data = model.GetModelData();
    data.name = "SkinningMathModel";

    // 2 kosci: Root w (0,0,0) oraz Hand
    GltfJoint root;
    root.name = "Root";
    root.parentIndex = -1;
    root.inverseBindMatrix = { 1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1 };

    GltfJoint hand;
    hand.name = "Hand";
    hand.parentIndex = 0;
    hand.inverseBindMatrix = { 1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1 };

    data.skin.joints.push_back(root);
    data.skin.joints.push_back(hand);

    // Wierzcholek V0: (10, 0, 0), normalna (1, 0, 0), zwiazany 100% z Hand (indeks 1)
    GltfVertex v0;
    v0.position = { 10.0f, 0.0f, 0.0f };
    v0.normal = { 1.0f, 0.0f, 0.0f };
    v0.uv0 = { 0.0f, 0.0f };
    v0.uv1 = { 0.0f, 0.0f };
    v0.jointIndices = { 1, 0, 0, 0 };
    v0.jointWeights = { 1.0f, 0.0f, 0.0f, 0.0f };
    data.vertices.push_back(v0);

    // Animacja: Hand obrocony o 90 stopni wokol osi Z przy t = 0.5s
    GltfMotionData anim;
    anim.name = "rotate";
    anim.duration = 1.0f;

    GltfAnimationChannel ch;
    ch.jointIndex = 1;
    ch.targetNodeName = "Hand";
    ch.pathType = GltfAnimationPathType::Rotation;
    ch.times = { 0.0f, 0.5f, 1.0f };
    // 0s: 0 deg, 0.5s: 90 deg wokol Z, 1s: 180 deg wokol Z
    ch.values = {
        { 0.0f, 0.0f, 0.0f, 1.0f },
        { 0.0f, 0.0f, 0.7071068f, 0.7071068f },
        { 0.0f, 0.0f, 1.0f, 0.0f }
    };
    anim.channels.push_back(ch);
    data.motions.push_back(anim);

    model.SetLoaded(true);

    CGltfModelInstance instance(&model);
    instance.PlayMotion("rotate", false);

    // t = 0.5s -> rotacja o 90 stopni: (10, 0, 0) -> (0, -10, 0)
    instance.Update(0.5f);
    instance.Deform();

    const auto& defVerts = instance.GetDeformedVertices();
    assert(defVerts.size() == 1);
    assert(ApproxEqualVec3(defVerts[0].position, 0.0f, -10.0f, 0.0f));
    assert(ApproxEqualVec3(defVerts[0].normal, 0.0f, -1.0f, 0.0f));

    // t = 1.0s (kolejne dt = 0.5s) -> rotacja o 180 stopni: (10, 0, 0) -> (-10, 0, 0)
    instance.Update(0.5f);
    instance.Deform();

    const auto& defVerts180 = instance.GetDeformedVertices();
    assert(ApproxEqualVec3(defVerts180[0].position, -10.0f, 0.0f, 0.0f));
    assert(ApproxEqualVec3(defVerts180[0].normal, -1.0f, 0.0f, 0.0f));

    std::cout << "[PASS] TestSkinningCorrectness passed." << std::endl;
}

// ------------------------------------------------------------------------------------------------
// Test 4: Bezpieczne rzutowanie materialow (CheckedMaterialCast)
// ------------------------------------------------------------------------------------------------
struct DummyBase { virtual ~DummyBase() = default; int baseId = 1; };
struct DummyMaterial : public DummyBase { int materialId = 42; };
struct DummyOther : public DummyBase { int otherId = 99; };

void TestCheckedMaterialCast()
{
    std::cout << "[RUN] TestCheckedMaterialCast..." << std::endl;

    // 1. Obsluga nullptr dla typow niepolimorficznych
    GltfMaterial* pNullMat = nullptr;
    GltfMaterial* pCastNull = CheckedMaterialCast<GltfMaterial>(pNullMat);
    assert(pCastNull == nullptr);

    // 2. Poprawne rzutowanie typu niepolimorficznego
    GltfMaterial sampleMat;
    sampleMat.name = "SampleMaterial";
    sampleMat.diffuseTexture = "texture.dds";
    GltfMaterial* pCastMat = CheckedMaterialCast<GltfMaterial>(&sampleMat);
    assert(pCastMat != nullptr);
    assert(pCastMat->name == "SampleMaterial");

    // 3. Obsluga nullptr dla typow polimorficznych
    DummyBase* pNullBase = nullptr;
    DummyMaterial* pNullDerived = CheckedMaterialCast<DummyMaterial>(pNullBase);
    assert(pNullDerived == nullptr);

    // 4. Poprawne rzutowanie polimorficzne typu bazowego na pochodny
    DummyMaterial matObj;
    DummyBase* pBaseToDerived = &matObj;
    DummyMaterial* pDerivedCast = CheckedMaterialCast<DummyMaterial>(pBaseToDerived);
    assert(pDerivedCast != nullptr);
    assert(pDerivedCast->materialId == 42);

    // 5. Bezpieczne zwrocenie nullptr przy niepasujacym typie polimorficznym
    DummyOther otherObj;
    DummyBase* pBaseOther = &otherObj;
    DummyMaterial* pInvalidCast = CheckedMaterialCast<DummyMaterial>(pBaseOther);
    assert(pInvalidCast == nullptr);

    // 6. Test GetCheckedMaterial na CGltfModelInstance
    CGltfModel model;
    GltfModelData& data = model.GetModelData();
    data.name = "MaterialModel";
    GltfMaterial mat1;
    mat1.name = "MatA";
    data.materials.push_back(mat1);
    model.SetLoaded(true);

    CGltfModelInstance instance(&model);
    const GltfMaterial* pFoundMat = instance.GetCheckedMaterial(0);
    assert(pFoundMat != nullptr);
    assert(pFoundMat->name == "MatA");

    // Indeks poza zakresem zwraca nullptr
    const GltfMaterial* pOutOfBounds = instance.GetCheckedMaterial(1);
    assert(pOutOfBounds == nullptr);

    std::cout << "[PASS] TestCheckedMaterialCast passed." << std::endl;
}

int main()
{
    std::cout << "=== START GltfSkinningTest ===" << std::endl;

    TestDeformedBufferZeroAllocation();
    TestTwoDeformInFrameOneSkinning();
    TestSkinningCorrectness();
    TestCheckedMaterialCast();

    std::cout << "=== ALL TESTS PASSED: GltfSkinningTest ===" << std::endl;
    return 0;
}

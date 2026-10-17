#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <cmath>
#include <cstring>

#include "EterModelLib/GltfTypes.h"
#include "EterModelLib/SkeletalMath.h"
#include "EterModelLib/GltfModel.h"
#include "EterModelLib/GltfModelInstance.h"
#include "EterLib/Render/ActorSkinningPipeline.h"

namespace {

std::string ReadProjectFile(const std::string& relativePath)
{
    const char* prefixes[] = { "", "../", "../../", "../../../" };
    for (const char* prefix : prefixes)
    {
        std::ifstream ifs(std::string(prefix) + relativePath);
        if (ifs.is_open())
        {
            std::stringstream ss;
            ss << ifs.rdbuf();
            return ss.str();
        }
    }
    return "";
}

// Emulacja matematyki shadera vs_main 4-bone skinning z rejestrow GPU
float3 SimulateHlslVertexSkinning(
    const float3& pos,
    const uint4& indices,
    const float4& weightsIn,
    const std::vector<float>& boneRegisters4x3)
{
    float w[4] = {
        std::max(0.0f, weightsIn.x),
        std::max(0.0f, weightsIn.y),
        std::max(0.0f, weightsIn.z),
        std::max(0.0f, weightsIn.w)
    };
    float sum = w[0] + w[1] + w[2] + w[3];
    if (sum <= 0.0001f)
        return pos;

    for (int i = 0; i < 4; ++i)
        w[i] /= sum;

    const unsigned int idx[4] = { indices.x, indices.y, indices.z, indices.w };
    float3 skinned = { 0.0f, 0.0f, 0.0f };

    for (int i = 0; i < 4; ++i)
    {
        if (w[i] > 0.0001f)
        {
            int boneIdx = std::min<int>((int)idx[i], 59);
            int baseFloat = boneIdx * 12; // 3 rejestry * 4 floaty

            const float* r0 = &boneRegisters4x3[baseFloat + 0];
            const float* r1 = &boneRegisters4x3[baseFloat + 4];
            const float* r2 = &boneRegisters4x3[baseFloat + 8];

            float lx = pos.x * r0[0] + pos.y * r0[1] + pos.z * r0[2] + 1.0f * r0[3];
            float ly = pos.x * r1[0] + pos.y * r1[1] + pos.z * r1[2] + 1.0f * r1[3];
            float lz = pos.x * r2[0] + pos.y * r2[1] + pos.z * r2[2] + 1.0f * r2[3];

            skinned.x += lx * w[i];
            skinned.y += ly * w[i];
            skinned.z += lz * w[i];
        }
    }

    return skinned;
}

} // namespace

// 1. Weryfikacja bezkolizyjnego ukladu rejestrow stalych HLSL dla palety 60 kosci
TEST_CASE("HlslConstantRegistersHaveZeroCollision")
{
    using Consts = EterLib::Render::ActorSkinningShaderConstants;

    // Globale zajmuja rejestry c0..c13
    CHECK(Consts::kRegViewProj == 0u);
    CHECK(Consts::kRegWorld == 4u);
    CHECK(Consts::kRegLightDir == 8u);
    CHECK(Consts::kRegLightDiffuse == 9u);
    CHECK(Consts::kRegLightAmbient == 10u);
    CHECK(Consts::kRegMaterialColor == 11u);
    CHECK(Consts::kRegFogParams == 12u);
    CHECK(Consts::kRegSkinningInfo == 13u);

    // Paleta kosci zaczyna sie od c16 i obejmuje 60 * 3 = 180 rejestrow (c16..c195)
    CHECK(Consts::kRegBonePalette > Consts::kRegSkinningInfo);
    CHECK(Consts::kRegBonePalette == 16u);
    CHECK(Consts::kLastBoneReg == 195u);
    CHECK(Consts::kLastBoneReg < 256u); // Miesci sie w limicie 256 rejestrow vs_3_0

    std::string hlslSource = ReadProjectFile("src/EterLib/actor_skinning.hlsl");
    REQUIRE_FALSE(hlslSource.empty());

    // Upewnij sie, ze stare kolidujace rejestry c61..c65 nie sa juz uzywane dla swiatla/fog
    CHECK(hlslSource.find("register(c61)") == std::string::npos);
    CHECK(hlslSource.find("register(c62)") == std::string::npos);
    CHECK(hlslSource.find("register(c63)") == std::string::npos);
    CHECK(hlslSource.find("register(c64)") == std::string::npos);
    CHECK(hlslSource.find("register(c65)") == std::string::npos);
    CHECK(hlslSource.find("register(c16)") != std::string::npos);
}

// 2. Weryfikacja ukladu pamieci GpuSkinnedVertex i zgodnosci ze struktura GltfVertex (72B)
TEST_CASE("VertexLayoutStrideAndOffsetsMatch")
{
    using EterLib::Render::GpuSkinnedVertex;
    CHECK(sizeof(GltfVertex) == 72u);
    CHECK(sizeof(GpuSkinnedVertex) == 72u);

    CHECK(offsetof(GpuSkinnedVertex, position) == 0u);
    CHECK(offsetof(GpuSkinnedVertex, normal) == 12u);
    CHECK(offsetof(GpuSkinnedVertex, uv0) == 24u);
    CHECK(offsetof(GpuSkinnedVertex, uv1) == 32u);
    CHECK(offsetof(GpuSkinnedVertex, jointIndices) == 40u);
    CHECK(offsetof(GpuSkinnedVertex, jointWeights) == 56u);
}

// 3. Weryfikacja pakowania macierzy 4x3 do rejestrow i matematycznej zgodnosci ze skinningiem
TEST_CASE("GpuSkinningMathMatchesModelInstancePalette")
{
    GltfModelData data;
    data.name = "test_gpu_skinned_mesh";

    GltfVertex v{};
    v.position = { 2.0f, 4.0f, 6.0f };
    v.normal = { 0.0f, 1.0f, 0.0f };
    v.jointIndices = { 0, 1, 0, 0 };
    v.jointWeights = { 0.5f, 0.5f, 0.0f, 0.0f };
    data.vertices.push_back(v);
    data.indices = { 0, 0, 0 };

    GltfJoint rootJoint;
    rootJoint.name = "Root";
    rootJoint.parentIndex = -1;
    rootJoint.localTranslation = { 10.0f, 0.0f, 0.0f };
    rootJoint.localRotation = { 0.0f, 0.0f, 0.0f, 1.0f };
    rootJoint.localScale = { 1.0f, 1.0f, 1.0f };
    rootJoint.inverseBindMatrix = { 1.0f, 0.0f, 0.0f, 0.0f,  0.0f, 1.0f, 0.0f, 0.0f,  0.0f, 0.0f, 1.0f, 0.0f,  0.0f, 0.0f, 0.0f, 1.0f };

    GltfJoint childJoint;
    childJoint.name = "Child";
    childJoint.parentIndex = 0;
    childJoint.localTranslation = { 0.0f, 20.0f, 0.0f };
    childJoint.localRotation = { 0.0f, 0.0f, 0.0f, 1.0f };
    childJoint.localScale = { 1.0f, 1.0f, 1.0f };
    childJoint.inverseBindMatrix = { 1.0f, 0.0f, 0.0f, 0.0f,  0.0f, 1.0f, 0.0f, 0.0f,  0.0f, 0.0f, 1.0f, 0.0f,  0.0f, 0.0f, 0.0f, 1.0f };

    data.skin.joints.push_back(rootJoint);
    data.skin.joints.push_back(childJoint);

    EterModelLib::CGltfModel model;
    model.SetModelData(data);

    EterModelLib::CGltfModelInstance instance(&model);
    instance.Update(0.0f);

    const auto& skinMatrices = instance.GetSkinningMatrices();
    REQUIRE(skinMatrices.size() == 2u);

    std::vector<float> gpuBoneRegs;
    EterLib::Render::ActorSkinningPipeline::PackBoneMatricesTo4x3Registers(
        &skinMatrices[0].m[0][0],
        (UINT)skinMatrices.size(),
        gpuBoneRegs);

    REQUIRE(gpuBoneRegs.size() == 2u * 12u);

    // Kosc 0 przesuwa o (+10, 0, 0) -> (12, 4, 6)
    // Kosc 1 przesuwa o (+10, +20, 0) -> (12, 24, 6)
    // Blend 50/50 -> (12, 14, 6)
    float3 gpuSkinnedPos = SimulateHlslVertexSkinning(
        v.position, v.jointIndices, v.jointWeights, gpuBoneRegs);

    CHECK(std::abs(gpuSkinnedPos.x - 12.0f) < 1e-3f);
    CHECK(std::abs(gpuSkinnedPos.y - 14.0f) < 1e-3f);
    CHECK(std::abs(gpuSkinnedPos.z - 6.0f) < 1e-3f);
}

// 4. Weryfikacja usuniecia DrawIndexedPrimitiveUP i CPU DeformVertices z LODController
TEST_CASE("LodControllerEliminatesDrawIndexedPrimitiveUPAndCpuDeform")
{
    std::string lodControllerSrc = ReadProjectFile("src/EterGrnLib/LODController.cpp");
    REQUIRE_FALSE(lodControllerSrc.empty());

    CHECK(lodControllerSrc.find("DrawIndexedPrimitiveUP") == std::string::npos);
    CHECK(lodControllerSrc.find("DrawIndexedPrimitive(") != std::string::npos);
    CHECK(lodControllerSrc.find("ActorSkinningPipeline") != std::string::npos);
    CHECK(lodControllerSrc.find("m_pGltfModelInstance->DeformVertices()") == std::string::npos);
}

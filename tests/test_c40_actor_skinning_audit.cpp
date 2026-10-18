#include "doctest.h"
#define TEST_MODE_DISABLE_STDAFX
#ifdef _WIN32
#include <windows.h>
#endif
#include "../src/EterLib/Render/ActorSkinningPipeline.h"
#include "../src/EterLib/Render/ActorSkinningPipeline.cpp"

using namespace EterLib::Render;

TEST_CASE("ActorSkinningPipeline Matrix Packing Verification") {
    SUBCASE("Nullptr matrix array falls back to identity properly") {
        std::vector<float> packedData(12, 0.5f);
        ActorSkinningPipeline::PackBonePaletteTo4x3Registers(nullptr, 1, std::span<float>{packedData});

        CHECK(packedData[0] == 1.0f);
        CHECK(packedData[1] == 0.0f);
        CHECK(packedData[2] == 0.0f);
        CHECK(packedData[3] == 0.0f);

        CHECK(packedData[4] == 0.0f);
        CHECK(packedData[5] == 1.0f);
        CHECK(packedData[6] == 0.0f);
        CHECK(packedData[7] == 0.0f);

        CHECK(packedData[8] == 0.0f);
        CHECK(packedData[9] == 0.0f);
        CHECK(packedData[10] == 1.0f);
        CHECK(packedData[11] == 0.0f);
    }

    SUBCASE("Valid D3DXMATRIX transforms correctly to 4x3 column-major register format") {
        float mat[16] = {
            1.0f,  2.0f,  3.0f,  4.0f,
            5.0f,  6.0f,  7.0f,  8.0f,
            9.0f,  10.0f, 11.0f, 12.0f,
            13.0f, 14.0f, 15.0f, 16.0f
        };

        std::vector<float> packedData(12, 0.0f);
        ActorSkinningPipeline::PackBonePaletteTo4x3Registers(mat, 1, std::span<float>{packedData});

        // dst[0] = m[0], dst[1] = m[4], dst[2] = m[8], dst[3] = m[12]
        CHECK(packedData[0] == 1.0f);
        CHECK(packedData[1] == 5.0f);
        CHECK(packedData[2] == 9.0f);
        CHECK(packedData[3] == 13.0f);

        // dst[4] = m[1], dst[5] = m[5], dst[6] = m[9], dst[7] = m[13]
        CHECK(packedData[4] == 2.0f);
        CHECK(packedData[5] == 6.0f);
        CHECK(packedData[6] == 10.0f);
        CHECK(packedData[7] == 14.0f);

        // dst[8] = m[2], dst[9] = m[6], dst[10] = m[10], dst[11] = m[14]
        CHECK(packedData[8] == 3.0f);
        CHECK(packedData[9] == 7.0f);
        CHECK(packedData[10] == 11.0f);
        CHECK(packedData[11] == 15.0f);
    }

    SUBCASE("Proxy vector override ensures capacity") {
        float mat[16] = {0}; // Any content
        std::vector<float> packedData;
        ActorSkinningPipeline::PackBonePaletteTo4x3Registers(mat, 1, packedData);
        CHECK(packedData.size() == 12);
    }

    SUBCASE("Array exhaustion and limits respect maximum bones (80)") {
        float mats[85 * 16] = {0}; // Over kMaxBones
        std::vector<float> packedData;
        ActorSkinningPipeline::PackBonePaletteTo4x3Registers(mats, 85, packedData);
        CHECK(packedData.size() == 80 * 12); // Clamped
    }

    SUBCASE("Insufficient span size prevents writing out of bounds") {
        float mat[16] = {1.0f};
        std::vector<float> smallData(10, 0.0f); // Less than 12
        ActorSkinningPipeline::PackBonePaletteTo4x3Registers(mat, 1, std::span<float>{smallData});
        
        // Should not have modified smallData elements because it aborts due to size constraint
        for (int i = 0; i < 10; ++i) {
            CHECK(smallData[i] == 0.0f);
        }
    }
}

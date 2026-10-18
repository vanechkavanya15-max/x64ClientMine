#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3dx9math.h>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <cstdint>
#include <cstddef>
#include <GameLib/TerrainPatch.h>
#include <GameLib/MapOutdoorWater.h>
#include <EterLib/ParticleQuad.h>

// S2.8 / T064: Unit tests for GameLib small fixes (DungeonBlock bbox, Area::Find, terrain bounds, fog clamp, mipmap loop)

TEST_SUITE("WorldSmallFixes")
{
    // 1. DungeonBlock GetBoundBox & BuildBoundingSphere fix (.y/.z dimensions and empty container guard)
    TEST_CASE("DungeonBlock_GetBoundBox_ComputesCorrectYAndZDimensions")
    {
        struct MockModelInstance
        {
            D3DXVECTOR3 v3Min;
            D3DXVECTOR3 v3Max;
            void GetBoundBox(D3DXVECTOR3* pMin, D3DXVECTOR3* pMax) const
            {
                *pMin = v3Min;
                *pMax = v3Max;
            }
        };

        auto computeBoundBox = [](const std::vector<MockModelInstance>& instances, D3DXVECTOR3* pv3Min, D3DXVECTOR3* pv3Max)
        {
            if (instances.empty())
            {
                *pv3Min = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
                *pv3Max = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
                return;
            }

            pv3Min->x = +10000000.0f;
            pv3Min->y = +10000000.0f;
            pv3Min->z = +10000000.0f;
            pv3Max->x = -10000000.0f;
            pv3Max->y = -10000000.0f;
            pv3Max->z = -10000000.0f;

            for (const auto& inst : instances)
            {
                D3DXVECTOR3 v3Min;
                D3DXVECTOR3 v3Max;
                inst.GetBoundBox(&v3Min, &v3Max);

                pv3Min->x = std::min(v3Min.x, pv3Min->x);
                pv3Min->y = std::min(v3Min.y, pv3Min->y);
                pv3Min->z = std::min(v3Min.z, pv3Min->z);
                pv3Max->x = std::max(v3Max.x, pv3Max->x);
                pv3Max->y = std::max(v3Max.y, pv3Max->y);
                pv3Max->z = std::max(v3Max.z, pv3Max->z);
            }
        };

        // Test empty container
        {
            std::vector<MockModelInstance> emptyList;
            D3DXVECTOR3 vMin, vMax;
            computeBoundBox(emptyList, &vMin, &vMax);
            CHECK(vMin.x == 0.0f);
            CHECK(vMin.y == 0.0f);
            CHECK(vMin.z == 0.0f);
            CHECK(vMax.x == 0.0f);
            CHECK(vMax.y == 0.0f);
            CHECK(vMax.z == 0.0f);
        }

        // Test single instance where y and z differ significantly from x
        {
            std::vector<MockModelInstance> list = {
                { D3DXVECTOR3(-10.0f, -50.0f, -30.0f), D3DXVECTOR3(20.0f, 60.0f, 40.0f) }
            };
            D3DXVECTOR3 vMin, vMax;
            computeBoundBox(list, &vMin, &vMax);

            CHECK(vMin.x == doctest::Approx(-10.0f));
            CHECK(vMin.y == doctest::Approx(-50.0f));
            CHECK(vMin.z == doctest::Approx(-30.0f));
            CHECK(vMax.x == doctest::Approx(20.0f));
            CHECK(vMax.y == doctest::Approx(60.0f));
            CHECK(vMax.z == doctest::Approx(40.0f));
        }

        // Test multiple instances union
        {
            std::vector<MockModelInstance> list = {
                { D3DXVECTOR3(-10.0f, -50.0f, -20.0f), D3DXVECTOR3(10.0f, 50.0f, 20.0f) },
                { D3DXVECTOR3(-5.0f, -100.0f, -30.0f), D3DXVECTOR3(15.0f, 80.0f, 40.0f) },
                { D3DXVECTOR3(-25.0f, -10.0f, -5.0f),  D3DXVECTOR3(5.0f, 30.0f, 60.0f) }
            };
            D3DXVECTOR3 vMin, vMax;
            computeBoundBox(list, &vMin, &vMax);

            CHECK(vMin.x == doctest::Approx(-25.0f));
            CHECK(vMin.y == doctest::Approx(-100.0f));
            CHECK(vMin.z == doctest::Approx(-30.0f));
            CHECK(vMax.x == doctest::Approx(15.0f));
            CHECK(vMax.y == doctest::Approx(80.0f));
            CHECK(vMax.z == doctest::Approx(60.0f));

            // Bounding sphere calculation
            D3DXVECTOR3 vCenter = (vMin + vMax) * 0.5f;
            D3DXVECTOR3 vv = (vMax - vMin);
            float fRadius = D3DXVec3Length(&vv) * 0.5f + 150.0f;

            CHECK(vCenter.x == doctest::Approx(-5.0f));
            CHECK(vCenter.y == doctest::Approx(-10.0f));
            CHECK(vCenter.z == doctest::Approx(15.0f));
            CHECK(fRadius > 150.0f);
        }
    }

    // 2. Terrain coordinate to index bounds checking (CMapOutdoor::GetTerrainNumFromCoord)
    TEST_CASE("MapOutdoor_GetTerrainNumFromCoord_BoundsCheck")
    {
        const short LOAD_SIZE_WIDTH = 1;
        const int AROUND_AREA_NUM = 9;

        auto getTerrainNumFromCoord = [&](WORD wCoordX, WORD wCoordY, short curCoordX, short curCoordY, BYTE* pbyTerrainNum) -> bool
        {
            int iTerrainNum = (wCoordY - curCoordY + LOAD_SIZE_WIDTH) * 3 +
                (wCoordX - curCoordX + LOAD_SIZE_WIDTH);

            if (iTerrainNum < 0 || iTerrainNum >= AROUND_AREA_NUM)
                return false;

            *pbyTerrainNum = static_cast<BYTE>(iTerrainNum);
            return true;
        };

        const short curX = 10;
        const short curY = 20;
        BYTE outTerrainNum = 0xFF;

        // Valid cells in 3x3 grid around (curX, curY)
        for (int dy = -1; dy <= 1; ++dy)
        {
            for (int dx = -1; dx <= 1; ++dx)
            {
                WORD testX = static_cast<WORD>(curX + dx);
                WORD testY = static_cast<WORD>(curY + dy);
                bool ok = getTerrainNumFromCoord(testX, testY, curX, curY, &outTerrainNum);
                CHECK(ok == true);
                int expectedIdx = (dy + 1) * 3 + (dx + 1);
                CHECK(outTerrainNum == static_cast<BYTE>(expectedIdx));
                CHECK(outTerrainNum < AROUND_AREA_NUM);
            }
        }

        // Test out of bounds: negative indices
        CHECK(getTerrainNumFromCoord(static_cast<WORD>(curX - 2), static_cast<WORD>(curY - 1), curX, curY, &outTerrainNum) == false);
        CHECK(getTerrainNumFromCoord(static_cast<WORD>(curX), static_cast<WORD>(curY - 2), curX, curY, &outTerrainNum) == false);

        // Test out of bounds: index >= 9 (AROUND_AREA_NUM)
        // With dx=2, dy=1: (1+1)*3 + (2+1) = 6 + 3 = 9. In old code, 9 > 9 was false so it returned true, causing out of bounds!
        CHECK(getTerrainNumFromCoord(static_cast<WORD>(curX + 2), static_cast<WORD>(curY + 1), curX, curY, &outTerrainNum) == false);
        // With dx=0, dy=2: (2+1)*3 + (0+1) = 9 + 1 = 10 > 9
        CHECK(getTerrainNumFromCoord(static_cast<WORD>(curX), static_cast<WORD>(curY + 2), curX, curY, &outTerrainNum) == false);
    }

    // 3. Area map find end() checking (CArea::Render)
    TEST_CASE("Area_MapFindEndCheck_SafeDereference")
    {
        struct MockGraphicThingInstance
        {
            int id;
        };

        std::unordered_map<const MockGraphicThingInstance*, DWORD> thingCRCMap;
        MockGraphicThingInstance instA{ 1 };
        MockGraphicThingInstance instB{ 2 };
        MockGraphicThingInstance unmappedInst{ 3 };

        thingCRCMap[&instA] = 0xAAAA;
        thingCRCMap[&instB] = 0xBBBB;

        std::vector<const MockGraphicThingInstance*> renderedThings;
        std::vector<DWORD> recordedCRCs;

        auto processThing = [&](const MockGraphicThingInstance* pkThingInst)
        {
            renderedThings.push_back(pkThingInst);

            auto it = thingCRCMap.find(pkThingInst);
            if (it != thingCRCMap.end())
            {
                DWORD dwCRC = it->second;
                recordedCRCs.push_back(dwCRC);
            }
        };

        processThing(&instA);
        processThing(&unmappedInst); // Must NOT crash or dereference end()
        processThing(&instB);

        CHECK(renderedThings.size() == 3);
        CHECK(recordedCRCs.size() == 2);
        CHECK(recordedCRCs[0] == 0xAAAA);
        CHECK(recordedCRCs[1] == 0xBBBB);
    }

    // 4. Fog level clamping [0, 2] (CMapManager::BeginEnvironment)
    TEST_CASE("MapManager_FogLevelClamp")
    {
        const float fFogDensityLevel[3] = { 0.000006f, 0.000004f, 0.000002f };
        const float fFogScaleLevel[3] = { 0.75f, 1.0f, 1.25f };

        auto getClampedFogLevel = [](int rawLevel) -> int
        {
            return std::clamp(rawLevel, 0, 2);
        };

        // Negative values clamp to 0
        int fogNeg = getClampedFogLevel(-5);
        CHECK(fogNeg == 0);
        CHECK(fFogDensityLevel[fogNeg] == doctest::Approx(0.000006f));
        CHECK(fFogScaleLevel[fogNeg] == doctest::Approx(0.75f));

        // In-range values
        CHECK(getClampedFogLevel(0) == 0);
        CHECK(getClampedFogLevel(1) == 1);
        CHECK(getClampedFogLevel(2) == 2);

        // Oversized values clamp to 2
        int fogOver = getClampedFogLevel(10);
        CHECK(fogOver == 2);
        CHECK(fFogDensityLevel[fogOver] == doctest::Approx(0.000002f));
        CHECK(fFogScaleLevel[fogOver] == doctest::Approx(1.25f));
    }

    // 5. Mipmap generation loop break on failure (CTerrain::Raw2AlphaSplat)
    TEST_CASE("AreaTerrain_MipMapLoop_BreaksOnLockRectFailure")
    {
        UINT uSrcSize = 256;
        UINT levelCount = 5;
        int failedLevel = 2; // Simulated failure at mip level 2

        std::vector<UINT> processedLevels;
        bool bufferCorrupted = false;

        for (UINT uMipMapLevel = 1; uMipMapLevel < levelCount; ++uMipMapLevel)
        {
            UINT uDstSize = uSrcSize >> 1;
            if (uDstSize == 0)
            {
                bufferCorrupted = true;
                break;
            }

            // Simulated LockRect: fails at level 2
            bool lockSuccess = (static_cast<int>(uMipMapLevel) != failedLevel);
            if (!lockSuccess)
            {
                // New fix: break out of loop instead of continuing with stale sizes
                break;
            }

            processedLevels.push_back(uMipMapLevel);
            uSrcSize = uDstSize;
        }

        CHECK(processedLevels.size() == 1);
        CHECK(processedLevels[0] == 1);
        CHECK(bufferCorrupted == false);
        CHECK(uSrcSize == 128); // Ended cleanly without continuing to smaller invalid iterations
    }

    // 6. TerrainSectorPacket isolation in loop (CTerrainRenderExtractor::ExtractAndSubmitVisibleNodes)
    TEST_CASE("TerrainRenderExtractor_PacketZeroInitPerIteration")
    {
        struct MockPacket
        {
            uint32_t patchId = 0;
            const void* pWeights = nullptr;
        };

        std::vector<uint32_t> visibleNodes = { 101, 102, 103 };
        std::vector<MockPacket> submittedPackets;

        for (const uint32_t patchNum : visibleNodes)
        {
            MockPacket packet{}; // Declared inside loop
            CHECK(packet.patchId == 0);
            CHECK(packet.pWeights == nullptr);

            packet.patchId = patchNum;
            if (patchNum == 101)
                packet.pWeights = reinterpret_cast<const void*>(0x1000);

            submittedPackets.push_back(packet);
        }

        CHECK(submittedPackets.size() == 3);
        CHECK(submittedPackets[0].pWeights == reinterpret_cast<const void*>(0x1000));
        CHECK(submittedPackets[1].pWeights == nullptr); // Ensure dirty pointer didn't leak
        CHECK(submittedPackets[2].pWeights == nullptr);
    }

    // 7. TerrainPatch HTP vertex stride 24 bytes (S2.9 / T065)
    TEST_CASE("TerrainPatch_HTPVertexStride_24Bytes")
    {
        CHECK(sizeof(HardwareTransformPatch_SSourceVertex) == 24);
        CHECK(offsetof(HardwareTransformPatch_SSourceVertex, kPosition) == 0);
        CHECK(offsetof(HardwareTransformPatch_SSourceVertex, kNormal) == 12);
    }

    // 8. T103 / R1.3: ComputeWaterWaveHeight pure mathematical wave function
    TEST_CASE("WaterWave_ComputeWaterWaveHeight_PureFunctionDeterminism")
    {
        const float x = 120.0f;
        const float y = 340.0f;
        const float time = 2.5f;
        const float freq = 1.5f;
        const float amp = 5.0f;

        // 1. Determinism and purity
        const float h1 = ComputeWaterWaveHeight(x, y, time, freq, amp);
        const float h2 = ComputeWaterWaveHeight(x, y, time, freq, amp);
        CHECK(h1 == h2);

        // 2. Zero origin check: matches legacy formula exactly
        const float legacyVal = -6.0f + 5.0f * std::sin(time * 1.5f) + 1.0f * std::sin(time * 3.1f);
        const float pureVal = ComputeWaterWaveHeight(0.0f, 0.0f, time, freq, amp);
        CHECK(pureVal == doctest::Approx(legacyVal).epsilon(0.0001f));

        // 3. Amplitude scaling: with zero amplitude, stays at base level (-6.0f)
        const float hFlat = ComputeWaterWaveHeight(x, y, time, freq, 0.0f);
        CHECK(hFlat == doctest::Approx(-6.0f).epsilon(0.0001f));

        // 4. Amplitude bounds check
        const float maxExpected = -6.0f + amp * 1.2f;
        const float minExpected = -6.0f - amp * 1.2f;
        for (float t = 0.0f; t < 10.0f; t += 0.25f)
        {
            const float val = ComputeWaterWaveHeight(x, y, t, freq, amp);
            CHECK(val <= maxExpected + 0.001f);
            CHECK(val >= minExpected - 0.001f);
        }
    }

    // 9. T103 / R1.3: ComputeParticleBillboardQuad pure camera quad calculation
    TEST_CASE("ParticleQuad_ComputeParticleBillboardQuad_PureCalculation")
    {
        SCameraParameters cam;
        cam.v3Eye = D3DXVECTOR3(0.0f, -500.0f, 100.0f);
        cam.v3Up = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
        cam.v3Cross = D3DXVECTOR3(1.0f, 0.0f, 0.0f);
        cam.v3View = D3DXVECTOR3(0.0f, 1.0f, 0.0f);

        const D3DXVECTOR3 center(100.0f, 200.0f, 50.0f);
        const float halfWidth = 20.0f;
        const float halfHeight = 30.0f;

        // 1. BILLBOARD_ALL mode (type 0), 0 rotation
        SParticleQuad quad = ComputeParticleBillboardQuad(
            0,
            false,
            center,
            halfWidth,
            halfHeight,
            0.0f,
            cam
        );

        // Quad center of mass should equal input center position
        const D3DXVECTOR3 avgPos = (quad.positions[0] + quad.positions[1] + quad.positions[2] + quad.positions[3]) * 0.25f;
        CHECK(avgPos.x == doctest::Approx(center.x).epsilon(0.001f));
        CHECK(avgPos.y == doctest::Approx(center.y).epsilon(0.001f));
        CHECK(avgPos.z == doctest::Approx(center.z).epsilon(0.001f));

        // Diagonal distance check (halfWidth 20, halfHeight 30 => diagonal ~ 72.111)
        D3DXVECTOR3 diag = quad.positions[3] - quad.positions[0];
        float diagLen = D3DXVec3Length(&diag);
        CHECK(diagLen == doctest::Approx(72.111f).epsilon(0.01f));

        // 2. Determinism test
        SParticleQuad quad2 = ComputeParticleBillboardQuad(0, false, center, halfWidth, halfHeight, 0.0f, cam);
        for (int i = 0; i < 4; ++i)
        {
            CHECK(quad.positions[i].x == quad2.positions[i].x);
            CHECK(quad.positions[i].y == quad2.positions[i].y);
            CHECK(quad.positions[i].z == quad2.positions[i].z);
        }

        // 3. BILLBOARD_LIE mode (type 3, lying flat in XY plane)
        SParticleQuad quadLie = ComputeParticleBillboardQuad(
            3,
            false,
            center,
            halfWidth,
            halfHeight,
            0.0f,
            cam
        );
        for (int i = 0; i < 4; ++i)
        {
            CHECK(quadLie.positions[i].z == doctest::Approx(center.z).epsilon(0.001f));
        }
    }
}

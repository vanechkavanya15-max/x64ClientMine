#include "../src/UserInterface/MiniMap/MiniMapCoordinateProjector.h"
#include <iostream>
#include <cassert>
#include <cmath>

using namespace MiniMap;

int main()
{
    std::cout << "[TEST] Start MiniMapCoordinateProjector unit tests..." << std::endl;

    // 1. Test ComputeCenterCell
    {
        Vector2 cell1 = MiniMapCoordinateProjector::ComputeCenterCell(12800.0f, 12800.0f, 0, 0, 25600.0f, 25600.0f, 200.0f);
        assert(std::fabs(cell1.x - 64.0f) < 0.001f);
        assert(std::fabs(cell1.y - 64.0f) < 0.001f);

        Vector2 cell2 = MiniMapCoordinateProjector::ComputeCenterCell(25600.0f + 12800.0f, 25600.0f + 6400.0f, 1, 1, 25600.0f, 25600.0f, 200.0f);
        assert(std::fabs(cell2.x - 64.0f) < 0.001f);
        assert(std::fabs(cell2.y - 32.0f) < 0.001f);
        std::cout << "  -> ComputeCenterCell: PASS" << std::endl;
    }

    // 2. Test NormalizeRotation
    {
        assert(std::fabs(MiniMapCoordinateProjector::NormalizeRotation(180.0f) - 360.0f) < 0.001f ||
               std::fabs(MiniMapCoordinateProjector::NormalizeRotation(180.0f) - 0.0f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::NormalizeRotation(0.0f) - 180.0f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::NormalizeRotation(90.0f) - 90.0f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::NormalizeRotation(270.0f) - 270.0f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::NormalizeRotation(540.0f) - 0.0f) < 0.001f);
        std::cout << "  -> NormalizeRotation: PASS" << std::endl;
    }

    // 3. Test ClampScale, StepScaleUp, StepScaleDown
    {
        assert(std::fabs(MiniMapCoordinateProjector::ClampScale(0.1f) - 0.5f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::ClampScale(10.0f) - 4.0f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::ClampScale(2.0f) - 2.0f) < 0.001f);

        assert(std::fabs(MiniMapCoordinateProjector::StepScaleUp(2.0f) - 4.0f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::StepScaleUp(4.0f) - 4.0f) < 0.001f);

        assert(std::fabs(MiniMapCoordinateProjector::StepScaleDown(2.0f) - 1.0f) < 0.001f);
        assert(std::fabs(MiniMapCoordinateProjector::StepScaleDown(0.5f) - 0.5f) < 0.001f);
        std::cout << "  -> Scale clamping & stepping: PASS" << std::endl;
    }

    // 4. Test ComputeMiniMapRadius
    {
        // 128x128 window, border 9 -> windowRadius = 64 - 9 = 55
        float r1 = MiniMapCoordinateProjector::ComputeMiniMapRadius(128.0f, 128.0f, 1.0f, 200.0f, 9.0f);
        // maxCellRadius = (6400 / 200) * 1.0 = 32.0f. min(32, 55) = 32.0f
        assert(std::fabs(r1 - 32.0f) < 0.001f);

        float r2 = MiniMapCoordinateProjector::ComputeMiniMapRadius(128.0f, 128.0f, 2.0f, 200.0f, 9.0f);
        // maxCellRadius = (6400 / 200) * 2.0 = 64.0f. min(64, 55) = 55.0f
        assert(std::fabs(r2 - 55.0f) < 0.001f);
        std::cout << "  -> ComputeMiniMapRadius: PASS" << std::endl;
    }

    // 5. Test ProjectWorldToMiniMap and ProjectWorld
    {
        float outX = 0.0f, outY = 0.0f, outDist = 0.0f;
        bool inside = MiniMapCoordinateProjector::ProjectWorldToMiniMap(
            5000.0f, 5000.0f,
            5000.0f, 5000.0f,
            2.0f,
            128.0f, 128.0f,
            55.0f,
            100.0f, 200.0f,
            6.0f, 6.0f,
            &outX, &outY,
            &outDist,
            200.0f);

        assert(inside);
        assert(std::fabs(outDist - 0.0f) < 0.001f);
        // center is (128 - 6)/2 = 61 + screen pos (100, 200) -> 161, 261
        assert(std::fabs(outX - 161.0f) < 0.001f);
        assert(std::fabs(outY - 261.0f) < 0.001f);

        // Outside test
        bool outside = MiniMapCoordinateProjector::ProjectWorldToMiniMap(
            50000.0f, 50000.0f,
            5000.0f, 5000.0f,
            2.0f,
            128.0f, 128.0f,
            55.0f,
            0.0f, 0.0f,
            6.0f, 6.0f,
            &outX, &outY);
        assert(!outside);
        std::cout << "  -> ProjectWorldToMiniMap: PASS" << std::endl;
    }

    // 6. Test ScreenToWorldPosition (Reverse Projection & Roundtrip)
    {
        float worldX = 0.0f, worldY = 0.0f;
        bool ok = MiniMapCoordinateProjector::ScreenToWorldPosition(
            64.0f, 64.0f,
            0.0f, 0.0f,
            128.0f, 128.0f,
            55.0f,
            10000.0f, 20000.0f,
            2.0f,
            &worldX, &worldY,
            200.0f);

        assert(ok);
        assert(std::fabs(worldX - 10000.0f) < 0.001f);
        assert(std::fabs(worldY - 20000.0f) < 0.001f);

        // Roundtrip check: World -> Screen -> World
        float origWorldX = 11200.0f, origWorldY = 20800.0f;
        float scrX = 0.0f, scrY = 0.0f;
        bool projOk = MiniMapCoordinateProjector::ProjectWorldToMiniMap(
            origWorldX, origWorldY,
            10000.0f, 20000.0f,
            2.0f,
            128.0f, 128.0f,
            55.0f,
            0.0f, 0.0f,
            0.0f, 0.0f,
            &scrX, &scrY,
            nullptr,
            200.0f);
        assert(projOk);

        float recoveredX = 0.0f, recoveredY = 0.0f;
        bool revOk = MiniMapCoordinateProjector::ScreenToWorldPosition(
            scrX, scrY,
            0.0f, 0.0f,
            128.0f, 128.0f,
            55.0f,
            10000.0f, 20000.0f,
            2.0f,
            &recoveredX, &recoveredY,
            200.0f);
        assert(revOk);
        assert(std::fabs(recoveredX - origWorldX) < 0.01f);
        assert(std::fabs(recoveredY - origWorldY) < 0.01f);
        std::cout << "  -> ScreenToWorldPosition roundtrip: PASS" << std::endl;
    }

    // 7. Test ProjectTargetToEdge (Clamping to circular edge)
    {
        Vector2 targetEdge = MiniMapCoordinateProjector::ProjectTargetToEdge(
            10000.0f, 100000.0f, // far along Y
            10000.0f, 20000.0f,
            2.0f,
            128.0f, 128.0f,
            0.0f, 0.0f,
            6.0f, 6.0f,
            9.0f,
            200.0f);

        // Center on X = (128 - 6)/2 = 61.0f
        assert(std::fabs(targetEdge.x - 61.0f) < 0.01f);
        // Center on Y + radius 55 = 61.0f + 55.0f = 116.0f
        assert(std::fabs(targetEdge.y - 116.0f) < 0.01f);
        std::cout << "  -> ProjectTargetToEdge: PASS" << std::endl;
    }

    // 8. Test InterpolateObserver
    {
        Vector2 pStart = MiniMapCoordinateProjector::InterpolateObserver(100.0f, 200.0f, 300.0f, 400.0f, 1000, 2000, 1000);
        assert(std::fabs(pStart.x - 100.0f) < 0.001f);
        assert(std::fabs(pStart.y - 200.0f) < 0.001f);

        Vector2 pMid = MiniMapCoordinateProjector::InterpolateObserver(100.0f, 200.0f, 300.0f, 400.0f, 1000, 2000, 1500);
        assert(std::fabs(pMid.x - 200.0f) < 0.001f);
        assert(std::fabs(pMid.y - 300.0f) < 0.001f);

        Vector2 pEnd = MiniMapCoordinateProjector::InterpolateObserver(100.0f, 200.0f, 300.0f, 400.0f, 1000, 2000, 2500);
        assert(std::fabs(pEnd.x - 300.0f) < 0.001f);
        assert(std::fabs(pEnd.y - 400.0f) < 0.001f);
        std::cout << "  -> InterpolateObserver: PASS" << std::endl;
    }

    // 9. Test Atlas Projections & Roundtrip
    {
        AtlasOffsets offsets = MiniMapCoordinateProjector::ComputeAtlasCenteringOffsets(512.0f, 512.0f, 25600.0f, 25600.0f);
        assert(std::fabs(offsets.uniformScale - (512.0f / 25600.0f)) < 0.0001f);
        assert(std::fabs(offsets.offsetX - 0.0f) < 0.001f);
        assert(std::fabs(offsets.offsetY - 0.0f) < 0.001f);

        float origWx = 12345.0f, origWy = 16789.0f;
        Vector2 atlasPos = MiniMapCoordinateProjector::GlobalToAtlasPosition(origWx, origWy, offsets.uniformScale, offsets.offsetX, offsets.offsetY);
        Vector2 roundtrip = MiniMapCoordinateProjector::AtlasToGlobalPosition(atlasPos.x, atlasPos.y, offsets.uniformScale, offsets.offsetX, offsets.offsetY);
        assert(std::fabs(roundtrip.x - origWx) < 0.01f);
        assert(std::fabs(roundtrip.y - origWy) < 0.01f);
        std::cout << "  -> Atlas Projections Roundtrip: PASS" << std::endl;
    }

    // 10. Test ComputeTerrainMeshTransform
    {
        TerrainMeshTransform t = MiniMapCoordinateProjector::ComputeTerrainMeshTransform(128.0f, 128.0f, 2.0f, 32.0f, 32.0f, 10.0f, 20.0f);
        assert(std::fabs(t.scaleX - 256.0f) < 0.001f);
        assert(std::fabs(t.scaleY - 256.0f) < 0.001f);
        // translationX: (1 + 2) * 128 * 0.5 - 32 * 2 + 10 = 192 - 64 + 10 = 138
        assert(std::fabs(t.translationX - 138.0f) < 0.001f);
        // translationY: 192 - 64 + 20 = 148
        assert(std::fabs(t.translationY - 148.0f) < 0.001f);
        std::cout << "  -> ComputeTerrainMeshTransform: PASS" << std::endl;
    }

    // 11. Test Stateful Projector Object
    {
        MiniMapCoordinateProjector projector;
        projector.SetDimensions(256.0f, 256.0f);
        projector.SetScale(2.0f);
        projector.SetCenterPosition(1000.0f, 2000.0f);
        projector.SetScreenPosition(50.0f, 60.0f);
        assert(std::fabs(projector.GetWidth() - 256.0f) < 0.001f);
        assert(std::fabs(projector.GetScale() - 2.0f) < 0.001f);

        float px = 0.0f, py = 0.0f;
        bool ok = projector.Project(1000.0f, 2000.0f, 8.0f, 8.0f, &px, &py);
        assert(ok);
        // (256 - 8)/2 + 0 + 50 = 124 + 50 = 174
        assert(std::fabs(px - 174.0f) < 0.001f);
        // 124 + 60 = 184
        assert(std::fabs(py - 184.0f) < 0.001f);
        std::cout << "  -> Stateful Projector: PASS" << std::endl;
    }

    std::cout << "[SUCCESS] ALL 11 TESTS PASSED 100%!" << std::endl;
    return 0;
}

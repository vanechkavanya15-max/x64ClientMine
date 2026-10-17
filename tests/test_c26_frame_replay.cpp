#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <cstdio>
#include <cmath>

#include "EterLib/RenderQueueManager.h"
#include "EterLib/FrameReplayPlayer.h"

TEST_CASE("FrameReplay: Export, Load, Inspect and Replay M2FR v2 dump")
{
    const char* c_pszDumpFile = "test_frame_dump_v2.bin";

    // 1. Przygotowanie klatki testowej w kolejce renderera
    CRenderQueueManager queue;
    queue.BeginFrame();

    TRenderFrameUniforms uniforms{};
    uniforms.dwFrameIndex = 42;
    uniforms.fTime = 123.456f;
    uniforms.fFogNear = 100.0f;
    uniforms.fFogFar = 5000.0f;
    queue.SetFrameUniforms(uniforms);

    // Pakiet SkyBox
    TSkyBoxPacket skyPkt{};
    skyPkt.dwSkyTextureID = 10;
    queue.SubmitSkyBox(skyPkt);

    // Pakiet Terrain
    TTerrainSectorPacket terrainPkt{};
    terrainPkt.dwSectorX = 3;
    terrainPkt.dwSectorY = 7;
    queue.SubmitTerrain(terrainPkt);

    // Pakiet Water
    TWaterPacket waterPkt{};
    waterPkt.patchNum = 15;
    waterPkt.fWaterHeight = -6.0f;
    queue.SubmitWater(waterPkt);

    // Pakiet Dungeon
    TDungeonBlockPacket dungeonPkt{};
    dungeonPkt.dwDiffuseTextureID = 99;
    queue.SubmitDungeonBlock(dungeonPkt);

    // Eksport do pliku v2
    bool bExportOk = queue.ExportFrameToBinaryV2(c_pszDumpFile);
    CHECK(bExportOk == true);

    // 2. Ladowanie przez CFrameReplayPlayer
    CFrameReplayPlayer player;
    bool bLoadOk = player.LoadDump(c_pszDumpFile);
    CHECK(bLoadOk == true);
    CHECK(player.GetDumpVersion() == 2u);
    CHECK(player.GetChunkCount() >= 4u);

    // 3. Sprawdzenie podsumowania zrzutu
    FrameReplay::SDumpSummary summary = player.GetSummary();
    CHECK(summary.dwSkyBoxPackets == 1u);
    CHECK(summary.dwTerrainPackets == 1u);
    CHECK(summary.dwWaterPackets == 1u);
    CHECK(summary.dwDungeonPackets == 1u);
    CHECK(summary.dwFrameIndex == 42u);
    CHECK(std::abs(summary.fTime - 123.456f) < 1e-3f);

    const FrameReplay::SReplayStats& stats = player.GetStats();
    CHECK(stats.bSuccess == true);
    CHECK(stats.dwDumpVersion == 2u);
    CHECK(stats.dwTotalPackets >= 4u);

    // 4. Replay do innej instancji kolejki
    CRenderQueueManager targetQueue;
    bool bReplayOk = player.ReplayToQueue(targetQueue);
    CHECK(bReplayOk == true);

    CHECK(targetQueue.GetSkyBoxPackets().size() == 1u);
    CHECK(targetQueue.GetSkyBoxPackets()[0].dwSkyTextureID == 10u);

    CHECK(targetQueue.GetTerrainPackets().size() == 1u);
    CHECK(targetQueue.GetTerrainPackets()[0].dwSectorX == 3u);
    CHECK(targetQueue.GetTerrainPackets()[0].dwSectorY == 7u);

    CHECK(targetQueue.GetWaterPackets().size() == 1u);
    CHECK(targetQueue.GetWaterPackets()[0].patchNum == 15u);
    CHECK(targetQueue.GetWaterPackets()[0].fWaterHeight == -6.0f);

    CHECK(targetQueue.GetDungeonPackets().size() == 1u);
    CHECK(targetQueue.GetDungeonPackets()[0].dwDiffuseTextureID == 99u);

    CHECK(targetQueue.GetFrameUniforms().dwFrameIndex == 42u);
    CHECK(std::abs(targetQueue.GetFrameUniforms().fTime - 123.456f) < 1e-3f);

    // Czyszczenie pliku testowego
    remove(c_pszDumpFile);
}

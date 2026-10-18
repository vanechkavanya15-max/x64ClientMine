#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/WgpuRenderBackend.h"
#include <vector>
#include <cstring>

extern bool g_bHeadlessMode;

TEST_CASE("WGPU Backend Caching: BeginFrame Clears All Caches")
{
    g_bHeadlessMode = true;
    CWgpuRenderBackend backend;

    TRenderFrameUniforms uniforms{};
    uniforms.fTime = 12.34f;
    backend.BeginFrame(uniforms);

    TTerrainSectorPacket terrainPkt{};
    terrainPkt.dwSectorX = 100;
    backend.RenderTerrain(&terrainPkt, 1);

    TActorRenderPacket actorPkt{};
    actorPkt.dwActorID = 42;
    backend.RenderActors(&actorPkt, 1);

    CHECK(backend.GetFramePacket().dwTerrainCount == 1);
    CHECK(backend.GetFramePacket().dwActorCount == 1);
    CHECK(backend.GetCachedTerrainPackets().size() == 1);
    CHECK(backend.GetCachedActorPackets().size() == 1);

    // Kolejna klatka powinna wyczyscic wszystko
    TRenderFrameUniforms uniformsNext{};
    uniformsNext.fTime = 56.78f;
    backend.BeginFrame(uniformsNext);

    CHECK(backend.GetFramePacket().dwTerrainCount == 0);
    CHECK(backend.GetFramePacket().pTerrainPackets == nullptr);
    CHECK(backend.GetFramePacket().dwActorCount == 0);
    CHECK(backend.GetFramePacket().pActorPackets == nullptr);
    CHECK(backend.GetCachedTerrainPackets().empty());
    CHECK(backend.GetCachedActorPackets().empty());
    CHECK(backend.GetFramePacket().uniforms.fTime == 56.78f);
}

TEST_CASE("WGPU Backend Caching: Prevent Use-After-Free When Source Queue Is Cleared")
{
    g_bHeadlessMode = true;
    CWgpuRenderBackend backend;
    TRenderFrameUniforms uniforms{};
    backend.BeginFrame(uniforms);

    // Tworzymy tymczasowy wektor zrodlowy
    std::vector<TTerrainSectorPacket> sourceTerrain(3);
    sourceTerrain[0].dwSectorX = 10;
    sourceTerrain[1].dwSectorX = 20;
    sourceTerrain[2].dwSectorX = 30;

    std::vector<TActorRenderPacket> sourceActors(2);
    sourceActors[0].dwActorID = 1001;
    sourceActors[1].dwActorID = 1002;

    std::vector<TWaterPacket> sourceWater(1);
    sourceWater[0].dwSectorX = 55;

    std::vector<TSpeedTreePacket> sourceTrees(4);
    sourceTrees[0].dwTreeCRC = 777;

    std::vector<TEffectInstancePacket> sourceEffects(2);
    sourceEffects[0].dwEffectID = 999;

    std::vector<TDungeonBlockPacket> sourceDungeons(2);
    sourceDungeons[0].dwVertexCount = 128;

    std::vector<TSkyBoxPacket> sourceSky(1);
    sourceSky[0].dwFaceCount = 6;

    std::vector<TUIBatchPacket> sourceUI(3);
    sourceUI[0].dwVertexCount = 24;

    // Przekazanie do backendu
    backend.RenderTerrain(sourceTerrain.data(), static_cast<unsigned int>(sourceTerrain.size()));
    backend.RenderActors(sourceActors.data(), static_cast<unsigned int>(sourceActors.size()));
    backend.RenderWater(sourceWater.data(), static_cast<unsigned int>(sourceWater.size()));
    backend.RenderSpeedTree(sourceTrees.data(), static_cast<unsigned int>(sourceTrees.size()));
    backend.RenderEffects(sourceEffects.data(), static_cast<unsigned int>(sourceEffects.size()));
    backend.RenderDungeonBlocks(sourceDungeons.data(), static_cast<unsigned int>(sourceDungeons.size()));
    backend.RenderSkyBox(sourceSky.data(), static_cast<unsigned int>(sourceSky.size()));
    backend.RenderUI(sourceUI.data(), static_cast<unsigned int>(sourceUI.size()));

    // Symulujemy Use-After-Free: zasmiecamy zrodlowe bufory pamieci i czyscimy je
    std::memset(sourceTerrain.data(), 0xDD, sizeof(TTerrainSectorPacket) * sourceTerrain.size());
    std::memset(sourceActors.data(), 0xDD, sizeof(TActorRenderPacket) * sourceActors.size());
    std::memset(sourceWater.data(), 0xDD, sizeof(TWaterPacket) * sourceWater.size());
    std::memset(sourceTrees.data(), 0xDD, sizeof(TSpeedTreePacket) * sourceTrees.size());
    std::memset(sourceEffects.data(), 0xDD, sizeof(TEffectInstancePacket) * sourceEffects.size());
    std::memset(sourceDungeons.data(), 0xDD, sizeof(TDungeonBlockPacket) * sourceDungeons.size());
    std::memset(sourceSky.data(), 0xDD, sizeof(TSkyBoxPacket) * sourceSky.size());
    std::memset(sourceUI.data(), 0xDD, sizeof(TUIBatchPacket) * sourceUI.size());

    sourceTerrain.clear();
    sourceActors.clear();
    sourceWater.clear();
    sourceTrees.clear();
    sourceEffects.clear();
    sourceDungeons.clear();
    sourceSky.clear();
    sourceUI.clear();

    // Weryfikacja: m_framePacket nadal zawiera nienaruszone, prawidlowe dane z glebokiej kopii!
    const TRenderFramePacket& fp = backend.GetFramePacket();

    CHECK(fp.dwTerrainCount == 3);
    REQUIRE(fp.pTerrainPackets != nullptr);
    CHECK(fp.pTerrainPackets[0].dwSectorX == 10);
    CHECK(fp.pTerrainPackets[1].dwSectorX == 20);
    CHECK(fp.pTerrainPackets[2].dwSectorX == 30);

    CHECK(fp.dwActorCount == 2);
    REQUIRE(fp.pActorPackets != nullptr);
    CHECK(fp.pActorPackets[0].dwActorID == 1001);
    CHECK(fp.pActorPackets[1].dwActorID == 1002);

    CHECK(fp.dwWaterCount == 1);
    REQUIRE(fp.pWaterPackets != nullptr);
    CHECK(fp.pWaterPackets[0].dwSectorX == 55);

    CHECK(fp.dwSpeedTreeCount == 4);
    REQUIRE(fp.pSpeedTreePackets != nullptr);
    CHECK(fp.pSpeedTreePackets[0].dwTreeCRC == 777);

    CHECK(fp.dwEffectCount == 2);
    REQUIRE(fp.pEffectPackets != nullptr);
    CHECK(fp.pEffectPackets[0].dwEffectID == 999);

    CHECK(fp.dwDungeonCount == 2);
    REQUIRE(fp.pDungeonPackets != nullptr);
    CHECK(fp.pDungeonPackets[0].dwVertexCount == 128);

    CHECK(fp.dwSkyBoxCount == 1);
    REQUIRE(fp.pSkyBoxPackets != nullptr);
    CHECK(fp.pSkyBoxPackets[0].dwFaceCount == 6);

    CHECK(fp.dwUICount == 3);
    REQUIRE(fp.pUIPackets != nullptr);
    CHECK(fp.pUIPackets[0].dwVertexCount == 24);
}

TEST_CASE("WGPU Backend Caching: MultiBatch Accumulation")
{
    g_bHeadlessMode = true;
    CWgpuRenderBackend backend;
    TRenderFrameUniforms uniforms{};
    backend.BeginFrame(uniforms);

    // Partia 1: 2 aktorow
    TActorRenderPacket batch1[2]{};
    batch1[0].dwActorID = 10;
    batch1[1].dwActorID = 20;
    backend.RenderActors(batch1, 2);

    CHECK(backend.GetFramePacket().dwActorCount == 2);

    // Partia 2: 3 aktorow
    TActorRenderPacket batch2[3]{};
    batch2[0].dwActorID = 30;
    batch2[1].dwActorID = 40;
    batch2[2].dwActorID = 50;
    backend.RenderActors(batch2, 3);

    // Lacznie powinno byc 5 aktorow, a wskaznik pActorPackets wskazuje na pelne 5 elementow
    const TRenderFramePacket& fp = backend.GetFramePacket();
    CHECK(fp.dwActorCount == 5);
    REQUIRE(fp.pActorPackets != nullptr);
    CHECK(fp.pActorPackets[0].dwActorID == 10);
    CHECK(fp.pActorPackets[1].dwActorID == 20);
    CHECK(fp.pActorPackets[2].dwActorID == 30);
    CHECK(fp.pActorPackets[3].dwActorID == 40);
    CHECK(fp.pActorPackets[4].dwActorID == 50);
}

TEST_CASE("WGPU Backend Caching: Nullptr And Zero Count Safety")
{
    g_bHeadlessMode = true;
    CWgpuRenderBackend backend;
    TRenderFrameUniforms uniforms{};
    backend.BeginFrame(uniforms);

    backend.RenderTerrain(nullptr, 0);
    backend.RenderTerrain(nullptr, 10);
    TTerrainSectorPacket dummy{};
    backend.RenderTerrain(&dummy, 0);

    CHECK(backend.GetFramePacket().dwTerrainCount == 0);
    CHECK(backend.GetFramePacket().pTerrainPackets == nullptr);
}

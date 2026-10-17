#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/IRenderBackend.h"
#include "EterLib/FrameReplayPlayer.h"
#include <cstdio>
#include <vector>
#include <cstring>

namespace
{
class MockReplayBackend : public IRenderBackend
{
public:
	bool Initialize(HWND hWnd, unsigned int width, unsigned int height) override { return true; }
	void Shutdown() override {}
	void Resize(unsigned int width, unsigned int height) override {}

	void BeginFrame(const TRenderFrameUniforms& uniforms) override
	{
		m_bBeginFrameCalled = true;
		m_lastUniforms = uniforms;
	}

	void RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count) override
	{
		m_skyBoxCount += count;
		if (count > 0 && pPackets)
			m_lastSkyBox = pPackets[0];
	}

	void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) override
	{
		m_terrainCount += count;
		if (count > 0 && pPackets)
			m_lastTerrain = pPackets[0];
	}

	void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) override
	{
		m_actorCount += count;
		if (count > 0 && pPackets)
			m_lastActor = pPackets[0];
	}

	void RenderWater(const TWaterPacket* pPackets, unsigned int count) override
	{
		m_waterCount += count;
		if (count > 0 && pPackets)
			m_lastWater = pPackets[0];
	}

	void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) override
	{
		m_speedTreeCount += count;
		if (count > 0 && pPackets)
		{
			m_lastSpeedTree = pPackets[0];
			m_lastTreeInstanceCount += pPackets[0].dwInstanceCount;
		}
	}

	void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) override
	{
		m_effectCount += count;
		if (count > 0 && pPackets)
		{
			m_lastEffect = pPackets[0];
			m_lastParticleCount += pPackets[0].dwParticleCount;
		}
	}

	void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) override
	{
		m_uiCount += count;
		if (count > 0 && pPackets)
		{
			m_lastUI = pPackets[0];
			m_lastUIVertexCount += pPackets[0].dwVertexCount;
		}
	}

	void RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count) override
	{
		m_dungeonCount += count;
		if (count > 0 && pPackets)
			m_lastDungeon = pPackets[0];
	}

	void EndFrame() override
	{
		m_bEndFrameCalled = true;
	}

	const char* GetBackendName() const override { return "MockReplayBackend"; }
	bool IsDeviceLost() const override { return false; }
	bool CaptureFrame(const char* szFilename) override { return true; }

	void Reset()
	{
		m_bBeginFrameCalled = false;
		m_bEndFrameCalled = false;
		m_skyBoxCount = 0;
		m_terrainCount = 0;
		m_actorCount = 0;
		m_waterCount = 0;
		m_speedTreeCount = 0;
		m_effectCount = 0;
		m_uiCount = 0;
		m_dungeonCount = 0;
		m_lastTreeInstanceCount = 0;
		m_lastParticleCount = 0;
		m_lastUIVertexCount = 0;
		std::memset(&m_lastUniforms, 0, sizeof(m_lastUniforms));
		std::memset(&m_lastSkyBox, 0, sizeof(m_lastSkyBox));
		std::memset(&m_lastTerrain, 0, sizeof(m_lastTerrain));
		std::memset(&m_lastActor, 0, sizeof(m_lastActor));
		std::memset(&m_lastWater, 0, sizeof(m_lastWater));
		std::memset(&m_lastSpeedTree, 0, sizeof(m_lastSpeedTree));
		std::memset(&m_lastEffect, 0, sizeof(m_lastEffect));
		std::memset(&m_lastUI, 0, sizeof(m_lastUI));
		std::memset(&m_lastDungeon, 0, sizeof(m_lastDungeon));
	}

	bool m_bBeginFrameCalled{ false };
	bool m_bEndFrameCalled{ false };
	unsigned int m_skyBoxCount{ 0 };
	unsigned int m_terrainCount{ 0 };
	unsigned int m_actorCount{ 0 };
	unsigned int m_waterCount{ 0 };
	unsigned int m_speedTreeCount{ 0 };
	unsigned int m_effectCount{ 0 };
	unsigned int m_uiCount{ 0 };
	unsigned int m_dungeonCount{ 0 };
	uint32_t m_lastTreeInstanceCount{ 0 };
	uint32_t m_lastParticleCount{ 0 };
	uint32_t m_lastUIVertexCount{ 0 };
	TRenderFrameUniforms m_lastUniforms{};
	TSkyBoxPacket m_lastSkyBox{};
	TTerrainSectorPacket m_lastTerrain{};
	TActorRenderPacket m_lastActor{};
	TWaterPacket m_lastWater{};
	TSpeedTreePacket m_lastSpeedTree{};
	TEffectInstancePacket m_lastEffect{};
	TUIBatchPacket m_lastUI{};
	TDungeonBlockPacket m_lastDungeon{};
};
}

TEST_CASE("M2FR - Validation Rules")
{
	using namespace FrameDumpV2;

	TDumpParsedFrame frame{};

	// 1. Pusta ramka bez magic/version
	CHECK(ValidateFrameDump(frame) == false);

	// 2. Ramka z prawidlowym naglowkiem ale bez chunkow
	frame.header.dwMagic = M2FR_MAGIC;
	frame.header.dwVersion = M2FR_VERSION;
	CHECK(ValidateFrameDump(frame) == false);

	// 3. Ramka z naglowkiem i co najmniej 1 chunkiem
	TChunkHeader chunk{};
	chunk.dwTag = CHUNK_UNIFORMS;
	frame.chunkHeaders.push_back(chunk);
	CHECK(ValidateFrameDump(frame) == true);

	// 4. Nieprawidlowy magic
	frame.header.dwMagic = 0x12345678;
	CHECK(ValidateFrameDump(frame) == false);

	// 5. Nieprawidlowa wersja
	frame.header.dwMagic = M2FR_MAGIC;
	frame.header.dwVersion = 99;
	CHECK(ValidateFrameDump(frame) == false);
}

TEST_CASE("M2FR - Replay to Mock Backend")
{
	using namespace FrameDumpV2;

	TDumpParsedFrame frame{};
	frame.header.dwMagic = M2FR_MAGIC;
	frame.header.dwVersion = M2FR_VERSION;
	frame.chunkHeaders.push_back({ CHUNK_UNIFORMS, 1, sizeof(TRenderFrameUniforms), sizeof(TRenderFrameUniforms) });

	frame.uniforms.dwFrameIndex = 777;
	frame.uniforms.fTime = 12.5f;

	// SkyBox
	TSkyBoxPacket sky{};
	sky.dwSkyTextureID = 10;
	frame.skyBoxPackets.push_back(sky);

	// Terrain
	TTerrainSectorPacket terr{};
	terr.splatTextureId = 20;
	frame.terrainPackets.push_back(terr);

	// Dungeon
	TDungeonBlockPacket dung{};
	dung.dwDiffuseTextureID = 30;
	frame.dungeonPackets.push_back(dung);

	// Water
	TWaterPacket water{};
	water.dwWaterTextureID = 40;
	frame.waterPackets.push_back(water);

	// SpeedTree
	TDumpSpeedTreeBatch treeBatch{};
	treeBatch.header.uTreeTypeID = 50;
	treeBatch.header.dwInstanceCount = 2;
	treeBatch.instances.resize(2);
	treeBatch.instances[0].v3Position = { 1.0f, 2.0f, 3.0f };
	treeBatch.instances[1].v3Position = { 4.0f, 5.0f, 6.0f };
	frame.speedTreeBatches.push_back(treeBatch);

	// Actor
	TActorRenderPacket actor{};
	actor.dwActorID = 60;
	frame.actorPackets.push_back(actor);

	// Effect
	TDumpEffectBatch effectBatch{};
	effectBatch.header.uTextureID = 70;
	effectBatch.header.dwParticleCount = 3;
	effectBatch.particles.resize(3);
	frame.effectBatches.push_back(effectBatch);

	// UI
	TDumpUIBatch uiBatch{};
	uiBatch.header.uTextureID = 80;
	uiBatch.header.dwVertexCount = 4;
	uiBatch.vertices.resize(4);
	frame.uiBatches.push_back(uiBatch);

	MockReplayBackend backend;
	TReplayStats stats{};

	// Null backend test
	CHECK(ReplayFrame(frame, nullptr, &stats) == false);

	// Prawidlowy replay
	CHECK(ReplayFrame(frame, &backend, &stats) == true);
	CHECK(stats.bSuccess == true);
	CHECK(stats.skyBoxCount == 1);
	CHECK(stats.terrainCount == 1);
	CHECK(stats.dungeonCount == 1);
	CHECK(stats.waterCount == 1);
	CHECK(stats.speedTreeBatchCount == 1);
	CHECK(stats.actorCount == 1);
	CHECK(stats.effectBatchCount == 1);
	CHECK(stats.uiBatchCount == 1);
	CHECK(stats.totalPackets == 8);

	// Weryfikacja stanow backendu
	CHECK(backend.m_bBeginFrameCalled == true);
	CHECK(backend.m_bEndFrameCalled == true);
	CHECK(backend.m_lastUniforms.dwFrameIndex == 777);
	CHECK(backend.m_skyBoxCount == 1);
	CHECK(backend.m_lastSkyBox.dwSkyTextureID == 10);
	CHECK(backend.m_terrainCount == 1);
	CHECK(backend.m_lastTerrain.splatTextureId == 20);
	CHECK(backend.m_dungeonCount == 1);
	CHECK(backend.m_lastDungeon.dwDiffuseTextureID == 30);
	CHECK(backend.m_waterCount == 1);
	CHECK(backend.m_lastWater.dwWaterTextureID == 40);
	CHECK(backend.m_speedTreeCount == 1);
	CHECK(backend.m_lastTreeInstanceCount == 2);
	CHECK(backend.m_actorCount == 1);
	CHECK(backend.m_lastActor.dwActorID == 60);
	CHECK(backend.m_effectCount == 1);
	CHECK(backend.m_lastParticleCount == 3);
	CHECK(backend.m_uiCount == 1);
	CHECK(backend.m_lastUIVertexCount == 4);
}

TEST_CASE("M2FR - End-to-End File Export and Replay")
{
	using namespace FrameDumpV2;

	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	TRenderFrameUniforms uniforms{};
	uniforms.dwFrameIndex = 1234;
	uniforms.fTime = 99.0f;
	queue.SetFrameUniforms(uniforms);

	TSkyBoxPacket sky{};
	sky.dwSkyTextureID = 555;
	queue.SubmitSkyBox(sky);

	TTerrainSectorPacket terr{};
	terr.splatTextureId = 666;
	queue.SubmitTerrain(terr);

	TDungeonBlockPacket dung{};
	dung.dwDiffuseTextureID = 777;
	queue.SubmitDungeonBlock(dung);

	const char* dumpPath = "test_replay_e2e.bin";
	CHECK(queue.ExportFrameToBinaryV2(dumpPath) == true);

	MockReplayBackend backend;
	TReplayStats stats{};

	CHECK(ReplayFrameDumpV2(dumpPath, &backend, &stats) == true);
	CHECK(stats.bSuccess == true);
	CHECK(stats.skyBoxCount == 1);
	CHECK(stats.terrainCount == 1);
	CHECK(stats.dungeonCount == 1);
	CHECK(backend.m_bBeginFrameCalled == true);
	CHECK(backend.m_bEndFrameCalled == true);
	CHECK(backend.m_lastUniforms.dwFrameIndex == 1234);
	CHECK(backend.m_lastSkyBox.dwSkyTextureID == 555);
	CHECK(backend.m_lastTerrain.splatTextureId == 666);
	CHECK(backend.m_lastDungeon.dwDiffuseTextureID == 777);

	std::remove(dumpPath);
}

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

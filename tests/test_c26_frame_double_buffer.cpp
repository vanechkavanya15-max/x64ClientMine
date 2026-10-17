#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderPacketTypes.h"
#include "EterLib/IRenderBackend.h"
#include <cstring>
#include <vector>

namespace
{
class MockDoubleBufferBackend : public IRenderBackend
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

	void RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count) override { m_skyBoxCount += count; }
	void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) override
	{
		m_terrainCount += count;
		if (count > 0 && pPackets)
		{
			m_lastTerrainPacket = pPackets[0];
			if (pPackets[0].pPassData && pPackets[0].dwPassCount > 0)
			{
				const auto* pPasses = reinterpret_cast<const STerrainSplatPass*>(pPackets[0].pPassData);
				m_lastFirstPass = pPasses[0];
			}
		}
	}
	void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) override
	{
		m_actorCount += count;
		if (count > 0 && pPackets)
		{
			m_lastActorPacket = pPackets[0];
		}
	}
	void RenderWater(const TWaterPacket* pPackets, unsigned int count) override { m_waterCount += count; }
	void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) override { m_speedTreeCount += count; }
	void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) override { m_effectCount += count; }
	void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) override { m_uiCount += count; }
	void RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count) override { m_dungeonCount += count; }
	void EndFrame() override { m_bEndFrameCalled = true; }
	const char* GetBackendName() const override { return "MockDoubleBufferBackend"; }
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
		std::memset(&m_lastUniforms, 0, sizeof(m_lastUniforms));
		std::memset(&m_lastTerrainPacket, 0, sizeof(m_lastTerrainPacket));
		std::memset(&m_lastFirstPass, 0, sizeof(m_lastFirstPass));
		std::memset(&m_lastActorPacket, 0, sizeof(m_lastActorPacket));
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
	TRenderFrameUniforms m_lastUniforms{};
	TTerrainSectorPacket m_lastTerrainPacket{};
	STerrainSplatPass m_lastFirstPass{};
	TActorRenderPacket m_lastActorPacket{};
};
}

TEST_CASE("DoubleBuffer - Buffer Independence and Distinct Memory Pools")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	const size_t activeIdx = queue.GetActiveFrameIndex();
	const size_t workIdx = queue.GetWorkFrameIndex();

	CHECK(activeIdx != workIdx);
	CHECK((activeIdx ^ 1) == workIdx);
	CHECK((workIdx ^ 1) == activeIdx);

	const SFrameBuffer& frame0 = queue.GetFrame(0);
	const SFrameBuffer& frame1 = queue.GetFrame(1);

	// Pule pamieci w obu buforach musza byc calkowicie rozlaczne
	CHECK(&frame0 != &frame1);
	CHECK(frame0.boneMatrixPool.data() != frame1.boneMatrixPool.data());
	CHECK(frame0.particleDataPool.data() != frame1.particleDataPool.data());
	CHECK(frame0.speedTreeInstancePool.data() != frame1.speedTreeInstancePool.data());
	CHECK(frame0.weaponTraceVertexPool.data() != frame1.weaponTraceVertexPool.data());
	CHECK(frame0.effectMeshVertexPool.data() != frame1.effectMeshVertexPool.data());
	CHECK(frame0.frameDataPool.data() != frame1.frameDataPool.data());

	CHECK(frame0.boneMatrixPool.size() > 0);
	CHECK(frame1.boneMatrixPool.size() > 0);
	CHECK(frame0.frameDataPool.size() >= 4 * 1024 * 1024);
	CHECK(frame1.frameDataPool.size() >= 4 * 1024 * 1024);
}

TEST_CASE("DoubleBuffer - SwapFrameBuffers Rotation and State Transfer")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	const size_t initialActive = queue.GetActiveFrameIndex();
	const size_t initialWork = queue.GetWorkFrameIndex();

	// Zasil klatke robocza pakietami
	TTerrainSectorPacket terrainPkt{};
	terrainPkt.patchNum = 101;
	terrainPkt.dwSectorX = 7;
	terrainPkt.dwSectorY = 8;
	queue.SubmitTerrain(terrainPkt);

	TActorRenderPacket actorPkt{};
	actorPkt.dwActorID = 4567;
	queue.SubmitActor(actorPkt);

	// Zaalokuj dane z puli klatki roboczej
	D3DXMATRIX* pBones = queue.AllocateBoneMatrices(2);
	REQUIRE(pBones != nullptr);
	pBones[0]._11 = 42.0f;
	pBones[1]._22 = 84.0f;

	STerrainSplatPass* pPasses = queue.AllocateFrameData<STerrainSplatPass>(1);
	REQUIRE(pPasses != nullptr);
	pPasses[0].layerIndex = 99;
	pPasses[0].textureId = 777;

	CHECK(queue.GetWorkFrame().terrainPackets.size() == 1);
	CHECK(queue.GetWorkFrame().actorPackets.size() == 1);
	CHECK(queue.GetWorkFrame().boneMatrixOffset == 2);
	CHECK(queue.GetWorkFrame().frameDataOffset >= sizeof(STerrainSplatPass));

	// Rotacja buforow
	queue.SwapFrameBuffers();

	// Po rotacji bufor roboczy z klatki N stal sie buforem aktywnym do renderowania
	CHECK(queue.GetActiveFrameIndex() == initialWork);
	CHECK(queue.GetWorkFrameIndex() == initialActive);

	const SFrameBuffer& activeFrame = queue.GetActiveFrame();
	REQUIRE(activeFrame.terrainPackets.size() == 1);
	CHECK(activeFrame.terrainPackets[0].patchNum == 101);
	CHECK(activeFrame.terrainPackets[0].dwSectorX == 7);
	REQUIRE(activeFrame.actorPackets.size() == 1);
	CHECK(activeFrame.actorPackets[0].dwActorID == 4567);

	// Sprawdz nienaruszone dane w puli bufora aktywnego
	const D3DXMATRIX* pActiveBones = queue.GetBoneMatrixData();
	REQUIRE(pActiveBones != nullptr);
	CHECK(pActiveBones[0]._11 == doctest::Approx(42.0f));
	CHECK(pActiveBones[1]._22 == doctest::Approx(84.0f));

	// Nowy bufor roboczy musi byc kompletnie wyczyszczony i gotowy na klatke N+1
	const SFrameBuffer& newWorkFrame = queue.GetWorkFrame();
	CHECK(newWorkFrame.terrainPackets.empty());
	CHECK(newWorkFrame.actorPackets.empty());
	CHECK(newWorkFrame.boneMatrixOffset == 0);
	CHECK(newWorkFrame.frameDataOffset == 0);
	CHECK(newWorkFrame.GetTotalPacketCount() == 0);
}

TEST_CASE("DoubleBuffer - Zero Data Corruption Across Consecutive Frames (N and N+1)")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	// === ETAP 1: Logika produkuje klatke N ===
	TRenderFrameUniforms uniformsN{};
	uniformsN.fTime = 100.0f;
	uniformsN.dwFrameIndex = 10;
	queue.SetFrameUniforms(uniformsN);

	// Alokacja danych klatki N
	STerrainSplatPass* pPassesN = queue.AllocateFrameData<STerrainSplatPass>(2);
	REQUIRE(pPassesN != nullptr);
	pPassesN[0].layerIndex = 1;
	pPassesN[0].textureId = 1111;
	pPassesN[0].bDisableAlphaOp = true;
	pPassesN[1].layerIndex = 2;
	pPassesN[1].textureId = 2222;
	pPassesN[1].bDisableAlphaOp = false;

	D3DXMATRIX* pBonesN = queue.AllocateBoneMatrices(3);
	REQUIRE(pBonesN != nullptr);
	pBonesN[0]._11 = 123.456f;
	pBonesN[1]._22 = 654.321f;
	pBonesN[2]._33 = 789.012f;

	TParticleInstanceData* pParticlesN = queue.AllocateParticleData(2);
	REQUIRE(pParticlesN != nullptr);
	pParticlesN[0].fRotation = 1.57f;
	pParticlesN[1].fRotation = 3.14f;

	TTerrainSectorPacket terrainN{};
	terrainN.dwSectorX = 10;
	terrainN.dwSectorY = 20;
	terrainN.patchNum = 50;
	terrainN.pPassData = pPassesN;
	terrainN.dwPassCount = 2;
	queue.SubmitTerrain(terrainN);

	TActorRenderPacket actorN{};
	actorN.dwActorID = 10001;
	actorN.pBoneMatrices = pBonesN;
	actorN.dwBoneCount = 3;
	queue.SubmitActor(actorN);

	// Przelaczenie bufora: klatka N staje sie klatka aktywna dla backendu
	queue.SwapFrameBuffers();

	// Zapamietaj wskazniki i wartosci klatki N
	const STerrainSplatPass* pObservedPassesN = pPassesN;
	const D3DXMATRIX* pObservedBonesN = pBonesN;
	const TParticleInstanceData* pObservedParticlesN = pParticlesN;

	// === ETAP 2: Logika produkuje klatke N+1 podczas gdy backend zuzywa N ===
	TRenderFrameUniforms uniformsN1{};
	uniformsN1.fTime = 100.016f;
	uniformsN1.dwFrameIndex = 11;
	queue.SetFrameUniforms(uniformsN1);

	// Producent alokuje dane dla klatki N+1 z zupelnie innymi wartosciami
	STerrainSplatPass* pPassesN1 = queue.AllocateFrameData<STerrainSplatPass>(3);
	REQUIRE(pPassesN1 != nullptr);
	pPassesN1[0].layerIndex = 10;
	pPassesN1[0].textureId = 9999;
	pPassesN1[1].layerIndex = 20;
	pPassesN1[1].textureId = 8888;
	pPassesN1[2].layerIndex = 30;
	pPassesN1[2].textureId = 7777;

	D3DXMATRIX* pBonesN1 = queue.AllocateBoneMatrices(5);
	REQUIRE(pBonesN1 != nullptr);
	for (int i = 0; i < 5; ++i)
	{
		pBonesN1[i]._11 = -999.0f * (i + 1);
	}

	TParticleInstanceData* pParticlesN1 = queue.AllocateParticleData(4);
	REQUIRE(pParticlesN1 != nullptr);
	for (int i = 0; i < 4; ++i)
	{
		pParticlesN1[i].fRotation = -1.0f * (i + 1);
	}

	TTerrainSectorPacket terrainN1{};
	terrainN1.dwSectorX = 99;
	terrainN1.dwSectorY = 88;
	terrainN1.patchNum = 77;
	terrainN1.pPassData = pPassesN1;
	terrainN1.dwPassCount = 3;
	queue.SubmitTerrain(terrainN1);

	TActorRenderPacket actorN1{};
	actorN1.dwActorID = 20002;
	actorN1.pBoneMatrices = pBonesN1;
	actorN1.dwBoneCount = 5;
	queue.SubmitActor(actorN1);

	// === ETAP 3: WERYFIKACJA ZERO-CORRUPTION DLA KLATKI N ===
	// 1. Wskazniki puli nie moga na siebie nachodzic
	CHECK(pPassesN != pPassesN1);
	CHECK(pBonesN != pBonesN1);
	CHECK(pParticlesN != pParticlesN1);

	// 2. Dane klatki N w pamieci nie moga byc w zaden sposob zmienione
	CHECK(pObservedPassesN[0].layerIndex == 1u);
	CHECK(pObservedPassesN[0].textureId == 1111u);
	CHECK(pObservedPassesN[0].bDisableAlphaOp == true);
	CHECK(pObservedPassesN[1].layerIndex == 2u);
	CHECK(pObservedPassesN[1].textureId == 2222u);
	CHECK(pObservedPassesN[1].bDisableAlphaOp == false);

	CHECK(pObservedBonesN[0]._11 == doctest::Approx(123.456f));
	CHECK(pObservedBonesN[1]._22 == doctest::Approx(654.321f));
	CHECK(pObservedBonesN[2]._33 == doctest::Approx(789.012f));

	CHECK(pObservedParticlesN[0].fRotation == doctest::Approx(1.57f));
	CHECK(pObservedParticlesN[1].fRotation == doctest::Approx(3.14f));

	// 3. Pakiety w klatce aktywnej (N) musza zachowac pelna integralnosc
	const auto& activeTerrain = queue.GetActiveTerrainPackets();
	REQUIRE(activeTerrain.size() == 1);
	CHECK(activeTerrain[0].dwSectorX == 10u);
	CHECK(activeTerrain[0].dwSectorY == 20u);
	CHECK(activeTerrain[0].patchNum == 50u);
	CHECK(activeTerrain[0].dwPassCount == 2u);
	CHECK(activeTerrain[0].pPassData == pObservedPassesN);

	const auto& activeActors = queue.GetActiveActorPackets();
	REQUIRE(activeActors.size() == 1);
	CHECK(activeActors[0].dwActorID == 10001u);
	CHECK(activeActors[0].pBoneMatrices == pObservedBonesN);
	CHECK(activeActors[0].dwBoneCount == 3u);

	// 4. Klatka robocza (N+1) zawiera wlasne dane
	const auto& workTerrain = queue.GetWorkTerrainPackets();
	REQUIRE(workTerrain.size() == 1);
	CHECK(workTerrain[0].dwSectorX == 99u);
	CHECK(workTerrain[0].dwSectorY == 88u);
	CHECK(workTerrain[0].patchNum == 77u);
	CHECK(workTerrain[0].dwPassCount == 3u);
	CHECK(workTerrain[0].pPassData == pPassesN1);

	// === ETAP 4: Backend renderuje klatke N przez Flush ===
	MockDoubleBufferBackend backend;
	queue.Flush(&backend);

	CHECK(backend.m_bBeginFrameCalled == true);
	CHECK(backend.m_terrainCount == 1);
	CHECK(backend.m_actorCount == 1);
	CHECK(backend.m_lastTerrainPacket.dwSectorX == 10u);
	CHECK(backend.m_lastTerrainPacket.patchNum == 50u);
	CHECK(backend.m_lastFirstPass.layerIndex == 1u);
	CHECK(backend.m_lastFirstPass.textureId == 1111u);
	CHECK(backend.m_lastActorPacket.dwActorID == 10001u);
	CHECK(backend.m_bEndFrameCalled == true);
}

TEST_CASE("DoubleBuffer - EndFrame Automates Swap and Dump Flow")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	const size_t prevActive = queue.GetActiveFrameIndex();

	TTerrainSectorPacket pkt{};
	pkt.patchNum = 777;
	queue.SubmitTerrain(pkt);

	// EndFrame wywoluje SwapFrameBuffers
	queue.EndFrame();

	CHECK(queue.GetActiveFrameIndex() != prevActive);
	REQUIRE(queue.GetActiveFrame().terrainPackets.size() == 1);
	CHECK(queue.GetActiveFrame().terrainPackets[0].patchNum == 777);
	CHECK(queue.GetWorkFrame().GetTotalPacketCount() == 0);
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/IRenderBackend.h"
#include <string>
#include <cstring>

namespace
{
class MockRenderBackend : public IRenderBackend
{
public:
	bool Initialize(HWND hWnd, unsigned int width, unsigned int height) override { return true; }
	void Shutdown() override {}
	void Resize(unsigned int width, unsigned int height) override {}

	void BeginFrame(const TRenderFrameUniforms& uniforms) override
	{
		m_beginFrameCalled = true;
		m_lastUniforms = uniforms;
	}

	void RenderSkyBox(const TSkyBoxPacket* pPackets, unsigned int count) override
	{
		m_skyBoxCount += count;
	}

	void RenderTerrain(const TTerrainSectorPacket* pPackets, unsigned int count) override
	{
		m_terrainCount += count;
	}

	void RenderActors(const TActorRenderPacket* pPackets, unsigned int count) override
	{
		m_actorCount += count;
	}

	void RenderWater(const TWaterPacket* pPackets, unsigned int count) override
	{
		m_waterCount += count;
	}

	void RenderSpeedTree(const TSpeedTreePacket* pPackets, unsigned int count) override
	{
		m_speedTreeCount += count;
	}

	void RenderEffects(const TEffectInstancePacket* pPackets, unsigned int count) override
	{
		m_effectCount += count;
	}

	void RenderUI(const TUIBatchPacket* pPackets, unsigned int count) override
	{
		m_uiCount += count;
	}

	void RenderDungeonBlocks(const TDungeonBlockPacket* pPackets, unsigned int count) override
	{
		m_dungeonBlockCount += count;
	}

	void EndFrame() override
	{
		m_endFrameCalled = true;
	}

	const char* GetBackendName() const override
	{
		return "MockRenderBackend";
	}

	bool IsDeviceLost() const override
	{
		return false;
	}

	bool CaptureFrame(const char* szFilename) override
	{
		m_captureFrameCalled = true;
		m_capturedFilename = szFilename ? szFilename : "";
		return true;
	}

	void Reset()
	{
		m_beginFrameCalled = false;
		m_endFrameCalled = false;
		m_captureFrameCalled = false;
		m_capturedFilename.clear();
		m_skyBoxCount = 0;
		m_terrainCount = 0;
		m_actorCount = 0;
		m_waterCount = 0;
		m_speedTreeCount = 0;
		m_effectCount = 0;
		m_uiCount = 0;
		m_dungeonBlockCount = 0;
		memset(&m_lastUniforms, 0, sizeof(m_lastUniforms));
	}

	bool m_beginFrameCalled{ false };
	bool m_endFrameCalled{ false };
	bool m_captureFrameCalled{ false };
	std::string m_capturedFilename;
	unsigned int m_skyBoxCount{ 0 };
	unsigned int m_terrainCount{ 0 };
	unsigned int m_actorCount{ 0 };
	unsigned int m_waterCount{ 0 };
	unsigned int m_speedTreeCount{ 0 };
	unsigned int m_effectCount{ 0 };
	unsigned int m_uiCount{ 0 };
	unsigned int m_dungeonBlockCount{ 0 };
	TRenderFrameUniforms m_lastUniforms{};
};
}

TEST_CASE("RenderQueueManager - Submit and Flush on Mock IRenderBackend")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();
	CHECK(queue.GetTotalPacketCount() == 0u);

	TRenderFrameUniforms uniforms{};
	uniforms.fTime = 123.45f;
	queue.SetFrameUniforms(uniforms);

	TSkyBoxPacket skyPkt{};
	queue.SubmitSkyBox(skyPkt);

	TTerrainSectorPacket terrainPkt{};
	queue.SubmitTerrain(terrainPkt);

	TDungeonBlockPacket dungeonPkt{};
	queue.SubmitDungeonBlock(dungeonPkt);

	TWaterPacket waterPkt{};
	queue.SubmitWater(waterPkt);

	TSpeedTreePacket treePkt{};
	queue.SubmitSpeedTree(treePkt);

	TActorRenderPacket actorPkt{};
	queue.SubmitActor(actorPkt);

	TEffectInstancePacket effectPkt{};
	queue.SubmitEffect(effectPkt);

	TUIBatchPacket uiPkt{};
	queue.SubmitUI(uiPkt);

	CHECK(queue.GetTotalPacketCount() == 8u);
	CHECK(queue.GetSkyBoxPackets().size() == 1u);
	CHECK(queue.GetTerrainPackets().size() == 1u);
	CHECK(queue.GetDungeonPackets().size() == 1u);
	CHECK(queue.GetWaterPackets().size() == 1u);
	CHECK(queue.GetSpeedTreePackets().size() == 1u);
	CHECK(queue.GetActorPackets().size() == 1u);
	CHECK(queue.GetEffectPackets().size() == 1u);
	CHECK(queue.GetUIPackets().size() == 1u);

	MockRenderBackend mockBackend;
	queue.Flush(&mockBackend);

	CHECK(mockBackend.m_beginFrameCalled);
	CHECK(mockBackend.m_endFrameCalled);
	CHECK(mockBackend.m_lastUniforms.fTime == 123.45f);
	CHECK(mockBackend.m_skyBoxCount == 1u);
	CHECK(mockBackend.m_terrainCount == 1u);
	CHECK(mockBackend.m_dungeonBlockCount == 1u);
	CHECK(mockBackend.m_waterCount == 1u);
	CHECK(mockBackend.m_speedTreeCount == 1u);
	CHECK(mockBackend.m_actorCount == 1u);
	CHECK(mockBackend.m_effectCount == 1u);
	CHECK(mockBackend.m_uiCount == 1u);

	// Test CaptureReferenceScreenshot przez delegacje do aktywnego backendu
	queue.SetActiveBackend(&mockBackend);
	CHECK(queue.GetActiveBackend() == &mockBackend);

	bool bCapture = queue.CaptureReferenceScreenshot("frame_reference.png");
	CHECK(bCapture == true);
	CHECK(mockBackend.m_captureFrameCalled == true);
	CHECK(mockBackend.m_capturedFilename == "frame_reference.png");

	queue.SetActiveBackend(nullptr);
	CHECK(queue.GetActiveBackend() == nullptr);
}

TEST_CASE("RenderQueueManager - Dropped Packet Counting on Pool Overflow")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	CHECK(queue.GetDroppedParticles() == 0u);
	CHECK(queue.GetDroppedSpeedTree() == 0u);

	const size_t particleCapacity = queue.GetParticlePoolCapacity();
	CHECK(particleCapacity > 0u);

	// Prawidlowa alokacja czesci bufora
	TParticleInstanceData* pValid = queue.AllocateParticleData(100);
	CHECK(pValid != nullptr);
	CHECK(queue.GetParticleDataOffset() == 100u);
	CHECK(queue.GetDroppedParticles() == 0u);

	// Zapelnienie reszty bufora
	const size_t remainingParticles = particleCapacity - queue.GetParticleDataOffset();
	TParticleInstanceData* pRestParticles = queue.AllocateParticleData(remainingParticles);
	CHECK(pRestParticles != nullptr);
	CHECK(queue.GetParticleDataOffset() == particleCapacity);
	CHECK(queue.GetDroppedParticles() == 0u);

	// Przepelnienie bufora czasteczek po zapelnieniu
	TParticleInstanceData* pOverflow1 = queue.AllocateParticleData(10);
	CHECK(pOverflow1 == nullptr);
	CHECK(queue.GetDroppedParticles() == 10u);

	// Kolejna proba po przepelnieniu zwieksza licznik
	TParticleInstanceData* pOverflow2 = queue.AllocateParticleData(250);
	CHECK(pOverflow2 == nullptr);
	CHECK(queue.GetDroppedParticles() == 260u);

	// Alokacja instancji SpeedTree
	const size_t speedTreeCapacity = queue.GetSpeedTreePoolCapacity();
	CHECK(speedTreeCapacity > 0u);

	TSpeedTreeInstanceData* pTreeValid = queue.AllocateSpeedTreeInstanceData(50);
	CHECK(pTreeValid != nullptr);
	CHECK(queue.GetSpeedTreeInstanceOffset() == 50u);
	CHECK(queue.GetDroppedSpeedTree() == 0u);

	// Zapelnienie reszty bufora SpeedTree
	const size_t remainingTrees = speedTreeCapacity - queue.GetSpeedTreeInstanceOffset();
	TSpeedTreeInstanceData* pRestTrees = queue.AllocateSpeedTreeInstanceData(remainingTrees);
	CHECK(pRestTrees != nullptr);
	CHECK(queue.GetSpeedTreeInstanceOffset() == speedTreeCapacity);
	CHECK(queue.GetDroppedSpeedTree() == 0u);

	// Przepelnienie bufora SpeedTree
	TSpeedTreeInstanceData* pTreeOverflow1 = queue.AllocateSpeedTreeInstanceData(20);
	CHECK(pTreeOverflow1 == nullptr);
	CHECK(queue.GetDroppedSpeedTree() == 20u);

	TSpeedTreeInstanceData* pTreeOverflow2 = queue.AllocateSpeedTreeInstanceData(100);
	CHECK(pTreeOverflow2 == nullptr);
	CHECK(queue.GetDroppedSpeedTree() == 120u);

	// Reset licznikow
	queue.ResetDroppedCounters();
	CHECK(queue.GetDroppedParticles() == 0u);
	CHECK(queue.GetDroppedSpeedTree() == 0u);

	// Ponowne przepelnienie i weryfikacja resetu w BeginFrame()
	queue.AllocateParticleData(particleCapacity + 10);
	CHECK(queue.GetDroppedParticles() == particleCapacity + 10u);

	queue.BeginFrame();
	CHECK(queue.GetDroppedParticles() == 0u);
	CHECK(queue.GetDroppedSpeedTree() == 0u);
	CHECK(queue.GetParticleDataOffset() == 0u);
	CHECK(queue.GetSpeedTreeInstanceOffset() == 0u);
}

TEST_CASE("RenderQueueManager - ABI Version and Packet Sizes")
{
	CHECK(M2_RENDER_ABI_VERSION == 2u);
	CHECK(CRenderQueueManager::ABI_VERSION == 2u);
	CHECK(CRenderQueueManager::GetABIVersion() == 2u);

	const auto sizeList = CRenderQueueManager::GetPacketSizes();
	const auto sizeMap = CRenderQueueManager::GetPacketSizeMap();

	CHECK(sizeList.size() == 14u);
	CHECK(sizeMap.size() == 14u);

	// Weryfikacja zgodnosci kazdego typu z sizeof w C++
	CHECK(sizeMap.at("TRenderFrameUniforms") == sizeof(TRenderFrameUniforms));
	CHECK(sizeMap.at("TTerrainSectorPacket") == sizeof(TTerrainSectorPacket));
	CHECK(sizeMap.at("TActorRenderPacket") == sizeof(TActorRenderPacket));
	CHECK(sizeMap.at("TWaterPacket") == sizeof(TWaterPacket));
	CHECK(sizeMap.at("TSkyBoxPacket") == sizeof(TSkyBoxPacket));
	CHECK(sizeMap.at("TSpeedTreeInstanceData") == sizeof(TSpeedTreeInstanceData));
	CHECK(sizeMap.at("TSpeedTreePacket") == sizeof(TSpeedTreePacket));
	CHECK(sizeMap.at("TParticleInstanceData") == sizeof(TParticleInstanceData));
	CHECK(sizeMap.at("TEffectInstancePacket") == sizeof(TEffectInstancePacket));
	CHECK(sizeMap.at("TUIVertex") == sizeof(TUIVertex));
	CHECK(sizeMap.at("TUIBatchPacket") == sizeof(TUIBatchPacket));
	CHECK(sizeMap.at("TDungeonBlockPacket") == sizeof(TDungeonBlockPacket));
	CHECK(sizeMap.at("TRenderFramePacket") == sizeof(TRenderFramePacket));

#if defined(_WIN64)
	// Dokladne rozmiary struktur x64 zgodne z FFI Rust (packets.rs)
	CHECK(sizeMap.at("TRenderFrameUniforms") == 288u);
	CHECK(sizeMap.at("TTerrainSectorPacket") == 176u);
	CHECK(sizeMap.at("TActorRenderPacket") == 5040u);
	CHECK(sizeMap.at("TWaterPacket") == 176u);
	CHECK(sizeMap.at("TSkyBoxPacket") == 112u);
	CHECK(sizeMap.at("TSpeedTreeInstanceData") == 112u);
	CHECK(sizeMap.at("TSpeedTreePacket") == 48u);
	CHECK(sizeMap.at("TParticleInstanceData") == 64u);
	CHECK(sizeMap.at("TEffectInstancePacket") == 48u);
	CHECK(sizeMap.at("TUIVertex") == 24u);
	CHECK(sizeMap.at("TUIBatchPacket") == 64u);
	CHECK(sizeMap.at("TDungeonBlockPacket") == 160u);
	CHECK(sizeMap.at("TEffectMeshPacket") == 88u);
	CHECK(sizeMap.at("TRenderFramePacket") == 416u);
#endif

	// Weryfikacja wolnych funkcji pomocniczych
	CHECK(GetRenderPacketSizes().size() == 14u);
	CHECK(GetRenderPacketSizeMap().size() == 14u);
}

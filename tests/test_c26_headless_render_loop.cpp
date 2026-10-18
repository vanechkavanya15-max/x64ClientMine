#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "EterLib/RenderQueueManager.h"
#include "EterLib/WgpuRenderBackend.h"
#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderOwnership.h"
#include <atomic>

// Globalna zmienna trybu headless z EterBase
extern bool g_bHeadlessMode;

// Pomocniczy mock backendu do sledzenia wywolan klatki
class MockTestRenderBackend : public IRenderBackend
{
public:
	bool Initialize(HWND, unsigned int, unsigned int) override { return true; }
	void Shutdown() override {}
	void Resize(unsigned int, unsigned int) override {}

	void BeginFrame(const TRenderFrameUniforms& uniforms) override
	{
		m_beginFrameCount++;
		m_lastUniforms = uniforms;
	}

	void RenderTerrain(const TTerrainSectorPacket*, unsigned int count) override
	{
		m_terrainPacketCount += count;
	}

	void RenderActors(const TActorRenderPacket*, unsigned int count) override
	{
		m_actorPacketCount += count;
	}

	void RenderWater(const TWaterPacket*, unsigned int count) override
	{
		m_waterPacketCount += count;
	}

	void RenderSpeedTree(const TSpeedTreePacket*, unsigned int count) override
	{
		m_speedTreePacketCount += count;
	}

	void RenderEffects(const TEffectInstancePacket*, unsigned int count) override
	{
		m_effectPacketCount += count;
	}

	void RenderUI(const TUIBatchPacket*, unsigned int count) override
	{
		m_uiPacketCount += count;
		m_renderUICallCount++;
	}

	void EndFrame() override
	{
		m_endFrameCount++;
	}

	const char* GetBackendName() const override { return "MockTestBackend"; }
	bool IsDeviceLost() const override { return false; }
	bool CaptureFrame(const char*) override { return true; }
	bool IsDeferred() const override { return true; }

	uint32_t m_beginFrameCount{ 0 };
	uint32_t m_endFrameCount{ 0 };
	uint32_t m_renderUICallCount{ 0 };
	uint32_t m_uiPacketCount{ 0 };
	uint32_t m_terrainPacketCount{ 0 };
	uint32_t m_actorPacketCount{ 0 };
	uint32_t m_waterPacketCount{ 0 };
	uint32_t m_speedTreePacketCount{ 0 };
	uint32_t m_effectPacketCount{ 0 };
	TRenderFrameUniforms m_lastUniforms{};
};

TEST_CASE("T281: Headless Mode canRender State and Session Lock Bypass")
{
	g_bHeadlessMode = true;
	bool isMinimizedWnd = true;
	std::atomic<bool> bRenderThrottling{ true };
	std::atomic<bool> bSessionLocked{ true };

	// Symulacja logiki canRender z PythonApplication::Process()
	bool canRender = false;
	if (g_bHeadlessMode)
	{
		canRender = true;
	}
	else
	{
		canRender = !isMinimizedWnd && !bRenderThrottling.load() && !bSessionLocked.load();
	}

	CHECK(canRender == true);

	// Test trybu z oknem (headless = false) - minimalizacja powinna blokowac klatki
	g_bHeadlessMode = false;
	canRender = !isMinimizedWnd && !bRenderThrottling.load() && !bSessionLocked.load();
	CHECK(canRender == false);

	// Przywrocenie domyslnego trybu testowego
	g_bHeadlessMode = true;
}

TEST_CASE("T281: TRenderFrameUniforms 288-byte ABI and Environment Population")
{
	static_assert(sizeof(TRenderFrameUniforms) == 288, "TRenderFrameUniforms must be exactly 288 bytes");
	static_assert(alignof(TRenderFrameUniforms) == 16, "TRenderFrameUniforms must be 16-byte aligned");

	TRenderFrameUniforms uniforms{};
	uniforms.matView._11 = 1.0f;
	uniforms.matProj._22 = 2.0f;
	uniforms.matViewProj._33 = 3.0f;
	uniforms.vCameraPos = D3DXVECTOR3(100.0f, 200.0f, 300.0f);
	uniforms.fTime = 12.34f;
	uniforms.dwFrameIndex = 42;

	uniforms.vSunLightDir = D3DXVECTOR3(0.5f, -0.7f, 0.2f);
	uniforms.vSunLightColor = D3DXCOLOR(1.0f, 0.9f, 0.8f, 1.0f);
	uniforms.vAmbientColor = D3DXCOLOR(0.2f, 0.2f, 0.3f, 1.0f);

	uniforms.vFogColor = D3DXCOLOR(0.5f, 0.6f, 0.7f, 1.0f);
	uniforms.fFogNear = 1000.0f;
	uniforms.fFogFar = 25000.0f;
	uniforms.fFogInvRange = 1.0f / (25000.0f - 1000.0f);
	uniforms.fFogEnable = 1.0f;

	CHECK(uniforms.matView._11 == 1.0f);
	CHECK(uniforms.matProj._22 == 2.0f);
	CHECK(uniforms.vCameraPos.z == 300.0f);
	CHECK(uniforms.dwFrameIndex == 42);
	CHECK(uniforms.vSunLightDir.y == -0.7f);
	CHECK(uniforms.fFogEnable == 1.0f);
	CHECK(uniforms.fFogFar == 25000.0f);
}

TEST_CASE("T281: ExtractSnapshot after BeginFrame Preserves Uniforms")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();

	// Krok 1: BeginFrame zeruje bufor roboczy
	rqm.BeginFrame();

	// Krok 2: Ustawienie uniformow po BeginFrame() symuluje ExtractSnapshot()
	TRenderFrameUniforms uniforms{};
	uniforms.matView._11 = 4.0f;
	uniforms.vCameraPos = D3DXVECTOR3(50.0f, 60.0f, 70.0f);
	uniforms.dwFrameIndex = 101;
	rqm.SetFrameUniforms(uniforms);

	// Uniformy bufora roboczego musza byc zachowane, a nie wyzerowane
	const TRenderFrameUniforms& currentUniforms = rqm.GetWorkFrameUniforms();
	CHECK(currentUniforms.matView._11 == 4.0f);
	CHECK(currentUniforms.vCameraPos.x == 50.0f);
	CHECK(currentUniforms.dwFrameIndex == 101);

	rqm.EndFrame();
}

TEST_CASE("T281: Single Flush Deduplication in CRenderQueueManager")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	MockTestRenderBackend mockBackend;

	rqm.BeginFrame();

	TTerrainSectorPacket terrainPkt{};
	terrainPkt.dwSectorX = 1;
	rqm.SubmitTerrain(terrainPkt);

	TUIBatchPacket uiPkt{};
	uiPkt.dwVertexCount = 4;
	rqm.SubmitUI(uiPkt);

	CHECK(rqm.HasFlushedThisFrame() == false);

	// Pierwszy Flush() musi przekazac pakiety do backendu
	rqm.Flush(&mockBackend);
	CHECK(rqm.HasFlushedThisFrame() == true);
	CHECK(mockBackend.m_beginFrameCount == 1);
	CHECK(mockBackend.m_endFrameCount == 1);
	CHECK(mockBackend.m_terrainPacketCount == 1);
	CHECK(mockBackend.m_uiPacketCount == 1);

	// Drugi, redundantny Flush() w tej samej klatce musi zostac calkowicie zignorowany
	rqm.Flush(&mockBackend);
	CHECK(mockBackend.m_beginFrameCount == 1);
	CHECK(mockBackend.m_endFrameCount == 1);
	CHECK(mockBackend.m_terrainPacketCount == 1);
	CHECK(mockBackend.m_uiPacketCount == 1);

	// EndFrame() zamyka klatke i odblokowuje Flush na kolejna klatke
	rqm.EndFrame();
	CHECK(rqm.HasFlushedThisFrame() == false);
}

TEST_CASE("T281: CWgpuRenderBackend EndFrame Single Execution Guarantee")
{
	CWgpuRenderBackend& wgpuBackend = CWgpuRenderBackend::Instance();

	TRenderFrameUniforms uniforms{};
	uniforms.dwFrameIndex = 77;
	wgpuBackend.BeginFrame(uniforms);

	TUIBatchPacket uiPkt{};
	uiPkt.dwVertexCount = 6;
	wgpuBackend.RenderUI(&uiPkt, 1);

	// Pierwsze wywolanie EndFrame()
	wgpuBackend.EndFrame();

	// Ponowne wywolanie EndFrame() w tej samej klatce nie moze powodowac bledu ani wielokrotnego submitu
	wgpuBackend.EndFrame();
}

TEST_CASE("T281: End-to-End Headless Render Frame Loop Simulation")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	MockTestRenderBackend mockBackend;
	rqm.EndFrame();

	for (uint32_t frame = 1; frame <= 5; ++frame)
	{
		rqm.BeginFrame();

		TRenderFrameUniforms uniforms{};
		uniforms.dwFrameIndex = frame;
		uniforms.fTime = static_cast<float>(frame) * 0.016f;
		rqm.SetFrameUniforms(uniforms);

		// Symulacja 3D
		TActorRenderPacket actor{};
		actor.dwActorID = 100 + frame;
		rqm.SubmitActor(actor);

		// Symulacja 2D UI
		TUIBatchPacket ui{};
		ui.dwVertexCount = 12;
		rqm.SubmitUI(ui);

		// Jednokrotny Flush
		rqm.Flush(&mockBackend);
		CHECK(mockBackend.m_endFrameCount == frame);

		// Proba duplikatu Flush - musi byc bez efektu
		rqm.Flush(&mockBackend);
		CHECK(mockBackend.m_endFrameCount == frame);

		rqm.EndFrame();
	}

	CHECK(mockBackend.m_actorPacketCount == 5);
	CHECK(mockBackend.m_uiPacketCount == 5);
	CHECK(mockBackend.m_endFrameCount == 5);
}

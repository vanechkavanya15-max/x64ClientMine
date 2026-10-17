#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "EterBase/Timer.h"
#include "EterLib/SkyBox.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/D3D9RenderBackend.h"

static CTimer s_testTimer;

TEST_CASE("SkyBox Description - Texture Mode 6 Faces and Quad Count")
{
	CSkyBox skyBox;
	skyBox.SetRenderMode(CSkyObject::SKY_RENDER_MODE_TEXTURE);
	skyBox.Refresh();

	SSkyDescription desc = skyBox.BuildSkyDescription();

	// 1. Weryfikacja 6 scian
	CHECK(desc.faces[0].strFaceName == "front");
	CHECK(desc.faces[1].strFaceName == "back");
	CHECK(desc.faces[2].strFaceName == "left");
	CHECK(desc.faces[3].strFaceName == "right");
	CHECK(desc.faces[4].strFaceName == "top");
	CHECK(desc.faces[5].strFaceName == "bottom");

	// 2. Weryfikacja sumy quadow w kazdej scianie i lacznie
	size_t sumQuadsInVectors = 0;
	for (size_t f = 0; f < 6; ++f)
	{
		CHECK(desc.faces[f].quadVector.size() == 1);
		CHECK(skyBox.GetFace(f).m_SkyObjectQuadVector.size() == 1);
		sumQuadsInVectors += desc.faces[f].quadVector.size();
		CHECK(desc.faces[f].vertices.size() == 4);
	}

	CHECK(sumQuadsInVectors == 6);
	CHECK(desc.totalQuadCount == 6);
	CHECK(desc.gradientVertices.size() == 24);
	CHECK(desc.ucRenderMode == CSkyObject::SKY_RENDER_MODE_TEXTURE);
}

TEST_CASE("SkyBox Description - Diffuse Gradient Mode Face and Quad Count")
{
	CSkyBox skyBox;
	skyBox.SetRenderMode(CSkyObject::SKY_RENDER_MODE_DIFFUSE);
	// upper = 3, lower = 2 -> 5 quadow per vertical face (0..3)
	// top (4) = 1 quad, bottom (5) = 1 quad
	// lacznie: 4 * 5 + 2 = 22 quady
	skyBox.SetGradientLevel(3, 2);
	skyBox.Refresh();

	SSkyDescription desc = skyBox.BuildSkyDescription();

	CHECK(desc.faces[0].strFaceName == "front");
	CHECK(desc.faces[1].strFaceName == "back");
	CHECK(desc.faces[2].strFaceName == "left");
	CHECK(desc.faces[3].strFaceName == "right");
	CHECK(desc.faces[4].strFaceName == "top");
	CHECK(desc.faces[5].strFaceName == "bottom");

	size_t sumQuadsInVectors = 0;
	for (size_t f = 0; f < 4; ++f)
	{
		CHECK(desc.faces[f].quadVector.size() == 5);
		CHECK(skyBox.GetFace(f).m_SkyObjectQuadVector.size() == 5);
		sumQuadsInVectors += desc.faces[f].quadVector.size();
		CHECK(desc.faces[f].vertices.size() == 20); // 5 quadow * 4 wierzcholki
	}

	// top i bottom po 1 quadzie
	CHECK(desc.faces[4].quadVector.size() == 1);
	CHECK(desc.faces[5].quadVector.size() == 1);
	sumQuadsInVectors += desc.faces[4].quadVector.size();
	sumQuadsInVectors += desc.faces[5].quadVector.size();

	CHECK(sumQuadsInVectors == 22);
	CHECK(desc.totalQuadCount == 22);
	CHECK(desc.gradientVertices.size() == 22 * 4);
	CHECK(desc.ucRenderMode == CSkyObject::SKY_RENDER_MODE_DIFFUSE);
}

TEST_CASE("SkyBox Description - Cloud State and Matrices")
{
	CSkyBox skyBox;
	skyBox.SetRenderMode(CSkyObject::SKY_RENDER_MODE_TEXTURE);
	skyBox.SetCloudScale(D3DXVECTOR2(2.5f, 3.5f));
	skyBox.SetCloudHeight(1500.0f);
	skyBox.SetCloudScrollSpeed(D3DXVECTOR2(0.04f, 0.08f));
	skyBox.SetCloudTextureScale(D3DXVECTOR2(1.5f, 2.5f));
	skyBox.Refresh();

	SSkyDescription desc = skyBox.BuildSkyDescription();

	CHECK(desc.fCloudScaleX == doctest::Approx(2.5f));
	CHECK(desc.fCloudScaleY == doctest::Approx(3.5f));
	CHECK(desc.fCloudHeight == doctest::Approx(1500.0f));
	CHECK(desc.fCloudScrollSpeedU == doctest::Approx(0.04f));
	CHECK(desc.fCloudScrollSpeedV == doctest::Approx(0.08f));

	CHECK(desc.matCloudWorld._11 == doctest::Approx(2.5f));
	CHECK(desc.matCloudWorld._22 == doctest::Approx(3.5f));
	CHECK(desc.matCloudTexture._11 == doctest::Approx(1.5f));
	CHECK(desc.matCloudTexture._22 == doctest::Approx(2.5f));

	// Quad chmury
	CHECK(desc.faceCloud.quadVector.size() == 1);
	CHECK(desc.faceCloud.vertices.size() == 4);
}

TEST_CASE("SkyBox Description - RenderQueueManager and TSkyBoxPacket v2 Integration")
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();
	rqm.BeginFrame();

	CHECK(rqm.GetSkyBoxPackets().empty());
	CHECK(!rqm.HasSkyDescription());
	CHECK(rqm.GetSkyDescription() == nullptr);

	CSkyBox skyBox;
	skyBox.SetRenderMode(CSkyObject::SKY_RENDER_MODE_TEXTURE);
	skyBox.SetSkyBoxScale(D3DXVECTOR3(500.0f, 500.0f, 500.0f));
	skyBox.SetCloudScale(D3DXVECTOR2(2.0f, 2.0f));
	skyBox.SetCloudHeight(800.0f);
	skyBox.SetCloudScrollSpeed(D3DXVECTOR2(0.02f, 0.01f));
	skyBox.Refresh();

	TRenderFrameUniforms uniforms{};
	uniforms.vFogColor = D3DXCOLOR(0.4f, 0.6f, 0.8f, 1.0f);
	skyBox.SetAtmosphereUniforms(uniforms);

	// CSkyBox::Render() submituje pakiet i opis
	skyBox.Render();

	const auto& skyPackets = rqm.GetSkyBoxPackets();
	REQUIRE(skyPackets.size() == 1);
	const TSkyBoxPacket& pkt = skyPackets[0];

	CHECK(pkt.dwFaceCount == 6);
	CHECK(pkt.fCloudScale == doctest::Approx(2.0f));
	CHECK(pkt.fCloudHeight == doctest::Approx(800.0f));
	CHECK(pkt.fCloudScrollSpeed == doctest::Approx(0.02f));
	CHECK(pkt.vSkyColor.r == doctest::Approx(0.4f));
	CHECK(pkt.vSkyColor.g == doctest::Approx(0.6f));
	CHECK(pkt.vSkyColor.b == doctest::Approx(0.8f));

	REQUIRE(rqm.HasSkyDescription());
	const SSkyDescription* pDesc = rqm.GetSkyDescription();
	REQUIRE(pDesc != nullptr);
	CHECK(pDesc->totalQuadCount == 6);
	CHECK(pDesc->fCloudHeight == doctest::Approx(800.0f));
}

TEST_CASE("SkyBox Description - Ownership Guard and D3D9 Backend Execution")
{
	// 1. Sprawdzenie stanu domyslnego (Legacy)
	RenderOwnership::ResetDefaults();
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::SkyBox));
	CHECK(RenderOwnership::Owner(ERenderCategory::SkyBox) == ERenderOwner::Legacy);

	TSkyBoxPacket dummyPkt{};
	dummyPkt.dwFaceCount = 6;
	dummyPkt.matWorld._11 = 1.0f;
	dummyPkt.matWorld._22 = 1.0f;
	dummyPkt.matWorld._33 = 1.0f;
	dummyPkt.matWorld._44 = 1.0f;

	// W trybie Legacy backend D3D9 natychmiast wychodzi (bramka LegacyDraws)
	CD3D9RenderBackend::Instance().RenderSkyBox(&dummyPkt, 1);

	// 2. Przelaczenie wlasnosci na D3D9Backend
	RenderOwnership::SetOwner(ERenderCategory::SkyBox, ERenderOwner::D3D9Backend);
	CHECK(!RenderOwnership::LegacyDraws(ERenderCategory::SkyBox));
	CHECK(RenderOwnership::Owner(ERenderCategory::SkyBox) == ERenderOwner::D3D9Backend);

	// Gdy brak zainicjalizowanego urzadzenia D3D9, GetDevice() zwraca nullptr i bezpiecznie wychodzi
	CD3D9RenderBackend::Instance().RenderSkyBox(&dummyPkt, 1);

	// Przywrocenie stanu domyslnego
	RenderOwnership::ResetDefaults();
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::SkyBox));
}

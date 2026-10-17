#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <d3d9.h>
#include <d3dx9.h>
#include <cmath>
#include <cstdint>
#include <cstring>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderConventions.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderResourceRegistry.h"

TEST_CASE("WaterPacket - Layout and ABI Compatibility")
{
	CHECK(sizeof(TWaterPacket) == 176u);
	CHECK(alignof(TWaterPacket) == 16u);

	CHECK(offsetof(TWaterPacket, fWorldPosX) == 0u);
	CHECK(offsetof(TWaterPacket, fWorldPosY) == 4u);
	CHECK(offsetof(TWaterPacket, dwSectorX) == 8u);
	CHECK(offsetof(TWaterPacket, dwSectorY) == 12u);
	CHECK(offsetof(TWaterPacket, sectorX) == 16u);
	CHECK(offsetof(TWaterPacket, sectorY) == 18u);
	CHECK(offsetof(TWaterPacket, fWaterHeight) == 20u);
	CHECK(offsetof(TWaterPacket, fWaveTime) == 24u);
	CHECK(offsetof(TWaterPacket, fWaveFrequency) == 28u);
	CHECK(offsetof(TWaterPacket, fWaveAmplitude) == 32u);
	CHECK(offsetof(TWaterPacket, vWaterColor) == 36u);
	CHECK(offsetof(TWaterPacket, fAlpha) == 52u);
	CHECK(offsetof(TWaterPacket, dwWaterTextureID) == 56u);
	CHECK(offsetof(TWaterPacket, dwPadTexture) == 60u);
	CHECK(offsetof(TWaterPacket, pWaterTexture) == 64u);
	CHECK(offsetof(TWaterPacket, dwCellCount) == 72u);
	CHECK(offsetof(TWaterPacket, dwVertexCount) == 76u);
	CHECK(offsetof(TWaterPacket, pVertexBuffer) == 80u);
	CHECK(offsetof(TWaterPacket, dwFaceCount) == 88u);
	CHECK(offsetof(TWaterPacket, patchNum) == 92u);
	CHECK(offsetof(TWaterPacket, bFarPass) == 96u);
	CHECK(offsetof(TWaterPacket, bAlphaBlend) == 97u);
	CHECK(offsetof(TWaterPacket, fFogDistance) == 100u);
	CHECK(offsetof(TWaterPacket, fPad) == 104u);
	CHECK(offsetof(TWaterPacket, dwPadAlign16) == 108u);
	CHECK(offsetof(TWaterPacket, matTexTransformWater) == 112u);
	CHECK(offsetof(TWaterPacket, matUV) == 112u);

	TWaterPacket defPkt{};
	CHECK(defPkt.fWaveFrequency == 1.5f);
	CHECK(defPkt.fWaveAmplitude == 5.0f);
	CHECK(defPkt.fAlpha == 0.6f);
	CHECK(defPkt.bAlphaBlend == 1u);
	CHECK(defPkt.bFarPass == 0u);
	CHECK(defPkt.dwWaterTextureID == 0u);
	CHECK(defPkt.pWaterTexture == nullptr);
	CHECK(defPkt.pVertexBuffer == nullptr);
}

TEST_CASE("WaterPacket - Animation Frame Index Calculation")
{
	CHECK(CalculateWaterAnimationFrameIndex(0) == 0u);
	CHECK(CalculateWaterAnimationFrameIndex(69) == 0u);
	CHECK(CalculateWaterAnimationFrameIndex(70) == 1u);
	CHECK(CalculateWaterAnimationFrameIndex(139) == 1u);
	CHECK(CalculateWaterAnimationFrameIndex(140) == 2u);

	// Ostatnia klatka w cyklu 2100 ms (29 * 70 = 2030 .. 2099 ms)
	CHECK(CalculateWaterAnimationFrameIndex(2030) == 29u);
	CHECK(CalculateWaterAnimationFrameIndex(2099) == 29u);

	// Zawiniecie petli animacji co 30 klatek (2100 ms)
	CHECK(CalculateWaterAnimationFrameIndex(2100) == 0u);
	CHECK(CalculateWaterAnimationFrameIndex(4200) == 0u);

	for (uint32_t t = 0; t < 10000; t += 25)
	{
		uint32_t frame = CalculateWaterAnimationFrameIndex(t);
		CHECK(frame < 30u);
	}
}

TEST_CASE("WaterPacket - Wave Height Pure Function")
{
	float h0 = CalculateWaterWaveHeight(0.0f, 0.0f, 0.0f);
	CHECK(std::abs(h0 - (-6.0f)) < 1e-4f);

	for (float t = 0.0f; t < 20.0f; t += 0.25f)
	{
		float h = CalculateWaterWaveHeight(500.0f, 600.0f, t);
		CHECK(h >= -12.0001f);
		CHECK(h <= 0.0001f);
	}
}

TEST_CASE("WaterPacket - UV Transform Matrix Construction")
{
	D3DXMATRIX matViewInv;
	D3DXMatrixIdentity(&matViewInv);

	const float fTexScale = 0.0025f;
	D3DXMATRIX matUV;
	BuildWaterUVTransformMatrix(&matUV, matViewInv, fTexScale);

	CHECK(matUV._11 == doctest::Approx(fTexScale));
	CHECK(matUV._22 == doctest::Approx(-fTexScale));
	CHECK(matUV._33 == doctest::Approx(0.0f));
	CHECK(matUV._44 == doctest::Approx(1.0f));
	CHECK(matUV._41 == doctest::Approx(0.0f));
	CHECK(matUV._42 == doctest::Approx(0.0f));

	// Przesuniecie widoku kamery w ViewInverse
	D3DXMatrixTranslation(&matViewInv, 1000.0f, 2000.0f, 50.0f);
	BuildWaterUVTransformMatrix(&matUV, matViewInv, fTexScale);

	CHECK(matUV._11 == doctest::Approx(fTexScale));
	CHECK(matUV._22 == doctest::Approx(-fTexScale));
	CHECK(matUV._41 == doctest::Approx(1000.0f * fTexScale));
	CHECK(matUV._42 == doctest::Approx(2000.0f * -fTexScale));
}

TEST_CASE("WaterPacket - Submission to RenderQueueManager and Parity State")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	D3DXMATRIX matUV;
	D3DXMatrixIdentity(&matUV);

	// 1. Pakiet pasma Near
	TWaterPacket nearPkt{};
	nearPkt.patchNum = 42;
	nearPkt.sectorX = 2;
	nearPkt.sectorY = 3;
	nearPkt.fWaterHeight = -4.5f;
	nearPkt.fWaveTime = 1.25f;
	nearPkt.bFarPass = 0;
	nearPkt.bAlphaBlend = 1;
	nearPkt.fFogDistance = 2500.0f;
	nearPkt.dwWaterTextureID = 101;
	nearPkt.dwFaceCount = 128;
	nearPkt.matTexTransformWater = matUV;
	queue.SubmitWater(nearPkt);

	// 2. Pakiet pasma Far
	TWaterPacket farPkt{};
	farPkt.patchNum = 43;
	farPkt.sectorX = 2;
	farPkt.sectorY = 3;
	farPkt.fWaterHeight = -4.5f;
	farPkt.fWaveTime = 1.25f;
	farPkt.bFarPass = 1;
	farPkt.bAlphaBlend = 0;
	farPkt.fFogDistance = 2500.0f;
	farPkt.dwWaterTextureID = 0;
	farPkt.dwFaceCount = 64;
	farPkt.matTexTransformWater = matUV;
	queue.SubmitWater(farPkt);

	const auto& packets = queue.GetWaterPackets();
	REQUIRE(packets.size() >= 2u);

	const auto& submittedNear = packets[packets.size() - 2];
	CHECK(submittedNear.patchNum == 42u);
	CHECK(submittedNear.bFarPass == 0u);
	CHECK(submittedNear.bAlphaBlend == 1u);
	CHECK(submittedNear.dwWaterTextureID == 101u);
	CHECK(submittedNear.dwFaceCount == 128u);
	CHECK(submittedNear.fWaterHeight == -4.5f);

	const auto& submittedFar = packets[packets.size() - 1];
	CHECK(submittedFar.patchNum == 43u);
	CHECK(submittedFar.bFarPass == 1u);
	CHECK(submittedFar.bAlphaBlend == 0u);
	CHECK(submittedFar.dwWaterTextureID == 0u);
	CHECK(submittedFar.dwFaceCount == 64u);

	auto sizeMap = CRenderQueueManager::GetPacketSizeMap();
	CHECK(sizeMap.at("TWaterPacket") == 176u);
}

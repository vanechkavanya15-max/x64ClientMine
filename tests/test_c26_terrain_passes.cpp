#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>
#include <vector>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderResourceRegistry.h"

TEST_CASE("TerrainSectorPacket - ABI Layout and Exact Size")
{
	CHECK(sizeof(TTerrainSectorPacket) == 176u);
	CHECK(alignof(TTerrainSectorPacket) == 16u);
	CHECK(sizeof(TTerrainPassPacket) == 96u);

	TTerrainSectorPacket pkt{};
	CHECK(pkt.nCoordX == 0);
	CHECK(pkt.nCoordY == 0);
	CHECK(pkt.dwSectorX == 0u);
	CHECK(pkt.dwSectorY == 0u);
	CHECK(pkt.sectorX == 0u);
	CHECK(pkt.sectorY == 0u);
	CHECK(pkt.fWorldPosX == 0.0f);
	CHECK(pkt.fWorldPosY == 0.0f);
	CHECK(pkt.dwPrimitiveType == 0u);
	CHECK(pkt.dwPrimitiveCount == 0u);
	CHECK(pkt.patchNum == 0u);
	CHECK(pkt.pVertexData == nullptr);
	CHECK(pkt.dwVertexCount == 0u);
	CHECK(pkt.pPassPackets == nullptr);
	CHECK(pkt.dwPassCount == 0u);
	CHECK(pkt.pIndexBuffer == nullptr);
	CHECK(pkt.pShadowTexture == nullptr);
	CHECK(pkt.pChrShadowTexture == nullptr);
	CHECK(pkt.bHasWater == false);
	CHECK(pkt.byBand == 0u);
	CHECK(pkt.byLODLevel == 0u);
	CHECK(pkt.bDrawShadow == 0u);
	CHECK(pkt.HasShadow() == false);
	CHECK(pkt.HasCharacterShadow() == false);

	pkt.bDrawShadow = 1;
	CHECK(pkt.HasShadow() == true);

	pkt.SetCharacterShadow(true);
	CHECK(pkt.HasCharacterShadow() == true);

	pkt.SetCharacterShadow(false);
	CHECK(pkt.HasCharacterShadow() == false);
}

TEST_CASE("TerrainPasses - Splat Pass Structure and State Parity")
{
	STerrainSplatPass pass1{};
	pass1.layerIndex = 1;
	pass1.textureId = 101;
	pass1.alphaTextureId = 201;
	pass1.bDisableAlphaOp = true;
	pass1.bShadowPass = false;

	D3DXMatrixIdentity(&pass1.matTextureTransform);
	pass1.matTextureTransform._11 = 2.0f;
	D3DXMatrixIdentity(&pass1.matAlphaTransform);
	pass1.matAlphaTransform._22 = 0.5f;

	CHECK(pass1.layerIndex == 1u);
	CHECK(pass1.textureId == 101u);
	CHECK(pass1.alphaTextureId == 201u);
	CHECK(pass1.bDisableAlphaOp == true);
	CHECK(pass1.matTextureTransform._11 == 2.0f);
	CHECK(pass1.matAlphaTransform._22 == 0.5f);

	STerrainSplatPass pass2{};
	pass2.layerIndex = 2;
	pass2.textureId = 102;
	pass2.alphaTextureId = 202;
	pass2.bDisableAlphaOp = false;
	CHECK(pass2.bDisableAlphaOp == false);

	// Sprawdzenie wartosci enum pasm renderowania terenu
	CHECK(static_cast<uint8_t>(TERRAIN_BAND_NEAR) == 0u);
	CHECK(static_cast<uint8_t>(TERRAIN_BAND_FOG) == 1u);
	CHECK(static_cast<uint8_t>(TERRAIN_BAND_FAR) == 2u);
}

TEST_CASE("TerrainSector - Splat Limit and Pass Truncation Logic")
{
	constexpr int iSplatLimit = 5;
	int iRenderedSplatNum = 0;
	std::vector<STerrainSplatPass> passes;

	// Symulacja patcha z 8 potencjalnymi warstwami
	for (uint32_t j = 1; j <= 8; ++j)
	{
		STerrainSplatPass pass{};
		pass.layerIndex = j;
		pass.bDisableAlphaOp = (passes.empty());
		passes.push_back(pass);

		++iRenderedSplatNum;
		if (iRenderedSplatNum >= iSplatLimit)
			break;
	}

	CHECK(passes.size() == 5u);
	CHECK(iRenderedSplatNum == 5);
	CHECK(passes[0].bDisableAlphaOp == true);
	CHECK(passes[1].bDisableAlphaOp == false);
	CHECK(passes[4].bDisableAlphaOp == false);

	// Pasmo Far (RenderPatchNone) ma miec 0 przebiegow splattingu
	TTerrainSectorPacket farPkt{};
	farPkt.byBand = TERRAIN_BAND_FAR;
	farPkt.dwPassCount = 0;
	farPkt.pPassPackets = nullptr;
	farPkt.dwPrimitiveType = D3DPT_TRIANGLELIST;
	farPkt.dwPrimitiveCount = 128;

	CHECK(farPkt.byBand == TERRAIN_BAND_FAR);
	CHECK(farPkt.dwPassCount == 0u);
	CHECK(farPkt.dwPrimitiveCount == 128u);
}

TEST_CASE("TerrainSector - Submission to RenderQueueManager and Packet Retrieval")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	std::vector<STerrainSplatPass> mockPasses;
	mockPasses.resize(3);
	mockPasses[0].layerIndex = 1;
	mockPasses[0].textureId = 11;
	mockPasses[0].bDisableAlphaOp = true;
	mockPasses[1].layerIndex = 2;
	mockPasses[1].textureId = 12;
	mockPasses[1].bDisableAlphaOp = false;
	mockPasses[2].layerIndex = 3;
	mockPasses[2].textureId = 13;
	mockPasses[2].bDisableAlphaOp = false;

	TTerrainSectorPacket pkt{};
	pkt.dwSectorX = 3;
	pkt.dwSectorY = 4;
	pkt.patchNum = 42;
	pkt.fWorldPosX = 3000.0f;
	pkt.fWorldPosY = 4000.0f;
	pkt.pVertexData = reinterpret_cast<void*>(static_cast<uintptr_t>(0x12345678));
	pkt.dwVertexCount = 289;
	pkt.pIndexBuffer = reinterpret_cast<void*>(static_cast<uintptr_t>(0x87654321));
	pkt.dwPrimitiveType = D3DPT_TRIANGLELIST;
	pkt.dwPrimitiveCount = 512;
	pkt.byLODLevel = 1;
	pkt.byBand = TERRAIN_BAND_NEAR;
	pkt.pPassData = mockPasses.data();
	pkt.dwPassCount = static_cast<uint32_t>(mockPasses.size());
	pkt.bDrawShadow = 1;
	pkt.SetCharacterShadow(true);
	pkt.pShadowTexture = reinterpret_cast<void*>(static_cast<uintptr_t>(0xAAAABBBB));
	pkt.pChrShadowTexture = reinterpret_cast<void*>(static_cast<uintptr_t>(0xCCCCDDDD));
	pkt.bHasWater = true;
	pkt.fWaterHeight = -100.0f;

	queue.SubmitTerrain(pkt);

	const auto& submittedPackets = queue.GetTerrainPackets();
	REQUIRE(submittedPackets.size() >= 1u);

	const auto& submitted = submittedPackets.back();
	CHECK(submitted.dwSectorX == 3u);
	CHECK(submitted.dwSectorY == 4u);
	CHECK(submitted.patchNum == 42u);
	CHECK(submitted.fWorldPosX == doctest::Approx(3000.0f));
	CHECK(submitted.fWorldPosY == doctest::Approx(4000.0f));
	CHECK(submitted.dwVertexCount == 289u);
	CHECK(submitted.dwPrimitiveType == static_cast<uint32_t>(D3DPT_TRIANGLELIST));
	CHECK(submitted.dwPrimitiveCount == 512u);
	CHECK(submitted.byLODLevel == 1u);
	CHECK(submitted.byBand == TERRAIN_BAND_NEAR);
	CHECK(submitted.dwPassCount == 3u);
	CHECK(submitted.HasShadow() == true);
	CHECK(submitted.HasCharacterShadow() == true);
	CHECK(submitted.bHasWater == true);
	CHECK(submitted.fWaterHeight == doctest::Approx(-100.0f));

	const auto* retrievedPasses = reinterpret_cast<const STerrainSplatPass*>(submitted.pPassData);
	REQUIRE(retrievedPasses != nullptr);
	CHECK(retrievedPasses[0].layerIndex == 1u);
	CHECK(retrievedPasses[0].textureId == 11u);
	CHECK(retrievedPasses[0].bDisableAlphaOp == true);
	CHECK(retrievedPasses[1].layerIndex == 2u);
	CHECK(retrievedPasses[1].bDisableAlphaOp == false);
	CHECK(retrievedPasses[2].layerIndex == 3u);
	CHECK(retrievedPasses[2].bDisableAlphaOp == false);
}

TEST_CASE("TerrainPassPacket - ABI Layout and Field Mapping")
{
	TTerrainPassPacket passPkt{};
	passPkt.layerIndex = 2;
	passPkt.alphaSplatHandle = 301;
	passPkt.lodLevel = 1;
	passPkt.indexBufferHandle = 10;
	passPkt.nearDistance = 2400.0f;
	passPkt.fogDistance = 4500.0f;
	passPkt.farDistance = 9000.0f;
	passPkt.shadowTextureHandle = 501;

	D3DXMATRIX matId;
	D3DXMatrixIdentity(&matId);
	matId._41 = 100.0f;
	passPkt.matLayer = ToAbi(matId);

	CHECK(passPkt.layerIndex == 2u);
	CHECK(passPkt.alphaSplatHandle == 301u);
	CHECK(passPkt.lodLevel == 1u);
	CHECK(passPkt.indexBufferHandle == 10u);
	CHECK(passPkt.nearDistance == doctest::Approx(2400.0f));
	CHECK(passPkt.fogDistance == doctest::Approx(4500.0f));
	CHECK(passPkt.farDistance == doctest::Approx(9000.0f));
	CHECK(passPkt.shadowTextureHandle == 501u);
	CHECK(passPkt.matLayer.m[12] == doctest::Approx(100.0f));
}

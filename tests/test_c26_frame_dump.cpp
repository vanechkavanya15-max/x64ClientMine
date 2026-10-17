#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderQueueManager.h"
#include "EterBase/CRC32.h"
#include <vector>
#include <fstream>
#include <cstring>
#include <cstdio>

namespace
{
std::vector<uint8_t> ReadBinaryFile(const char* filename)
{
	std::vector<uint8_t> data;
	FILE* fp = nullptr;
	if (fopen_s(&fp, filename, "rb") != 0 || !fp)
		return data;

	fseek(fp, 0, SEEK_END);
	long sz = ftell(fp);
	if (sz > 0)
	{
		fseek(fp, 0, SEEK_SET);
		data.resize(static_cast<size_t>(sz));
		fread(data.data(), 1, static_cast<size_t>(sz), fp);
	}
	fclose(fp);
	return data;
}

void PopulateSampleFrame(CRenderQueueManager& queue)
{
	queue.BeginFrame();

	TRenderFrameUniforms uniforms{};
	uniforms.fTime = 42.125f;
	uniforms.vCameraPos = { 100.0f, 200.0f, 300.0f };
	uniforms.vSunLightDir = { 0.5f, -0.8f, 0.3f };
	uniforms.dwFrameIndex = 999;
	uniforms.vSunLightColor = { 1.0f, 0.95f, 0.9f, 1.0f };
	uniforms.vAmbientColor = { 0.2f, 0.2f, 0.25f, 1.0f };
	uniforms.vFogColor = { 0.6f, 0.7f, 0.8f, 1.0f };
	uniforms.fFogNear = 100.0f;
	uniforms.fFogFar = 5000.0f;
	uniforms.fFogEnable = 1.0f;
	queue.SetFrameUniforms(uniforms);

	// 1. SkyBox
	TSkyBoxPacket skyPkt{};
	skyPkt.dwFaceCount = 6;
	skyPkt.dwCloudTextureID = 101;
	skyPkt.dwSkyTextureID = 102;
	skyPkt.fCloudScale = 2.5f;
	skyPkt.fCloudHeight = 600.0f;
	skyPkt.fCloudScrollSpeed = 0.08f;
	skyPkt.vSkyColor = { 0.3f, 0.5f, 0.9f, 1.0f };
	queue.SubmitSkyBox(skyPkt);

	// 2. Terrain
	TTerrainSectorPacket terrPkt{};
	terrPkt.dwSectorX = 5;
	terrPkt.dwSectorY = 8;
	terrPkt.splatTextureId = 201;
	terrPkt.layerTextureIds[0] = 301;
	terrPkt.layerTextureIds[1] = 302;
	terrPkt.layerTextureIds[2] = 303;
	terrPkt.layerTextureIds[3] = 304;
	terrPkt.dwVertexCount = 256;
	terrPkt.bHasWater = true;
	terrPkt.fWaterHeight = 120.0f;
	queue.SubmitTerrain(terrPkt);

	// 3. Dungeon Block (z 64-bit identyfikatorami tekstur)
	TDungeonBlockPacket dungPkt{};
	dungPkt.dwDiffuseTextureID = 0x1122334455667788ULL;
	dungPkt.dwLightmapTextureID = 0x99AABBCCDDEEFF00ULL;
	dungPkt.dwVertexCount = 144;
	dungPkt.dwIndexCount = 432;
	dungPkt.dwPrimitiveCount = 144;
	dungPkt.dwStartIndex = 0;
	dungPkt.nBaseVertexIndex = 0;
	dungPkt.fAlpha = 0.85f;
	dungPkt.bAlphaBlend = 1;
	queue.SubmitDungeonBlock(dungPkt);

	// 4. Water
	TWaterPacket waterPkt{};
	waterPkt.fWaterHeight = 120.0f;
	waterPkt.fWaveTime = 1.5f;
	waterPkt.fWaveFrequency = 2.0f;
	waterPkt.fWaveAmplitude = 4.5f;
	waterPkt.dwWaterTextureID = 401;
	waterPkt.dwCellCount = 16;
	waterPkt.dwVertexCount = 64;
	queue.SubmitWater(waterPkt);

	// 5. SpeedTree (instancje z monotonicznej puli)
	TSpeedTreeInstanceData* pTreeInst = queue.AllocateSpeedTreeInstanceData(2);
	REQUIRE(pTreeInst != nullptr);
	pTreeInst[0].v3Position = { 10.0f, 20.0f, 30.0f };
	pTreeInst[0].dwTreeCRC = 0x12345678;
	pTreeInst[0].fWindStrength = 0.5f;
	pTreeInst[1].v3Position = { 40.0f, 50.0f, 60.0f };
	pTreeInst[1].dwTreeCRC = 0x87654321;
	pTreeInst[1].fWindStrength = 0.8f;

	TSpeedTreePacket treePkt{};
	treePkt.dwTreeCRC = 0xAABBCCDD;
	treePkt.bRenderBranches = true;
	treePkt.bRenderLeaves = true;
	treePkt.dwTreeTypeID = 501;
	treePkt.dwTrunkTextureID = 601;
	treePkt.dwLeafTextureID = 602;
	treePkt.dwInstanceCount = 2;
	treePkt.pInstances = pTreeInst;
	queue.SubmitSpeedTree(treePkt);

	// 6. Actor
	TActorRenderPacket actorPkt{};
	actorPkt.dwActorID = 10001;
	actorPkt.dwMeshID = 2001;
	actorPkt.dwTextureID = 3001;
	actorPkt.dwBoneCount = 32;
	actorPkt.dwWeaponMeshID = 4001;
	actorPkt.dwWeaponTextureID = 5001;
	actorPkt.fAlpha = 1.0f;
	actorPkt.fShadowSize = 15.0f;
	actorPkt.bRenderShadow = true;
	queue.SubmitActor(actorPkt);

	// 7. Effect (czasteczki z monotonicznej puli, 64-bit tekstura)
	TParticleInstanceData* pParts = queue.AllocateParticleData(2);
	REQUIRE(pParts != nullptr);
	pParts[0].vWorldPos = { 1.0f, 2.0f, 3.0f, 1.0f };
	pParts[0].fRotation = 0.25f;
	pParts[0].fHalfWidth = 10.0f;
	pParts[0].fHalfHeight = 10.0f;
	pParts[1].vWorldPos = { 4.0f, 5.0f, 6.0f, 1.0f };
	pParts[1].fRotation = 0.75f;
	pParts[1].fHalfWidth = 20.0f;
	pParts[1].fHalfHeight = 20.0f;

	TEffectInstancePacket effectPkt{};
	effectPkt.dwEffectID = 777;
	effectPkt.dwTextureID = 0xDEADBEEFCAFE0001ULL;
	effectPkt.dwBlendMode = 1;
	effectPkt.dwParticleCount = 2;
	effectPkt.pParticles = pParts;
	queue.SubmitEffect(effectPkt);

	// 8. UI (64-bit tekstura)
	static TUIVertex s_uiVerts[3];
	s_uiVerts[0].position = { 0.0f, 0.0f, 0.0f };
	s_uiVerts[0].color = 0xFFFFFFFF;
	s_uiVerts[1].position = { 100.0f, 0.0f, 0.0f };
	s_uiVerts[1].color = 0xFF00FF00;
	s_uiVerts[2].position = { 0.0f, 100.0f, 0.0f };
	s_uiVerts[2].color = 0xFFFF0000;

	TUIBatchPacket uiPkt{};
	uiPkt.dwTextureID = 0xFEDCBA9876543210ULL;
	uiPkt.bUseScissor = 1;
	uiPkt.scissorRect = { 10, 10, 500, 400 };
	uiPkt.dwBlendMode = 0;
	uiPkt.dwVertexCount = 3;
	uiPkt.pVertices = s_uiVerts;
	queue.SubmitUI(uiPkt);
}
}

TEST_CASE("FrameDumpV2 - All Packet Types Export and Chunk Verification")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	PopulateSampleFrame(queue);

	const char* dumpPath = "test_dump_all_packets.bin";
	bool bExport = queue.ExportFrameToBinary(dumpPath);
	CHECK(bExport == true);

	FrameDumpV2::TDumpParsedFrame parsed{};
	bool bParse = FrameDumpV2::ParseFrameDumpV2(dumpPath, parsed);
	CHECK(bParse == true);

	// Naglowek pliku v2
	CHECK(parsed.header.dwMagic == FrameDumpV2::M2FR_MAGIC);
	CHECK(parsed.header.dwVersion == FrameDumpV2::M2FR_VERSION);
	CHECK(parsed.header.dwChunkCount == 9u);
	CHECK(parsed.header.dwReserved == 0u);

	// Weryfikacja liczby chunkow
	REQUIRE(parsed.chunkHeaders.size() == 9u);

	// 1. UNIFORMS
	CHECK(parsed.chunkHeaders[0].dwTag == FrameDumpV2::CHUNK_UNIFORMS);
	CHECK(parsed.chunkHeaders[0].dwCount == 1u);
	CHECK(parsed.chunkHeaders[0].dwElementSize == sizeof(TRenderFrameUniforms));
	CHECK(parsed.chunkHeaders[0].dwDataSize == sizeof(TRenderFrameUniforms));
	CHECK(parsed.uniforms.fTime == 42.125f);
	CHECK(parsed.uniforms.dwFrameIndex == 999u);
	CHECK(parsed.uniforms.vCameraPos.x == 100.0f);

	// 2. SKYBOX
	CHECK(parsed.chunkHeaders[1].dwTag == FrameDumpV2::CHUNK_SKYBOX);
	CHECK(parsed.chunkHeaders[1].dwCount == 1u);
	CHECK(parsed.chunkHeaders[1].dwElementSize == sizeof(TSkyBoxPacket));
	CHECK(parsed.chunkHeaders[1].dwDataSize == sizeof(TSkyBoxPacket));
	REQUIRE(parsed.skyBoxPackets.size() == 1u);
	CHECK(parsed.skyBoxPackets[0].dwFaceCount == 6u);
	CHECK(parsed.skyBoxPackets[0].dwCloudTextureID == 101u);
	CHECK(parsed.skyBoxPackets[0].dwSkyTextureID == 102u);
	CHECK(parsed.skyBoxPackets[0].fCloudScale == 2.5f);

	// 3. TERRAIN
	CHECK(parsed.chunkHeaders[2].dwTag == FrameDumpV2::CHUNK_TERRAIN);
	CHECK(parsed.chunkHeaders[2].dwCount == 1u);
	CHECK(parsed.chunkHeaders[2].dwElementSize == sizeof(TTerrainSectorPacket));
	CHECK(parsed.chunkHeaders[2].dwDataSize == sizeof(TTerrainSectorPacket));
	REQUIRE(parsed.terrainPackets.size() == 1u);
	CHECK(parsed.terrainPackets[0].dwSectorX == 5u);
	CHECK(parsed.terrainPackets[0].dwSectorY == 8u);
	CHECK(parsed.terrainPackets[0].splatTextureId == 201u);
	CHECK(parsed.terrainPackets[0].pSplatTexture == nullptr);
	CHECK(parsed.terrainPackets[0].pVertexData == nullptr);

	// 4. DUNGEON
	CHECK(parsed.chunkHeaders[3].dwTag == FrameDumpV2::CHUNK_DUNGEON);
	CHECK(parsed.chunkHeaders[3].dwCount == 1u);
	CHECK(parsed.chunkHeaders[3].dwElementSize == sizeof(TDungeonBlockPacket));
	CHECK(parsed.chunkHeaders[3].dwDataSize == sizeof(TDungeonBlockPacket));
	REQUIRE(parsed.dungeonPackets.size() == 1u);
	CHECK(parsed.dungeonPackets[0].dwDiffuseTextureID == 0x1122334455667788ULL);
	CHECK(parsed.dungeonPackets[0].dwLightmapTextureID == 0x99AABBCCDDEEFF00ULL);
	CHECK(parsed.dungeonPackets[0].dwPrimitiveCount == 144u);
	CHECK(parsed.dungeonPackets[0].bAlphaBlend == 1u);
	CHECK(parsed.dungeonPackets[0].pDiffuseTexture == nullptr);
	CHECK(parsed.dungeonPackets[0].pVertexBuffer == nullptr);

	// 5. WATER
	CHECK(parsed.chunkHeaders[4].dwTag == FrameDumpV2::CHUNK_WATER);
	CHECK(parsed.chunkHeaders[4].dwCount == 1u);
	CHECK(parsed.chunkHeaders[4].dwElementSize == sizeof(TWaterPacket));
	CHECK(parsed.chunkHeaders[4].dwDataSize == sizeof(TWaterPacket));
	REQUIRE(parsed.waterPackets.size() == 1u);
	CHECK(parsed.waterPackets[0].fWaterHeight == 120.0f);
	CHECK(parsed.waterPackets[0].dwWaterTextureID == 401u);
	CHECK(parsed.waterPackets[0].pWaterTexture == nullptr);

	// 6. SPEEDTREE
	CHECK(parsed.chunkHeaders[5].dwTag == FrameDumpV2::CHUNK_SPEEDTREE);
	CHECK(parsed.chunkHeaders[5].dwCount == 1u);
	CHECK(parsed.chunkHeaders[5].dwElementSize == 0u);
	const uint32_t expectedTreeDataSize = sizeof(FrameDumpV2::TSpeedTreeBatchHeader) + 2 * sizeof(TSpeedTreeInstanceData);
	CHECK(parsed.chunkHeaders[5].dwDataSize == expectedTreeDataSize);
	REQUIRE(parsed.speedTreeBatches.size() == 1u);
	CHECK(parsed.speedTreeBatches[0].header.dwTreeCRC == 0xAABBCCDDu);
	CHECK(parsed.speedTreeBatches[0].header.uTreeTypeID == 501ULL);
	CHECK(parsed.speedTreeBatches[0].header.uTrunkTextureID == 601ULL);
	CHECK(parsed.speedTreeBatches[0].header.uLeafTextureID == 602ULL);
	CHECK(parsed.speedTreeBatches[0].header.dwInstanceCount == 2u);
	REQUIRE(parsed.speedTreeBatches[0].instances.size() == 2u);
	CHECK(parsed.speedTreeBatches[0].instances[0].dwTreeCRC == 0x12345678u);
	CHECK(parsed.speedTreeBatches[0].instances[1].dwTreeCRC == 0x87654321u);

	// 7. ACTORS
	CHECK(parsed.chunkHeaders[6].dwTag == FrameDumpV2::CHUNK_ACTORS);
	CHECK(parsed.chunkHeaders[6].dwCount == 1u);
	CHECK(parsed.chunkHeaders[6].dwElementSize == sizeof(TActorRenderPacket));
	CHECK(parsed.chunkHeaders[6].dwDataSize == sizeof(TActorRenderPacket));
	REQUIRE(parsed.actorPackets.size() == 1u);
	CHECK(parsed.actorPackets[0].dwActorID == 10001u);
	CHECK(parsed.actorPackets[0].dwMeshID == 2001u);
	CHECK(parsed.actorPackets[0].dwTextureID == 3001u);
	CHECK(parsed.actorPackets[0].pBoneMatrices == nullptr);

	// 8. EFFECTS
	CHECK(parsed.chunkHeaders[7].dwTag == FrameDumpV2::CHUNK_EFFECTS);
	CHECK(parsed.chunkHeaders[7].dwCount == 1u);
	CHECK(parsed.chunkHeaders[7].dwElementSize == 0u);
	const uint32_t expectedEffectDataSize = sizeof(FrameDumpV2::TEffectBatchHeader) + 2 * sizeof(TParticleInstanceData);
	CHECK(parsed.chunkHeaders[7].dwDataSize == expectedEffectDataSize);
	REQUIRE(parsed.effectBatches.size() == 1u);
	CHECK(parsed.effectBatches[0].header.dwEffectID == 777u);
	CHECK(parsed.effectBatches[0].header.uTextureID == 0xDEADBEEFCAFE0001ULL);
	CHECK(parsed.effectBatches[0].header.dwParticleCount == 2u);
	REQUIRE(parsed.effectBatches[0].particles.size() == 2u);
	CHECK(parsed.effectBatches[0].particles[0].fHalfWidth == 10.0f);
	CHECK(parsed.effectBatches[0].particles[1].fHalfHeight == 20.0f);

	// 9. UI
	CHECK(parsed.chunkHeaders[8].dwTag == FrameDumpV2::CHUNK_UI);
	CHECK(parsed.chunkHeaders[8].dwCount == 1u);
	CHECK(parsed.chunkHeaders[8].dwElementSize == 0u);
	const uint32_t expectedUIDataSize = sizeof(FrameDumpV2::TUIBatchHeader) + 3 * sizeof(TUIVertex);
	CHECK(parsed.chunkHeaders[8].dwDataSize == expectedUIDataSize);
	REQUIRE(parsed.uiBatches.size() == 1u);
	CHECK(parsed.uiBatches[0].header.uTextureID == 0xFEDCBA9876543210ULL);
	CHECK(parsed.uiBatches[0].header.bUseScissor == 1u);
	CHECK(parsed.uiBatches[0].header.dwVertexCount == 3u);
	REQUIRE(parsed.uiBatches[0].vertices.size() == 3u);
	CHECK(parsed.uiBatches[0].vertices[1].position.x == 100.0f);

	std::remove(dumpPath);
}

TEST_CASE("FrameDumpV2 - Byte Determinism and CRC32 Equality")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();

	const char* fileA = "test_det_a.bin";
	const char* fileB = "test_det_b.bin";

	// Zapis 1
	PopulateSampleFrame(queue);
	CHECK(queue.ExportFrameToBinary(fileA) == true);

	// Zapis 2 (dokladnie ta sama klatka po wyczyszczeniu i ponownym wypelnieniu)
	PopulateSampleFrame(queue);
	CHECK(queue.ExportFrameToBinary(fileB) == true);

	// Weryfikacja binarnej tozsamosci
	const auto bytesA = ReadBinaryFile(fileA);
	const auto bytesB = ReadBinaryFile(fileB);

	REQUIRE(bytesA.size() > 0u);
	REQUIRE(bytesA.size() == bytesB.size());
	CHECK(std::memcmp(bytesA.data(), bytesB.data(), bytesA.size()) == 0);

	// Weryfikacja zgodnosci sum kontrolnych CRC32
	DWORD crcA = GetFileCRC32(fileA);
	DWORD crcB = GetFileCRC32(fileB);
	CHECK(crcA != 0u);
	CHECK(crcA == crcB);

	std::remove(fileA);
	std::remove(fileB);
}

TEST_CASE("FrameDumpV2 - Empty Frame Chunked Structure")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	const char* emptyPath = "test_empty_frame.bin";
	CHECK(queue.ExportFrameToBinary(emptyPath) == true);

	FrameDumpV2::TDumpParsedFrame parsed{};
	CHECK(FrameDumpV2::ParseFrameDumpV2(emptyPath, parsed) == true);

	CHECK(parsed.header.dwMagic == FrameDumpV2::M2FR_MAGIC);
	CHECK(parsed.header.dwVersion == FrameDumpV2::M2FR_VERSION);
	CHECK(parsed.header.dwChunkCount == 9u);

	// Wszystkie kolekcje pakietow sa puste poza uniforms (1 element)
	CHECK(parsed.skyBoxPackets.empty());
	CHECK(parsed.terrainPackets.empty());
	CHECK(parsed.dungeonPackets.empty());
	CHECK(parsed.waterPackets.empty());
	CHECK(parsed.speedTreeBatches.empty());
	CHECK(parsed.actorPackets.empty());
	CHECK(parsed.effectBatches.empty());
	CHECK(parsed.uiBatches.empty());

	// Wymiar pustego pliku:
	// sizeof(THeader) [16] + 9 * sizeof(TChunkHeader) [9 * 16 = 144] + sizeof(TRenderFrameUniforms) [288] = 448 B
	const auto emptyBytes = ReadBinaryFile(emptyPath);
	const size_t expectedSize = sizeof(FrameDumpV2::THeader) + 9 * sizeof(FrameDumpV2::TChunkHeader) + sizeof(TRenderFrameUniforms);
	CHECK(emptyBytes.size() == expectedSize);
	CHECK(emptyBytes.size() == 448u);

	std::remove(emptyPath);
}

TEST_CASE("FrameDumpV2 - 64-bit ID Integrity")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	TDungeonBlockPacket dung{};
	dung.dwDiffuseTextureID = 0xFFFFFFFF00000001ULL;
	dung.dwLightmapTextureID = 0x8000000012345678ULL;
	queue.SubmitDungeonBlock(dung);

	TParticleInstanceData* pParts = queue.AllocateParticleData(1);
	REQUIRE(pParts != nullptr);
	pParts[0].fHalfWidth = 5.0f;
	pParts[0].fHalfHeight = 5.0f;

	TEffectInstancePacket effect{};
	effect.dwTextureID = 0xAAAAAAAAAAAAAAAAULL;
	effect.dwParticleCount = 1;
	effect.pParticles = pParts;
	queue.SubmitEffect(effect);

	TUIVertex uiV{};
	TUIBatchPacket ui{};
	ui.dwTextureID = 0xBBBBBBBBBBBBBBBBULL;
	ui.dwVertexCount = 1;
	ui.pVertices = &uiV;
	queue.SubmitUI(ui);

	TSpeedTreeInstanceData* pTree = queue.AllocateSpeedTreeInstanceData(1);
	REQUIRE(pTree != nullptr);
	TSpeedTreePacket tree{};
	tree.dwTreeTypeID = 0x12345678;
	tree.dwTrunkTextureID = 0x99998888;
	tree.dwLeafTextureID = 0x77776666;
	tree.dwInstanceCount = 1;
	tree.pInstances = pTree;
	queue.SubmitSpeedTree(tree);

	const char* highBitsPath = "test_high_bits.bin";
	CHECK(queue.ExportFrameToBinary(highBitsPath) == true);

	FrameDumpV2::TDumpParsedFrame parsed{};
	CHECK(FrameDumpV2::ParseFrameDumpV2(highBitsPath, parsed) == true);

	REQUIRE(parsed.dungeonPackets.size() == 1u);
	CHECK(parsed.dungeonPackets[0].dwDiffuseTextureID == 0xFFFFFFFF00000001ULL);
	CHECK(parsed.dungeonPackets[0].dwLightmapTextureID == 0x8000000012345678ULL);

	REQUIRE(parsed.effectBatches.size() == 1u);
	CHECK(parsed.effectBatches[0].header.uTextureID == 0xAAAAAAAAAAAAAAAAULL);

	REQUIRE(parsed.uiBatches.size() == 1u);
	CHECK(parsed.uiBatches[0].header.uTextureID == 0xBBBBBBBBBBBBBBBBULL);

	REQUIRE(parsed.speedTreeBatches.size() == 1u);
	CHECK(parsed.speedTreeBatches[0].header.uTrunkTextureID == 0x99998888ULL);
	CHECK(parsed.speedTreeBatches[0].header.uLeafTextureID == 0x77776666ULL);
	CHECK(parsed.speedTreeBatches[0].header.uTreeTypeID == 0x12345678ULL);

	std::remove(highBitsPath);
}

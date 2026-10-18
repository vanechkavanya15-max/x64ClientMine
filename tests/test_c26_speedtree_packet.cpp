#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <d3d9.h>
#include <d3dx9.h>
#include <cstdint>
#include <cstring>
#include <vector>

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/D3D9RenderBackend.h"
#include "SpeedTreeLib/SpeedTreeWrapper.h"

TEST_CASE("SpeedTreePacket - Layout and ABI Compatibility")
{
	CHECK(sizeof(TSpeedTreePacket) == 48u);
	CHECK(alignof(TSpeedTreePacket) == 16u);

	CHECK(sizeof(TSpeedTreeInstanceData) == 112u);
	CHECK(alignof(TSpeedTreeInstanceData) == 16u);

	CHECK(sizeof(TSpeedTreeBillboardVertex) == 20u);

	// Offsety TSpeedTreePacket
	CHECK(offsetof(TSpeedTreePacket, pMainTree) == 0u);
	CHECK(offsetof(TSpeedTreePacket, dwTreeCRC) == 8u);
	CHECK(offsetof(TSpeedTreePacket, bRenderBranches) == 12u);
	CHECK(offsetof(TSpeedTreePacket, bRenderFronds) == 13u);
	CHECK(offsetof(TSpeedTreePacket, bRenderLeaves) == 14u);
	CHECK(offsetof(TSpeedTreePacket, bRenderBillboards) == 15u);
	CHECK(offsetof(TSpeedTreePacket, dwTreeTypeID) == 16u);
	CHECK(offsetof(TSpeedTreePacket, dwTrunkTextureID) == 20u);
	CHECK(offsetof(TSpeedTreePacket, dwLeafTextureID) == 24u);
	CHECK(offsetof(TSpeedTreePacket, dwInstanceCount) == 28u);
	CHECK(offsetof(TSpeedTreePacket, pInstances) == 32u);
	CHECK(offsetof(TSpeedTreePacket, fPad) == 40u);
	CHECK(offsetof(TSpeedTreePacket, fAlphaRef) == 40u);
	CHECK(offsetof(TSpeedTreePacket, ucAlphaRefBranch) == 40u);
	CHECK(offsetof(TSpeedTreePacket, ucAlphaRefFrond) == 41u);
	CHECK(offsetof(TSpeedTreePacket, ucAlphaRefLeaf) == 42u);
	CHECK(offsetof(TSpeedTreePacket, ucAlphaRefBillboard) == 43u);
	CHECK(offsetof(TSpeedTreePacket, nDiscreteLod) == 44u);
	CHECK(offsetof(TSpeedTreePacket, wFlags) == 46u);

	// Offsety TSpeedTreeInstanceData
	CHECK(offsetof(TSpeedTreeInstanceData, pTreeInstance) == 0u);
	CHECK(offsetof(TSpeedTreeInstanceData, matWorld) == 16u);
	CHECK(offsetof(TSpeedTreeInstanceData, v3Position) == 80u);
	CHECK(offsetof(TSpeedTreeInstanceData, dwTreeCRC) == 92u);
	CHECK(offsetof(TSpeedTreeInstanceData, fWindStrength) == 96u);
	CHECK(offsetof(TSpeedTreeInstanceData, fPad) == 100u);
	CHECK(offsetof(TSpeedTreeInstanceData, fAlphaRefBranch) == 100u);
	CHECK(offsetof(TSpeedTreeInstanceData, fAlphaRefFrond) == 104u);
	CHECK(offsetof(TSpeedTreeInstanceData, fAlphaRefLeaf) == 108u);
	CHECK(offsetof(TSpeedTreeInstanceData, ucAlphaRefBranch) == 100u);
	CHECK(offsetof(TSpeedTreeInstanceData, ucAlphaRefFrond) == 101u);
	CHECK(offsetof(TSpeedTreeInstanceData, ucAlphaRefLeaf0) == 102u);
	CHECK(offsetof(TSpeedTreeInstanceData, ucAlphaRefLeaf1) == 103u);
	CHECK(offsetof(TSpeedTreeInstanceData, nBranchLod) == 104u);
	CHECK(offsetof(TSpeedTreeInstanceData, nFrondLod) == 105u);
	CHECK(offsetof(TSpeedTreeInstanceData, nLeafLod0) == 106u);
	CHECK(offsetof(TSpeedTreeInstanceData, nLeafLod1) == 107u);
	CHECK(offsetof(TSpeedTreeInstanceData, ucAlphaRefBillboard0) == 108u);
	CHECK(offsetof(TSpeedTreeInstanceData, ucAlphaRefBillboard1) == 109u);
	CHECK(offsetof(TSpeedTreeInstanceData, wInstanceFlags) == 110u);

	// Offsety TSpeedTreeBillboardVertex
	CHECK(offsetof(TSpeedTreeBillboardVertex, position) == 0u);
	CHECK(offsetof(TSpeedTreeBillboardVertex, texCoord) == 12u);
}

TEST_CASE("SpeedTreePacket - Initialization and Value Fidelity")
{
	TSpeedTreePacket pkt{};
	CHECK(pkt.pMainTree == nullptr);
	CHECK(pkt.dwTreeCRC == 0u);
	CHECK(pkt.bRenderBranches == false);
	CHECK(pkt.bRenderFronds == false);
	CHECK(pkt.bRenderLeaves == false);
	CHECK(pkt.bRenderBillboards == false);
	CHECK(pkt.dwTreeTypeID == 0u);
	CHECK(pkt.dwTrunkTextureID == 0u);
	CHECK(pkt.dwLeafTextureID == 0u);
	CHECK(pkt.dwInstanceCount == 0u);
	CHECK(pkt.pInstances == nullptr);
	CHECK(pkt.ucAlphaRefBranch == 0u);
	CHECK(pkt.nDiscreteLod == 0);

	TSpeedTreeInstanceData inst{};
	CHECK(inst.pTreeInstance == nullptr);
	CHECK(inst.dwTreeCRC == 0u);
	CHECK(inst.fWindStrength == 0.0f);
	CHECK(inst.ucAlphaRefBranch == 0u);
	CHECK(inst.nBranchLod == 0);

	TSpeedTreeBillboardVertex vert(100.0f, 200.0f, 300.0f, 0.25f, 0.75f);
	CHECK(vert.position.x == 100.0f);
	CHECK(vert.position.y == 200.0f);
	CHECK(vert.position.z == 300.0f);
	CHECK(vert.texCoord.x == 0.25f);
	CHECK(vert.texCoord.y == 0.75f);
}

TEST_CASE("SpeedTree - Zero Copy GetInstances Verification")
{
	auto tree = std::make_shared<CSpeedTreeWrapper>();
	UINT count1 = 0;
	UINT count2 = 0;

	const auto& insts1 = tree->GetInstances(count1);
	const auto& insts2 = tree->GetInstances(count2);

	// Sprawdzenie, ze obie referencje wskazuja na ten sam adres pamieci (brak klonowania wektora)
	CHECK(&insts1 == &insts2);
	CHECK(count1 == 0u);
	CHECK(count2 == 0u);

	auto instChild = tree->MakeInstance();
	REQUIRE(instChild != nullptr);

	UINT countAfter = 0;
	const auto& instsAfter = tree->GetInstances(countAfter);
	CHECK(&instsAfter == &insts1);
	CHECK(countAfter == 1u);
	CHECK(instsAfter.size() == 1u);
	CHECK(instsAfter[0] == instChild);
}

TEST_CASE("SpeedTreePacket - Submission to RenderQueueManager")
{
	CRenderQueueManager& queue = CRenderQueueManager::Instance();
	queue.BeginFrame();

	constexpr size_t kNumInstances = 3;
	TSpeedTreeInstanceData* pSlice = queue.AllocateSpeedTreeInstanceData(kNumInstances);
	REQUIRE(pSlice != nullptr);

	for (size_t i = 0; i < kNumInstances; ++i)
	{
		TSpeedTreeInstanceData& inst = pSlice[i];
		inst.dwTreeCRC = 0x12345678;
		inst.fWindStrength = 0.5f;
		inst.v3Position = D3DXVECTOR3(static_cast<float>(i * 100), 0.0f, 50.0f);
		D3DXMatrixTranslation(&inst.matWorld, inst.v3Position.x, inst.v3Position.y, inst.v3Position.z);
		inst.ucAlphaRefBranch = 84;
		inst.ucAlphaRefFrond = 84;
		inst.ucAlphaRefLeaf0 = 120;
		inst.ucAlphaRefLeaf1 = 140;
		inst.nBranchLod = static_cast<int8_t>(i);
		inst.nFrondLod = static_cast<int8_t>(i);
		inst.nLeafLod0 = 0;
		inst.nLeafLod1 = 1;
		inst.ucAlphaRefBillboard0 = 90;
		inst.ucAlphaRefBillboard1 = 110;
		inst.wInstanceFlags = 0;
	}

	// 1. Zgloszenie pakietu galezi
	TSpeedTreePacket branchPkt{};
	branchPkt.dwTreeCRC = 0x12345678;
	branchPkt.dwTreeTypeID = 0x12345678;
	branchPkt.bRenderBranches = true;
	branchPkt.bRenderFronds = true;
	branchPkt.dwInstanceCount = static_cast<uint32_t>(kNumInstances);
	branchPkt.pInstances = pSlice;
	branchPkt.ucAlphaRefBranch = 84;
	branchPkt.ucAlphaRefFrond = 84;
	branchPkt.nDiscreteLod = 0;
	queue.SubmitSpeedTree(branchPkt);

	// 2. Zgloszenie pakietu lisci
	TSpeedTreePacket leafPkt{};
	leafPkt.dwTreeCRC = 0x12345678;
	leafPkt.dwTreeTypeID = 0x12345678;
	leafPkt.bRenderLeaves = true;
	leafPkt.bRenderBillboards = true;
	leafPkt.dwInstanceCount = static_cast<uint32_t>(kNumInstances);
	leafPkt.pInstances = pSlice;
	leafPkt.ucAlphaRefLeaf = 120;
	leafPkt.ucAlphaRefBillboard = 90;
	leafPkt.nDiscreteLod = 0;
	queue.SubmitSpeedTree(leafPkt);

	const auto& packets = queue.GetSpeedTreePackets();
	REQUIRE(packets.size() >= 2u);

	const auto& submittedBranch = packets[packets.size() - 2];
	CHECK(submittedBranch.dwTreeCRC == 0x12345678u);
	CHECK(submittedBranch.bRenderBranches == true);
	CHECK(submittedBranch.bRenderFronds == true);
	CHECK(submittedBranch.bRenderLeaves == false);
	CHECK(submittedBranch.dwInstanceCount == 3u);
	CHECK(submittedBranch.ucAlphaRefBranch == 84u);

	const auto& submittedLeaf = packets[packets.size() - 1];
	CHECK(submittedLeaf.dwTreeCRC == 0x12345678u);
	CHECK(submittedLeaf.bRenderLeaves == true);
	CHECK(submittedLeaf.bRenderBillboards == true);
	CHECK(submittedLeaf.bRenderBranches == false);
	CHECK(submittedLeaf.dwInstanceCount == 3u);
	CHECK(submittedLeaf.ucAlphaRefLeaf == 120u);

	auto sizeMap = CRenderQueueManager::GetPacketSizeMap();
	CHECK(sizeMap.at("TSpeedTreePacket") == 48u);
	CHECK(sizeMap.at("TSpeedTreeInstanceData") == 112u);
}

TEST_CASE("SpeedTree - RenderOwnership and Backend Gate Semantics")
{
	RenderOwnership::ResetDefaults();

	// Domyslnie SpeedTree jest w trybie Legacy
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::SpeedTree) == true);
	CHECK(RenderOwnership::Owner(ERenderCategory::SpeedTree) == ERenderOwner::Legacy);

	CD3D9RenderBackend backend;

	// Gdy LegacyDraws jest aktywne, CD3D9RenderBackend::RenderSpeedTree nie powinno wykonywac pakietu
	TSpeedTreePacket dummyPkt{};
	backend.RenderSpeedTree(&dummyPkt, 1);

	// Przelaczenie wlasnosci na D3D9Backend
	RenderOwnership::SetOwner(ERenderCategory::SpeedTree, ERenderOwner::D3D9Backend);
	CHECK(RenderOwnership::LegacyDraws(ERenderCategory::SpeedTree) == false);
	CHECK(RenderOwnership::Owner(ERenderCategory::SpeedTree) == ERenderOwner::D3D9Backend);

	// Pusty pakiet lub count == 0 jest bezpiecznie ignorowany
	backend.RenderSpeedTree(nullptr, 0);
	backend.RenderSpeedTree(&dummyPkt, 0);

	RenderOwnership::ResetDefaults();
}

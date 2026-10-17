#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/GrpBase.h"
#include "EterLib/GrpScreen.h"

TEST_CASE("QueueStats - Initial Zero State")
{
	auto& queue = CRenderQueueManager::Instance();
	queue.ResetFrameStats();

	const auto& stats = queue.GetFrameStats();

	CHECK(stats.totalPackets == 0);
	CHECK(stats.totalDrawCalls == 0);
	CHECK(stats.totalFaceCount == 0);
	CHECK(stats.totalDropped == 0);

	const ERenderCategory allCategories[] = {
		ERenderCategory::SkyBox,
		ERenderCategory::Terrain,
		ERenderCategory::Water,
		ERenderCategory::Dungeon,
		ERenderCategory::SpeedTree,
		ERenderCategory::Model,
		ERenderCategory::WeaponTrace,
		ERenderCategory::Effect,
		ERenderCategory::EffectMesh,
		ERenderCategory::UI,
		ERenderCategory::Text,
		ERenderCategory::TextTail,
		ERenderCategory::MiniMap,
		ERenderCategory::Shadow,
		ERenderCategory::Debug,
		ERenderCategory::ImGui,
		ERenderCategory::Movie,
		ERenderCategory::Cursor
	};

	for (auto cat : allCategories)
	{
		const auto& catStats = stats.GetCategory(cat);
		CHECK(catStats.packetCount == 0);
		CHECK(catStats.drawCallCount == 0);
		CHECK(catStats.faceCount == 0);
		CHECK(catStats.droppedCount == 0);
		CHECK(catStats.GetPacketCount() == 0);
		CHECK(catStats.GetDrawCallCount() == 0);
		CHECK(catStats.GetFaceCount() == 0);
		CHECK(catStats.GetDroppedCount() == 0);
	}

	// Bezpieczenstwo dla kategorii out of bounds (Count)
	const auto& countStats = stats.GetCategory(ERenderCategory::Count);
	CHECK(countStats.packetCount == 0);
	CHECK(countStats.drawCallCount == 0);
	CHECK(countStats.faceCount == 0);
	CHECK(countStats.droppedCount == 0);
}

TEST_CASE("QueueStats - Packet Submission Increments Category Counters")
{
	auto& queue = CRenderQueueManager::Instance();
	queue.ResetFrameStats();

	// 1. SpeedTree
	TSpeedTreePacket speedTreePkt{};
	queue.SubmitSpeedTree(speedTreePkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::SpeedTree).packetCount == 1);

	// 2. UI
	TUIBatchPacket uiPkt{};
	queue.SubmitUI(uiPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::UI).packetCount == 1);

	// 3. Actor (Model)
	TActorRenderPacket actorPkt{};
	queue.SubmitActor(actorPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Model).packetCount == 1);

	// 4. Terrain
	TTerrainSectorPacket terrainPkt{};
	queue.SubmitTerrain(terrainPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Terrain).packetCount == 1);

	// 5. Effect
	TEffectInstancePacket effectPkt{};
	queue.SubmitEffect(effectPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Effect).packetCount == 1);

	// 6. Water
	TWaterPacket waterPkt{};
	queue.SubmitWater(waterPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Water).packetCount == 1);

	// 7. SkyBox
	TSkyBoxPacket skyBoxPkt{};
	queue.SubmitSkyBox(skyBoxPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::SkyBox).packetCount == 1);

	// 8. Dungeon
	TDungeonBlockPacket dungeonPkt{};
	queue.SubmitDungeonBlock(dungeonPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Dungeon).packetCount == 1);

	// 9. WeaponTrace
	TWeaponTracePacket tracePkt{};
	queue.SubmitWeaponTrace(tracePkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::WeaponTrace).packetCount == 1);

	// 10. EffectMesh
	TEffectMeshPacket effectMeshPkt{};
	queue.SubmitEffectMesh(effectMeshPkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::EffectMesh).packetCount == 1);

	// 11. DebugLine
	TDebugLinePacket debugLinePkt{};
	queue.SubmitDebugLine(debugLinePkt);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Debug).packetCount == 1);

	// 12. DebugLines (multi-count)
	TDebugLinePacket multiDebugLines[3]{};
	queue.SubmitDebugLines(multiDebugLines, 3);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Debug).packetCount == 4);

	// 13. Direct RecordPacket
	queue.RecordPacket(ERenderCategory::Text, 5);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Text).packetCount == 5);

	// Sumaryczna liczba pakietow: 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 3 + 5 = 19
	CHECK(queue.GetFrameStats().totalPackets == 19);
}

TEST_CASE("QueueStats - Draw Call and Face Count Recording")
{
	auto& queue = CRenderQueueManager::Instance();
	queue.ResetFrameStats();

	// Draw call dla UI
	queue.RecordDrawCall(ERenderCategory::UI, 2);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::UI).drawCallCount == 1);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::UI).faceCount == 2);
	CHECK(queue.GetFrameStats().totalDrawCalls == 1);
	CHECK(queue.GetFrameStats().totalFaceCount == 2);

	// Kolejny draw call dla UI
	queue.RecordDrawCall(ERenderCategory::UI, 4);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::UI).drawCallCount == 2);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::UI).faceCount == 6);
	CHECK(queue.GetFrameStats().totalDrawCalls == 2);
	CHECK(queue.GetFrameStats().totalFaceCount == 6);

	// Draw call dla Terrain
	queue.RecordDrawCall(ERenderCategory::Terrain, 1024);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Terrain).drawCallCount == 1);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Terrain).faceCount == 1024);
	CHECK(queue.GetFrameStats().totalDrawCalls == 3);
	CHECK(queue.GetFrameStats().totalFaceCount == 1030);

	// Draw call dla Model
	queue.RecordDrawCall(ERenderCategory::Model, 512);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Model).drawCallCount == 1);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Model).faceCount == 512);
	CHECK(queue.GetFrameStats().totalDrawCalls == 4);
	CHECK(queue.GetFrameStats().totalFaceCount == 1542);

	// Draw call bez przypisanej kategorii (Count)
	queue.RecordDrawCall(ERenderCategory::Count, 100);
	CHECK(queue.GetFrameStats().totalDrawCalls == 5);
	CHECK(queue.GetFrameStats().totalFaceCount == 1642);
}

TEST_CASE("QueueStats - Dropped Packet Recording")
{
	auto& queue = CRenderQueueManager::Instance();
	queue.ResetFrameStats();

	queue.RecordDroppedPacket(ERenderCategory::Effect, 8);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Effect).droppedCount == 8);
	CHECK(queue.GetFrameStats().totalDropped == 8);

	queue.RecordDroppedPacket(ERenderCategory::SpeedTree, 2);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::SpeedTree).droppedCount == 2);
	CHECK(queue.GetFrameStats().totalDropped == 10);

	queue.RecordDroppedPacket(ERenderCategory::Count, 5);
	CHECK(queue.GetFrameStats().totalDropped == 15);
}

TEST_CASE("QueueStats - Resetting via ResetFrameStats and BeginFrame")
{
	auto& queue = CRenderQueueManager::Instance();
	queue.ResetFrameStats();

	queue.RecordPacket(ERenderCategory::Water, 3);
	queue.RecordDrawCall(ERenderCategory::Water, 120);
	queue.RecordDroppedPacket(ERenderCategory::Water, 1);

	CHECK(queue.GetFrameStats().totalPackets == 3);
	CHECK(queue.GetFrameStats().totalDrawCalls == 1);
	CHECK(queue.GetFrameStats().totalFaceCount == 120);
	CHECK(queue.GetFrameStats().totalDropped == 1);

	// Test ResetFrameStats
	queue.ResetFrameStats();
	CHECK(queue.GetFrameStats().totalPackets == 0);
	CHECK(queue.GetFrameStats().totalDrawCalls == 0);
	CHECK(queue.GetFrameStats().totalFaceCount == 0);
	CHECK(queue.GetFrameStats().totalDropped == 0);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Water).packetCount == 0);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Water).drawCallCount == 0);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Water).faceCount == 0);
	CHECK(queue.GetFrameStats().GetCategory(ERenderCategory::Water).droppedCount == 0);

	// Ponowne dodanie danych i test BeginFrame
	TUIBatchPacket uiPkt{};
	queue.SubmitUI(uiPkt);
	queue.RecordDrawCall(ERenderCategory::UI, 10);
	queue.RecordDroppedPacket(ERenderCategory::UI, 2);

	CHECK(queue.GetFrameStats().totalPackets == 1);
	CHECK(queue.GetFrameStats().totalDrawCalls == 1);
	CHECK(queue.GetFrameStats().totalFaceCount == 10);
	CHECK(queue.GetFrameStats().totalDropped == 2);

	queue.BeginFrame();
	CHECK(queue.GetFrameStats().totalPackets == 0);
	CHECK(queue.GetFrameStats().totalDrawCalls == 0);
	CHECK(queue.GetFrameStats().totalFaceCount == 0);
	CHECK(queue.GetFrameStats().totalDropped == 0);
	CHECK(queue.GetUIPackets().empty());
}

TEST_CASE("QueueStats - CGraphicBase Face Count Unification")
{
	auto& queue = CRenderQueueManager::Instance();
	queue.ResetFrameStats();

	CGraphicBase grpBase;
	CHECK(grpBase.GetFaceCount() == 0);

	queue.RecordDrawCall(ERenderCategory::Model, 3456);
	CHECK(grpBase.GetFaceCount() == 3456);

	queue.RecordDrawCall(ERenderCategory::Terrain, 1000);
	CHECK(grpBase.GetFaceCount() == 4456);

	grpBase.ResetFaceCount();
	CHECK(grpBase.GetFaceCount() == 0);
	CHECK(queue.GetFrameStats().totalFaceCount == 0);
	CHECK(queue.GetFrameStats().totalDrawCalls == 0);
}

TEST_CASE("QueueStats - CScreen RenderStatistics Integration")
{
	auto& queue = CRenderQueueManager::Instance();
	queue.ResetFrameStats();

	queue.RecordDrawCall(ERenderCategory::UI, 77);
	queue.RecordDrawCall(ERenderCategory::Effect, 33);

	const auto& screenStats = CScreen::GetRenderStatistics();
	CHECK(screenStats.dwDrawCallCount == 2);
	CHECK(screenStats.dwFaceCount == 110);
	CHECK(screenStats.dwPrimitiveCount == 110);

	queue.ResetFrameStats();
	const auto& resetScreenStats = CScreen::GetRenderStatistics();
	CHECK(resetScreenStats.dwDrawCallCount == 0);
	CHECK(resetScreenStats.dwFaceCount == 0);
}

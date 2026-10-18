#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <vector>
#include <cmath>
#include <array>
#include <unordered_set>

#include "../src/GameLib/Terrain/TerrainCoordinates.h"
#include "../src/GameLib/Terrain/LinearSectorQuadtree.h"
#include "../src/GameLib/Terrain/TerrainHeightStorage.h"
#include "../src/GameLib/Terrain/ChunkStreamingCoordinator.h"
#include "../src/GameLib/Terrain/TerrainSubsystemFacade.h"
#include "../src/GameLib/Terrain/TerrainEvents.h"

using namespace GameLib::Terrain;

TEST_CASE("LinearSectorQuadtree Audit - Row-Major Patches & Accurate Z-Propagation") {
    LinearSectorQuadtree tree;
    CHECK(LinearSectorQuadtree::TotalNodes == 85);
    CHECK(LinearSectorQuadtree::TotalLeaves == 64);
    CHECK(LinearSectorQuadtree::Depth == 3);

    // Przygotuj siatke 131x131 w formacie Metin2
    std::vector<uint16_t> heights(131 * 131, 200);

    // Wstaw wyrazny szczyt w patchu o wspolrzednych (gx = 5, gy = 6)
    // patchId w row-major to: gy * 8 + gx = 6 * 8 + 5 = 53
    constexpr uint32_t targetGx = 5;
    constexpr uint32_t targetGy = 6;
    constexpr uint32_t expectedPatchId = targetGy * TerrainMetrics::PatchesPerSide + targetGx;
    static_assert(expectedPatchId == 53);

    uint32_t peakCellX = targetGx * TerrainMetrics::CellsPerPatch + 8;
    uint32_t peakCellY = targetGy * TerrainMetrics::CellsPerPatch + 8;
    uint32_t peakRawX = peakCellX + TerrainMetrics::RawBorderOffset;
    uint32_t peakRawY = peakCellY + TerrainMetrics::RawBorderOffset;
    heights[peakRawY * TerrainMetrics::HeightmapSize + peakRawX] = 8500;

    SectorCoord sector{10, 20};
    tree.BuildWithHeights(sector, heights, 1.0f, 0.0f);
    CHECK(tree.IsBuilt());

    WorldPosition origin = TerrainMetrics::SectorToWorld(sector);

    std::unordered_set<uint32_t> foundPatchIds;
    size_t leafCount = 0;
    const LinearQuadNode* targetLeaf = nullptr;

    // Przeszukaj wszystkie 85 wezlow drzewa
    for (size_t i = 0; i < LinearSectorQuadtree::TotalNodes; ++i) {
        const auto& node = tree.GetNode(i);
        if (node.isLeaf) {
            leafCount++;
            foundPatchIds.insert(node.patchId);

            // Sprawdz czy patchId miesci sie w przedziale 0..63
            CHECK(node.patchId < 64);

            // Oblicz z patchId oczekiwane gx, gy
            uint32_t expectedGx = node.patchId % TerrainMetrics::PatchesPerSide;
            uint32_t expectedGy = node.patchId / TerrainMetrics::PatchesPerSide;

            // Sprawdz zgodnosc AABB X/Y liscia z row-major patchId
            float expectedMinX = origin.x + static_cast<float>(expectedGx * TerrainMetrics::PatchSize);
            float expectedMinY = origin.y + static_cast<float>(expectedGy * TerrainMetrics::PatchSize);
            float expectedMaxX = expectedMinX + TerrainMetrics::PatchSize;
            float expectedMaxY = expectedMinY + TerrainMetrics::PatchSize;

            CHECK(std::abs(node.minX - expectedMinX) < 0.01f);
            CHECK(std::abs(node.minY - expectedMinY) < 0.01f);
            CHECK(std::abs(node.maxX - expectedMaxX) < 0.01f);
            CHECK(std::abs(node.maxY - expectedMaxY) < 0.01f);

            if (node.patchId == expectedPatchId) {
                targetLeaf = &node;
            } else {
                // Inne liscie nie moga zawierac wstawionego szczytu 8500
                CHECK(node.maxZ < 1000.0f);
            }
        } else {
            // Wezel wewnetrzny musi miec poprawne dziecko childBaseIndex
            CHECK(node.childBaseIndex > 0);
            CHECK(node.childBaseIndex + 3 < LinearSectorQuadtree::TotalNodes);

            // AABB rodzica musi dokladnie obejmowac AABB swoich 4 dzieci
            for (int c = 0; c < 4; ++c) {
                const auto& child = tree.GetNode(node.childBaseIndex + c);
                CHECK(node.minX <= child.minX);
                CHECK(node.minY <= child.minY);
                CHECK(node.minZ <= child.minZ);
                CHECK(node.maxX >= child.maxX);
                CHECK(node.maxY >= child.maxY);
                CHECK(node.maxZ >= child.maxZ);
            }
        }
    }

    // Dokladnie 64 unikalne liscie od 0 do 63
    CHECK(leafCount == 64);
    CHECK(foundPatchIds.size() == 64);

    // Sprawdz lisc z wstawionym szczytem
    REQUIRE(targetLeaf != nullptr);
    CHECK(targetLeaf->patchId == expectedPatchId);
    CHECK(targetLeaf->maxZ >= 8500.0f);

    // Korzen (wezel 0) rowniez musi propagowac ten szczyt
    const auto& root = tree.GetNode(0);
    CHECK(root.maxZ >= 8500.0f);
    CHECK(root.minZ <= 200.0f);
}

TEST_CASE("ChunkStreamingCoordinator & Facade Eviction - Memory Leak Prevention") {
    TerrainSubsystemFacade facade;

    std::vector<uint16_t> heights(131 * 131, 300);
    facade.LoadSectorData(SectorCoord{0, 0}, heights, {}, 1.0f, 0.0f);

    float frustumAll[6][4] = {
        {  1.0f,  0.0f,  0.0f, 100000.0f },
        { -1.0f,  0.0f,  0.0f, 100000.0f },
        {  0.0f,  1.0f,  0.0f, 100000.0f },
        {  0.0f, -1.0f,  0.0f, 100000.0f },
        {  0.0f,  0.0f,  1.0f, 100000.0f },
        {  0.0f,  0.0f, -1.0f, 100000.0f }
    };

    std::array<uint32_t, 64> visiblePatches{};
    size_t countBefore = facade.CullSectorPatches(SectorCoord{0, 0}, frustumAll, visiblePatches.data());
    CHECK(countBefore == 64);

    // Gracz przemieszcza sie bardzo daleko (300 000 jednostek)
    facade.Update(300000.0f, 300000.0f);

    // Sektor [0, 0] powinien zostac bezpiecznie wyewiktowany
    CHECK(!facade.GetStreamingCoordinator().IsChunkLoaded(SectorCoord{0, 0}));

    // Drzewo czworkowe sektora [0, 0] musialo zostac wyczyszczone z m_sectorTrees
    // W zwiazku z tym culling sektora [0, 0] musi zwrocic dokladnie 0 (brak zombi-drzew w pamieci)
    size_t countAfter = facade.CullSectorPatches(SectorCoord{0, 0}, frustumAll, visiblePatches.data());
    CHECK(countAfter == 0);
}

TEST_CASE("Deadlock-Free Event Dispatch - Reentrant Query on Eviction") {
    TerrainHeightStorage storage;
    ChunkStreamingCoordinator coordinator(&storage);

    // Zarejestruj sektor [5, 5]
    coordinator.RequestChunk(SectorCoord{5, 5});
    coordinator.MarkChunkActive(SectorCoord{5, 5});
    CHECK(coordinator.IsChunkLoaded(SectorCoord{5, 5}));

    // Zasubskrybuj zdarzenie ChunkEvictedEvent
    // Odbiorca w trakcie obslugi zdarzenia odpytuje koordynatora o stan
    bool handlerCalled = false;
    auto subId = UserInterface::Core::EventBus::GetInstance().Subscribe<ChunkEvictedEvent>(
        [&](const ChunkEvictedEvent& e) {
            handlerCalled = true;
            // Jesli koordynator trzymalby mutex podczas publikacji, nastepna linia spowoduje natychmiastowy deadlock!
            auto state = coordinator.GetChunkState(e.coord);
            CHECK(state == ChunkState::Unloaded);
            CHECK(!coordinator.IsChunkLoaded(e.coord));
        }
    );

    // Wymus ewikcje oddalonych kafelkow
    coordinator.EvictDistantChunks(0.0f, 0.0f, 100.0f);

    CHECK(handlerCalled == true);
    CHECK(!coordinator.IsChunkLoaded(SectorCoord{5, 5}));
}

TEST_CASE("TerrainHeightStorage Optimizations - WaterSize & Tan^2 WalkableSlope") {
    TerrainHeightStorage storage;

    ChunkHeightData data;
    data.heights.assign(131 * 131, 0);
    // Zbocze w komorce (0, 0): podniesienie o 200 jednostek na odcinku 200 jednostek -> nachylenie 45 stopni
    // (xslope = 1.0f)
    data.heights[1 * 131 + 1] = 0;
    data.heights[1 * 131 + 2] = 200; // dx = 200 -> slope = 1.0f
    data.heights[2 * 131 + 1] = 0;
    data.heights[2 * 131 + 2] = 200;

    // Bufor wody 128x128
    data.waterHeights.assign(128 * 128, 77);
    data.waterSize = 128; // Zapamietany wymiar siatki wody

    storage.InsertChunk(SectorCoord{0, 0}, std::move(data));

    SUBCASE("GetWaterHeight without std::sqrt") {
        float waterH = storage.SampleWaterHeight(100.0f, 100.0f);
        CHECK(waterH == 77.0f);
    }

    SUBCASE("IsWalkableSlope Tan^2 Precision") {
        // Nachylenie 45 stopni:
        // Dla maxAngle = 44.0f -> nie do przejscia (false)
        CHECK(storage.IsWalkableSlope(100.0f, 50.0f, 44.0f) == false);

        // Dla maxAngle = 45.1f -> do przejscia (true)
        CHECK(storage.IsWalkableSlope(100.0f, 50.0f, 45.1f) == true);

        // Dla 90 stopni -> zawsze true
        CHECK(storage.IsWalkableSlope(100.0f, 50.0f, 90.0f) == true);

        // Dla 0 stopni -> false (nachylenie wynosi 45 stopni)
        CHECK(storage.IsWalkableSlope(100.0f, 50.0f, 0.0f) == false);
    }
}

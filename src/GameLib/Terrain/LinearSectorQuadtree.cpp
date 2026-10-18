#include "../StdAfx.h"
#include "LinearSectorQuadtree.h"
#include "TerrainEvents.h"
#include "../../EterBase/LogModern.h"
#include <algorithm>
#include <cmath>

namespace GameLib::Terrain {

    std::unique_ptr<ITerrainQuadtreeCuller> CreateTerrainQuadtreeCuller() {
        return std::make_unique<LinearSectorQuadtree>();
    }

    void LinearSectorQuadtree::Clear() {
        m_nodes.fill(LinearQuadNode{});
        m_isBuilt = false;
        m_sectorCoord = SectorCoord{};
        m_origin = WorldPosition{};
    }

    void LinearSectorQuadtree::BuildQuadtree(int32_t sectorX, int32_t sectorY) {
        BuildWithHeights(SectorCoord{sectorX, sectorY}, {}, 1.0f, 0.0f);
    }

    void LinearSectorQuadtree::BuildWithHeights(
        SectorCoord coord, 
        std::span<const uint16_t> heights, 
        float heightScale, 
        float baseHeight) noexcept 
    {
        Clear();
        m_sectorCoord = coord;
        m_origin = TerrainMetrics::SectorToWorld(coord);

        uint32_t nextNodeIndex = 1;
        BuildNode(0, 0, 0, 0, nextNodeIndex, heights, heightScale, baseHeight);

        m_isBuilt = true;
        EterBase::ModernLogger::Info("LinearSectorQuadtree built for sector [{}, {}] ({} nodes, {} leaves).", 
            coord.x, coord.y, TotalNodes, TotalLeaves);

        QuadtreeBuiltEvent event{coord.x, coord.y, TotalNodes};
        UserInterface::Core::EventBus::GetInstance().Publish(event);
    }

    void LinearSectorQuadtree::BuildNode(
        uint32_t nodeIndex, 
        uint32_t gx, 
        uint32_t gy, 
        uint32_t depth, 
        uint32_t& nextFreeIndex, 
        std::span<const uint16_t> heights, 
        float heightScale, 
        float baseHeight) noexcept 
    {
        auto& node = m_nodes[nodeIndex];
        node.lodLevel = static_cast<uint8_t>(depth);

        // Rozmiar wezla w patchach (8 na poziomie 0, 4 na 1, 2 na 2, 1 na 3)
        uint32_t span = TerrainMetrics::PatchesPerSide >> depth;
        node.minX = m_origin.x + static_cast<float>(gx * TerrainMetrics::PatchSize);
        node.minY = m_origin.y + static_cast<float>(gy * TerrainMetrics::PatchSize);
        node.maxX = node.minX + static_cast<float>(span * TerrainMetrics::PatchSize);
        node.maxY = node.minY + static_cast<float>(span * TerrainMetrics::PatchSize);

        if (depth == Depth) {
            node.isLeaf = 1;
            node.childBaseIndex = 0;
            node.patchId = gy * TerrainMetrics::PatchesPerSide + gx; // Czysty row-major (0..63)

            bool hasHeights = !heights.empty() && 
                              (heights.size() >= (TerrainMetrics::HeightmapSize * TerrainMetrics::HeightmapSize));
            if (hasHeights) {
                CalculateLeafHeights(node, gx, gy, heights, heightScale, baseHeight);
            } else {
                node.minZ = -2000.0f;
                node.maxZ = 5000.0f;
            }
            return;
        }

        node.isLeaf = 0;
        node.patchId = 0;
        node.childBaseIndex = static_cast<uint16_t>(nextFreeIndex);
        uint16_t base = node.childBaseIndex;
        nextFreeIndex += 4;

        uint32_t halfSpan = span >> 1;
        BuildNode(base + 0, gx,            gy,            depth + 1, nextFreeIndex, heights, heightScale, baseHeight);
        BuildNode(base + 1, gx + halfSpan, gy,            depth + 1, nextFreeIndex, heights, heightScale, baseHeight);
        BuildNode(base + 2, gx,            gy + halfSpan, depth + 1, nextFreeIndex, heights, heightScale, baseHeight);
        BuildNode(base + 3, gx + halfSpan, gy + halfSpan, depth + 1, nextFreeIndex, heights, heightScale, baseHeight);

        // Bezposrednia agregacja Z z 4 zsynchronizowanych dzieci
        float minZ = m_nodes[base].minZ;
        float maxZ = m_nodes[base].maxZ;
        for (int c = 1; c < 4; ++c) {
            minZ = std::min(minZ, m_nodes[base + c].minZ);
            maxZ = std::max(maxZ, m_nodes[base + c].maxZ);
        }
        node.minZ = minZ;
        node.maxZ = maxZ;
    }

    void LinearSectorQuadtree::CalculateLeafHeights(
        LinearQuadNode& node, 
        uint32_t patchX, 
        uint32_t patchY, 
        std::span<const uint16_t> heights, 
        float heightScale, 
        float baseHeight) noexcept 
    {
        // Kazdy patch w Metin2 ma 16x16 komorek, czyli (16+1)x(16+1) wierzcholkow
        uint32_t startCellX = patchX * TerrainMetrics::CellsPerPatch;
        uint32_t startCellY = patchY * TerrainMetrics::CellsPerPatch;
        uint32_t endCellX = startCellX + TerrainMetrics::CellsPerPatch;
        uint32_t endCellY = startCellY + TerrainMetrics::CellsPerPatch;

        float minZ = 1e9f;
        float maxZ = -1e9f;

        for (uint32_t cy = startCellY; cy <= endCellY; ++cy) {
            for (uint32_t cx = startCellX; cx <= endCellX; ++cx) {
                // Uwzglednienie marginesu +1 w formacie Metin2
                uint32_t rawX = cx + TerrainMetrics::RawBorderOffset;
                uint32_t rawY = cy + TerrainMetrics::RawBorderOffset;
                if (rawX < TerrainMetrics::HeightmapSize && rawY < TerrainMetrics::HeightmapSize) {
                    uint32_t idx = rawY * TerrainMetrics::HeightmapSize + rawX;
                    float h = baseHeight + (static_cast<float>(heights[idx]) * heightScale);
                    minZ = std::min(minZ, h);
                    maxZ = std::max(maxZ, h);
                }
            }
        }

        if (minZ <= maxZ) {
            node.minZ = minZ - 100.0f; // Margines bezpieczenstwa dla obiektow i trawy
            node.maxZ = maxZ + 200.0f;
        }
    }

    size_t LinearSectorQuadtree::CullTerrainPatches(
        const float viewFrustum[6][4], 
        uint32_t* outVisiblePatchIds) 
    {
        if (!m_isBuilt || !outVisiblePatchIds) return 0;

        // Plaski stos o stalym rozmiarze do eliminacji alokacji i rekurencji
        std::array<uint16_t, 64> nodeStack;
        int32_t stackTop = 0;
        nodeStack[0] = 0; // Rozpoczecie od korzenia

        size_t count = 0;

        while (stackTop >= 0) {
            uint16_t currentIdx = nodeStack[stackTop--];
            const auto& node = m_nodes[currentIdx];

            // Test AABB wzgledem 6 plaszczyzn Frustum (P-vertex test)
            bool outside = false;
            for (int i = 0; i < 6; ++i) {
                float px = (viewFrustum[i][0] > 0.0f) ? node.maxX : node.minX;
                float py = (viewFrustum[i][1] > 0.0f) ? node.maxY : node.minY;
                float pz = (viewFrustum[i][2] > 0.0f) ? node.maxZ : node.minZ;

                if ((viewFrustum[i][0] * px + viewFrustum[i][1] * py + viewFrustum[i][2] * pz + viewFrustum[i][3]) < 0.0f) {
                    outside = true;
                    break;
                }
            }

            if (outside) continue;

            if (node.isLeaf) {
                if (count < TotalLeaves) {
                    outVisiblePatchIds[count++] = node.patchId;
                }
            } else {
                // Dodanie 4 dzieci na stos
                for (int c = 0; c < 4; ++c) {
                    if (stackTop + 1 < static_cast<int32_t>(nodeStack.size())) {
                        nodeStack[++stackTop] = node.childBaseIndex + c;
                    }
                }
            }
        }

        TerrainCullCompletedEvent event{count};
        UserInterface::Core::EventBus::GetInstance().Publish(event);

        return count;
    }

    uint8_t LinearSectorQuadtree::SelectLOD(float distance) const {
        if (distance < 6400.0f) {
            return 0; // Najwyzszy detal
        }
        if (distance < 16000.0f) {
            return 1; // Sredni detal
        }
        return 2;     // Najnizszy detal
    }

} // namespace GameLib::Terrain

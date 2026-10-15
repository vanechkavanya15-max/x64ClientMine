#include "TerrainFrustumCuller.h"

namespace Client::Terrain {

std::expected<void, CullerError> TerrainFrustumCuller::SetFrustumPlanes(std::span<const FrustumPlane> planes) noexcept {
    if (planes.size() != 6) {
        return std::unexpected(CullerError::InvalidPlanesCount);
    }
    for (size_t i = 0; i < 6; ++i) {
        m_planes[i] = planes[i];
    }
    m_planesSet = true;
    return {};
}

std::expected<bool, CullerError> TerrainFrustumCuller::IsBoxVisible(const TerrainAABB& box) const noexcept {
    if (!m_planesSet) {
        return std::unexpected(CullerError::PlanesNotSet);
    }
    
    for (int i = 0; i < 6; ++i) {
        float px = (m_planes[i].a > 0.0f) ? box.maxX : box.minX;
        float py = (m_planes[i].b > 0.0f) ? box.maxY : box.minY;
        float pz = (m_planes[i].c > 0.0f) ? box.maxZ : box.minZ;
        
        float dotProduct = (m_planes[i].a * px) + (m_planes[i].b * py) + (m_planes[i].c * pz);
        
        if (dotProduct < -m_planes[i].d) {
            return false;
        }
    }
    return true;
}

std::expected<void, CullerError> TerrainFrustumCuller::CullQuadTree(const TerrainAABB& root, std::vector<uint32_t>& visibleNodeIds) const {
    if (!m_planesSet) {
        return std::unexpected(CullerError::PlanesNotSet);
    }

    struct StackNode {
        TerrainAABB bounds;
        int depth;
        uint32_t patchIdStart;
    };
    
    std::array<StackNode, 64> stack;
    int top = 0;
    
    stack[top++] = {root, 4, 0};
    
    while (top > 0) {
        StackNode node = stack[--top];
        
        auto isVisibleOpt = IsBoxVisible(node.bounds);
        if (!isVisibleOpt.value()) {
            continue;
        }
        
        if (node.depth == 0) {
            visibleNodeIds.push_back(node.patchIdStart);
            continue;
        }
        
        float midX = (node.bounds.minX + node.bounds.maxX) * 0.5f;
        float midY = (node.bounds.minY + node.bounds.maxY) * 0.5f;
        
        uint32_t patchesPerChild = 1 << ((node.depth - 1) * 2);
        
        // Push Child 3 (q4)
        stack[top++] = {
            {midX, midY, node.bounds.minZ, node.bounds.maxX, node.bounds.maxY, node.bounds.maxZ}, 
            node.depth - 1, 
            node.patchIdStart + patchesPerChild * 3
        };
                        
        // Push Child 2 (q3)
        stack[top++] = {
            {node.bounds.minX, midY, node.bounds.minZ, midX, node.bounds.maxY, node.bounds.maxZ}, 
            node.depth - 1, 
            node.patchIdStart + patchesPerChild * 2
        };
                        
        // Push Child 1 (q2)
        stack[top++] = {
            {midX, node.bounds.minY, node.bounds.minZ, node.bounds.maxX, midY, node.bounds.maxZ}, 
            node.depth - 1, 
            node.patchIdStart + patchesPerChild * 1
        };
                        
        // Push Child 0 (q1)
        stack[top++] = {
            {node.bounds.minX, node.bounds.minY, node.bounds.minZ, midX, midY, node.bounds.maxZ}, 
            node.depth - 1, 
            node.patchIdStart
        };
    }
    
    return {};
}

} // namespace Client::Terrain

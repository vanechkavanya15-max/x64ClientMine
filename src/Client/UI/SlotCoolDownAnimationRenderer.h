#pragma once

#include "UIRenderBatcher.h"
#include <vector>
#include <expected>

namespace Client::UI {

class SlotCoolDownAnimationRenderer {
public:
    SlotCoolDownAnimationRenderer() = delete; // Utility class

    /**
     * Generates a series of UIVertex that form a radial wipe over a rectangular area.
     * The generated vertices are configured as a PrimitiveType::TriangleList.
     * 
     * @param x The top-left X coordinate.
     * @param y The top-left Y coordinate.
     * @param w The width of the rectangle.
     * @param h The height of the rectangle.
     * @param progress A value between 0.0f and 1.0f. 
     *                 0.0f means fully covered (all triangles generated).
     *                 1.0f means completely uncovered (0 triangles generated).
     *                 0.5f means half covered.
     * @param color The color of the generated vertices.
     * @return A vector of vertices forming a standard Triangle List.
     */
    static std::expected<std::vector<UIVertex>, UIBatchError> GenerateRadialWipe(
        float x, float y, float w, float h, float progress, Color color);
};

} // namespace Client::UI

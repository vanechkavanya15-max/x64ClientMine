#include "Client/Terrain/TerrainHeightSampler.h"
#include <algorithm>
#include <cmath>

namespace Client::Terrain {

void TerrainHeightSampler::SetHeightData(std::span<const float> grid, uint32_t width, uint32_t height, float scale) {
    gridWidth_ = width;
    gridHeight_ = height;
    gridScale_ = scale > 0.0f ? scale : 1.0f; // Zabezpieczenie przed dzieleniem przez zero

    heightGrid_.assign(grid.begin(), grid.end());
}

float TerrainHeightSampler::GetClampedHeight(int32_t gridX, int32_t gridY) const {
    if (heightGrid_.empty() || gridWidth_ == 0 || gridHeight_ == 0) {
        return 0.0f;
    }

    // Ograniczenie wspolrzednych do granic siatki
    int32_t clampedX = std::clamp(gridX, 0, static_cast<int32_t>(gridWidth_) - 1);
    int32_t clampedY = std::clamp(gridY, 0, static_cast<int32_t>(gridHeight_) - 1);

    size_t index = static_cast<size_t>(clampedY) * gridWidth_ + clampedX;
    if (index < heightGrid_.size()) {
        return heightGrid_[index];
    }
    
    return 0.0f;
}

float TerrainHeightSampler::GetHeight(float x, float y) const {
    if (heightGrid_.empty() || gridWidth_ == 0 || gridHeight_ == 0) {
        return 0.0f;
    }

    // Normalizacja do wspolrzednych siatki
    float gridX = x / gridScale_;
    float gridY = y / gridScale_;

    // Zabezpieczenie na wypadek NaN
    if (std::isnan(gridX) || std::isnan(gridY)) {
        return 0.0f;
    }

    // Wyznaczenie komorki siatki
    int32_t ix = static_cast<int32_t>(std::floor(gridX));
    int32_t iy = static_cast<int32_t>(std::floor(gridY));

    // Wagi interpolacji
    float u = gridX - static_cast<float>(ix);
    float v = gridY - static_cast<float>(iy);

    // Wysokosci 4 rogow komorki
    float h00 = GetClampedHeight(ix, iy);
    float h10 = GetClampedHeight(ix + 1, iy);
    float h01 = GetClampedHeight(ix, iy + 1);
    float h11 = GetClampedHeight(ix + 1, iy + 1);

    // Interpolacja dwuliniowa
    float h0 = h00 * (1.0f - u) + h10 * u;
    float h1 = h01 * (1.0f - u) + h11 * u;
    
    return h0 * (1.0f - v) + h1 * v;
}

Vector3 TerrainHeightSampler::GetNormal(float x, float y) const {
    // Normalizacja do wspolrzednych siatki
    float gridX = x / gridScale_;
    float gridY = y / gridScale_;

    if (std::isnan(gridX) || std::isnan(gridY) || gridScale_ <= 0.0f) {
        return Vector3{0.0f, 0.0f, 1.0f};
    }

    int32_t ix = static_cast<int32_t>(std::floor(gridX));
    int32_t iy = static_cast<int32_t>(std::floor(gridY));

    // Obliczenie roznic centralnych dla wektora normalnego
    // Zakladamy siatke, gdzie wysokosc podawana jest na wspolrzednej z
    // H(x, y) = GetClampedHeight
    // dH/dx
    float hL = GetClampedHeight(ix - 1, iy);
    float hR = GetClampedHeight(ix + 1, iy);
    
    // dH/dy
    float hU = GetClampedHeight(ix, iy - 1);
    float hD = GetClampedHeight(ix, iy + 1);

    // Aby wektory kierunkowe uwzglednialy skale siatki:
    // wektor dx = (2 * gridScale_, 0, hR - hL)
    // wektor dy = (0, 2 * gridScale_, hD - hU)
    // cross(dx, dy)
    // nx = -(hR - hL) * 2 * gridScale_
    // ny = -(hD - hU) * 2 * gridScale_
    // nz = 4 * gridScale_^2

    float dx = (hR - hL) / (2.0f * gridScale_);
    float dy = (hD - hU) / (2.0f * gridScale_);

    Vector3 normal;
    normal.x = -dx;
    normal.y = -dy;
    normal.z = 1.0f; // zakladamy z skierowane w gore

    // Normalizacja
    float length = std::sqrt(normal.x * normal.x + normal.y * normal.y + normal.z * normal.z);
    if (length > 0.0f) {
        normal.x /= length;
        normal.y /= length;
        normal.z /= length;
    } else {
        normal = Vector3{0.0f, 0.0f, 1.0f};
    }

    return normal;
}

} // namespace Client::Terrain

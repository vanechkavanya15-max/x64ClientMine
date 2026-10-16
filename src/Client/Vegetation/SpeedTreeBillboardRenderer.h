#pragma once

#include <vector>
#include <expected>
#include <cstdint>
#include <cmath>
#include <array>
#include "Client/Core/StrongTypes.h"

namespace Client::Vegetation {

struct Vector3 {
    float x = 0.0f;
    float y = 0.0f;
    float z = 0.0f;

    constexpr Vector3() noexcept = default;
    constexpr Vector3(float _x, float _y, float _z) noexcept : x(_x), y(_y), z(_z) {}

    constexpr Vector3 operator+(const Vector3& other) const noexcept { return {x + other.x, y + other.y, z + other.z}; }
    constexpr Vector3 operator-(const Vector3& other) const noexcept { return {x - other.x, y - other.y, z - other.z}; }
    constexpr Vector3 operator*(float scalar) const noexcept { return {x * scalar, y * scalar, z * scalar}; }

    constexpr Vector3 Cross(const Vector3& other) const noexcept {
        return {
            y * other.z - z * other.y,
            z * other.x - x * other.z,
            x * other.y - y * other.x
        };
    }

    constexpr float Dot(const Vector3& other) const noexcept {
        return x * other.x + y * other.y + z * other.z;
    }

    inline float Length() const noexcept {
        return std::sqrt(x * x + y * y + z * z);
    }

    inline Vector3 Normalize() const noexcept {
        float len = Length();
        if (len > 0.000001f) {
            return {x / len, y / len, z / len};
        }
        return {0.0f, 0.0f, 0.0f};
    }
    
    inline bool operator==(const Vector3& other) const noexcept {
        return std::abs(x - other.x) < 0.0001f && 
               std::abs(y - other.y) < 0.0001f && 
               std::abs(z - other.z) < 0.0001f;
    }
};

struct TexCoord {
    float u = 0.0f;
    float v = 0.0f;
    
    inline bool operator==(const TexCoord& other) const noexcept {
        return std::abs(u - other.u) < 0.0001f && std::abs(v - other.v) < 0.0001f;
    }
};

struct BillboardVertex {
    Vector3 position;
    TexCoord texCoord;
    
    inline bool operator==(const BillboardVertex& other) const noexcept {
        return position == other.position && texCoord == other.texCoord;
    }
};

struct TextureIdTag {};
using TextureId = Client::Core::StrongType<TextureIdTag, uint32_t, 0>;

struct BillboardQuad {
    std::array<BillboardVertex, 4> vertices;
    TextureId textureId;
};

struct BillboardInstance {
    Vector3 position;
    float width;
    float height;
    TextureId textureId;

    bool operator<(const BillboardInstance& other) const {
        return textureId < other.textureId;
    }
};

enum class RenderError {
    InvalidCameraVector,
    GenerationFailed
};

using RenderResult = std::expected<std::vector<BillboardQuad>, RenderError>;

class SpeedTreeBillboardRenderer {
public:
    SpeedTreeBillboardRenderer() = default;
    ~SpeedTreeBillboardRenderer() = default;

    SpeedTreeBillboardRenderer(const SpeedTreeBillboardRenderer&) = delete;
    SpeedTreeBillboardRenderer& operator=(const SpeedTreeBillboardRenderer&) = delete;
    
    SpeedTreeBillboardRenderer(SpeedTreeBillboardRenderer&&) noexcept = default;
    SpeedTreeBillboardRenderer& operator=(SpeedTreeBillboardRenderer&&) noexcept = default;

    void AddBillboard(const BillboardInstance& instance);
    void Clear() noexcept;

    [[nodiscard]] RenderResult GenerateQuads(const Vector3& cameraDir, const Vector3& worldUp) const;

private:
    std::vector<BillboardInstance> m_billboards;
};

} // namespace Client::Vegetation

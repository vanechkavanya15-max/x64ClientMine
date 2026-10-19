#pragma once

#include <cstddef>

namespace Client::Math {

    struct alignas(8) Vec2 {
        float x, y;
    };

    struct Vec3 {
        float x, y, z;
    };

    struct alignas(16) Vec4 {
        float x, y, z, w;
    };

    struct alignas(16) Quat {
        float x, y, z, w;
    };

    struct alignas(16) Mat4 {
        float m[16]; // 4x4 matrix
    };

} // namespace Client::Math

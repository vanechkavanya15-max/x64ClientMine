#pragma once

/**
 * @file PODMath.h
 * @brief Kanoniczny naglowek struktur matematycznych POD (Plain Old Data)
 *        w pelni zgodnych z C-ABI #[repr(C)] biblioteki glam (Rust) oraz potokiem WGPU.
 * 
 * Standard: C++20 / C++23.
 * Zero zaleznosci od d3dx9.h, Direct3D czy Windows SDK.
 */

#include <cmath>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <algorithm>

#if defined(_M_IX86) || defined(_M_X64) || defined(__i386__) || defined(__x86_64__)
    #include <immintrin.h>
    #define PODMATH_HAS_SIMD 1
#else
    #define PODMATH_HAS_SIMD 0
#endif

namespace MathPOD
{
    // =========================================================================
    // CleanVec2 (8 bajtow, alignof 4, odpowiednik glam::Vec2)
    // =========================================================================
    struct CleanVec2
    {
        float x;
        float y;

        constexpr CleanVec2() noexcept : x(0.0f), y(0.0f) {}
        constexpr CleanVec2(float inX, float inY) noexcept : x(inX), y(inY) {}

        [[nodiscard]] constexpr float Dot(const CleanVec2& rhs) const noexcept
        {
            return (x * rhs.x) + (y * rhs.y);
        }

        [[nodiscard]] constexpr float LengthSq() const noexcept
        {
            return Dot(*this);
        }

        [[nodiscard]] inline float Length() const noexcept
        {
            return std::sqrt(LengthSq());
        }

        [[nodiscard]] inline CleanVec2 Normalize() const noexcept
        {
            const float len = Length();
            if (len > 1e-6f)
            {
                const float inv = 1.0f / len;
                return CleanVec2(x * inv, y * inv);
            }
            return CleanVec2(0.0f, 0.0f);
        }

        constexpr CleanVec2 operator+(const CleanVec2& rhs) const noexcept { return CleanVec2(x + rhs.x, y + rhs.y); }
        constexpr CleanVec2 operator-(const CleanVec2& rhs) const noexcept { return CleanVec2(x - rhs.x, y - rhs.y); }
        constexpr CleanVec2 operator*(float scalar) const noexcept { return CleanVec2(x * scalar, y * scalar); }
        constexpr CleanVec2 operator/(float scalar) const noexcept { return CleanVec2(x / scalar, y / scalar); }
        constexpr bool operator==(const CleanVec2& rhs) const noexcept { return (x == rhs.x) && (y == rhs.y); }
        constexpr bool operator!=(const CleanVec2& rhs) const noexcept { return !(*this == rhs); }
    };

    // =========================================================================
    // CleanVec3 (12 bajtow, alignof 4, odpowiednik glam::Vec3)
    // =========================================================================
    struct CleanVec3
    {
        float x;
        float y;
        float z;

        constexpr CleanVec3() noexcept : x(0.0f), y(0.0f), z(0.0f) {}
        constexpr CleanVec3(float inX, float inY, float inZ) noexcept : x(inX), y(inY), z(inZ) {}

        [[nodiscard]] constexpr float Dot(const CleanVec3& rhs) const noexcept
        {
            return (x * rhs.x) + (y * rhs.y) + (z * rhs.z);
        }

        [[nodiscard]] constexpr CleanVec3 Cross(const CleanVec3& rhs) const noexcept
        {
            return CleanVec3(
                (y * rhs.z) - (z * rhs.y),
                (z * rhs.x) - (x * rhs.z),
                (x * rhs.y) - (y * rhs.x)
            );
        }

        [[nodiscard]] constexpr float LengthSq() const noexcept
        {
            return Dot(*this);
        }

        [[nodiscard]] inline float Length() const noexcept
        {
            return std::sqrt(LengthSq());
        }

        [[nodiscard]] inline CleanVec3 Normalize() const noexcept
        {
            const float len = Length();
            if (len > 1e-6f)
            {
                const float inv = 1.0f / len;
                return CleanVec3(x * inv, y * inv, z * inv);
            }
            return CleanVec3(0.0f, 0.0f, 0.0f);
        }

        constexpr CleanVec3 operator+(const CleanVec3& rhs) const noexcept { return CleanVec3(x + rhs.x, y + rhs.y, z + rhs.z); }
        constexpr CleanVec3 operator-(const CleanVec3& rhs) const noexcept { return CleanVec3(x - rhs.x, y - rhs.y, z - rhs.z); }
        constexpr CleanVec3 operator*(float scalar) const noexcept { return CleanVec3(x * scalar, y * scalar, z * scalar); }
        constexpr CleanVec3 operator/(float scalar) const noexcept { return CleanVec3(x / scalar, y / scalar, z / scalar); }
        constexpr bool operator==(const CleanVec3& rhs) const noexcept { return (x == rhs.x) && (y == rhs.y) && (z == rhs.z); }
        constexpr bool operator!=(const CleanVec3& rhs) const noexcept { return !(*this == rhs); }
    };

    // =========================================================================
    // CleanVec4 (16 bajtow, alignof 16, odpowiednik glam::Vec4)
    // =========================================================================
    alignas(16) struct CleanVec4
    {
        float x;
        float y;
        float z;
        float w;

        constexpr CleanVec4() noexcept : x(0.0f), y(0.0f), z(0.0f), w(0.0f) {}
        constexpr CleanVec4(float inX, float inY, float inZ, float inW) noexcept : x(inX), y(inY), z(inZ), w(inW) {}
        constexpr CleanVec4(const CleanVec3& v, float inW) noexcept : x(v.x), y(v.y), z(v.z), w(inW) {}

        [[nodiscard]] inline float Dot(const CleanVec4& rhs) const noexcept
        {
#if PODMATH_HAS_SIMD
            __m128 a = _mm_load_ps(&x);
            __m128 b = _mm_load_ps(&rhs.x);
            __m128 mul = _mm_mul_ps(a, b);
            __m128 hsum = _mm_hadd_ps(mul, mul);
            hsum = _mm_hadd_ps(hsum, hsum);
            return _mm_cvtss_f32(hsum);
#else
            return (x * rhs.x) + (y * rhs.y) + (z * rhs.z) + (w * rhs.w);
#endif
        }

        [[nodiscard]] constexpr float LengthSq() const noexcept
        {
            return (x * x) + (y * y) + (z * z) + (w * w);
        }

        [[nodiscard]] inline float Length() const noexcept
        {
            return std::sqrt(LengthSq());
        }

        [[nodiscard]] inline CleanVec4 Normalize() const noexcept
        {
            const float len = Length();
            if (len > 1e-6f)
            {
                const float inv = 1.0f / len;
                return CleanVec4(x * inv, y * inv, z * inv, w * inv);
            }
            return CleanVec4(0.0f, 0.0f, 0.0f, 0.0f);
        }

        constexpr CleanVec4 operator+(const CleanVec4& rhs) const noexcept { return CleanVec4(x + rhs.x, y + rhs.y, z + rhs.z, w + rhs.w); }
        constexpr CleanVec4 operator-(const CleanVec4& rhs) const noexcept { return CleanVec4(x - rhs.x, y - rhs.y, z - rhs.z, w - rhs.w); }
        constexpr CleanVec4 operator*(float scalar) const noexcept { return CleanVec4(x * scalar, y * scalar, z * scalar, w * scalar); }
        constexpr CleanVec4 operator/(float scalar) const noexcept { return CleanVec4(x / scalar, y / scalar, z / scalar, w / scalar); }
        constexpr bool operator==(const CleanVec4& rhs) const noexcept { return (x == rhs.x) && (y == rhs.y) && (z == rhs.z) && (w == rhs.w); }
        constexpr bool operator!=(const CleanVec4& rhs) const noexcept { return !(*this == rhs); }
    };

    // =========================================================================
    // CleanQuat (16 bajtow, alignof 16, odpowiednik glam::Quat)
    // =========================================================================
    alignas(16) struct CleanQuat
    {
        float x;
        float y;
        float z;
        float w;

        constexpr CleanQuat() noexcept : x(0.0f), y(0.0f), z(0.0f), w(1.0f) {}
        constexpr CleanQuat(float inX, float inY, float inZ, float inW) noexcept : x(inX), y(inY), z(inZ), w(inW) {}

        [[nodiscard]] static constexpr CleanQuat Identity() noexcept
        {
            return CleanQuat(0.0f, 0.0f, 0.0f, 1.0f);
        }

        [[nodiscard]] constexpr float Dot(const CleanQuat& rhs) const noexcept
        {
            return (x * rhs.x) + (y * rhs.y) + (z * rhs.z) + (w * rhs.w);
        }

        [[nodiscard]] constexpr float LengthSq() const noexcept
        {
            return Dot(*this);
        }

        [[nodiscard]] inline float Length() const noexcept
        {
            return std::sqrt(LengthSq());
        }

        [[nodiscard]] inline CleanQuat Normalize() const noexcept
        {
            const float len = Length();
            if (len > 1e-6f)
            {
                const float inv = 1.0f / len;
                return CleanQuat(x * inv, y * inv, z * inv, w * inv);
            }
            return Identity();
        }

        [[nodiscard]] constexpr CleanQuat Conjugate() const noexcept
        {
            return CleanQuat(-x, -y, -z, w);
        }

        // Mnozenie kwaternionow: this * rhs (odpowiada glam::Quat::mul)
        [[nodiscard]] constexpr CleanQuat Multiply(const CleanQuat& rhs) const noexcept
        {
            return CleanQuat(
                (w * rhs.x) + (x * rhs.w) + (y * rhs.z) - (z * rhs.y),
                (w * rhs.y) - (x * rhs.z) + (y * rhs.w) + (z * rhs.x),
                (w * rhs.z) + (x * rhs.y) - (y * rhs.x) + (z * rhs.w),
                (w * rhs.w) - (x * rhs.x) - (y * rhs.y) - (z * rhs.z)
            );
        }

        constexpr CleanQuat operator*(const CleanQuat& rhs) const noexcept { return Multiply(rhs); }
        constexpr bool operator==(const CleanQuat& rhs) const noexcept { return (x == rhs.x) && (y == rhs.y) && (z == rhs.z) && (w == rhs.w); }
        constexpr bool operator!=(const CleanQuat& rhs) const noexcept { return !(*this == rhs); }
    };

    // =========================================================================
    // CleanMat4 (64 bajty, alignof 16, Column-Major, odpowiednik glam::Mat4)
    // =========================================================================
    alignas(16) struct CleanMat4
    {
        CleanVec4 cols[4]; // cols[0] = x_axis, cols[1] = y_axis, cols[2] = z_axis, cols[3] = w_axis (translacja)

        constexpr CleanMat4() noexcept : cols{
            CleanVec4(1.0f, 0.0f, 0.0f, 0.0f),
            CleanVec4(0.0f, 1.0f, 0.0f, 0.0f),
            CleanVec4(0.0f, 0.0f, 1.0f, 0.0f),
            CleanVec4(0.0f, 0.0f, 0.0f, 1.0f)
        } {}

        constexpr CleanMat4(
            const CleanVec4& c0,
            const CleanVec4& c1,
            const CleanVec4& c2,
            const CleanVec4& c3
        ) noexcept : cols{ c0, c1, c2, c3 } {}

        [[nodiscard]] static constexpr CleanMat4 Identity() noexcept
        {
            return CleanMat4();
        }

        [[nodiscard]] static constexpr CleanMat4 Zero() noexcept
        {
            return CleanMat4(
                CleanVec4(0.0f, 0.0f, 0.0f, 0.0f),
                CleanVec4(0.0f, 0.0f, 0.0f, 0.0f),
                CleanVec4(0.0f, 0.0f, 0.0f, 0.0f),
                CleanVec4(0.0f, 0.0f, 0.0f, 0.0f)
            );
        }

        [[nodiscard]] static constexpr CleanMat4 Translation(float tx, float ty, float tz) noexcept
        {
            return CleanMat4(
                CleanVec4(1.0f, 0.0f, 0.0f, 0.0f),
                CleanVec4(0.0f, 1.0f, 0.0f, 0.0f),
                CleanVec4(0.0f, 0.0f, 1.0f, 0.0f),
                CleanVec4(tx, ty, tz, 1.0f)
            );
        }

        [[nodiscard]] static constexpr CleanMat4 Translation(const CleanVec3& pos) noexcept
        {
            return Translation(pos.x, pos.y, pos.z);
        }

        [[nodiscard]] static constexpr CleanMat4 Scaling(float sx, float sy, float sz) noexcept
        {
            return CleanMat4(
                CleanVec4(sx, 0.0f, 0.0f, 0.0f),
                CleanVec4(0.0f, sy, 0.0f, 0.0f),
                CleanVec4(0.0f, 0.0f, sz, 0.0f),
                CleanVec4(0.0f, 0.0f, 0.0f, 1.0f)
            );
        }

        [[nodiscard]] static inline CleanMat4 RotationZ(float radians) noexcept
        {
            const float s = std::sin(radians);
            const float c = std::cos(radians);
            return CleanMat4(
                CleanVec4(c, s, 0.0f, 0.0f),
                CleanVec4(-s, c, 0.0f, 0.0f),
                CleanVec4(0.0f, 0.0f, 1.0f, 0.0f),
                CleanVec4(0.0f, 0.0f, 0.0f, 1.0f)
            );
        }

        [[nodiscard]] static inline CleanMat4 FromQuaternion(const CleanQuat& q) noexcept
        {
            const float x2 = q.x + q.x;
            const float y2 = q.y + q.y;
            const float z2 = q.z + q.z;

            const float xx = q.x * x2;
            const float xy = q.x * y2;
            const float xz = q.x * z2;
            const float yy = q.y * y2;
            const float yz = q.y * z2;
            const float zz = q.z * z2;
            const float wx = q.w * x2;
            const float wy = q.w * y2;
            const float wz = q.w * z2;

            return CleanMat4(
                CleanVec4(1.0f - (yy + zz), xy + wz, xz - wy, 0.0f),
                CleanVec4(xy - wz, 1.0f - (xx + zz), yz + wx, 0.0f),
                CleanVec4(xz + wy, yz - wx, 1.0f - (xx + yy), 0.0f),
                CleanVec4(0.0f, 0.0f, 0.0f, 1.0f)
            );
        }

        // Mnozenie macierzy Column-Major: this * rhs (odpowiada glam::Mat4::mul)
        [[nodiscard]] inline CleanMat4 Multiply(const CleanMat4& rhs) const noexcept
        {
            CleanMat4 result;
            for (int col = 0; col < 4; ++col)
            {
                const CleanVec4& b = rhs.cols[col];
#if PODMATH_HAS_SIMD
                __m128 bx = _mm_set1_ps(b.x);
                __m128 by = _mm_set1_ps(b.y);
                __m128 bz = _mm_set1_ps(b.z);
                __m128 bw = _mm_set1_ps(b.w);

                __m128 c0 = _mm_load_ps(&cols[0].x);
                __m128 c1 = _mm_load_ps(&cols[1].x);
                __m128 c2 = _mm_load_ps(&cols[2].x);
                __m128 c3 = _mm_load_ps(&cols[3].x);

                __m128 r = _mm_add_ps(
                    _mm_add_ps(_mm_mul_ps(c0, bx), _mm_mul_ps(c1, by)),
                    _mm_add_ps(_mm_mul_ps(c2, bz), _mm_mul_ps(c3, bw))
                );
                _mm_store_ps(&result.cols[col].x, r);
#else
                result.cols[col].x = (cols[0].x * b.x) + (cols[1].x * b.y) + (cols[2].x * b.z) + (cols[3].x * b.w);
                result.cols[col].y = (cols[0].y * b.x) + (cols[1].y * b.y) + (cols[2].y * b.z) + (cols[3].y * b.w);
                result.cols[col].z = (cols[0].z * b.x) + (cols[1].z * b.y) + (cols[2].z * b.z) + (cols[3].z * b.w);
                result.cols[col].w = (cols[0].w * b.x) + (cols[1].w * b.y) + (cols[2].w * b.z) + (cols[3].w * b.w);
#endif
            }
            return result;
        }

        // Transformacja wektora 4D: this * v
        [[nodiscard]] inline CleanVec4 Transform(const CleanVec4& v) const noexcept
        {
#if PODMATH_HAS_SIMD
            __m128 vx = _mm_set1_ps(v.x);
            __m128 vy = _mm_set1_ps(v.y);
            __m128 vz = _mm_set1_ps(v.z);
            __m128 vw = _mm_set1_ps(v.w);

            __m128 c0 = _mm_load_ps(&cols[0].x);
            __m128 c1 = _mm_load_ps(&cols[1].x);
            __m128 c2 = _mm_load_ps(&cols[2].x);
            __m128 c3 = _mm_load_ps(&cols[3].x);

            CleanVec4 out;
            __m128 r = _mm_add_ps(
                _mm_add_ps(_mm_mul_ps(c0, vx), _mm_mul_ps(c1, vy)),
                _mm_add_ps(_mm_mul_ps(c2, vz), _mm_mul_ps(c3, vw))
            );
            _mm_store_ps(&out.x, r);
            return out;
#else
            return CleanVec4(
                (cols[0].x * v.x) + (cols[1].x * v.y) + (cols[2].x * v.z) + (cols[3].x * v.w),
                (cols[0].y * v.x) + (cols[1].y * v.y) + (cols[2].y * v.z) + (cols[3].y * v.w),
                (cols[0].z * v.x) + (cols[1].z * v.y) + (cols[2].z * v.z) + (cols[3].z * v.w),
                (cols[0].w * v.x) + (cols[1].w * v.y) + (cols[2].w * v.z) + (cols[3].w * v.w)
            );
#endif
        }

        // Transformacja punktu 3D (przy zalozeniu w = 1.0f)
        [[nodiscard]] inline CleanVec3 TransformPoint(const CleanVec3& p) const noexcept
        {
            const CleanVec4 res = Transform(CleanVec4(p.x, p.y, p.z, 1.0f));
            return CleanVec3(res.x, res.y, res.z);
        }

        // Transformacja wektora 3D (przy zalozeniu w = 0.0f)
        [[nodiscard]] inline CleanVec3 TransformVector(const CleanVec3& v) const noexcept
        {
            const CleanVec4 res = Transform(CleanVec4(v.x, v.y, v.z, 0.0f));
            return CleanVec3(res.x, res.y, res.z);
        }

        // Transpozycja macierzy
        [[nodiscard]] inline CleanMat4 Transpose() const noexcept
        {
#if PODMATH_HAS_SIMD
            __m128 r0 = _mm_load_ps(&cols[0].x);
            __m128 r1 = _mm_load_ps(&cols[1].x);
            __m128 r2 = _mm_load_ps(&cols[2].x);
            __m128 r3 = _mm_load_ps(&cols[3].x);
            _MM_TRANSPOSE4_PS(r0, r1, r2, r3);

            CleanMat4 res;
            _mm_store_ps(&res.cols[0].x, r0);
            _mm_store_ps(&res.cols[1].x, r1);
            _mm_store_ps(&res.cols[2].x, r2);
            _mm_store_ps(&res.cols[3].x, r3);
            return res;
#else
            return CleanMat4(
                CleanVec4(cols[0].x, cols[1].x, cols[2].x, cols[3].x),
                CleanVec4(cols[0].y, cols[1].y, cols[2].y, cols[3].y),
                CleanVec4(cols[0].z, cols[1].z, cols[2].z, cols[3].z),
                CleanVec4(cols[0].w, cols[1].w, cols[2].w, cols[3].w)
            );
#endif
        }

        constexpr CleanMat4 operator*(const CleanMat4& rhs) const noexcept { return Multiply(rhs); }
        constexpr CleanVec4 operator*(const CleanVec4& rhs) const noexcept { return Transform(rhs); }
    };

    // =========================================================================
    // Compile-time Invariants & Static Assertions (Zero Regressions / SIMD Guarantee)
    // =========================================================================

    // 1. Rozmiary struktur (sizeof)
    static_assert(sizeof(CleanVec2) == 8,  "CleanVec2 musi miec dokladnie 8 bajtow (glam::Vec2 parity)");
    static_assert(sizeof(CleanVec3) == 12, "CleanVec3 musi miec dokladnie 12 bajtow (glam::Vec3 parity)");
    static_assert(sizeof(CleanVec4) == 16, "CleanVec4 musi miec dokladnie 16 bajtow (glam::Vec4 parity)");
    static_assert(sizeof(CleanMat4) == 64, "CleanMat4 musi miec dokladnie 64 bajty (glam::Mat4 parity)");
    static_assert(sizeof(CleanQuat) == 16, "CleanQuat musi miec dokladnie 16 bajtow (glam::Quat parity)");

    // 2. Wyrownanie pamieci (alignof pod katem SIMD 16B)
    static_assert(alignof(CleanVec2) == 4,  "CleanVec2 wymaga wyrownania 4-bajtowego");
    static_assert(alignof(CleanVec3) == 4,  "CleanVec3 wymaga wyrownania 4-bajtowego");
    static_assert(alignof(CleanVec4) == 16, "CleanVec4 wymaga wyrownania 16-bajtowego dla wektorow SIMD SSE/AVX");
    static_assert(alignof(CleanMat4) == 16, "CleanMat4 wymaga wyrownania 16-bajtowego dla macierzy SIMD SSE/AVX");
    static_assert(alignof(CleanQuat) == 16, "CleanQuat wymaga wyrownania 16-bajtowego dla kwaternionow SIMD");

    // 3. Wlasnosci POD (Standard Layout i Trivially Copyable dla bezpiecznego C-ABI)
    static_assert(std::is_standard_layout_v<CleanVec2>, "CleanVec2 musi byc standard layout dla C-ABI");
    static_assert(std::is_standard_layout_v<CleanVec3>, "CleanVec3 musi byc standard layout dla C-ABI");
    static_assert(std::is_standard_layout_v<CleanVec4>, "CleanVec4 musi byc standard layout dla C-ABI");
    static_assert(std::is_standard_layout_v<CleanMat4>, "CleanMat4 musi byc standard layout dla C-ABI");
    static_assert(std::is_standard_layout_v<CleanQuat>, "CleanQuat musi byc standard layout dla C-ABI");

    static_assert(std::is_trivially_copyable_v<CleanVec2>, "CleanVec2 musi byc trywialnie kopiowalny");
    static_assert(std::is_trivially_copyable_v<CleanVec3>, "CleanVec3 musi byc trywialnie kopiowalny");
    static_assert(std::is_trivially_copyable_v<CleanVec4>, "CleanVec4 musi byc trywialnie kopiowalny");
    static_assert(std::is_trivially_copyable_v<CleanMat4>, "CleanMat4 musi byc trywialnie kopiowalny");
    static_assert(std::is_trivially_copyable_v<CleanQuat>, "CleanQuat musi byc trywialnie kopiowalny");

    // 4. Przesuniecia pol (offsetof)
    static_assert(offsetof(CleanVec2, x) == 0 && offsetof(CleanVec2, y) == 4, "Niezgodnosc offsetow w CleanVec2");
    static_assert(offsetof(CleanVec3, x) == 0 && offsetof(CleanVec3, y) == 4 && offsetof(CleanVec3, z) == 8, "Niezgodnosc offsetow w CleanVec3");
    static_assert(offsetof(CleanVec4, x) == 0 && offsetof(CleanVec4, y) == 4 && offsetof(CleanVec4, z) == 8 && offsetof(CleanVec4, w) == 12, "Niezgodnosc offsetow w CleanVec4");
    static_assert(offsetof(CleanQuat, x) == 0 && offsetof(CleanQuat, y) == 4 && offsetof(CleanQuat, z) == 8 && offsetof(CleanQuat, w) == 12, "Niezgodnosc offsetow w CleanQuat");
    static_assert(offsetof(CleanMat4, cols[0]) == 0 && offsetof(CleanMat4, cols[1]) == 16 && offsetof(CleanMat4, cols[2]) == 32 && offsetof(CleanMat4, cols[3]) == 48, "Niezgodnosc kolumn w CleanMat4");
}

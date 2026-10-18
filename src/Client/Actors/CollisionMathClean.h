#pragma once

#include <cmath>
#include <algorithm>
#include <numbers>
#include <optional>

namespace Client::Collision {

/// Czysta struktura wektora 3D w C++23 bez zaleznosci od DirectX/D3D9 (T315 [W8.5]).
struct CleanVec3 {
    float x{0.0f};
    float y{0.0f};
    float z{0.0f};

    constexpr CleanVec3() noexcept = default;
    constexpr CleanVec3(float fx, float fy, float fz) noexcept : x(fx), y(fy), z(fz) {}

    [[nodiscard]] constexpr float LengthSq() const noexcept {
        return x * x + y * y + z * z;
    }

    [[nodiscard]] constexpr float LengthSq2D() const noexcept {
        return x * x + y * y;
    }

    [[nodiscard]] float Length() const noexcept {
        return std::sqrt(LengthSq());
    }

    [[nodiscard]] float Length2D() const noexcept {
        return std::sqrt(LengthSq2D());
    }

    [[nodiscard]] constexpr float DistanceSq(const CleanVec3& o) const noexcept {
        float dx = x - o.x;
        float dy = y - o.y;
        float dz = z - o.z;
        return dx * dx + dy * dy + dz * dz;
    }

    [[nodiscard]] constexpr float DistanceSq2D(const CleanVec3& o) const noexcept {
        float dx = x - o.x;
        float dy = y - o.y;
        return dx * dx + dy * dy;
    }

    [[nodiscard]] float Distance(const CleanVec3& o) const noexcept {
        return std::sqrt(DistanceSq(o));
    }

    [[nodiscard]] float Distance2D(const CleanVec3& o) const noexcept {
        return std::sqrt(DistanceSq2D(o));
    }

    [[nodiscard]] constexpr float Dot(const CleanVec3& o) const noexcept {
        return x * o.x + y * o.y + z * o.z;
    }

    [[nodiscard]] constexpr CleanVec3 Cross(const CleanVec3& o) const noexcept {
        return CleanVec3{
            y * o.z - z * o.y,
            z * o.x - x * o.z,
            x * o.y - y * o.x
        };
    }

    constexpr CleanVec3 operator+(const CleanVec3& o) const noexcept {
        return CleanVec3{x + o.x, y + o.y, z + o.z};
    }

    constexpr CleanVec3 operator-(const CleanVec3& o) const noexcept {
        return CleanVec3{x - o.x, y - o.y, z - o.z};
    }

    constexpr CleanVec3 operator*(float s) const noexcept {
        return CleanVec3{x * s, y * s, z * s};
    }

    constexpr CleanVec3 operator/(float s) const noexcept {
        float inv = 1.0f / s;
        return CleanVec3{x * inv, y * inv, z * inv};
    }

    constexpr CleanVec3& operator+=(const CleanVec3& o) noexcept {
        x += o.x; y += o.y; z += o.z; return *this;
    }

    constexpr CleanVec3& operator-=(const CleanVec3& o) noexcept {
        x -= o.x; y -= o.y; z -= o.z; return *this;
    }

    constexpr CleanVec3& operator*=(float s) noexcept {
        x *= s; y *= s; z *= s; return *this;
    }

    constexpr bool operator==(const CleanVec3& o) const noexcept = default;
};

/// Reprezentacja dynamicznej sfery kolizyjnej w ruchu.
struct CleanDynamicSphere {
    CleanVec3 v3LastPosition;
    CleanVec3 v3Position;
    float fRadius{0.0f};

    constexpr CleanDynamicSphere() noexcept = default;
    constexpr CleanDynamicSphere(const CleanVec3& last, const CleanVec3& cur, float radius) noexcept
        : v3LastPosition(last), v3Position(cur), fRadius(radius) {}
};

/// Konwersja stopni na radiany w C++23.
[[nodiscard]] constexpr float CleanDegreeToRadian(float degree) noexcept {
    return degree * (std::numbers::pi_v<float> / 180.0f);
}

/// Konwersja radianow na stopnie w C++23.
[[nodiscard]] constexpr float CleanRadianToDegree(float radian) noexcept {
    return radian * (180.0f / std::numbers::pi_v<float>);
}

/// Obrot wektora 2D wokol osi Z o podany kat w stopniach (czysta trygonometria).
[[nodiscard]] inline CleanVec3 CleanRotateZ(const CleanVec3& pt, float degree) noexcept {
    float rad = CleanDegreeToRadian(degree);
    float c = std::cos(rad);
    float s = std::sin(rad);
    return CleanVec3{
        pt.x * c - pt.y * s,
        pt.x * s + pt.y * c,
        pt.z
    };
}

/// Wyznacza najblizsze punkty na dwoch odcinkach linii w 3D [A1, A2] oraz [B1, B2]
/// z uzyciem czystej algebry liniowej (algorytm najmniejszych kwadratow).
inline void CleanIntersectLineSegments(
    const CleanVec3& A1, const CleanVec3& A2,
    const CleanVec3& B1, const CleanVec3& B2,
    CleanVec3& outA, CleanVec3& outB) noexcept
{
    CleanVec3 u = A2 - A1;
    CleanVec3 v = B2 - B1;
    CleanVec3 w = A1 - B1;

    float a = u.Dot(u); // kwadrat dlugosci segmentu A
    float b = u.Dot(v);
    float c = v.Dot(v); // kwadrat dlugosci segmentu B
    float d = u.Dot(w);
    float e = v.Dot(w);
    float D = a * c - b * b; // wyznacznik

    float sc, sN, sD = D; // sc = sN / sD, default sD = D >= 0
    float tc, tN, tD = D; // tc = tN / tD, default tD = D >= 0

    constexpr float kEpsilon = 1e-6f;

    // Sprawdz czy linie sa prawie rownolegle
    if (D < kEpsilon) {
        sN = 0.0f;
        sD = 1.0f;
        tN = e;
        tD = c;
    } else {
        sN = (b * e - c * d);
        tN = (a * e - b * d);
        if (sN < 0.0f) {
            sN = 0.0f;
            tN = e;
            tD = c;
        } else if (sN > sD) {
            sN = sD;
            tN = e + b;
            tD = c;
        }
    }

    if (tN < 0.0f) {
        tN = 0.0f;
        if (-d < 0.0f)
            sN = 0.0f;
        else if (-d > a)
            sN = sD;
        else {
            sN = -d;
            sD = a;
        }
    } else if (tN > tD) {
        tN = tD;
        if ((-d + b) < 0.0f)
            sN = 0.0f;
        else if ((-d + b) > a)
            sN = sD;
        else {
            sN = (-d + b);
            sD = a;
        }
    }

    sc = (std::abs(sN) < kEpsilon ? 0.0f : sN / sD);
    tc = (std::abs(tN) < kEpsilon ? 0.0f : tN / tD);

    outA = A1 + u * sc;
    outB = B1 + v * tc;
}

/// Sprawdza kolizje dwoch dynamicznych sfer w ruchu (Sweep Test) bez DirectX/D3D9.
[[nodiscard]] inline bool CleanDetectCollisionDynamicSphere(
    const CleanDynamicSphere& s1,
    const CleanDynamicSphere& s2) noexcept
{
    float r = s1.fRadius + s2.fRadius;
    float rsq = r * r;

    // 1. Szybki test AABB (Bounding Box Early-Exit)
    CleanVec3 mi1 = s1.v3LastPosition;
    CleanVec3 mi2 = s1.v3Position;
    CleanVec3 mi3 = s2.v3LastPosition;
    CleanVec3 mi4 = s2.v3Position;

    if (mi1.x > mi2.x) std::swap(mi1.x, mi2.x);
    if (mi1.y > mi2.y) std::swap(mi1.y, mi2.y);
    if (mi1.z > mi2.z) std::swap(mi1.z, mi2.z);

    if (mi3.x > mi4.x) std::swap(mi3.x, mi4.x);
    if (mi3.y > mi4.y) std::swap(mi3.y, mi4.y);
    if (mi3.z > mi4.z) std::swap(mi3.z, mi4.z);

    mi1.x -= s1.fRadius; mi1.y -= s1.fRadius; mi1.z -= s1.fRadius;
    mi2.x += s1.fRadius; mi2.y += s1.fRadius; mi2.z += s1.fRadius;

    mi3.x -= s2.fRadius; mi3.y -= s2.fRadius; mi3.z -= s2.fRadius;
    mi4.x += s2.fRadius; mi4.y += s2.fRadius; mi4.z += s2.fRadius;

    if (mi4.x < mi1.x || mi2.x < mi3.x) return false;
    if (mi4.y < mi1.y || mi2.y < mi3.y) return false;
    if (mi4.z < mi1.z || mi2.z < mi3.z) return false;

    // 2. Precyzyjne wyznaczenie minimalnej odleglosci miedzy trajektoriami
    CleanVec3 vA, vB;
    CleanIntersectLineSegments(
        s1.v3LastPosition, s1.v3Position,
        s2.v3LastPosition, s2.v3Position,
        vA, vB
    );

    CleanVec3 diff = vA - vB;
    return diff.LengthSq() <= rsq;
}

/// Sprawdza kolizje dwoch dynamicznych walcow w plaszczyznie XY (Z-Cylinder) bez DirectX.
[[nodiscard]] inline bool CleanDetectCollisionDynamicZCylinder(
    const CleanDynamicSphere& c1,
    const CleanDynamicSphere& c2) noexcept
{
    CleanDynamicSphere cyl1 = c1;
    CleanDynamicSphere cyl2 = c2;
    cyl1.v3Position.z = 0.0f;
    cyl1.v3LastPosition.z = 0.0f;
    cyl2.v3Position.z = 0.0f;
    cyl2.v3LastPosition.z = 0.0f;

    return CleanDetectCollisionDynamicSphere(cyl1, cyl2);
}

/// Czysty raycast przeciecia promienia ze sfera (zastepujacy D3DXIntersect).
[[nodiscard]] inline std::optional<float> CleanRaySphereIntersect(
    const CleanVec3& rayOrigin,
    const CleanVec3& rayDirNormalized,
    const CleanVec3& sphereCenter,
    float sphereRadius) noexcept
{
    CleanVec3 m = rayOrigin - sphereCenter;
    float b = m.Dot(rayDirNormalized);
    float c = m.LengthSq() - sphereRadius * sphereRadius;

    // Jesli promien zaczyna sie poza sfera i jest skierowany w przeciwnym kierunku
    if (c > 0.0f && b > 0.0f)
        return std::nullopt;

    float discr = b * b - c;
    if (discr < 0.0f)
        return std::nullopt; // Brak przeciecia

    float t = -b - std::sqrt(discr);
    if (t < 0.0f)
        t = 0.0f; // Poczytek wewnatrz sfery

    return t;
}

/// Oblicza wektor slizgania wzdluz plaszczyzny o normalnej n (OnCollisionSlide) bez drgan.
[[nodiscard]] inline CleanVec3 CleanCalculateSlideVector(
    const CleanVec3& velocity,
    const CleanVec3& normalNormalized) noexcept
{
    float dot = velocity.Dot(normalNormalized);
    if (dot >= 0.0f) {
        return velocity; // Ruch w kierunku od sciany
    }
    return velocity - normalNormalized * dot;
}

/// Oblicza wektor slizgania w narozniku dwoch scian o normalnych n1 i n2 bez drgan i blokowania.
[[nodiscard]] inline CleanVec3 CleanCalculateCornerSlideVector(
    const CleanVec3& velocity,
    const CleanVec3& n1,
    const CleanVec3& n2) noexcept
{
    CleanVec3 slide1 = CleanCalculateSlideVector(velocity, n1);
    if (slide1.Dot(n2) >= 0.0f) {
        return slide1;
    }

    CleanVec3 crease = n1.Cross(n2);
    float creaseLenSq = crease.LengthSq();
    if (creaseLenSq < 1e-6f) {
        return CleanVec3{0.0f, 0.0f, 0.0f}; // Sciany przeciwstawne
    }

    CleanVec3 creaseDir = crease / std::sqrt(creaseLenSq);
    return creaseDir * velocity.Dot(creaseDir);
}

} // namespace Client::Collision

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <cmath>
#include <DirectXMath.h>

struct ProjectileVec3 {
    float x{0.0f}, y{0.0f}, z{0.0f};

    constexpr ProjectileVec3() = default;
    constexpr ProjectileVec3(float x, float y, float z) : x(x), y(y), z(z) {}

    float LengthSq() const { return x * x + y * y + z * z; }
    float Length() const { return std::sqrt(LengthSq()); }

    ProjectileVec3 Normalized() const {
        float l = Length();
        if (l < 1e-6f) return {0.0f, 0.0f, 0.0f};
        return {x / l, y / l, z / l};
    }

    ProjectileVec3 operator+(const ProjectileVec3& o) const { return {x + o.x, y + o.y, z + o.z}; }
    ProjectileVec3 operator-(const ProjectileVec3& o) const { return {x - o.x, y - o.y, z - o.z}; }
    ProjectileVec3 operator*(float s) const { return {x * s, y * s, z * s}; }
};

/// Ewaluacja punktu na kwadratowej krzywej Beziera (trajektoria luku/magii)
inline ProjectileVec3 EvaluateQuadraticBezier(const ProjectileVec3& p0, const ProjectileVec3& p1, const ProjectileVec3& p2, float t) {
    float u = 1.0f - t;
    return p0 * (u * u) + p1 * (2.0f * u * t) + p2 * (t * t);
}

/// Symulacja kroku Homing z ograniczeniem kata skretu (maxTurnAngleRad)
inline ProjectileVec3 StepHomingDirection(const ProjectileVec3& curPos, const ProjectileVec3& curVel, const ProjectileVec3& targetPos, float maxTurnAngleRad) {
    ProjectileVec3 toTarget = (targetPos - curPos).Normalized();
    ProjectileVec3 velDir = curVel.Normalized();
    float speed = curVel.Length();

    float dot = velDir.x * toTarget.x + velDir.y * toTarget.y + velDir.z * toTarget.z;
    dot = std::clamp(dot, -1.0f, 1.0f);
    float angle = std::acos(dot);

    if (angle <= maxTurnAngleRad || angle < 1e-5f) {
        return toTarget * speed;
    }

    float t = maxTurnAngleRad / angle;
    ProjectileVec3 blended = velDir * (1.0f - t) + toTarget * t;
    return blended.Normalized() * speed;
}

TEST_CASE("FlyingProjectiles - Quadratic Bezier Ballistics")
{
    ProjectileVec3 start{0.0f, 0.0f, 10.0f};
    ProjectileVec3 apex{50.0f, 0.0f, 40.0f}; // Wybicie strzaly w gore
    ProjectileVec3 target{100.0f, 0.0f, 0.0f};

    // t = 0 -> start
    ProjectileVec3 pStart = EvaluateQuadraticBezier(start, apex, target, 0.0f);
    CHECK(pStart.x == doctest::Approx(0.0f));
    CHECK(pStart.z == doctest::Approx(10.0f));

    // t = 0.5 -> srodek lotu w poblizu apexu
    ProjectileVec3 pMid = EvaluateQuadraticBezier(start, apex, target, 0.5f);
    CHECK(pMid.x == doctest::Approx(50.0f));
    CHECK(pMid.z == doctest::Approx(22.5f)); // 0.25*10 + 0.5*40 + 0.25*0 = 22.5

    // t = 1.0 -> target
    ProjectileVec3 pEnd = EvaluateQuadraticBezier(start, apex, target, 1.0f);
    CHECK(pEnd.x == doctest::Approx(100.0f));
    CHECK(pEnd.z == doctest::Approx(0.0f));
}

TEST_CASE("FlyingProjectiles - Homing Direction Convergence without Numerical Jitter")
{
    ProjectileVec3 pos{0.0f, 0.0f, 0.0f};
    ProjectileVec3 vel{100.0f, 0.0f, 0.0f}; // Porusza sie wzdluz osi X
    ProjectileVec3 target{50.0f, 100.0f, 0.0f}; // Cel znajduje sie na Y=100

    float maxTurnAngle = 0.1f; // ~5.7 stopnia na krok

    // Wykonaj 30 krokow homing
    for (int step = 0; step < 30; ++step) {
        vel = StepHomingDirection(pos, vel, target, maxTurnAngle);
        pos = pos + vel * 0.033f; // dt = 33 ms (~30 fps)

        CHECK_FALSE(std::isnan(vel.x));
        CHECK_FALSE(std::isnan(vel.y));
        CHECK_FALSE(std::isnan(pos.x));
        CHECK_FALSE(std::isnan(pos.y));
    }

    // Po 30 krokach wektor predkosci powinien wyraznie skrecic w kierunku Y > 0
    CHECK(vel.y > 20.0f);
}

TEST_CASE("FlyingProjectiles - Ballistic Gravity Integration")
{
    using namespace DirectX;

    XMVECTOR vPos = XMVectorSet(0.0f, 0.0f, 50.0f, 0.0f);
    XMVECTOR vVel = XMVectorSet(100.0f, 0.0f, 20.0f, 0.0f);
    XMVECTOR vAcc = XMVectorSet(0.0f, 0.0f, 0.0f, 0.0f);
    XMVECTOR vGravity = XMVectorSet(0.0f, 0.0f, -9.81f, 0.0f);
    XMVECTOR vDt = XMVectorReplicate(0.1f);

    for (int i = 0; i < 10; ++i) {
        vVel = XMVectorMultiplyAdd(vAcc, vDt, vVel);
        vVel = XMVectorMultiplyAdd(vGravity, vDt, vVel);
        vPos = XMVectorMultiplyAdd(vVel, vDt, vPos);
    }

    float finalZ = XMVectorGetZ(vPos);
    float finalVz = XMVectorGetZ(vVel);

    // Po 1 sekundzie predkosc Z spadla o ~9.81
    CHECK(finalVz == doctest::Approx(20.0f - 9.81f));
    CHECK(finalZ > 0.0f);
}

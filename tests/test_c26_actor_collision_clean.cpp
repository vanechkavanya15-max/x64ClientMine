#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/CollisionMathClean.h"

using namespace Client::Collision;

TEST_CASE("CollisionMathClean - CleanVec3 Basic Math and Vector Operations") {
    CleanVec3 v1{3.0f, 4.0f, 0.0f};
    CHECK(v1.LengthSq() == doctest::Approx(25.0f));
    CHECK(v1.Length() == doctest::Approx(5.0f));

    CleanVec3 v2{0.0f, 0.0f, 0.0f};
    CHECK(v1.DistanceSq(v2) == doctest::Approx(25.0f));
    CHECK(v1.Distance(v2) == doctest::Approx(5.0f));

    CleanVec3 a{1.0f, 0.0f, 0.0f};
    CleanVec3 b{0.0f, 1.0f, 0.0f};
    CHECK(a.Dot(b) == doctest::Approx(0.0f)); // Prostopadle

    CleanVec3 c = a.Cross(b);
    CHECK(c.x == doctest::Approx(0.0f));
    CHECK(c.y == doctest::Approx(0.0f));
    CHECK(c.z == doctest::Approx(1.0f)); // Zgodnie z regula prawej reki

    CleanVec3 sum = a + b;
    CHECK(sum.x == doctest::Approx(1.0f));
    CHECK(sum.y == doctest::Approx(1.0f));
    CHECK(sum.z == doctest::Approx(0.0f));

    CleanVec3 scaled = sum * 2.0f;
    CHECK(scaled.x == doctest::Approx(2.0f));
    CHECK(scaled.y == doctest::Approx(2.0f));
}

TEST_CASE("CollisionMathClean - Pure Trigonometry and Rotation without D3DXToRadian") {
    float deg90 = 90.0f;
    float rad90 = CleanDegreeToRadian(deg90);
    CHECK(rad90 == doctest::Approx(std::numbers::pi_v<float> * 0.5f));

    CleanVec3 pt{10.0f, 0.0f, 5.0f};
    CleanVec3 rotated = CleanRotateZ(pt, 90.0f);
    CHECK(rotated.x == doctest::Approx(0.0f));
    CHECK(rotated.y == doctest::Approx(10.0f));
    CHECK(rotated.z == doctest::Approx(5.0f));

    CleanVec3 rotated180 = CleanRotateZ(pt, 180.0f);
    CHECK(rotated180.x == doctest::Approx(-10.0f));
    CHECK(rotated180.y == doctest::Approx(0.0f));
    CHECK(rotated180.z == doctest::Approx(5.0f));
}

TEST_CASE("CollisionMathClean - Line Segment Nearest Points (Pure Analytical Geometry)") {
    // Dwa prostopadle przecinajace sie odcinki w plaszczyznie XY
    CleanVec3 A1{-10.0f, 0.0f, 0.0f};
    CleanVec3 A2{10.0f, 0.0f, 0.0f};
    CleanVec3 B1{0.0f, -10.0f, 0.0f};
    CleanVec3 B2{0.0f, 10.0f, 0.0f};

    CleanVec3 outA, outB;
    CleanIntersectLineSegments(A1, A2, B1, B2, outA, outB);

    CHECK(outA.DistanceSq(CleanVec3{0.0f, 0.0f, 0.0f}) < 1e-4f);
    CHECK(outB.DistanceSq(CleanVec3{0.0f, 0.0f, 0.0f}) < 1e-4f);
}

TEST_CASE("CollisionMathClean - Dynamic Sphere Sweep Collision") {
    // 1. Dwie sfery statyczne nakladajace sie
    CleanDynamicSphere s1{
        {0.0f, 0.0f, 0.0f},
        {0.0f, 0.0f, 0.0f},
        30.0f
    };
    CleanDynamicSphere s2{
        {50.0f, 0.0f, 0.0f},
        {50.0f, 0.0f, 0.0f},
        30.0f
    };
    CHECK(CleanDetectCollisionDynamicSphere(s1, s2)); // 50 <= 30+30=60 -> Kolizja

    // 2. Dwie sfery statyczne rozlaczne
    CleanDynamicSphere s3{
        {100.0f, 0.0f, 0.0f},
        {100.0f, 0.0f, 0.0f},
        30.0f
    };
    CHECK_FALSE(CleanDetectCollisionDynamicSphere(s1, s3)); // 100 > 60 -> Brak kolizji

    // 3. Dwie sfery w ruchu przecinajace swoje trajektorie (Sweep Test)
    // Sfera A porusza sie od (-50, 0, 0) do (50, 0, 0)
    // Sfera B porusza sie od (0, -50, 0) do (0, 50, 0)
    CleanDynamicSphere movingA{
        {-50.0f, 0.0f, 0.0f},
        {50.0f, 0.0f, 0.0f},
        10.0f
    };
    CleanDynamicSphere movingB{
        {0.0f, -50.0f, 0.0f},
        {0.0f, 50.0f, 0.0f},
        10.0f
    };
    CHECK(CleanDetectCollisionDynamicSphere(movingA, movingB));

    // 4. Dwie sfery poruszajace sie rownolegle w bezpiecznej odleglosci
    CleanDynamicSphere parallelB{
        {-50.0f, 100.0f, 0.0f},
        {50.0f, 100.0f, 0.0f},
        10.0f
    };
    CHECK_FALSE(CleanDetectCollisionDynamicSphere(movingA, parallelB));
}

TEST_CASE("CollisionMathClean - Dynamic Z-Cylinder Collision") {
    // Walce roznia sie wspolrzedna Z, ale na plaszczyznie XY przecinaja sie
    CleanDynamicSphere cyl1{
        {0.0f, 0.0f, 10.0f},
        {0.0f, 0.0f, 20.0f},
        25.0f
    };
    CleanDynamicSphere cyl2{
        {40.0f, 0.0f, 500.0f},
        {40.0f, 0.0f, 600.0f},
        25.0f
    };
    // Z-Cylinder ignoruje Z, r = 25+25=50, dist(XY) = 40 <= 50 -> Kolizja
    CHECK(CleanDetectCollisionDynamicZCylinder(cyl1, cyl2));

    // Walce rozlaczne w XY
    CleanDynamicSphere cyl3{
        {100.0f, 0.0f, 10.0f},
        {100.0f, 0.0f, 20.0f},
        25.0f
    };
    CHECK_FALSE(CleanDetectCollisionDynamicZCylinder(cyl1, cyl3));
}

TEST_CASE("CollisionMathClean - Ray Sphere Intersect (Pure Math without D3DXIntersect)") {
    CleanVec3 rayOrigin{0.0f, -100.0f, 0.0f};
    CleanVec3 rayDir{0.0f, 1.0f, 0.0f}; // Skierowany w strone +Y
    CleanVec3 sphereCenter{0.0f, 0.0f, 0.0f};
    float radius = 20.0f;

    auto hit = CleanRaySphereIntersect(rayOrigin, rayDir, sphereCenter, radius);
    REQUIRE(hit.has_value());
    // Odleglosc do krawedzi sfery: 100 - 20 = 80
    CHECK(*hit == doctest::Approx(80.0f));

    // Promien omijajacy sfere
    CleanVec3 rayMissDir{1.0f, 0.0f, 0.0f}; // Skierowany w strone +X
    auto miss = CleanRaySphereIntersect(rayOrigin, rayMissDir, sphereCenter, radius);
    CHECK_FALSE(miss.has_value());

    // Promien zaczynajacy sie wewnatrz sfery
    CleanVec3 insideOrigin{0.0f, 5.0f, 0.0f};
    auto insideHit = CleanRaySphereIntersect(insideOrigin, rayDir, sphereCenter, radius);
    REQUIRE(insideHit.has_value());
    CHECK(*insideHit == doctest::Approx(0.0f));
}

TEST_CASE("CollisionMathClean - Actor Combat Distance and Splash Radius Checks") {
    // Symulacja __SplashAttackProcess: sprawdzanie kwadratu dystansu do celu
    CleanVec3 attackerPos{1000.0f, 1000.0f, 50.0f};
    CleanVec3 victimClose{1200.0f, 1200.0f, 50.0f};
    CleanVec3 victimFar{2500.0f, 2500.0f, 50.0f};

    float splashRadius = 1000.0f;
    float splashRadiusSq = splashRadius * splashRadius;

    float distSqClose = attackerPos.DistanceSq(victimClose);
    float distSqFar = attackerPos.DistanceSq(victimFar);

    CHECK(distSqClose < splashRadiusSq); // Cel w zasiegu splash
    CHECK(distSqFar > splashRadiusSq);  // Cel poza zasiegiem splash
}

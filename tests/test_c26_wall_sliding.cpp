#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/CollisionMathClean.h"

using namespace Client::Collision;

TEST_CASE("Wall Sliding - Perpendicular Wall Collision")
{
    CleanVec3 velocity{100.0f, 0.0f, 0.0f};
    CleanVec3 wallNormal{-1.0f, 0.0f, 0.0f}; // Wall faces left

    CleanVec3 slide = CleanCalculateSlideVector(velocity, wallNormal);
    CHECK(slide.x == doctest::Approx(0.0f));
    CHECK(slide.y == doctest::Approx(0.0f));
    CHECK(slide.z == doctest::Approx(0.0f));
}

TEST_CASE("Wall Sliding - Diagonal 45-Degree Slide (Zero X Jitter)")
{
    CleanVec3 velocity{100.0f, 100.0f, 0.0f};
    CleanVec3 wallNormal{-1.0f, 0.0f, 0.0f}; // Wall along Y axis

    CleanVec3 slide = CleanCalculateSlideVector(velocity, wallNormal);
    CHECK(slide.x == doctest::Approx(0.0f));
    CHECK(slide.y == doctest::Approx(100.0f));
    CHECK(slide.z == doctest::Approx(0.0f));
}

TEST_CASE("Wall Sliding - Retreating Movement (No Friction)")
{
    CleanVec3 velocity{-50.0f, 80.0f, 0.0f};
    CleanVec3 wallNormal{-1.0f, 0.0f, 0.0f}; // Wall normal in same direction as X motion

    CleanVec3 slide = CleanCalculateSlideVector(velocity, wallNormal);
    CHECK(slide.x == doctest::Approx(-50.0f));
    CHECK(slide.y == doctest::Approx(80.0f));
    CHECK(slide.z == doctest::Approx(0.0f));
}

TEST_CASE("Wall Sliding - 90-Degree Corner Crease Sliding")
{
    // Two walls forming an inside corner: one facing -X, one facing -Y
    CleanVec3 n1{-1.0f, 0.0f, 0.0f};
    CleanVec3 n2{0.0f, -1.0f, 0.0f};

    // Velocity towards the corner with vertical Z motion (e.g. jumping or slope)
    CleanVec3 velocity{80.0f, 60.0f, 40.0f};

    CleanVec3 cornerSlide = CleanCalculateCornerSlideVector(velocity, n1, n2);

    // X and Y are both blocked into the corner, so slide proceeds along the crease (Z axis)
    CHECK(cornerSlide.x == doctest::Approx(0.0f));
    CHECK(cornerSlide.y == doctest::Approx(0.0f));
    CHECK(cornerSlide.z == doctest::Approx(40.0f));
}

TEST_CASE("Wall Sliding - Opposing Walls Jam (Clamp to Zero)")
{
    CleanVec3 n1{1.0f, 0.0f, 0.0f};
    CleanVec3 n2{-1.0f, 0.0f, 0.0f};
    CleanVec3 velocity{50.0f, 0.0f, 0.0f};

    CleanVec3 cornerSlide = CleanCalculateCornerSlideVector(velocity, n1, n2);
    CHECK(cornerSlide.x == doctest::Approx(0.0f));
    CHECK(cornerSlide.y == doctest::Approx(0.0f));
    CHECK(cornerSlide.z == doctest::Approx(0.0f));
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Math/PODMath.h"
#include <type_traits>

using namespace Client::Math;

TEST_CASE("PODMath Vec2 properties") {
    SUBCASE("Type traits") {
        CHECK(std::is_trivial_v<Vec2>);
        CHECK(std::is_standard_layout_v<Vec2>);
        CHECK(std::is_trivially_copyable_v<Vec2>);
        CHECK(std::is_trivially_destructible_v<Vec2>);
    }
    SUBCASE("Memory layout") {
        CHECK(sizeof(Vec2) == 8);
        CHECK(alignof(Vec2) == 8);
    }
}

TEST_CASE("PODMath Vec3 properties") {
    SUBCASE("Type traits") {
        CHECK(std::is_trivial_v<Vec3>);
        CHECK(std::is_standard_layout_v<Vec3>);
        CHECK(std::is_trivially_copyable_v<Vec3>);
        CHECK(std::is_trivially_destructible_v<Vec3>);
    }
    SUBCASE("Memory layout") {
        CHECK(sizeof(Vec3) == 12);
        CHECK(alignof(Vec3) == 4); // Standard 4 byte align for 3 floats unless specified otherwise
    }
}

TEST_CASE("PODMath Vec4 properties") {
    SUBCASE("Type traits") {
        CHECK(std::is_trivial_v<Vec4>);
        CHECK(std::is_standard_layout_v<Vec4>);
        CHECK(std::is_trivially_copyable_v<Vec4>);
        CHECK(std::is_trivially_destructible_v<Vec4>);
    }
    SUBCASE("Memory layout") {
        CHECK(sizeof(Vec4) == 16);
        CHECK(alignof(Vec4) == 16);
    }
}

TEST_CASE("PODMath Quat properties") {
    SUBCASE("Type traits") {
        CHECK(std::is_trivial_v<Quat>);
        CHECK(std::is_standard_layout_v<Quat>);
        CHECK(std::is_trivially_copyable_v<Quat>);
        CHECK(std::is_trivially_destructible_v<Quat>);
    }
    SUBCASE("Memory layout") {
        CHECK(sizeof(Quat) == 16);
        CHECK(alignof(Quat) == 16);
    }
}

TEST_CASE("PODMath Mat4 properties") {
    SUBCASE("Type traits") {
        CHECK(std::is_trivial_v<Mat4>);
        CHECK(std::is_standard_layout_v<Mat4>);
        CHECK(std::is_trivially_copyable_v<Mat4>);
        CHECK(std::is_trivially_destructible_v<Mat4>);
    }
    SUBCASE("Memory layout") {
        CHECK(sizeof(Mat4) == 64);
        CHECK(alignof(Mat4) == 16);
    }
}

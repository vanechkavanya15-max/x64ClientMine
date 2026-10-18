#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterModelLib/SkeletalMath.h"

using namespace EterModelLib;

TEST_CASE("Vector3 Normalization") {
    Vector3 v(3.0f, 0.0f, 4.0f);
    v.Normalize();
    CHECK(v.x == doctest::Approx(0.6f));
    CHECK(v.y == doctest::Approx(0.0f));
    CHECK(v.z == doctest::Approx(0.8f));
}

TEST_CASE("Quaternion Normalization") {
    Quaternion q(0.0f, 3.0f, 0.0f, 4.0f);
    q.Normalize();
    CHECK(q.x == doctest::Approx(0.0f));
    CHECK(q.y == doctest::Approx(0.6f));
    CHECK(q.z == doctest::Approx(0.0f));
    CHECK(q.w == doctest::Approx(0.8f));
}

TEST_CASE("Matrix4x4 Multiplication") {
    Matrix4x4 a;
    a.m[0][0] = 1; a.m[0][1] = 2; a.m[0][2] = 3; a.m[0][3] = 4;
    a.m[1][0] = 5; a.m[1][1] = 6; a.m[1][2] = 7; a.m[1][3] = 8;
    a.m[2][0] = 9; a.m[2][1] = 10; a.m[2][2] = 11; a.m[2][3] = 12;
    a.m[3][0] = 13; a.m[3][1] = 14; a.m[3][2] = 15; a.m[3][3] = 16;
    
    Matrix4x4 b = Matrix4x4::Identity();
    Matrix4x4 c = a * b;
    
    for(int i=0; i<4; ++i) {
        for(int j=0; j<4; ++j) {
            CHECK(c.m[i][j] == doctest::Approx(a.m[i][j]));
        }
    }
}

TEST_CASE("Matrix4x4 Inverse") {
    Matrix4x4 a = Matrix4x4::Identity();
    a.m[0][3] = 10.0f;
    a.m[1][3] = 20.0f;
    a.m[2][3] = 30.0f;
    
    Matrix4x4 inv = a.Inverse();
    CHECK(inv.m[0][3] == doctest::Approx(-10.0f));
    CHECK(inv.m[1][3] == doctest::Approx(-20.0f));
    CHECK(inv.m[2][3] == doctest::Approx(-30.0f));
}

TEST_CASE("EvaluateHierarchy with cycle") {
    std::vector<int32_t> parents = {1, 0}; // Node 0 -> parent 1, Node 1 -> parent 0
    std::vector<Matrix4x4> locals = { Matrix4x4::Identity(), Matrix4x4::Identity() };
    std::vector<Matrix4x4> worlds;
    std::vector<uint8_t> state;
    
    // Should not infinite loop and should break cycle
    EvaluateHierarchy(parents, locals, worlds, state);
    
    CHECK(worlds.size() == 2);
}

TEST_CASE("EvaluateHierarchy bounds check") {
    std::vector<int32_t> parents = { -1 }; 
    std::vector<Matrix4x4> locals = { Matrix4x4::Identity(), Matrix4x4::Identity() }; // Mismatched sizes
    std::vector<Matrix4x4> worlds;
    std::vector<uint8_t> state;
    
    // Should immediately return
    EvaluateHierarchy(parents, locals, worlds, state);
    
    CHECK(worlds.size() == 0);
}

TEST_CASE("ComputeSkinningMatrices pointer logic") {
    std::vector<Matrix4x4> world(2);
    world[0] = Matrix4x4::Identity();
    world[0].m[0][3] = 5.0f; // Translation x = 5
    world[1] = Matrix4x4::Identity();
    
    std::vector<Matrix4x4> invBind(2);
    invBind[0] = Matrix4x4::Identity();
    invBind[0].m[0][3] = -5.0f; // Inverse translation x = -5
    invBind[1] = Matrix4x4::Identity();
    
    std::vector<Matrix4x4> outSkin;
    
    ComputeSkinningMatrices(world, invBind, outSkin);
    
    CHECK(outSkin.size() == 2);
    // Matrix multiplication: invBind[0] * world[0]
    // Both are essentially identity with X translation.
    // Resulting translation should be 0.
    CHECK(outSkin[0].m[0][3] == doctest::Approx(0.0f));
}

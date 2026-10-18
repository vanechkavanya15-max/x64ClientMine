#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "SkeletalMath.h"

using namespace EterModelLib;

TEST_SUITE("SkeletalMath") {
    TEST_CASE("Vector3 Normalization") {
        Vector3 v1(10.0f, 0.0f, 0.0f);
        v1.Normalize();
        CHECK(v1.x == doctest::Approx(1.0f));
        CHECK(v1.y == doctest::Approx(0.0f));
        CHECK(v1.z == doctest::Approx(0.0f));

        Vector3 v2(0.0f, 0.0f, 0.0f);
        v2.Normalize();
        CHECK(v2.x == doctest::Approx(0.0f));
        CHECK(v2.y == doctest::Approx(0.0f));
        CHECK(v2.z == doctest::Approx(0.0f));

        Vector3 v3(0.0000001f, 0.0f, 0.0f);
        v3.Normalize();
        CHECK(v3.x == doctest::Approx(0.0f));
        CHECK(v3.y == doctest::Approx(0.0f));
        CHECK(v3.z == doctest::Approx(0.0f));
    }

    TEST_CASE("Quaternion Normalization") {
        Quaternion q1(10.0f, 0.0f, 0.0f, 0.0f);
        q1.Normalize();
        CHECK(q1.x == doctest::Approx(1.0f));
        CHECK(q1.y == doctest::Approx(0.0f));
        CHECK(q1.z == doctest::Approx(0.0f));
        CHECK(q1.w == doctest::Approx(0.0f));

        Quaternion q2(0.0f, 0.0f, 0.0f, 0.0f);
        q2.Normalize();
        CHECK(q2.x == doctest::Approx(0.0f));
        CHECK(q2.y == doctest::Approx(0.0f));
        CHECK(q2.z == doctest::Approx(0.0f));
        CHECK(q2.w == doctest::Approx(1.0f)); // Should fallback to Identity
    }

    TEST_CASE("Matrix4x4 Multiplication") {
        Matrix4x4 m1 = Matrix4x4::Identity();
        m1.m[0][3] = 1.0f;
        m1.m[1][3] = 2.0f;
        
        Matrix4x4 m2 = Matrix4x4::Identity();
        m2.m[0][3] = 3.0f;
        m2.m[1][3] = 4.0f;

        Matrix4x4 m3 = m1 * m2;
        
        CHECK(m3.m[0][3] == doctest::Approx(4.0f));
        CHECK(m3.m[1][3] == doctest::Approx(6.0f));
        CHECK(m3.m[2][3] == doctest::Approx(0.0f));
        CHECK(m3.m[3][3] == doctest::Approx(1.0f));
    }

    TEST_CASE("EvaluateHierarchy Bounds Protection") {
        std::vector<int32_t> parents = { -1 };
        std::vector<Matrix4x4> local(2, Matrix4x4::Identity());
        std::vector<Matrix4x4> world;

        // parentIndices size < localTransforms size
        EvaluateHierarchy(parents, local, world);
        
        // Output worldTransforms should match min of both arrays
        CHECK(world.size() == 1); 
    }

    TEST_CASE("EvaluateHierarchy Cycle Breaking") {
        std::vector<int32_t> parents = { 1, 0 }; // 0 points to 1, 1 points to 0
        std::vector<Matrix4x4> local(2, Matrix4x4::Identity());
        std::vector<Matrix4x4> world;

        // Shouldn't infinite loop/crash due to state map handling
        EvaluateHierarchy(parents, local, world);
        
        CHECK(world.size() == 2);
    }
}

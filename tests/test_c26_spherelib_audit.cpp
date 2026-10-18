#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "SphereLib/vector.h"
#include "SphereLib/sphere.h"
#include "SphereLib/spherepack.h"
#include "SphereLib/frustum.h"
#include "SphereLib/pool.h"

TEST_CASE("SphereLib Vector3d: Lerp and Cross aliasing") {
    SUBCASE("Lerp aliasing when *this == from") {
        Vector3d v(10.0f, 20.0f, 30.0f);
        Vector3d target(20.0f, 40.0f, 60.0f);
        
        v.Lerp(v, target, 0.5f);
        
        CHECK(v.x == doctest::Approx(15.0f));
        CHECK(v.y == doctest::Approx(30.0f));
        CHECK(v.z == doctest::Approx(45.0f));
    }

    SUBCASE("Cross aliasing when result is stored in first operand") {
        Vector3d v(1.0f, 0.0f, 0.0f);
        Vector3d yAxis(0.0f, 1.0f, 0.0f);
        
        v.Cross(v, yAxis); // X cross Y = Z (0, 0, 1)
        
        CHECK(v.x == doctest::Approx(0.0f));
        CHECK(v.y == doctest::Approx(0.0f));
        CHECK(v.z == doctest::Approx(1.0f));
    }

    SUBCASE("Normalize zero vector protection and non-zero normalization") {
        Vector3d zero(0.0f, 0.0f, 0.0f);
        float len0 = zero.Normalize();
        CHECK(len0 == 0.0f);
        CHECK(zero.x == 0.0f);
        CHECK(zero.y == 0.0f);
        CHECK(zero.z == 0.0f);

        Vector3d normal(3.0f, 0.0f, 4.0f);
        float len = normal.Normalize();
        CHECK(len == doctest::Approx(5.0f));
        CHECK(normal.x == doctest::Approx(0.6f));
        CHECK(normal.y == doctest::Approx(0.0f));
        CHECK(normal.z == doctest::Approx(0.8f));
    }
}

TEST_CASE("SphereLib Sphere: Intersection and empty compute") {
    SUBCASE("Ray originating inside sphere") {
        Sphere sphere;
        sphere.Set(Vector3d(0.0f, 0.0f, 0.0f), 10.0f);

        Vector3d insideOrigin(1.0f, 2.0f, 3.0f);
        Vector3d dir(0.0f, 0.0f, 1.0f);
        Vector3d hitPoint;

        bool hit = sphere.RayIntersection(insideOrigin, dir, 50.0f, &hitPoint);
        CHECK(hit == true);
        CHECK(hitPoint.x == doctest::Approx(insideOrigin.x));
        CHECK(hitPoint.y == doctest::Approx(insideOrigin.y));
        CHECK(hitPoint.z == doctest::Approx(insideOrigin.z));
    }

    SUBCASE("Empty vertex source in Compute") {
        class EmptySource : public SphereInterface {
        public:
            int GetVertexCount(void) const override { return 0; }
            bool GetVertex(int, Vector3d&) const override { return false; }
        } emptySrc;

        Sphere sphere;
        sphere.Compute(emptySrc);
        CHECK(sphere.GetRadius() == 0.0f);
        CHECK(sphere.GetCenter().x == 0.0f);
        CHECK(sphere.GetCenter().y == 0.0f);
        CHECK(sphere.GetCenter().z == 0.0f);
    }
}

TEST_CASE("SphereLib Frustum: BuildViewFrustum2 analytical radius") {
    Frustum frustum;
    D3DXMATRIX mat;
    D3DXMatrixIdentity(&mat);

    const float fNear = 10.0f;
    const float fFar = 1000.0f;
    const float fFov = D3DXToRadian(60.0f);
    const float fAspect = 16.0f / 9.0f;
    const D3DXVECTOR3 vCamera(0.0f, 0.0f, 0.0f);
    const D3DXVECTOR3 vLook(0.0f, 0.0f, 1.0f);

    frustum.BuildViewFrustum2(mat, fNear, fFar, fFov, fAspect, vCamera, vLook);

    const float tanHalf = tanf(fFov * 0.5f);
    const float hFar = fFar * tanHalf;
    const float wFar = hFar * fAspect;
    const float halfLen = 0.5f * (fFar - fNear);
    const float expectedRadius = sqrtf(wFar * wFar + hFar * hFar + halfLen * halfLen);

    CHECK(frustum.GetRadius() == doctest::Approx(expectedRadius));
    CHECK(frustum.GetCenter().z == doctest::Approx(fNear + halfLen));
}

TEST_CASE("SphereLib Pool: Single and zero item edge cases") {
    struct TestItem {
        TestItem* next = nullptr;
        TestItem* prev = nullptr;
        void SetNext(TestItem* n) { next = n; }
        void SetPrevious(TestItem* p) { prev = p; }
        TestItem* GetNext() const { return next; }
    };

    Pool<TestItem> poolZero;
    poolZero.Set(0);
    CHECK(poolZero.GetFreeCount() == 0);

    Pool<TestItem> poolOne;
    poolOne.Set(1);
    CHECK(poolOne.GetFreeCount() == 1);
}

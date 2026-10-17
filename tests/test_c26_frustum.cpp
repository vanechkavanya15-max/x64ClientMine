#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <windows.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <cmath>

#include "SphereLib/frustum.h"

TEST_SUITE("FrustumUnifiedModule")
{
    TEST_CASE("Frustum - ExtractPlanes from known perspective projection matrix")
    {
        // 1. Zdefiniowanie znanej macierzy rzutowania perspektywicznego
        // Eye w (0, 0, 0), patrzy w (0, 0, 1), up (0, 1, 0) -> macierz widoku to macierz jednostkowa
        D3DXMATRIX matView;
        D3DXMatrixIdentity(&matView);

        const float fovY = D3DXToRadian(60.0f);
        const float aspect = 4.0f / 3.0f;
        const float zNear = 10.0f;
        const float zFar = 1000.0f;

        D3DXMATRIX matProj;
        D3DXMatrixPerspectiveFovLH(&matProj, fovY, aspect, zNear, zFar);

        D3DXMATRIX matViewProj = matView * matProj;

        // 2. Ekstrakcja 6 plaszczyzn za pomoca zunifikowanego modulu CFrustum
        D3DXPLANE planes[FRUSTUM_PLANE_COUNT];
        CFrustum::ExtractPlanes(matViewProj, planes, false);

        // 3. Weryfikacja normalizacji: kazda plaszczyzna musi miec wektor normalny o dlugosci 1.0f
        for (int i = 0; i < FRUSTUM_PLANE_COUNT; ++i)
        {
            const float lenSq = planes[i].a * planes[i].a + planes[i].b * planes[i].b + planes[i].c * planes[i].c;
            CHECK(lenSq == doctest::Approx(1.0f).epsilon(0.001f));
        }

        // 4. Weryfikacja plaszczyzny Near (indeks 0):
        // Normalna powinna byc skierowana scisle wzdluz dodatniej osi Z (0, 0, 1) z odlegloscia d = -zNear = -10.0f
        const D3DXPLANE& nearPlane = planes[FRUSTUM_PLANE_NEAR];
        CHECK(nearPlane.a == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(nearPlane.b == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(nearPlane.c == doctest::Approx(1.0f).epsilon(0.001f));
        CHECK(nearPlane.d == doctest::Approx(-zNear).epsilon(0.001f));

        // Punkt lezacy dokladnie na plaszczyznie Near: odleglosc == 0
        D3DXVECTOR3 ptOnNear(0.0f, 0.0f, zNear);
        CHECK(D3DXPlaneDotCoord(&nearPlane, &ptOnNear) == doctest::Approx(0.0f).epsilon(0.001f));

        // Punkt wewnatrz bryly widzenia za Near: odleglosc > 0
        D3DXVECTOR3 ptInsideNear(0.0f, 0.0f, zNear + 15.0f);
        CHECK(D3DXPlaneDotCoord(&nearPlane, &ptInsideNear) > 0.0f);

        // Punkt przed plaszczyzna Near (poza frustum): odleglosc < 0
        D3DXVECTOR3 ptOutsideNear(0.0f, 0.0f, zNear - 5.0f);
        CHECK(D3DXPlaneDotCoord(&nearPlane, &ptOutsideNear) < 0.0f);

        // 5. Weryfikacja plaszczyzny Far (indeks 1):
        // Normalna powinna byc skierowana scisle wzdluz ujemnej osi Z (0, 0, -1) z odlegloscia d = +zFar = 1000.0f
        const D3DXPLANE& farPlane = planes[FRUSTUM_PLANE_FAR];
        CHECK(farPlane.a == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(farPlane.b == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(farPlane.c == doctest::Approx(-1.0f).epsilon(0.001f));
        CHECK(farPlane.d == doctest::Approx(zFar).epsilon(0.001f));

        // Punkt lezacy dokladnie na plaszczyznie Far
        D3DXVECTOR3 ptOnFar(0.0f, 0.0f, zFar);
        CHECK(std::abs(D3DXPlaneDotCoord(&farPlane, &ptOnFar)) < 0.01f);

        // Punkt za plaszczyzna Far (poza frustum)
        D3DXVECTOR3 ptPastFar(0.0f, 0.0f, zFar + 20.0f);
        CHECK(D3DXPlaneDotCoord(&farPlane, &ptPastFar) < 0.0f);

        // 6. Weryfikacja symetrii plaszczyzn bocznych Left i Right (indeksy 2 i 3):
        const D3DXPLANE& leftPlane = planes[FRUSTUM_PLANE_LEFT];
        const D3DXPLANE& rightPlane = planes[FRUSTUM_PLANE_RIGHT];
        CHECK(leftPlane.a == doctest::Approx(-rightPlane.a).epsilon(0.001f));
        CHECK(leftPlane.b == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(rightPlane.b == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(leftPlane.c == doctest::Approx(rightPlane.c).epsilon(0.001f));
        CHECK(leftPlane.d == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(rightPlane.d == doctest::Approx(0.0f).epsilon(0.001f));

        // 7. Weryfikacja symetrii plaszczyzn Top i Bottom (indeksy 4 i 5):
        const D3DXPLANE& bottomPlane = planes[FRUSTUM_PLANE_BOTTOM];
        const D3DXPLANE& topPlane = planes[FRUSTUM_PLANE_TOP];
        CHECK(bottomPlane.a == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(topPlane.a == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(bottomPlane.b == doctest::Approx(-topPlane.b).epsilon(0.001f));
        CHECK(bottomPlane.c == doctest::Approx(topPlane.c).epsilon(0.001f));
        CHECK(bottomPlane.d == doctest::Approx(0.0f).epsilon(0.001f));
        CHECK(topPlane.d == doctest::Approx(0.0f).epsilon(0.001f));

        // 8. Punkt w centrum bryly widzenia (0, 0, 500) ma dodatnie odleglosci od wszystkich 6 plaszczyzn
        D3DXVECTOR3 ptCenter(0.0f, 0.0f, 500.0f);
        for (int i = 0; i < FRUSTUM_PLANE_COUNT; ++i)
        {
            CHECK(D3DXPlaneDotCoord(&planes[i], &ptCenter) > 0.0f);
        }
    }

    TEST_CASE("Frustum - BuildViewFrustum and ViewVolumeTest for spheres")
    {
        D3DXMATRIX matProj;
        D3DXMatrixPerspectiveFovLH(&matProj, D3DXToRadian(60.0f), 4.0f / 3.0f, 10.0f, 1000.0f);

        CFrustum frustum;
        frustum.BuildViewFrustum(matProj);

        // Kula w centrum frustum
        CHECK(frustum.ViewVolumeTest(Vector3d(0.0f, 0.0f, 500.0f), 20.0f, 0.0f) == VS_INSIDE);
        CHECK(frustum.IsSphereVisible(Vector3d(0.0f, 0.0f, 500.0f), 20.0f, 0.0f) == true);

        // Kula przecinajaca Near (srodek w z=12, r=5, plaszczyzna zNear=10 -> przecina)
        CHECK(frustum.ViewVolumeTest(Vector3d(0.0f, 0.0f, 12.0f), 5.0f, 0.0f) == VS_PARTIAL);
        CHECK(frustum.IsSphereVisible(Vector3d(0.0f, 0.0f, 12.0f), 5.0f, 0.0f) == true);

        // Kula calkowicie przed Near (z=-20, r=5)
        CHECK(frustum.ViewVolumeTest(Vector3d(0.0f, 0.0f, -20.0f), 5.0f, 0.0f) == VS_OUTSIDE);
        CHECK(frustum.IsSphereVisible(Vector3d(0.0f, 0.0f, -20.0f), 5.0f, 0.0f) == false);

        // Kula calkowicie za Far (z=1050, r=10)
        CHECK(frustum.ViewVolumeTest(Vector3d(0.0f, 0.0f, 1050.0f), 10.0f, 0.0f) == VS_OUTSIDE);
        CHECK(frustum.IsSphereVisible(Vector3d(0.0f, 0.0f, 1050.0f), 10.0f, 0.0f) == false);

        // Kula daleko z boku poza lewa krawedzia frustum
        CHECK(frustum.ViewVolumeTest(Vector3d(-1000.0f, 0.0f, 200.0f), 10.0f, 0.0f) == VS_OUTSIDE);
        CHECK(frustum.IsSphereVisible(Vector3d(-1000.0f, 0.0f, 200.0f), 10.0f, 0.0f) == false);
    }

    TEST_CASE("Frustum - IsBoxVisible and ViewVolumeTestAABB")
    {
        D3DXMATRIX matProj;
        D3DXMatrixPerspectiveFovLH(&matProj, D3DXToRadian(60.0f), 4.0f / 3.0f, 10.0f, 1000.0f);

        CFrustum frustum;
        frustum.BuildViewFrustum(matProj);

        // Box wewnatrz frustum
        Vector3d bMinInside(-10.0f, -10.0f, 100.0f);
        Vector3d bMaxInside(10.0f, 10.0f, 120.0f);
        CHECK(frustum.IsBoxVisible(bMinInside, bMaxInside, 0.0f) == true);
        CHECK(frustum.ViewVolumeTestAABB(bMinInside, bMaxInside, 0.0f) == VS_INSIDE);

        // Box calkowicie za Far
        Vector3d bMinFar(-10.0f, -10.0f, 1050.0f);
        Vector3d bMaxFar(10.0f, 10.0f, 1100.0f);
        CHECK(frustum.IsBoxVisible(bMinFar, bMaxFar, 0.0f) == false);
        CHECK(frustum.ViewVolumeTestAABB(bMinFar, bMaxFar, 0.0f) == VS_OUTSIDE);

        // Box przecinajacy plaszczyzne Near
        Vector3d bMinNear(-5.0f, -5.0f, 5.0f);
        Vector3d bMaxNear(5.0f, 5.0f, 20.0f);
        CHECK(frustum.IsBoxVisible(bMinNear, bMaxNear, 0.0f) == true);
        CHECK(frustum.ViewVolumeTestAABB(bMinNear, bMaxNear, 0.0f) == VS_PARTIAL);
    }

    TEST_CASE("Frustum - Metin2 Terrain Convention (Y Negation)")
    {
        // Utworzenie kamery patrzacej w dol pod katem
        D3DXVECTOR3 eye(0.0f, 200.0f, 0.0f);
        D3DXVECTOR3 look(0.0f, 0.0f, 300.0f);
        D3DXVECTOR3 up(0.0f, 1.0f, 0.0f);

        D3DXMATRIX matView;
        D3DXMatrixLookAtLH(&matView, &eye, &look, &up);

        D3DXMATRIX matProj;
        D3DXMatrixPerspectiveFovLH(&matProj, D3DXToRadian(60.0f), 4.0f / 3.0f, 10.0f, 1000.0f);

        D3DXMATRIX matViewProj = matView * matProj;

        CFrustum frustum;
        frustum.BuildViewFrustum(matViewProj);

        // Punkt z dodatnia wspolrzedna Y w terenie Metin2 odpowiada ujemnej wspolrzednej Y w ukladzie kamery
        Vector3d terrainCenter(0.0f, 150.0f, 250.0f);
        const float radius = 25.0f;

        // Test z flaga bNegateY = true musi byc dokladnie rowny testowi punktu ze zanegowanym Y recznie
        ViewState stateWithNegate = frustum.ViewVolumeTest(terrainCenter, radius, 0.0f, true);
        ViewState stateManualNegate = frustum.ViewVolumeTest(Vector3d(terrainCenter.x, -terrainCenter.y, terrainCenter.z), radius, 0.0f, false);
        CHECK(stateWithNegate == stateManualNegate);

        // Metoda pomocnicza ViewVolumeTestTerrain musi dawac dokladnie ten sam wynik
        ViewState stateTerrain = frustum.ViewVolumeTestTerrain(terrainCenter, radius, 0.0f);
        CHECK(stateTerrain == stateWithNegate);

        // Ekstrakcja plaszczyzn z flaga bNegatePlaneY = true
        D3DXPLANE planesNormal[FRUSTUM_PLANE_COUNT];
        D3DXPLANE planesNegY[FRUSTUM_PLANE_COUNT];
        CFrustum::ExtractPlanes(matViewProj, planesNormal, false);
        CFrustum::ExtractPlanes(matViewProj, planesNegY, true);

        for (int i = 0; i < FRUSTUM_PLANE_COUNT; ++i)
        {
            CHECK(planesNegY[i].a == doctest::Approx(planesNormal[i].a).epsilon(0.001f));
            CHECK(planesNegY[i].b == doctest::Approx(-planesNormal[i].b).epsilon(0.001f));
            CHECK(planesNegY[i].c == doctest::Approx(planesNormal[i].c).epsilon(0.001f));
            CHECK(planesNegY[i].d == doctest::Approx(planesNormal[i].d).epsilon(0.001f));

            // Iloczyn skalarny z punktem (x, y, z) na plaszczyznie z zanegowanym Y musi byc rowny
            // iloczynowi ze zwykla plaszczyzna dla punktu (x, -y, z)
            D3DXVECTOR3 testPt(10.0f, 50.0f, 200.0f);
            D3DXVECTOR3 testPtInverted(10.0f, -50.0f, 200.0f);
            float distNeg = D3DXPlaneDotCoord(&planesNegY[i], &testPt);
            float distNormal = D3DXPlaneDotCoord(&planesNormal[i], &testPtInverted);
            CHECK(distNeg == doctest::Approx(distNormal).epsilon(0.001f));
        }
    }

    TEST_CASE("Frustum - Zero Matrix Robustness (no crash / divide by zero)")
    {
        D3DXMATRIX zeroMat;
        memset(&zeroMat, 0, sizeof(zeroMat));

        D3DXPLANE planes[FRUSTUM_PLANE_COUNT];
        CFrustum::ExtractPlanes(zeroMat, planes, false);

        for (int i = 0; i < FRUSTUM_PLANE_COUNT; ++i)
        {
            CHECK_FALSE(std::isnan(planes[i].a));
            CHECK_FALSE(std::isnan(planes[i].b));
            CHECK_FALSE(std::isnan(planes[i].c));
            CHECK_FALSE(std::isnan(planes[i].d));
        }

        CFrustum frustum;
        frustum.BuildViewFrustum(zeroMat);
        CHECK(frustum.GetPlanes() != nullptr);
    }
}

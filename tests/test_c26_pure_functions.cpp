#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <d3d9.h>
#include <d3dx9.h>
#include <cmath>

#include "EterLib/RenderConventions.h"
#include "GameLib/MapOutdoor.h"
#include "GameLib/MapOutdoorWater.h"
#include "GameLib/MapOutdoorCharacterShadow.h"

TEST_CASE("PureFunctions: MapOutdoor constants encapsulation")
{
    CHECK(kLoadSizeWidth == 1);
    CHECK(kAroundAreaNum == 9);
    CHECK(AROUND_AREA_NUM == 9);
}

TEST_CASE("PureFunctions: CalculateWaterWaveHeight determinism and bounds")
{
    float h0 = CalculateWaterWaveHeight(0.0f, 0.0f, 0.0f);
    CHECK(std::abs(h0 - (-6.0f)) < 1e-4f);

    for (float t = 0.0f; t < 20.0f; t += 0.25f)
    {
        float h = CalculateWaterWaveHeight(500.0f, 600.0f, t);
        // Formula: -6.0 + 5.0*sin + 1.0*sin -> zakres od -12.0f do 0.0f
        CHECK(h >= -12.0001f);
        CHECK(h <= 0.0001f);
    }
}

TEST_CASE("PureFunctions: CalculateWaterAnimationFrameIndex cycling")
{
    CHECK(CalculateWaterAnimationFrameIndex(0u) == 0u);
    CHECK(CalculateWaterAnimationFrameIndex(69u) == 0u);
    CHECK(CalculateWaterAnimationFrameIndex(70u) == 1u);
    CHECK(CalculateWaterAnimationFrameIndex(139u) == 1u);
    CHECK(CalculateWaterAnimationFrameIndex(140u) == 2u);

    // Ostatnia klatka w cyklu 2100 ms (29 * 70 = 2030 .. 2099 ms)
    CHECK(CalculateWaterAnimationFrameIndex(2030u) == 29u);
    CHECK(CalculateWaterAnimationFrameIndex(2099u) == 29u);

    // Zawiniecie petli animacji co 30 klatek (2100 ms)
    CHECK(CalculateWaterAnimationFrameIndex(2100u) == 0u);
    CHECK(CalculateWaterAnimationFrameIndex(4200u) == 0u);

    for (uint32_t t = 0; t < 10000; t += 25)
    {
        uint32_t frame = CalculateWaterAnimationFrameIndex(t);
        CHECK(frame < 30u);
    }
}

TEST_CASE("PureFunctions: BuildWaterUVTransformMatrix construction")
{
    D3DXMATRIX matViewInv;
    D3DXMatrixIdentity(&matViewInv);

    const float fTexScale = 0.0025f;
    D3DXMATRIX matUV;
    BuildWaterUVTransformMatrix(&matUV, matViewInv, fTexScale);

    CHECK(matUV._11 == doctest::Approx(fTexScale));
    CHECK(matUV._22 == doctest::Approx(-fTexScale));
    CHECK(matUV._33 == doctest::Approx(0.0f));
    CHECK(matUV._44 == doctest::Approx(1.0f));
    CHECK(matUV._41 == doctest::Approx(0.0f));
    CHECK(matUV._42 == doctest::Approx(0.0f));

    // Przesuniecie kamery w ViewInverse
    D3DXMatrixTranslation(&matViewInv, 1000.0f, 2000.0f, 50.0f);
    BuildWaterUVTransformMatrix(&matUV, matViewInv, fTexScale);

    CHECK(matUV._11 == doctest::Approx(fTexScale));
    CHECK(matUV._22 == doctest::Approx(-fTexScale));
    CHECK(matUV._41 == doctest::Approx(1000.0f * fTexScale));
    CHECK(matUV._42 == doctest::Approx(2000.0f * -fTexScale));
}

TEST_CASE("PureFunctions: BuildCharacterShadowMatrix validity")
{
    D3DXMATRIX mat;
    D3DXVECTOR3 target(100.0f, 200.0f, 50.0f);
    D3DXVECTOR3 lightDir(1.732f, 1.0f, -3.464f);

    BuildCharacterShadowMatrix(&mat, lightDir, target, 5000.0f);

    float det = D3DXMatrixDeterminant(&mat);
    CHECK(std::abs(det) > 1e-3f);

    // Fallback przy zerowym wektorze swiatla
    D3DXMATRIX matFallback;
    BuildCharacterShadowMatrix(&matFallback, D3DXVECTOR3(0, 0, 0), target, 5000.0f);
    float detFallback = D3DXMatrixDeterminant(&matFallback);
    CHECK(std::abs(detFallback) > 1e-3f);
}

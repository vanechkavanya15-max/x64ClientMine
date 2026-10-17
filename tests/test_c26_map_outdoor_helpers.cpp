#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <d3d9.h>
#include <d3dx9.h>
#include <cmath>

#include "GameLib/MapOutdoorCharacterShadow.h"
#include "GameLib/MapOutdoorWater.h"

TEST_CASE("WaterWaveHeightCalculation")
{
    float h0 = CalculateWaterWaveHeight(0.0f, 0.0f, 0.0f);
    CHECK(std::abs(h0 - (-6.0f)) < 1e-4f);

    for (float t = 0.0f; t < 10.0f; t += 0.5f)
    {
        float h = CalculateWaterWaveHeight(100.0f, 200.0f, t);
        // Maksimum z 5.0 + 1.0 = 6.0, minimum = -6.0 -> zakres [-12.0, 0.0]
        CHECK(h >= -12.0001f);
        CHECK(h <= 0.0001f);
    }
}

TEST_CASE("CharacterShadowMatrixConstruction")
{
    D3DXMATRIX mat;
    D3DXVECTOR3 target(100.0f, 200.0f, 50.0f);
    D3DXVECTOR3 lightDir(1.732f, 1.0f, -3.464f);

    BuildCharacterShadowMatrix(&mat, lightDir, target, 5000.0f);

    float det = D3DXMatrixDeterminant(&mat);
    CHECK(std::abs(det) > 1e-3f);

    // Test z zerowym wektorem swiatla (fallback do domyslnego)
    D3DXMATRIX matFallback;
    BuildCharacterShadowMatrix(&matFallback, D3DXVECTOR3(0, 0, 0), target, 5000.0f);
    float detFallback = D3DXMatrixDeterminant(&matFallback);
    CHECK(std::abs(detFallback) > 1e-3f);
}

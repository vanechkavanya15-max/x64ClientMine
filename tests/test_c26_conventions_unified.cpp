#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <EterLib/RenderConventions.h>
#include <cmath>

TEST_CASE("RenderConventions: Core UI and projection constants")
{
    CHECK(kUiHalfPixelOffset == 0.5f);
    CHECK(std::abs(kCloudAspect - (4.0f / 3.0f)) < 0.0001f);
    CHECK(std::abs(kCloudFovRad - 1.04719755f) < 0.00001f);
}

TEST_CASE("RenderConventions: Color packing helper ToBgraDword")
{
    // Test pelnej nieprzezroczystosci
    uint32_t red = ToBgraDword(255, 0, 0, 255);
    CHECK(red == 0xFFFF0000);

    uint32_t green = ToBgraDword(0, 255, 0, 255);
    CHECK(green == 0xFF00FF00);

    uint32_t blue = ToBgraDword(0, 0, 255, 255);
    CHECK(blue == 0xFF0000FF);

    uint32_t white = ToBgraDword(255, 255, 255, 255);
    CHECK(white == 0xFFFFFFFF);

    uint32_t transparentBlack = ToBgraDword(0, 0, 0, 0);
    CHECK(transparentBlack == 0x00000000);

    // Test arbitralnych skladowych
    uint32_t custom = ToBgraDword(0x12, 0x34, 0x56, 0x78);
    CHECK(custom == 0x78123456);
}

TEST_CASE("RenderConventions: Camera and culling constants")
{
    CHECK(kDefaultCameraMinDistance == 200.0f);
    CHECK(kDefaultCameraMaxDistance == 2500.0f);
    CHECK(kDefaultCullingRadius == 1600.0f);
}

TEST_CASE("RenderConventions: TextTail and WeaponTrace constants")
{
    CHECK(kTextTailNamePositionOffset == -10.0f);
    CHECK(kWeaponTraceDefaultDuration == 0.18f);
    CHECK(kWeaponTraceSamplingInterval == 0.003f);
}

TEST_CASE("RenderConventions: Screen space half-pixel offset alignment")
{
    float uiX = 150.0f;
    float uiY = 220.0f;

    float alignedX = uiX - kUiHalfPixelOffset;
    float alignedY = uiY - kUiHalfPixelOffset;

    CHECK(alignedX == 149.5f);
    CHECK(alignedY == 219.5f);
}

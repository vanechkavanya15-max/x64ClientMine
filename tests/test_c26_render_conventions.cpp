#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderConventions.h"
#include <cmath>

TEST_CASE("RenderConventions - UI Half-Pixel Offset")
{
	CHECK(kUiHalfPixelOffset == 0.5f);
	CHECK(EterLib::kUiHalfPixelOffset == 0.5f);

	// Direct3D 9 texel-to-pixel mapping test:
	// Sampling pixel (100, 200) with half-pixel offset
	const float pixelX = 100.0f;
	const float pixelY = 200.0f;
	const float vertexX = pixelX - kUiHalfPixelOffset;
	const float vertexY = pixelY - kUiHalfPixelOffset;

	CHECK(vertexX == doctest::Approx(99.5f));
	CHECK(vertexY == doctest::Approx(199.5f));
}

TEST_CASE("RenderConventions - Cloud Projection Constants")
{
	CHECK(kCloudFovRad == 1.04719755f);
	CHECK(EterLib::kCloudFovRad == 1.04719755f);

	CHECK(kCloudAspect == doctest::Approx(1.33333333f));
	CHECK(EterLib::kCloudAspect == doctest::Approx(1.33333333f));

	// 60 degrees in radians is pi / 3 (~1.047197551)
	const float expectedRad = 3.14159265358979323846f / 3.0f;
	CHECK(std::abs(kCloudFovRad - expectedRad) < 1e-6f);

	// 4:3 aspect ratio is 4.0 / 3.0 (~1.33333333)
	const float expectedAspect = 4.0f / 3.0f;
	CHECK(std::abs(kCloudAspect - expectedAspect) < 1e-6f);
}

TEST_CASE("RenderConventions - ToBgraDword Encoding")
{
	// Test full alpha default
	uint32_t colorOpaque = ToBgraDword(0x12, 0x34, 0x56);
	CHECK(colorOpaque == 0xFF123456);

	// Test custom alpha
	uint32_t colorTranslucent = ToBgraDword(0xAA, 0xBB, 0xCC, 0x80);
	CHECK(colorTranslucent == 0x80AABBCC);

	// Test zero color
	uint32_t colorZero = ToBgraDword(0, 0, 0, 0);
	CHECK(colorZero == 0x00000000);

	// Test namespace alias
	uint32_t colorNs = EterLib::ToBgraDword(0xFF, 0x00, 0x7F, 0xFE);
	CHECK(colorNs == 0xFEFF007F);

	// Verify channel unpack consistency
	uint8_t a = (colorTranslucent >> 24) & 0xFF;
	uint8_t r = (colorTranslucent >> 16) & 0xFF;
	uint8_t g = (colorTranslucent >> 8) & 0xFF;
	uint8_t b = (colorTranslucent) & 0xFF;

	CHECK(a == 0x80);
	CHECK(r == 0xAA);
	CHECK(g == 0xBB);
	CHECK(b == 0xCC);
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/LensFlare.h"
#include "EterLib/CullingManager.h"

TEST_CASE("S8.6 - LensFlare Named Constants")
{
	CHECK(kLensFlareMaxDistance == doctest::Approx(99999999.0f));
	CHECK(kFlareIntensityScale == doctest::Approx(0.85f));
	CHECK(c_fHalfMaxBright == doctest::Approx(0.45f));
	CHECK(c_fDistanceScale == doctest::Approx(1.0f));
	CHECK(c_nDepthTestDimension == 15);
}

TEST_CASE("S8.6 - CullingManager Constexpr Constants")
{
	// Class constants
	CHECK(CCullingManager::kDefaultMaxSpheres == 10000);
	CHECK(CCullingManager::kDefaultRootRadius == doctest::Approx(6400.0f));
	CHECK(CCullingManager::kDefaultLeafRadius == doctest::Approx(1600.0f));
	CHECK(CCullingManager::kDefaultExtraRadius == doctest::Approx(400.0f));
	CHECK(CCullingManager::kDefaultRayFarDistance == doctest::Approx(-1.0f));

	// Namespace EterLib aliases
	CHECK(EterLib::kDefaultCullingMaxSpheres == 10000);
	CHECK(EterLib::kDefaultCullingRootRadius == doctest::Approx(6400.0f));
	CHECK(EterLib::kDefaultCullingLeafRadius == doctest::Approx(1600.0f));
	CHECK(EterLib::kDefaultCullingExtraRadius == doctest::Approx(400.0f));
	CHECK(EterLib::kDefaultCullingRayFarDistance == doctest::Approx(-1.0f));
}

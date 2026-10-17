#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "EterLib/RenderConventions.h"
#include "EterLib/Camera.h"
#include "EterLib/CullingManager.h"
#include "EterLib/LensFlare.h"
#include <type_traits>
#include <cmath>

TEST_CASE("S8.6 - Camera Distance Constants (Typy, Wartosci, Niezmienniki)")
{
	// 1. Poprawnosc typow
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultCameraMinDistance)>>, "kDefaultCameraMinDistance musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultCameraMaxDistance)>>, "kDefaultCameraMaxDistance musi byc float");

	// 2. Weryfikacja compile-time constexpr
	static_assert(kDefaultCameraMinDistance > 0.0f, "Minimalny dystans kamery musi byc dodatni");
	static_assert(kDefaultCameraMaxDistance > kDefaultCameraMinDistance, "Maksymalny dystans kamery musi przewyzszac minimalny");

	// 3. Poprawnosc wartosci numerycznych
	CHECK(kDefaultCameraMinDistance == doctest::Approx(200.0f));
	CHECK(kDefaultCameraMaxDistance == doctest::Approx(2500.0f));

	// 4. Poprawnosc aliasow w przestrzeni nazw EterLib
	CHECK(EterLib::kDefaultCameraMinDistance == doctest::Approx(200.0f));
	CHECK(EterLib::kDefaultCameraMaxDistance == doctest::Approx(2500.0f));

	// 5. Niezmiennosc pol statycznych w klasie CCamera
	CHECK(CCamera::GetCameraMinDistance() == doctest::Approx(kDefaultCameraMinDistance));
	CHECK(CCamera::GetCameraMaxDistance() == doctest::Approx(kDefaultCameraMaxDistance));

	// 6. Test tolerancji i zakresu zoomu kamery
	const float zoomRatio = kDefaultCameraMaxDistance / kDefaultCameraMinDistance;
	CHECK(zoomRatio == doctest::Approx(12.5f));
	CHECK(std::abs(zoomRatio - 12.5f) < 1e-5f);
}

TEST_CASE("S8.6 - Culling and Shading Constants (Typy, Wartosci, Hierarchia Sfer)")
{
	// 1. Poprawnosc typow
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultCullingRadius)>>, "kDefaultCullingRadius musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultCullingLeafRadius)>>, "kDefaultCullingLeafRadius musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultCullingRootRadius)>>, "kDefaultCullingRootRadius musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultCullingExtraRadius)>>, "kDefaultCullingExtraRadius musi byc float");
	static_assert(std::is_same_v<int, std::remove_cv_t<decltype(kDefaultCullingMaxSpheres)>>, "kDefaultCullingMaxSpheres musi byc int");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultCullingRayFarDistance)>>, "kDefaultCullingRayFarDistance musi byc float");

	// 2. Weryfikacja compile-time constexpr
	static_assert(kDefaultCullingRadius > 0.0f, "Promien sfery liscia musi byc dodatni");
	static_assert(kDefaultCullingRootRadius > kDefaultCullingRadius, "Promien korzenia musi byc wiekszy od liscia");
	static_assert(kDefaultCullingExtraRadius > 0.0f, "Promien marginesu sfery musi byc dodatni");
	static_assert(kDefaultCullingMaxSpheres >= 1000, "Maksymalna liczba sfer powinna miescic co najmniej 1000 obiektow");
	static_assert(kDefaultCullingRayFarDistance < 0.0f, "Domyslna odleglosc ray trace bez limitu powinna byc ujemna");

	// 3. Poprawnosc wartosci numerycznych
	CHECK(kDefaultCullingRadius == doctest::Approx(1600.0f));
	CHECK(kDefaultCullingLeafRadius == doctest::Approx(1600.0f));
	CHECK(kDefaultCullingRootRadius == doctest::Approx(6400.0f));
	CHECK(kDefaultCullingExtraRadius == doctest::Approx(400.0f));
	CHECK(kDefaultCullingMaxSpheres == 10000);
	CHECK(kDefaultCullingRayFarDistance == doctest::Approx(-1.0f));

	// 4. Poprawnosc aliasow w przestrzeni nazw EterLib
	CHECK(EterLib::kDefaultCullingRadius == doctest::Approx(1600.0f));
	CHECK(EterLib::kDefaultCullingLeafRadius == doctest::Approx(1600.0f));
	CHECK(EterLib::kDefaultCullingRootRadius == doctest::Approx(6400.0f));
	CHECK(EterLib::kDefaultCullingExtraRadius == doctest::Approx(400.0f));
	CHECK(EterLib::kDefaultCullingMaxSpheres == 10000);
	CHECK(EterLib::kDefaultCullingRayFarDistance == doctest::Approx(-1.0f));

	// 5. Integracja i zgodnosc z klasa CCullingManager
	CHECK(CCullingManager::kDefaultLeafRadius == doctest::Approx(kDefaultCullingRadius));
	CHECK(CCullingManager::kDefaultRootRadius == doctest::Approx(kDefaultCullingRootRadius));
	CHECK(CCullingManager::kDefaultExtraRadius == doctest::Approx(kDefaultCullingExtraRadius));
	CHECK(CCullingManager::kDefaultMaxSpheres == kDefaultCullingMaxSpheres);
	CHECK(CCullingManager::kDefaultRayFarDistance == doctest::Approx(kDefaultCullingRayFarDistance));

	// 6. Test tolerancji i niezmiennikow wielkosci drzewa sfer (Root : Leaf = 4:1)
	const float treeRatio = kDefaultCullingRootRadius / kDefaultCullingRadius;
	CHECK(treeRatio == doctest::Approx(4.0f));
	CHECK(std::abs(treeRatio - 4.0f) < 1e-5f);
}

TEST_CASE("S8.6 - GrpScreen and LensFlare Optical Constants")
{
	// 1. Poprawnosc typow GrpScreen
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultPickRayDistance)>>, "kDefaultPickRayDistance musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultPlaneHalfExtent)>>, "kDefaultPlaneHalfExtent musi byc float");
	static_assert(kDefaultPickRayDistance > 0.0f);
	static_assert(kDefaultPlaneHalfExtent > kDefaultPickRayDistance);

	// 2. Poprawnosc wartosci GrpScreen
	CHECK(kDefaultPickRayDistance == doctest::Approx(51200.0f));
	CHECK(kDefaultPlaneHalfExtent == doctest::Approx(90000000.0f));
	CHECK(EterLib::kDefaultPickRayDistance == doctest::Approx(51200.0f));
	CHECK(EterLib::kDefaultPlaneHalfExtent == doctest::Approx(90000000.0f));

	// 3. Poprawnosc typow LensFlare
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultLensFlareMaxDistance)>>, "kDefaultLensFlareMaxDistance musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kDefaultFlareIntensityScale)>>, "kDefaultFlareIntensityScale musi byc float");
	static_assert(kDefaultLensFlareMaxDistance > 0.0f);
	static_assert(kDefaultFlareIntensityScale > 0.0f && kDefaultFlareIntensityScale <= 1.0f);

	// 4. Poprawnosc wartosci LensFlare
	CHECK(kDefaultLensFlareMaxDistance == doctest::Approx(99999999.0f));
	CHECK(kDefaultFlareIntensityScale == doctest::Approx(0.85f));
	CHECK(EterLib::kDefaultLensFlareMaxDistance == doctest::Approx(99999999.0f));
	CHECK(EterLib::kDefaultFlareIntensityScale == doctest::Approx(0.85f));

	// 5. Zgodnosc z LensFlare.h
	CHECK(kLensFlareMaxDistance == doctest::Approx(kDefaultLensFlareMaxDistance));
	CHECK(kFlareIntensityScale == doctest::Approx(kDefaultFlareIntensityScale));
}

TEST_CASE("S8.6 - TextTail Visibility and Layout Constants")
{
	// 1. Poprawnosc typow
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kTextTailDefaultDistance)>>, "kTextTailDefaultDistance musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kTextTailFadeOutDistance)>>, "kTextTailFadeOutDistance musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kTextTailBaseHeightOffset)>>, "kTextTailBaseHeightOffset musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kTextTailNamePositionOffset)>>, "kTextTailNamePositionOffset musi byc float");

	// 2. Compile-time relacje
	static_assert(kTextTailDefaultDistance > 0.0f);
	static_assert(kTextTailFadeOutDistance > kTextTailDefaultDistance);
	static_assert((kTextTailFadeOutDistance - kTextTailDefaultDistance) == 1000.0f, "Strefa fade-out musi wynosic 1000 jednostek");
	static_assert(kTextTailBaseHeightOffset > 0.0f);
	static_assert(kTextTailNamePositionOffset < 0.0f);

	// 3. Poprawnosc numeryczna
	CHECK(kTextTailDefaultDistance == doctest::Approx(2500.0f));
	CHECK(kTextTailFadeOutDistance == doctest::Approx(3500.0f));
	CHECK(kTextTailBaseHeightOffset == doctest::Approx(10.0f));
	CHECK(kTextTailNamePositionOffset == doctest::Approx(-10.0f));

	// 4. Przestrzen nazw EterLib
	CHECK(EterLib::kTextTailDefaultDistance == doctest::Approx(2500.0f));
	CHECK(EterLib::kTextTailFadeOutDistance == doctest::Approx(3500.0f));
	CHECK(EterLib::kTextTailBaseHeightOffset == doctest::Approx(10.0f));
	CHECK(EterLib::kTextTailNamePositionOffset == doctest::Approx(-10.0f));

	// 5. Niezmiennik strefy zanikania alpha fade
	const float fadeSpan = kTextTailFadeOutDistance - kTextTailDefaultDistance;
	CHECK(fadeSpan == doctest::Approx(1000.0f));
}

TEST_CASE("S8.6 - WeaponTrace Trajectory Constants (Typy, Czas Trwania, Probkowanie)")
{
	// 1. Poprawnosc typow
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kWeaponTraceDefaultDuration)>>, "kWeaponTraceDefaultDuration musi byc float");
	static_assert(std::is_same_v<float, std::remove_cv_t<decltype(kWeaponTraceSamplingInterval)>>, "kWeaponTraceSamplingInterval musi byc float");

	// 2. Compile-time relacje
	static_assert(kWeaponTraceDefaultDuration > 0.0f, "Czas trwania smugi broni musi byc dodatni");
	static_assert(kWeaponTraceSamplingInterval > 0.0f, "Krok probkowania smugi musi byc dodatni");
	static_assert(kWeaponTraceSamplingInterval < kWeaponTraceDefaultDuration, "Krok probkowania musi byc mniejszy od calkowitego czasu");

	// 3. Poprawnosc wartosci numerycznych
	CHECK(kWeaponTraceDefaultDuration == doctest::Approx(0.18f));
	CHECK(kWeaponTraceSamplingInterval == doctest::Approx(0.003f));

	// 4. Przestrzen nazw EterLib
	CHECK(EterLib::kWeaponTraceDefaultDuration == doctest::Approx(0.18f));
	CHECK(EterLib::kWeaponTraceSamplingInterval == doctest::Approx(0.003f));

	// 5. Niezmiennik liczby probek interpolacji (dokladnie 60 probek dla standardowej konfiguracji)
	const float sampleCount = kWeaponTraceDefaultDuration / kWeaponTraceSamplingInterval;
	CHECK(sampleCount == doctest::Approx(60.0f));
	CHECK(std::abs(sampleCount - 60.0f) < 1e-4f);
}

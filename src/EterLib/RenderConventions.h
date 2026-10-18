#pragma once

#include <cstdint>

/**
 * Konwencje renderera silnika Metin2:
 *
 * 1. Uklad wspolrzednych:
 *    - Prawoskretny uklad wspolrzednych (RH - Right-Handed).
 *    - Os Z skierowana do gory (Z-up). Osie X i Y rozpinaja plaszczyzne pozioma podloza / mapy.
 *
 * 2. Algebra liniowa i transformacje:
 *    - Wektory wierszowe (row-vectors, v' = v * M).
 *    - Macierze w ukladzie wierszowym (row-major), zgodne z konwencja D3DX.
 *
 * 3. Reprezentacja katow:
 *    - Stopnie (degrees) w skryptach, plikach konfiguracyjnych i danych (.msa, .mse, Python).
 *    - Radiany (radians) w wewnetrznych obliczeniach silnika C++ oraz w potoku GPU.
 *
 * 4. Rzutowanie ortogonalne UI i przesuniecie polpiksela:
 *    - W potoku Direct3D 9 centrum piksela znajduje sie pod adresem (x + 0.5, y + 0.5).
 *    - W celu uzyskania idealnego mapowania teksel-do-piksela (1:1) bez artefaktow rozmycia
 *      przy rzutowaniu ortogonalnym 2D/UI stosuje sie przesuniecie wierzcholkow o -0.5f (kUiHalfPixelOffset).
 */

#include <cmath>
#include <d3d9.h>
#include <d3dx9.h>

constexpr float kUiHalfPixelOffset = 0.5f;
constexpr float kCloudFovRad = 1.04719755f; // 60 deg
constexpr float kCloudAspect = 1.3333333f; // 4:3 zachowane dla chmur

// -------------------------------------------------------------------------
// Stale odleglosci widzenia kamery (Camera)
// -------------------------------------------------------------------------
constexpr float kDefaultCameraMinDistance = 200.0f; // [cm] minimalny dopuszczalny dystans kamery TPP od celu
constexpr float kDefaultCameraMaxDistance = 2500.0f; // [cm] maksymalny domyslny dystans kamery TPP od celu

// -------------------------------------------------------------------------
// Domyslne stale cieniowania i culling (CullingManager)
// -------------------------------------------------------------------------
constexpr float kDefaultCullingRadius = 1600.0f; // [cm] domyslny bazowy promien sfery cullingu (leaf radius)
constexpr float kDefaultCullingLeafRadius = kDefaultCullingRadius; // [cm] alias dla liscia drzewa sfer
constexpr float kDefaultCullingRootRadius = 6400.0f; // [cm] promien sfery korzenia hierarchii drzewa cullingu
constexpr float kDefaultCullingExtraRadius = 400.0f; // [cm] margines tolerancji sfery cullingu
constexpr int   kDefaultCullingMaxSpheres = 10000; // [liczba] maksymalna pojemnosc fabryki sfer cullingu
constexpr float kDefaultCullingRayFarDistance = -1.0f; // [cm] domyslny brak ograniczenia zasiegu promienia (< 0: bez limitu)

// -------------------------------------------------------------------------
// Stale ekranowe i geometrii pomocniczej (GrpScreen)
// -------------------------------------------------------------------------
constexpr float kDefaultPickRayDistance = 51200.0f; // [cm] zasieg promienia testowego ray-picking terenu w CScreen
constexpr float kDefaultPlaneHalfExtent = 90000000.0f; // [cm] polowa rozpietosci pozornej plaszczyzny rzutowania kursora

// -------------------------------------------------------------------------
// Stale efektow optycznych slonca (LensFlare)
// -------------------------------------------------------------------------
constexpr float kDefaultLensFlareMaxDistance = 99999999.0f; // [cm] pozorny dystans zrodla swiatla slonecznego
constexpr float kDefaultFlareIntensityScale = 0.85f; // [wspolczynnik 0.0-1.0] skala tlumienia jasnosci przy okluzji

// -------------------------------------------------------------------------
// Stale dla etykiet tekstowych (PythonTextTail)
// -------------------------------------------------------------------------
constexpr float kTextTailDefaultDistance = 2500.0f; // [cm] dystans rozpoczecia zanikania (fade-out) etykiet TextTail
constexpr float kTextTailFadeOutDistance = 3500.0f; // [cm] maksymalny dystans widocznosci / calkowite odciecie TextTail
constexpr float kTextTailBaseHeightOffset = 10.0f; // [cm] domyslne bazowe przesuniecie wysokosci TextTail nad postacia
constexpr float kTextTailNamePositionOffset = -10.0f; // [cm] pionowe przesuniecie etykiety nazwy

// -------------------------------------------------------------------------
// Stale dla smugi miecza (WeaponTrace)
// -------------------------------------------------------------------------
constexpr float kWeaponTraceDefaultDuration = 0.18f; // [s] domyslny czas trwania / zaniku punktow smugi broni (m_fLifeTime)
constexpr float kWeaponTraceSamplingInterval = 0.003f; // [s] interwal czasowy probkowania punktow interpolacji splajnu (m_fSamplingTime)

inline constexpr uint32_t ToBgraDword(uint8_t r, uint8_t g, uint8_t b, uint8_t a = 255)
{
	return (static_cast<uint32_t>(a) << 24) |
	       (static_cast<uint32_t>(r) << 16) |
	       (static_cast<uint32_t>(g) << 8)  |
	       static_cast<uint32_t>(b);
}

[[nodiscard]] inline float ComputeWaterWaveHeight(float x, float y, float time, float freq, float amp) noexcept
{
	const float spatialPhase = 0.002f * x + 0.003f * y;
	const float harmonicFreq = (freq != 0.0f) ? (freq * 2.0666667f) : 3.1f;
	const float harmonicAmp = amp * 0.2f;

	return -6.0f + amp * std::sin(time * freq + spatialPhase)
	             + harmonicAmp * std::sin(time * harmonicFreq + spatialPhase * 1.5f);
}

inline float CalculateWaterWaveHeight(float x, float y, float fTime)
{
	return ComputeWaterWaveHeight(x, y, fTime, 1.5f, 5.0f);
}

inline void BuildWaterUVTransformMatrix(D3DXMATRIX* pOutMat, const D3DXMATRIX& matViewInverse, float fWaterTexCoordBase)
{
	if (!pOutMat)
		return;
	D3DXMatrixScaling(pOutMat, fWaterTexCoordBase, -fWaterTexCoordBase, 0.0f);
	D3DXMatrixMultiply(pOutMat, &matViewInverse, pOutMat);
}

inline uint32_t CalculateWaterAnimationFrameIndex(uint32_t dwTimeMs)
{
	return (dwTimeMs / 70) % 30;
}

namespace EterLib
{
	using ::kUiHalfPixelOffset;
	using ::kCloudFovRad;
	using ::kCloudAspect;
	using ::ToBgraDword;
	using ::CalculateWaterWaveHeight;
	using ::ComputeWaterWaveHeight;
	using ::BuildWaterUVTransformMatrix;
	using ::CalculateWaterAnimationFrameIndex;

	using ::kDefaultCameraMinDistance;
	using ::kDefaultCameraMaxDistance;
	using ::kDefaultCullingRadius;
	using ::kDefaultCullingLeafRadius;
	using ::kDefaultCullingRootRadius;
	using ::kDefaultCullingExtraRadius;
	using ::kDefaultCullingMaxSpheres;
	using ::kDefaultCullingRayFarDistance;
	using ::kDefaultPickRayDistance;
	using ::kDefaultPlaneHalfExtent;
	using ::kDefaultLensFlareMaxDistance;
	using ::kDefaultFlareIntensityScale;
	using ::kTextTailDefaultDistance;
	using ::kTextTailFadeOutDistance;
	using ::kTextTailBaseHeightOffset;
	using ::kTextTailNamePositionOffset;
	using ::kWeaponTraceDefaultDuration;
	using ::kWeaponTraceSamplingInterval;
}

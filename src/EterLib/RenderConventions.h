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

constexpr float kUiHalfPixelOffset = 0.5f;
constexpr float kCloudFovRad = 1.04719755f; // 60 deg
constexpr float kCloudAspect = 1.33333333f; // 4:3 zachowane dla chmur

inline uint32_t ToBgraDword(uint8_t r, uint8_t g, uint8_t b, uint8_t a = 255)
{
	return (static_cast<uint32_t>(a) << 24) |
	       (static_cast<uint32_t>(r) << 16) |
	       (static_cast<uint32_t>(g) << 8)  |
	       static_cast<uint32_t>(b);
}

namespace EterLib
{
	using ::kUiHalfPixelOffset;
	using ::kCloudFovRad;
	using ::kCloudAspect;
	using ::ToBgraDword;
}

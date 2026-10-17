#pragma once

#include <cmath>

inline float CalculateWaterWaveHeight(float /*x*/, float /*y*/, float fTime)
{
	return -6.0f + 5.0f * sinf(fTime * 1.5f) + 1.0f * sinf(fTime * 3.1f);
}

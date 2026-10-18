#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

// We will write a test for some math/logic in MapOutdoorWater.h / RenderConventions.h
#include "EterLib/RenderConventions.h"

TEST_CASE("MapOutdoor Audit Test")
{
    CHECK(CalculateWaterAnimationFrameIndex(0) == 0);
    CHECK(CalculateWaterAnimationFrameIndex(70) == 1);
    CHECK(CalculateWaterAnimationFrameIndex(70 * 30) == 0);
}

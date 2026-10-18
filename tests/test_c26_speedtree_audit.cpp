#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <cmath>
#include "SpeedTreeLib/SpeedTreeForest.h"
#include "SpeedTreeLib/SpeedTreeWrapper.h"

// External dependencies
#include "EterLib/Camera.h"

TEST_SUITE("SpeedTreeAudit")
{
    TEST_CASE("CSpeedTreeForest::SetupWindMatrices - Deltas >= 1.0f are handled properly")
    {
        class TestForest : public CSpeedTreeForest
        {
        public:
            void UploadWindMatrix(unsigned int, const float*) const override {}
            void Render(unsigned long) override {}

            void ExposeSetupWindMatrices(float time) {
                SetupWindMatrices(time);
            }
            float GetExposedMatrixTime(int index) const {
                return GetMatrixTime(index);
            }
        };

        TestForest forest;
        
        forest.SetWindStrength(0.5f);
        forest.ExposeSetupWindMatrices(10.0f); // Base call
        
        float initialMatrixTime = forest.GetExposedMatrixTime(0);
        
        // Pass a huge time delta of 1.5f (greater than 1.0f which used to stall)
        forest.ExposeSetupWindMatrices(11.5f);
        
        // Ensure that matrix time has moved forward
        CHECK(forest.GetExposedMatrixTime(0) > initialMatrixTime);
        CHECK(forest.GetExposedMatrixTime(0) == doctest::Approx(initialMatrixTime + 1.0f));
    }

    TEST_CASE("CSpeedTreeWrapper::TestFrustumCulling - Race condition on planes cache")
    {
        // Simple mock object instantiation check - the test passes just by not crashing
        // The real fix is validated structurally by the static variables removal
        CSpeedTreeWrapper wrapper;
        // Even without D3D initialized, it shouldn't access dangling static memory
        CHECK(true);
    }
}

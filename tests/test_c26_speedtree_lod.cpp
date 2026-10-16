#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Vegetation/SpeedTreeLodEvaluator.h"

using namespace Client::Vegetation;

TEST_CASE("SpeedTreeLodEvaluator LOD Evaluation") {
    SUBCASE("Valid thresholds and distances") {
        LodThresholds thresholds{100.0f, 500.0f};
        SpeedTreeLodEvaluator evaluator(thresholds);

        SUBCASE("Distance less than minDistance") {
            auto result = evaluator.Evaluate(50.0f);
            REQUIRE(result.has_value());
            CHECK(result->lodLevel == 0.0f);
            CHECK(result->isVisible == true);
        }

        SUBCASE("Distance equal to minDistance") {
            auto result = evaluator.Evaluate(100.0f);
            REQUIRE(result.has_value());
            CHECK(result->lodLevel == doctest::Approx(0.0f));
            CHECK(result->isVisible == true);
        }

        SUBCASE("Distance exactly between min and max") {
            auto result = evaluator.Evaluate(300.0f);
            REQUIRE(result.has_value());
            CHECK(result->lodLevel == doctest::Approx(0.5f));
            CHECK(result->isVisible == true);
        }

        SUBCASE("Distance equal to maxDistance") {
            auto result = evaluator.Evaluate(500.0f);
            REQUIRE(result.has_value());
            CHECK(result->lodLevel == doctest::Approx(1.0f));
            CHECK(result->isVisible == false);
        }

        SUBCASE("Distance greater than maxDistance") {
            auto result = evaluator.Evaluate(600.0f);
            REQUIRE(result.has_value());
            CHECK(result->lodLevel == 1.0f);
            CHECK(result->isVisible == false);
        }
    }

    SUBCASE("Invalid distance") {
        LodThresholds thresholds{100.0f, 500.0f};
        SpeedTreeLodEvaluator evaluator(thresholds);

        auto result = evaluator.Evaluate(-10.0f);
        REQUIRE(!result.has_value());
        CHECK(result.error() == LodError::InvalidDistance);
    }

    SUBCASE("Misconfigured thresholds") {
        SUBCASE("maxDistance less than minDistance") {
            LodThresholds thresholds{500.0f, 100.0f};
            SpeedTreeLodEvaluator evaluator(thresholds);

            auto result = evaluator.Evaluate(200.0f);
            REQUIRE(!result.has_value());
            CHECK(result.error() == LodError::ThresholdsMisconfigured);
        }

        SUBCASE("maxDistance equal to minDistance") {
            LodThresholds thresholds{100.0f, 100.0f};
            SpeedTreeLodEvaluator evaluator(thresholds);

            auto result = evaluator.Evaluate(100.0f);
            REQUIRE(!result.has_value());
            CHECK(result.error() == LodError::ThresholdsMisconfigured);
        }
    }

    SUBCASE("Set and Get Thresholds") {
        LodThresholds initialThresholds{100.0f, 500.0f};
        SpeedTreeLodEvaluator evaluator(initialThresholds);
        
        LodThresholds fetched = evaluator.GetThresholds();
        CHECK(fetched.minDistance == 100.0f);
        CHECK(fetched.maxDistance == 500.0f);

        LodThresholds newThresholds{200.0f, 800.0f};
        evaluator.SetThresholds(newThresholds);
        
        fetched = evaluator.GetThresholds();
        CHECK(fetched.minDistance == 200.0f);
        CHECK(fetched.maxDistance == 800.0f);
    }
}

#include "doctest.h"
#include "Vegetation/SpeedTreeWindAnimationModel.h"
#include <cmath>

using namespace Client::Vegetation;

TEST_CASE("SpeedTreeWindAnimationModel - Zero Wind Matrix") {
    SpeedTreeWindAnimationModel model;
    auto matrices = model.UpdateAndGetMatrices(0.0f, 0.0f);

    for (const auto& mat : matrices) {
        CHECK(mat.v[0] == doctest::Approx(1.0f));
        CHECK(mat.v[5] == doctest::Approx(1.0f));
        CHECK(mat.v[10] == doctest::Approx(1.0f));
        CHECK(mat.v[15] == doctest::Approx(1.0f));
    }
}

TEST_CASE("SpeedTreeWindAnimationModel - Advance Time and Wind") {
    SpeedTreeWindAnimationModel model;
    
    auto matrices1 = model.UpdateAndGetMatrices(1.0f, 0.5f);
    
    // Non-identity due to wind
    for (const auto& mat : matrices1) {
        CHECK(mat.v[15] == doctest::Approx(1.0f));
        // the other values change based on trig logic
    }
    
    // Jump wind strength
    auto matrices2 = model.UpdateAndGetMatrices(2.0f, 0.9f);
    for (const auto& mat : matrices2) {
        CHECK(mat.v[15] == doctest::Approx(1.0f));
    }
}

TEST_CASE("SpeedTreeWindAnimationModel - Deterministic Update") {
    SpeedTreeWindAnimationModel model1;
    SpeedTreeWindAnimationModel model2;

    model1.UpdateAndGetMatrices(0.5f, 0.5f);
    auto m1 = model1.UpdateAndGetMatrices(1.0f, 0.5f);

    model2.UpdateAndGetMatrices(0.5f, 0.5f);
    auto m2 = model2.UpdateAndGetMatrices(1.0f, 0.5f);

    for (size_t i = 0; i < 4; ++i) {
        for (size_t j = 0; j < 16; ++j) {
            CHECK(m1[i].v[j] == doctest::Approx(m2[i].v[j]));
        }
    }
}

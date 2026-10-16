#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/MiniMap/MiniMapAtlasScaler.h"

using namespace Client::MiniMap;

TEST_CASE("MiniMapAtlasScaler - Initialization") {
    MiniMapAtlasScaler scaler;

    SUBCASE("Default state is not initialized") {
        CHECK_FALSE(scaler.IsInitialized());
    }

    SUBCASE("Valid initialization succeeds") {
        auto result = scaler.Initialize(1024, 1024, 0.0f, 0.0f, 10000.0f, 10000.0f);
        CHECK(result.has_value());
        CHECK(scaler.IsInitialized());
        
        // Obliczana skala (1024 / 10000 = 0.1024)
        CHECK(scaler.GetScaleX() == doctest::Approx(0.1024f));
        CHECK(scaler.GetScaleY() == doctest::Approx(0.1024f));
    }

    SUBCASE("Zero dimensions returns InvalidDimensions") {
        auto result = scaler.Initialize(0, 1024, 0.0f, 0.0f, 10000.0f, 10000.0f);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ScalerError::InvalidDimensions);
        
        result = scaler.Initialize(1024, 0, 0.0f, 0.0f, 10000.0f, 10000.0f);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ScalerError::InvalidDimensions);
    }

    SUBCASE("Invalid bounds returns InvalidBounds") {
        auto result = scaler.Initialize(1024, 1024, 10000.0f, 0.0f, 0.0f, 10000.0f); // minX > maxX
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ScalerError::InvalidBounds);
        
        result = scaler.Initialize(1024, 1024, 0.0f, 10000.0f, 10000.0f, 0.0f); // minY > maxY
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ScalerError::InvalidBounds);
        
        result = scaler.Initialize(1024, 1024, 5000.0f, 5000.0f, 5000.0f, 5000.0f); // min == max
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ScalerError::InvalidBounds);
    }
}

TEST_CASE("MiniMapAtlasScaler - Coordinate Conversion (WorldToAtlas)") {
    MiniMapAtlasScaler scaler;
    
    SUBCASE("Fails when not initialized") {
        auto result = scaler.WorldToAtlas({5000.0f, 5000.0f});
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ScalerError::NotInitialized);
    }

    SUBCASE("Valid conversions") {
        REQUIRE(scaler.Initialize(100, 100, 0.0f, 0.0f, 1000.0f, 1000.0f).has_value()); // Skala 0.1

        auto res1 = scaler.WorldToAtlas({0.0f, 0.0f});
        REQUIRE(res1.has_value());
        CHECK(res1->x == 0);
        CHECK(res1->y == 0);

        auto res2 = scaler.WorldToAtlas({500.0f, 500.0f});
        REQUIRE(res2.has_value());
        CHECK(res2->x == 50);
        CHECK(res2->y == 50);

        // Krawedzie (powinny uciac do wewnatrz - index max = width - 1)
        auto res3 = scaler.WorldToAtlas({1000.0f, 1000.0f});
        REQUIRE(res3.has_value());
        CHECK(res3->x == 99);
        CHECK(res3->y == 99);
    }

    SUBCASE("Out of bounds WorldToAtlas") {
        REQUIRE(scaler.Initialize(100, 100, 0.0f, 0.0f, 1000.0f, 1000.0f).has_value());

        auto result1 = scaler.WorldToAtlas({-10.0f, 500.0f});
        CHECK_FALSE(result1.has_value());
        CHECK(result1.error() == ScalerError::OutOfBounds);
        
        auto result2 = scaler.WorldToAtlas({500.0f, 1010.0f});
        CHECK_FALSE(result2.has_value());
        CHECK(result2.error() == ScalerError::OutOfBounds);
    }
}

TEST_CASE("MiniMapAtlasScaler - Coordinate Conversion (AtlasToWorld)") {
    MiniMapAtlasScaler scaler;

    SUBCASE("Fails when not initialized") {
        auto result = scaler.AtlasToWorld({50, 50});
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == ScalerError::NotInitialized);
    }

    SUBCASE("Valid conversions") {
        REQUIRE(scaler.Initialize(100, 100, 0.0f, 0.0f, 1000.0f, 1000.0f).has_value()); // Skala 0.1

        auto res1 = scaler.AtlasToWorld({0, 0});
        REQUIRE(res1.has_value());
        CHECK(res1->x == doctest::Approx(0.0f));
        CHECK(res1->y == doctest::Approx(0.0f));

        auto res2 = scaler.AtlasToWorld({50, 50});
        REQUIRE(res2.has_value());
        CHECK(res2->x == doctest::Approx(500.0f));
        CHECK(res2->y == doctest::Approx(500.0f));
    }

    SUBCASE("Out of bounds AtlasToWorld") {
        REQUIRE(scaler.Initialize(100, 100, 0.0f, 0.0f, 1000.0f, 1000.0f).has_value());

        auto result1 = scaler.AtlasToWorld({100, 50});
        CHECK_FALSE(result1.has_value());
        CHECK(result1.error() == ScalerError::OutOfBounds);

        auto result2 = scaler.AtlasToWorld({50, 100});
        CHECK_FALSE(result2.has_value());
        CHECK(result2.error() == ScalerError::OutOfBounds);
    }
}

TEST_CASE("MiniMapAtlasScaler - Offset Map Coordinate Conversions") {
    MiniMapAtlasScaler scaler;
    
    // Mapa, ktora nie zaczyna sie w 0,0, np. -5000 do 5000
    REQUIRE(scaler.Initialize(200, 200, -5000.0f, -5000.0f, 5000.0f, 5000.0f).has_value()); 
    // Szerokosc/Wysokosc to 10000. Skala: 200 / 10000 = 0.02
    
    SUBCASE("WorldToAtlas mapping") {
        auto res1 = scaler.WorldToAtlas({-5000.0f, -5000.0f});
        REQUIRE(res1.has_value());
        CHECK(res1->x == 0);
        CHECK(res1->y == 0);

        auto res2 = scaler.WorldToAtlas({0.0f, 0.0f});
        REQUIRE(res2.has_value());
        CHECK(res2->x == 100);
        CHECK(res2->y == 100);
    }
    
    SUBCASE("AtlasToWorld mapping") {
        auto res1 = scaler.AtlasToWorld({0, 0});
        REQUIRE(res1.has_value());
        CHECK(res1->x == doctest::Approx(-5000.0f));
        CHECK(res1->y == doctest::Approx(-5000.0f));

        auto res2 = scaler.AtlasToWorld({100, 100});
        REQUIRE(res2.has_value());
        CHECK(res2->x == doctest::Approx(0.0f));
        CHECK(res2->y == doctest::Approx(0.0f));
    }
}

TEST_CASE("MiniMapAtlasScaler - to_string formats") {
    CHECK(std::string_view(to_string(ScalerError::None)) == "ScalerError::None");
    CHECK(std::string_view(to_string(ScalerError::InvalidDimensions)).find("Wymiary atlasu") != std::string_view::npos);
}

namespace doctest {
template <>
struct StringMaker<Client::MiniMap::ScalerError> {
    static String convert(const Client::MiniMap::ScalerError& value) {
        return String(std::string(Client::MiniMap::to_string(value)).c_str());
    }
};
}

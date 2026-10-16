#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>

#include "Client/Terrain/TerrainNormalMapGenerator.h"
#include <vector>
#include <cmath>
#include <cstdlib> // For malloc/free
#include <new>     // For std::bad_alloc

// Flaga do symulacji bledu alokacji pamieci
static bool g_SimulateBadAlloc = false;

// Nadpisanie globalnych operatorow new i delete w celu testowania wyciekow i bledow alokacji
void* operator new(std::size_t size) {
    if (g_SimulateBadAlloc) {
        throw std::bad_alloc{};
    }
    void* p = std::malloc(size);
    if (!p) {
        throw std::bad_alloc{};
    }
    return p;
}

void operator delete(void* p) noexcept {
    std::free(p);
}

void operator delete(void* p, std::size_t) noexcept {
    std::free(p);
}

namespace Client::Terrain::Tests {

    TEST_SUITE("TerrainNormalMapGenerator") {
        
        TEST_CASE("Initialize with valid flat terrain") {
            TerrainNormalMapGenerator generator;
            std::vector<float> heights = {
                0.0f, 0.0f, 0.0f,
                0.0f, 0.0f, 0.0f,
                0.0f, 0.0f, 0.0f
            };
            
            auto result = generator.Generate(3, 3, heights);
            
            REQUIRE(result.has_value());
            auto normals = result.value();
            REQUIRE(normals.size() == 9);
            
            // Plaski teren powinien miec wektor normalny (0, 0, 1) w kazdym punkcie
            for (const auto& normal : normals) {
                CHECK(normal.x == doctest::Approx(0.0f));
                CHECK(normal.y == doctest::Approx(0.0f));
                CHECK(normal.z == doctest::Approx(1.0f));
            }
        }

        TEST_CASE("Initialize with sloping terrain") {
            TerrainNormalMapGenerator generator;
            // Teren nachylony wzdluz osi X
            // H(x, y) = x
            std::vector<float> heights = {
                0.0f, 1.0f, 2.0f,
                0.0f, 1.0f, 2.0f,
                0.0f, 1.0f, 2.0f
            };
            
            auto result = generator.Generate(3, 3, heights);
            
            REQUIRE(result.has_value());
            auto normals = result.value();
            
            // dfdx = 1.0f, dfdy = 0.0f
            // n_unnorm = (-1, 0, 1)
            // length = sqrt(1^2 + 0^2 + 1^2) = sqrt(2)
            // n_norm = (-1/sqrt(2), 0, 1/sqrt(2))
            
            float expected_nx = -1.0f / std::sqrt(2.0f);
            float expected_nz = 1.0f / std::sqrt(2.0f);
            
            // Sprawdzamy srodkowy piksel (aby ominac obsluge krawedzi w teście)
            auto centerNormal = normals[1 * 3 + 1];
            CHECK(centerNormal.x == doctest::Approx(expected_nx));
            CHECK(centerNormal.y == doctest::Approx(0.0f));
            CHECK(centerNormal.z == doctest::Approx(expected_nz));
        }

        TEST_CASE("Error handling - InvalidGridSize") {
            TerrainNormalMapGenerator generator;
            std::vector<float> heights = { 0.0f };
            
            auto result = generator.Generate(1, 1, heights);
            
            REQUIRE_FALSE(result.has_value());
            CHECK(result.error() == NormalMapError::InvalidGridSize);
        }

        TEST_CASE("Error handling - InvalidSpanSize") {
            TerrainNormalMapGenerator generator;
            std::vector<float> heights = { 0.0f, 0.0f, 0.0f }; // Za mało danych dla 2x2
            
            auto result = generator.Generate(2, 2, heights);
            
            REQUIRE_FALSE(result.has_value());
            CHECK(result.error() == NormalMapError::InvalidSpanSize);
        }

        TEST_CASE("Error handling - AllocationFailed (Bad Alloc Simulation)") {
            TerrainNormalMapGenerator generator;
            std::vector<float> heights(100 * 100, 0.0f); // Używamy większej gridy, żeby wektor się zaalokował
            
            g_SimulateBadAlloc = true;
            auto result = generator.Generate(100, 100, heights);
            g_SimulateBadAlloc = false;
            
            REQUIRE_FALSE(result.has_value());
            CHECK(result.error() == NormalMapError::AllocationFailed);
        }
    }

} // namespace Client::Terrain::Tests

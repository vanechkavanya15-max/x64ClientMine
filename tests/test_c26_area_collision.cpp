#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include "../src/Client/World/AreaCollisionMeshIndex.h"

using namespace Client::World;

TEST_SUITE("AreaCollisionMeshIndex") {
    TEST_CASE("Puste indeksowanie i dodawanie pierwszej siatki") {
        AreaCollisionMeshIndex index(1024.0f);
        CHECK(index.Count() == 0);

        MeshId id1{1};
        AABB aabb1{{0.0f, 0.0f, 0.0f}, {100.0f, 100.0f, 100.0f}};

        auto result = index.Insert(id1, aabb1);
        CHECK(result.has_value());
        CHECK(index.Count() == 1);

        auto bounds = index.GetBounds(id1);
        REQUIRE(bounds.has_value());
        CHECK(bounds->min.x == 0.0f);
        CHECK(bounds->max.x == 100.0f);
    }

    TEST_CASE("Zapobieganie wstawianiu duplikatow i nieprawidlowych AABB") {
        AreaCollisionMeshIndex index;
        MeshId id1{1};
        AABB aabb1{{0.0f, 0.0f, 0.0f}, {10.0f, 10.0f, 10.0f}};

        index.Insert(id1, aabb1);
        
        SUBCASE("Duplikat ID") {
            auto result = index.Insert(id1, aabb1);
            CHECK_FALSE(result.has_value());
            CHECK(result.error() == "Siatka o podanym ID juz istnieje.");
        }

        SUBCASE("Nieprawidlowe AABB (min > max)") {
            MeshId id2{2};
            AABB invalidAabb{{100.0f, 100.0f, 100.0f}, {0.0f, 0.0f, 0.0f}};
            auto result = index.Insert(id2, invalidAabb);
            CHECK_FALSE(result.has_value());
            CHECK(result.error() == "Nieprawidlowe AABB przy probie wstawienia.");
        }
    }

    TEST_CASE("Usuwanie istniejacych i nieistniejacych siatek") {
        AreaCollisionMeshIndex index;
        MeshId id1{1};
        index.Insert(id1, AABB{{0.0f, 0.0f, 0.0f}, {10.0f, 10.0f, 10.0f}});

        SUBCASE("Sukces usuwania") {
            auto result = index.Remove(id1);
            CHECK(result.has_value());
            CHECK(index.Count() == 0);
            CHECK_FALSE(index.GetBounds(id1).has_value());
        }

        SUBCASE("Nieistniejaca siatka") {
            MeshId id2{2};
            auto result = index.Remove(id2);
            CHECK_FALSE(result.has_value());
            CHECK(result.error() == "Siatka o podanym ID nie istnieje.");
        }
    }

    TEST_CASE("Aktualizacja siatki miedzy komorkami i w tej samej komorce") {
        AreaCollisionMeshIndex index(100.0f);
        MeshId id1{1};
        index.Insert(id1, AABB{{10.0f, 10.0f, 0.0f}, {20.0f, 20.0f, 10.0f}});

        SUBCASE("Aktualizacja w tej samej komorce") {
            auto result = index.Update(id1, AABB{{15.0f, 15.0f, 0.0f}, {25.0f, 25.0f, 10.0f}});
            CHECK(result.has_value());
            auto bounds = index.GetBounds(id1);
            CHECK(bounds->min.x == 15.0f);
        }

        SUBCASE("Przeniesienie do innej komorki") {
            auto result = index.Update(id1, AABB{{150.0f, 150.0f, 0.0f}, {200.0f, 200.0f, 10.0f}});
            CHECK(result.has_value());
            
            // Sprawdzenie kwerenda czy siatka faktycznie zmienila lokacje
            auto queryOld = index.QueryIntersecting(AABB{{0.0f, 0.0f, 0.0f}, {50.0f, 50.0f, 50.0f}});
            CHECK(queryOld.empty());

            auto queryNew = index.QueryIntersecting(AABB{{100.0f, 100.0f, 0.0f}, {300.0f, 300.0f, 50.0f}});
            CHECK(queryNew.size() == 1);
            CHECK(queryNew[0] == id1);
        }
    }

    TEST_CASE("Kwerendy przecinajace siatki i filtrowanie zbednych") {
        AreaCollisionMeshIndex index(100.0f);

        MeshId id1{1}; // Mesh in (0,0)
        MeshId id2{2}; // Mesh spreading (0,0) to (1,1) cells
        MeshId id3{3}; // Mesh far away

        index.Insert(id1, AABB{{10.0f, 10.0f, 0.0f}, {20.0f, 20.0f, 10.0f}});
        index.Insert(id2, AABB{{80.0f, 80.0f, 0.0f}, {120.0f, 120.0f, 10.0f}});
        index.Insert(id3, AABB{{500.0f, 500.0f, 0.0f}, {600.0f, 600.0f, 10.0f}});

        SUBCASE("Kwerenda precyzyjna tylko id1") {
            auto result = index.QueryIntersecting(AABB{{0.0f, 0.0f, 0.0f}, {50.0f, 50.0f, 50.0f}});
            REQUIRE(result.size() == 1);
            CHECK(result[0] == id1);
        }

        SUBCASE("Kwerenda duza id1 i id2") {
            auto result = index.QueryIntersecting(AABB{{0.0f, 0.0f, 0.0f}, {150.0f, 150.0f, 50.0f}});
            REQUIRE(result.size() == 2);
            bool hasId1 = (result[0] == id1 || result[1] == id1);
            bool hasId2 = (result[0] == id2 || result[1] == id2);
            CHECK(hasId1);
            CHECK(hasId2);
        }

        SUBCASE("Kwerenda nie obejmujaca niczego pomimo dotykania komorki id1") {
            auto result = index.QueryIntersecting(AABB{{30.0f, 30.0f, 0.0f}, {50.0f, 50.0f, 10.0f}});
            CHECK(result.empty());
        }

        SUBCASE("Duza kwerenda typu O(N) fallback") {
            // Wymusza przeliczenie O(N) skanujac duzy obszar komorek
            auto result = index.QueryIntersecting(AABB{{-1000.0f, -1000.0f, -1000.0f}, {1000.0f, 1000.0f, 1000.0f}});
            REQUIRE(result.size() == 3);
        }
    }
}

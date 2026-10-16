#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/IPC/HarnessWorldQueryRouter.h"
#include <rapidjson/document.h>

using namespace Client::IPC;

TEST_CASE("HarnessWorldQueryRouter: Legacy World snapshot JSON serialization") {
    HarnessWorldQueryRouter router;
    router.AddEntity({1001, 101, 50.0f, 60.0f, 500, 500, EntityType::NPC, "Guard", "IDLE"});
    router.AddEntity({1002, 8001, 70.0f, 80.0f, 10000, 10000, EntityType::Monster, "Orc", "COMBAT"});

    std::string json = router.SerializeWorldSnapshot(100);
    CHECK(json.find("\"player_vid\":100") != std::string::npos);
    CHECK(json.find("\"entity_count\":2") != std::string::npos);
    CHECK(json.find("\"vid\":1001") != std::string::npos);
}

TEST_CASE("HarnessWorldQueryRouter: GetState") {
    HarnessWorldQueryRouter router;

    SUBCASE("No player state") {
        auto res = router.GetState();
        CHECK(!res.has_value());
        CHECK(res.error() == QueryError::PlayerNotFound);
    }

    SUBCASE("With player state") {
        PlayerState state{100, 500, 1000, 10.0f, 20.0f, 50000, 30, "Hero"};
        router.SetPlayerState(state);

        auto res = router.GetState();
        REQUIRE(res.has_value());

        rapidjson::Document doc;
        doc.Parse(res->c_str());
        REQUIRE(!doc.HasParseError());

        CHECK(doc["vid"].GetUint() == 100);
        CHECK(std::string(doc["name"].GetString()) == "Hero");
        CHECK(doc["hp"].GetInt64() == 500);
        CHECK(doc["x"].GetDouble() == doctest::Approx(10.0));
        CHECK(doc["y"].GetDouble() == doctest::Approx(20.0));
    }
}

TEST_CASE("HarnessWorldQueryRouter: GetInventory") {
    HarnessWorldQueryRouter router;

    SUBCASE("Empty inventory") {
        auto res = router.GetInventory();
        CHECK(!res.has_value());
        CHECK(res.error() == QueryError::InventoryEmpty);
    }

    SUBCASE("With items") {
        router.AddItem({10, 0, 200});
        router.AddItem({20, 1, 1});

        auto res = router.GetInventory();
        REQUIRE(res.has_value());

        rapidjson::Document doc;
        doc.Parse(res->c_str());
        REQUIRE(!doc.HasParseError());
        REQUIRE(doc.IsArray());
        REQUIRE(doc.Size() == 2u);

        CHECK(doc[0]["vnum"].GetUint() == 10);
        CHECK(doc[0]["slot"].GetUint() == 0);
        CHECK(doc[0]["count"].GetUint() == 200);

        CHECK(doc[1]["vnum"].GetUint() == 20);
    }
}

TEST_CASE("HarnessWorldQueryRouter: GetNearbyEntities") {
    HarnessWorldQueryRouter router;

    SUBCASE("No player state") {
        auto res = router.GetNearbyEntities(100.0f);
        CHECK(!res.has_value());
        CHECK(res.error() == QueryError::PlayerNotFound);
    }

    SUBCASE("With player state and entities") {
        PlayerState state{100, 500, 1000, 0.0f, 0.0f, 50000, 30, "Hero"};
        router.SetPlayerState(state);

        router.AddEntity({1, 101, 10.0f, 0.0f, 100, 100, EntityType::NPC, "N1", "IDLE"});
        router.AddEntity({2, 102, 100.0f, 0.0f, 100, 100, EntityType::NPC, "N2", "IDLE"});
        router.AddEntity({3, 103, 5.0f, 0.0f, 100, 100, EntityType::NPC, "N3", "IDLE"});

        auto res = router.GetNearbyEntities(50.0f);
        REQUIRE(res.has_value());

        rapidjson::Document doc;
        doc.Parse(res->c_str());
        REQUIRE(!doc.HasParseError());
        REQUIRE(doc.IsArray());
        REQUIRE(doc.Size() == 2u);

        // Should be sorted by distance
        CHECK(doc[0]["vid"].GetUint() == 3); // distance 5
        CHECK(doc[1]["vid"].GetUint() == 1); // distance 10
    }
}

TEST_CASE("HarnessWorldQueryRouter: WorldMegaSnapshot") {
    HarnessWorldQueryRouter router;
    
    PlayerState state{100, 500, 1000, 10.0f, 20.0f, 50000, 30, "Hero"};
    router.SetPlayerState(state);
    
    router.AddEntity({1, 101, 10.0f, 0.0f, 100, 100, EntityType::NPC, "N1", "IDLE"});
    router.AddItem({10, 0, 200});

    std::string json = router.WorldMegaSnapshot();
    rapidjson::Document doc;
    doc.Parse(json.c_str());
    REQUIRE(!doc.HasParseError());

    REQUIRE(doc.HasMember("player"));
    CHECK(doc["player"]["vid"].GetUint() == 100);

    REQUIRE(doc.HasMember("entities"));
    REQUIRE(doc["entities"].IsArray());
    CHECK(doc["entities"].Size() == 1u);

    REQUIRE(doc.HasMember("inventory"));
    REQUIRE(doc["inventory"].IsArray());
    CHECK(doc["inventory"].Size() == 1u);
}

TEST_CASE("HarnessWorldQueryRouter: AuditState") {
    HarnessWorldQueryRouter router;
    router.RecordAuditLog("LOGIN", "Player logged in");
    router.RecordAuditLog("ITEM_DROP", "Dropped sword");

    std::string json = router.AuditState();
    rapidjson::Document doc;
    doc.Parse(json.c_str());
    REQUIRE(!doc.HasParseError());
    REQUIRE(doc.IsArray());
    REQUIRE(doc.Size() == 2u);

    CHECK(std::string(doc[0]["event_type"].GetString()) == "LOGIN");
    CHECK(std::string(doc[1]["event_type"].GetString()) == "ITEM_DROP");
}

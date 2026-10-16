#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Events/EventStateFlagStorage.h"

using namespace Client::Events;

TEST_CASE("EventStateFlagStorage - Basic operations") {
    EventStateFlagStorage storage;

    SUBCASE("Setting and getting a flag") {
        storage.SetFlag("WINTER_EVENT", 1);
        auto result = storage.GetFlag("WINTER_EVENT");
        REQUIRE(result.has_value());
        CHECK(result.value() == 1);
    }

    SUBCASE("Getting a non-existent flag returns NotFound") {
        auto result = storage.GetFlag("NON_EXISTENT");
        REQUIRE(!result.has_value());
        CHECK(result.error() == EventStateError::NotFound);
    }

    SUBCASE("Getting a flag with empty key returns InvalidKey") {
        auto result = storage.GetFlag("");
        REQUIRE(!result.has_value());
        CHECK(result.error() == EventStateError::InvalidKey);
    }

    SUBCASE("Updating an existing flag") {
        storage.SetFlag("HALLOWEEN", 5);
        storage.SetFlag("HALLOWEEN", 10);
        auto result = storage.GetFlag("HALLOWEEN");
        REQUIRE(result.has_value());
        CHECK(result.value() == 10);
    }

    SUBCASE("Clearing a flag") {
        storage.SetFlag("XMAS", 2);
        storage.ClearFlag("XMAS");
        auto result = storage.GetFlag("XMAS");
        REQUIRE(!result.has_value());
        CHECK(result.error() == EventStateError::NotFound);
    }

    SUBCASE("Clearing all flags") {
        storage.SetFlag("EVENT_1", 1);
        storage.SetFlag("EVENT_2", 2);
        storage.ClearAll();
        
        auto result1 = storage.GetFlag("EVENT_1");
        REQUIRE(!result1.has_value());
        CHECK(result1.error() == EventStateError::NotFound);
        
        auto result2 = storage.GetFlag("EVENT_2");
        REQUIRE(!result2.has_value());
        CHECK(result2.error() == EventStateError::NotFound);
    }
}

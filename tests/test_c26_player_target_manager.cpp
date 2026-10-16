#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Player/PlayerTargetManager.h"
#include "Core/StrongTypes.h"
#include "Core/ExpectedResult.h"

using namespace Client::Player;
using namespace Client::Core;

TEST_SUITE("PlayerTargetManager") {
    TEST_CASE("SetTarget sets a valid target") {
        PlayerTargetManager manager;
        EntityVid targetVid{123};

        auto result = manager.SetTarget(targetVid);
        CHECK(result.has_value());

        auto currentTarget = manager.GetTarget();
        REQUIRE(currentTarget.has_value());
        CHECK(currentTarget.value() == targetVid);
    }

    TEST_CASE("SetTarget with invalid entity returns InvalidEntity error") {
        PlayerTargetManager manager;
        EntityVid invalidVid{0};

        auto result = manager.SetTarget(invalidVid);
        REQUIRE(!result.has_value());
        CHECK(result.error() == TargetError::InvalidEntity);

        CHECK(!manager.GetTarget().has_value());
    }

    TEST_CASE("SetTarget to the same entity returns TargetAlreadySetToSameEntity error") {
        PlayerTargetManager manager;
        EntityVid targetVid{123};

        manager.SetTarget(targetVid);
        auto result = manager.SetTarget(targetVid);

        REQUIRE(!result.has_value());
        CHECK(result.error() == TargetError::TargetAlreadySetToSameEntity);
    }

    TEST_CASE("ClearTarget clears an existing target") {
        PlayerTargetManager manager;
        EntityVid targetVid{123};

        manager.SetTarget(targetVid);
        auto result = manager.ClearTarget();

        CHECK(result.has_value());
        CHECK(!manager.GetTarget().has_value());
    }

    TEST_CASE("ClearTarget without an existing target returns NoTargetSet error") {
        PlayerTargetManager manager;

        auto result = manager.ClearTarget();

        REQUIRE(!result.has_value());
        CHECK(result.error() == TargetError::NoTargetSet);
    }

    TEST_CASE("OnEntityDespawned clears target if matching") {
        PlayerTargetManager manager;
        EntityVid targetVid{123};

        manager.SetTarget(targetVid);
        manager.OnEntityDespawned(targetVid);

        CHECK(!manager.GetTarget().has_value());
    }

    TEST_CASE("OnEntityDespawned does not clear target if not matching") {
        PlayerTargetManager manager;
        EntityVid targetVid{123};
        EntityVid otherVid{456};

        manager.SetTarget(targetVid);
        manager.OnEntityDespawned(otherVid);

        auto currentTarget = manager.GetTarget();
        REQUIRE(currentTarget.has_value());
        CHECK(currentTarget.value() == targetVid);
    }

    TEST_CASE("OnEntityDied clears target if matching") {
        PlayerTargetManager manager;
        EntityVid targetVid{123};

        manager.SetTarget(targetVid);
        manager.OnEntityDied(targetVid);

        CHECK(!manager.GetTarget().has_value());
    }

    TEST_CASE("OnEntityDied does not clear target if not matching") {
        PlayerTargetManager manager;
        EntityVid targetVid{123};
        EntityVid otherVid{456};

        manager.SetTarget(targetVid);
        manager.OnEntityDied(otherVid);

        auto currentTarget = manager.GetTarget();
        REQUIRE(currentTarget.has_value());
        CHECK(currentTarget.value() == targetVid);
    }
}

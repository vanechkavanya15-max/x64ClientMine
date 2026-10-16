#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Actors/ActorMountComponent.h"
#include "../src/Client/Core/StrongTypes.h"

using namespace Client::Actors;
using namespace Client::Core;

TEST_CASE("ActorMountComponent - Mount and Unmount Logic") {
    ActorMountComponent component;

    SUBCASE("Initial state is not mounted") {
        CHECK(!component.IsMounted());
        CHECK(!component.GetMountVnum().has_value());
    }

    SUBCASE("Successful mount") {
        ItemVnum vnum(71114); // Example mount VNUM
        auto result = component.Mount(vnum);
        CHECK(result.has_value());
        CHECK(component.IsMounted());
        REQUIRE(component.GetMountVnum().has_value());
        CHECK(component.GetMountVnum().value() == vnum);
    }

    SUBCASE("Mounting when already mounted returns error") {
        ItemVnum vnum1(71114);
        ItemVnum vnum2(71115);
        
        auto result1 = component.Mount(vnum1);
        CHECK(result1.has_value());
        
        auto result2 = component.Mount(vnum2);
        CHECK(!result2.has_value());
        CHECK(result2.error() == MountError::AlreadyMounted);
        
        // Ensure the original mount is kept
        REQUIRE(component.GetMountVnum().has_value());
        CHECK(component.GetMountVnum().value() == vnum1);
    }

    SUBCASE("Mounting with invalid VNUM returns error") {
        ItemVnum vnum(0);
        auto result = component.Mount(vnum);
        CHECK(!result.has_value());
        CHECK(result.error() == MountError::InvalidVnum);
        CHECK(!component.IsMounted());
    }

    SUBCASE("Successful unmount") {
        ItemVnum vnum(71114);
        component.Mount(vnum);
        
        auto result = component.Unmount();
        CHECK(result.has_value());
        CHECK(!component.IsMounted());
        CHECK(!component.GetMountVnum().has_value());
    }

    SUBCASE("Unmounting when not mounted returns error") {
        auto result = component.Unmount();
        CHECK(!result.has_value());
        CHECK(result.error() == MountError::NotMounted);
    }
}

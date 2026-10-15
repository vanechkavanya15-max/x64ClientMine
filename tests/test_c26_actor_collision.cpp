#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/ActorCollisionComponent.h"
#include "Client/Core/StrongTypes.h"

using namespace Client::Actor;
using namespace Client::Core;

TEST_CASE("ActorCollisionComponent - Initialize and Error Handling") {
    ActorCollisionComponent component;

    SUBCASE("Valid initialization") {
        auto result = component.Initialize(50.0f, 150.0f);
        CHECK(result.has_value());
        CHECK(component.IsInitialized());
        CHECK(component.GetRadius() == 50.0f);
        CHECK(component.GetHeight() == 150.0f);
    }

    SUBCASE("Invalid radius") {
        auto result = component.Initialize(-10.0f, 150.0f);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == CollisionError::InvalidRadius);
        CHECK_FALSE(component.IsInitialized());
    }

    SUBCASE("Invalid height") {
        auto result = component.Initialize(50.0f, 0.0f);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == CollisionError::InvalidHeight);
        CHECK_FALSE(component.IsInitialized());
    }
}

TEST_CASE("ActorCollisionComponent - ContainsPoint") {
    ActorCollisionComponent component;
    REQUIRE(component.Initialize(100.0f, 200.0f).has_value());
    component.SetCenter({10.0f, 10.0f, 0.0f});

    SUBCASE("Point inside cylinder") {
        CHECK(component.ContainsPoint({15.0f, 15.0f, 100.0f}));
        CHECK(component.ContainsPoint({10.0f, 10.0f, 0.0f})); // Srodek bazy
        CHECK(component.ContainsPoint({10.0f, 10.0f, 200.0f})); // Srodek gory
    }

    SUBCASE("Point outside radius") {
        CHECK_FALSE(component.ContainsPoint({150.0f, 10.0f, 100.0f}));
    }

    SUBCASE("Point outside height (below)") {
        CHECK_FALSE(component.ContainsPoint({15.0f, 15.0f, -10.0f}));
    }

    SUBCASE("Point outside height (above)") {
        CHECK_FALSE(component.ContainsPoint({15.0f, 15.0f, 250.0f}));
    }

    SUBCASE("Not initialized") {
        ActorCollisionComponent uninitializedComponent;
        CHECK_FALSE(uninitializedComponent.ContainsPoint({15.0f, 15.0f, 100.0f}));
    }
}

TEST_CASE("ActorCollisionComponent - Intersects") {
    ActorCollisionComponent comp1;
    REQUIRE(comp1.Initialize(50.0f, 100.0f).has_value());
    comp1.SetCenter({0.0f, 0.0f, 0.0f});

    ActorCollisionComponent comp2;
    REQUIRE(comp2.Initialize(50.0f, 100.0f).has_value());

    SUBCASE("Intersecting cylinders (same center)") {
        comp2.SetCenter({0.0f, 0.0f, 0.0f});
        CHECK(comp1.Intersects(comp2));
        CHECK(comp2.Intersects(comp1));
    }

    SUBCASE("Intersecting cylinders (touching radiuses)") {
        comp2.SetCenter({100.0f, 0.0f, 0.0f});
        CHECK(comp1.Intersects(comp2));
        CHECK(comp2.Intersects(comp1));
    }

    SUBCASE("Intersecting cylinders (overlapping)") {
        comp2.SetCenter({80.0f, 0.0f, 50.0f});
        CHECK(comp1.Intersects(comp2));
        CHECK(comp2.Intersects(comp1));
    }

    SUBCASE("Non-intersecting cylinders (separated by X)") {
        comp2.SetCenter({150.0f, 0.0f, 0.0f});
        CHECK_FALSE(comp1.Intersects(comp2));
        CHECK_FALSE(comp2.Intersects(comp1));
    }

    SUBCASE("Non-intersecting cylinders (separated by Z above)") {
        comp2.SetCenter({0.0f, 0.0f, 150.0f});
        CHECK_FALSE(comp1.Intersects(comp2));
        CHECK_FALSE(comp2.Intersects(comp1));
    }

    SUBCASE("Non-intersecting cylinders (separated by Z below)") {
        comp2.SetCenter({0.0f, 0.0f, -150.0f});
        CHECK_FALSE(comp1.Intersects(comp2));
        CHECK_FALSE(comp2.Intersects(comp1));
    }
    
    SUBCASE("Not initialized") {
        ActorCollisionComponent uninitializedComponent;
        CHECK_FALSE(comp1.Intersects(uninitializedComponent));
        CHECK_FALSE(uninitializedComponent.Intersects(comp1));
    }
}

TEST_CASE("ActorCollisionComponent - ToString") {
    CHECK(ToString(CollisionError::None) == "None");
    CHECK(ToString(CollisionError::InvalidRadius) == "InvalidRadius");
    CHECK(ToString(CollisionError::InvalidHeight) == "InvalidHeight");
    CHECK(ToString(CollisionError::NoCollision) == "NoCollision");
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerEntityPicker.h"

using namespace Client::Player;
using namespace Client::Core;

namespace doctest {
    template<> struct StringMaker<std::expected<float, PickingError>> {
        static String convert(const std::expected<float, PickingError>& value) {
            if (value) {
                return String("Expected(") + toString(value.value()) + ")";
            } else {
                return String("Unexpected");
            }
        }
    };
    template<> struct StringMaker<EntityVid> {
        static String convert(const EntityVid& value) {
            return toString(value.get());
        }
    };
    template<> struct StringMaker<std::optional<EntityVid>> {
        static String convert(const std::optional<EntityVid>& value) {
            if (value) {
                return String("Optional(") + toString(value->get()) + ")";
            } else {
                return String("nullopt");
            }
        }
    };
}

TEST_CASE("PlayerEntityPicker: Pick actor and item directly") {
    PlayerEntityPicker picker;
    CHECK_FALSE(picker.GetPickedActor().has_value());
    picker.SetPickedActor(EntityVid{1234});
    CHECK(picker.GetPickedActor() == EntityVid{1234});
    picker.ClearPickedActor();
    CHECK_FALSE(picker.GetPickedActor().has_value());
}

TEST_CASE("PlayerEntityPicker Math: IntersectRaySphere") {
    Ray ray{{0.0f, 0.0f, 0.0f}, {1.0f, 0.0f, 0.0f}};
    
    SUBCASE("Hit direct") {
        BoundingSphere sphere{{5.0f, 0.0f, 0.0f}, 2.0f};
        auto result = PlayerEntityPicker::IntersectRaySphere(ray, sphere);
        REQUIRE(result.has_value());
        CHECK(result.value() == doctest::Approx(3.0f));
    }
    
    SUBCASE("Miss") {
        BoundingSphere sphere{{5.0f, 5.0f, 0.0f}, 2.0f};
        auto result = PlayerEntityPicker::IntersectRaySphere(ray, sphere);
        CHECK_FALSE(result.has_value());
    }

    SUBCASE("Start inside") {
        BoundingSphere sphere{{0.0f, 0.0f, 0.0f}, 5.0f};
        auto result = PlayerEntityPicker::IntersectRaySphere(ray, sphere);
        REQUIRE(result.has_value());
        CHECK(result.value() == doctest::Approx(0.0f));
    }
}

TEST_CASE("PlayerEntityPicker Math: IntersectRayAABB") {
    Ray ray{{0.0f, 0.0f, 0.0f}, {1.0f, 0.0f, 0.0f}};
    
    SUBCASE("Hit direct") {
        AABB aabb{{2.0f, -1.0f, -1.0f}, {4.0f, 1.0f, 1.0f}};
        auto result = PlayerEntityPicker::IntersectRayAABB(ray, aabb);
        REQUIRE(result.has_value());
        CHECK(result.value() == doctest::Approx(2.0f));
    }
    
    SUBCASE("Miss") {
        AABB aabb{{2.0f, 2.0f, -1.0f}, {4.0f, 4.0f, 1.0f}};
        auto result = PlayerEntityPicker::IntersectRayAABB(ray, aabb);
        CHECK_FALSE(result.has_value());
    }
}

TEST_CASE("PlayerEntityPicker: UpdatePicking logic") {
    PlayerEntityPicker picker;

    PickableActor actor1{EntityVid{1}, {10.0f, 0.0f, 0.0f}, BoundingCapsule{{10.0f, 0.0f, -5.0f}, {10.0f, 0.0f, 5.0f}, 2.0f}, false};
    PickableActor actor2{EntityVid{2}, {20.0f, 0.0f, 0.0f}, BoundingCapsule{{20.0f, 0.0f, -5.0f}, {20.0f, 0.0f, 5.0f}, 2.0f}, false};
    PickableItem item1{EntityVid{101}, {15.0f, 0.0f, 0.0f}, BoundingSphere{{15.0f, 0.0f, 0.0f}, 1.0f}};
    
    picker.RegisterActor(actor1);
    picker.RegisterActor(actor2);
    picker.RegisterItem(item1);

    Ray viewRay{{0.0f, 0.0f, 0.0f}, {1.0f, 0.0f, 0.0f}};

    SUBCASE("Picks closest actor") {
        picker.UpdatePicking(viewRay, std::nullopt);
        REQUIRE(picker.GetPickedActor().has_value());
        CHECK(picker.GetPickedActor().value() == EntityVid{1});
        CHECK_FALSE(picker.GetPickedItem().has_value());
    }

    SUBCASE("Actor unregister") {
        picker.UnregisterActor(EntityVid{1});
        picker.UpdatePicking(viewRay, std::nullopt);
        REQUIRE(picker.GetPickedActor().has_value());
        CHECK(picker.GetPickedActor().value() == EntityVid{2});
    }

    SUBCASE("Picks item if much closer") {
        PickableItem item_close{EntityVid{102}, {1.0f, 0.0f, 0.0f}, BoundingSphere{{1.0f, 0.0f, 0.0f}, 1.0f}};
        picker.RegisterItem(item_close);
        
        // actor 1 is at dist ~8, item_close is at dist ~0
        picker.UpdatePicking(viewRay, std::nullopt);
        
        // Since closestItemDist (0) < closestActorDist (8) - 100 is FALSE (-100), actually actor has priority even if item is closer in our current formula.
        // Wait, the formula is: if (closestItemDist < closestActorDist - 100.0f) SetPickedItem else SetPickedActor
        // If closestItemDist=0, closestActorDist=8, 0 < 8-100 is false. So Actor wins.
        REQUIRE(picker.GetPickedActor().has_value());
        CHECK(picker.GetPickedActor().value() == EntityVid{1});
    }

    SUBCASE("Picks item if actor is far") {
        picker.UnregisterActor(EntityVid{1});
        // Now closest actor is at 18
        // Let's add an item at 100 and actor at 250
        PickableItem item_far{EntityVid{103}, {100.0f, 0.0f, 0.0f}, BoundingSphere{{100.0f, 0.0f, 0.0f}, 1.0f}};
        PickableActor actor_far{EntityVid{3}, {250.0f, 0.0f, 0.0f}, BoundingCapsule{{250.0f, 0.0f, -5.0f}, {250.0f, 0.0f, 5.0f}, 2.0f}, false};
        
        picker.ClearAllRegistries();
        picker.RegisterItem(item_far);
        picker.RegisterActor(actor_far);

        // closestItemDist = 99, closestActorDist = 248
        // 99 < 248 - 100  ==> 99 < 148 is TRUE
        picker.UpdatePicking(viewRay, std::nullopt);
        REQUIRE(picker.GetPickedItem().has_value());
        CHECK(picker.GetPickedItem().value() == EntityVid{103});
        CHECK_FALSE(picker.GetPickedActor().has_value());
    }
}

#include "doctest.h"
#include "Client/Actors/ActorInteractionResolver.h"
#include "Client/World/ActorRegistry.h"
#include "Client/Actors/ActorCollisionComponent.h"
#include "Client/Actors/ActorTypeClassifier.h"

using namespace Client::Actors;
using namespace Client::World;
using namespace Client::Actor;

TEST_CASE("ActorInteractionResolver - C++23 Headless Physics and Interaction replacing CPythonCharacterManager") {
    ActorRegistry registry;
    ActorInteractionResolver resolver(registry);

    // Setup local player collision
    ActorCollisionComponent localCollision;
    auto initResult = localCollision.Initialize(50.0f, 150.0f);
    REQUIRE(initResult.has_value());
    localCollision.SetCenter({100.0f, 100.0f, 0.0f});

    EntityVid localVid{1};

    SUBCASE("CheckAdvancingCollision - no collision with empty registry") {
        auto result = resolver.CheckAdvancingCollision(localVid, localCollision);
        REQUIRE(result.has_value());
        CHECK_FALSE(result.value());
    }

    SUBCASE("CheckAdvancingCollision - blocked by door") {
        // Dodajemy drzwi (13000 to zazwyczaj drzwi/brama) do rejestru
        ActorRecord doorRecord{
            .vid = EntityVid{2},
            .race = 13000, // zakladamy ze ActorTypeClassifier klasyfikuje to jako IsDoor
            .type = 0,
            .x = 120.0f,
            .y = 120.0f,
            .z = 0.0f,
            .rotation = 0.0f,
            .name = "Door",
            .guildId = 0,
            .empire = 0,
            .isDead = false
        };
        registry.RegisterActor(doorRecord);

        // Z racji bliskosci, powinno zablokowac
        auto result = resolver.CheckAdvancingCollision(localVid, localCollision);
        REQUIRE(result.has_value());
        CHECK(result.value() == true);
    }

    SUBCASE("ProcessAttack - finds nearest valid target") {
        ActorRecord enemy1{
            .vid = EntityVid{2},
            .race = 101, // Wild Dog
            .type = 0,
            .x = 180.0f,
            .y = 180.0f,
            .z = 0.0f,
            .rotation = 0.0f,
            .name = "Dog 1",
            .guildId = 0,
            .empire = 0,
            .isDead = false
        };
        registry.RegisterActor(enemy1);

        ActorRecord enemy2{
            .vid = EntityVid{3},
            .race = 101, // Wild Dog
            .type = 0,
            .x = 120.0f, // Blizszy wrog
            .y = 120.0f,
            .z = 0.0f,
            .rotation = 0.0f,
            .name = "Dog 2",
            .guildId = 0,
            .empire = 0,
            .isDead = false
        };
        registry.RegisterActor(enemy2);

        // Skonfiguruj zasieg broni / atakujacy obszar
        ActorCollisionComponent weaponCollision;
        auto weaponInit = weaponCollision.Initialize(200.0f, 150.0f);
        REQUIRE(weaponInit.has_value());
        weaponCollision.SetCenter({100.0f, 100.0f, 0.0f});

        auto result = resolver.ProcessAttack(localVid, weaponCollision);
        REQUIRE(result.has_value());
        CHECK(result.value().value() == 3); // Zwraca najblizszy cel
    }

    SUBCASE("ProcessAttack - ignores dead targets") {
        ActorRecord deadEnemy{
            .vid = EntityVid{2},
            .race = 101,
            .type = 0,
            .x = 110.0f,
            .y = 110.0f,
            .z = 0.0f,
            .rotation = 0.0f,
            .name = "Dead Dog",
            .guildId = 0,
            .empire = 0,
            .isDead = true
        };
        registry.RegisterActor(deadEnemy);

        auto result = resolver.ProcessAttack(localVid, localCollision);
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == InteractionError::TargetNotFound);
    }
}

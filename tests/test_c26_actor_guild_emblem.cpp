#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Actors/ActorGuildEmblemComponent.h"
#include "../src/EterBase/EventBus.h"

using namespace Client::Actors;
using namespace EterBase;

TEST_CASE("ActorGuildEmblemComponent - Initialization") {
    EventBus& eventBus = EventBus::GetInstance();
    ActorGuildEmblemComponent component;

    SUBCASE("Successful Initialization") {
        auto result = component.Initialize(EntityId{1}, &eventBus);
        CHECK(result.has_value());
    }

    SUBCASE("Initialization twice fails") {
        component.Initialize(EntityId{1}, &eventBus);
        auto result = component.Initialize(EntityId{1}, &eventBus);
        CHECK(!result.has_value());
        CHECK(result.error() == GuildEmblemError::AlreadyInitialized);
    }
}

TEST_CASE("ActorGuildEmblemComponent - GetGuildId before Initialization") {
    ActorGuildEmblemComponent component;

    auto result = component.GetGuildId();
    CHECK(!result.has_value());
    CHECK(result.error() == GuildEmblemError::NotInitialized);
}

TEST_CASE("ActorGuildEmblemComponent - Set and Get GuildId") {
    EventBus& eventBus = EventBus::GetInstance();
    ActorGuildEmblemComponent component;
    component.Initialize(EntityId{1}, &eventBus);

    SUBCASE("Default GuildId is 0") {
        auto result = component.GetGuildId();
        CHECK(result.has_value());
        CHECK(result.value() == GuildId{0});
    }

    SUBCASE("Set valid GuildId") {
        auto set_result = component.SetGuildId(GuildId{42});
        CHECK(set_result.has_value());

        auto get_result = component.GetGuildId();
        CHECK(get_result.has_value());
        CHECK(get_result.value() == GuildId{42});
    }

    SUBCASE("Set GuildId without Initialization") {
        ActorGuildEmblemComponent uninitComponent;
        auto result = uninitComponent.SetGuildId(GuildId{42});
        CHECK(!result.has_value());
        CHECK(result.error() == GuildEmblemError::NotInitialized);
    }
}

TEST_CASE("ActorGuildEmblemComponent - Clear Guild Emblem") {
    EventBus& eventBus = EventBus::GetInstance();
    ActorGuildEmblemComponent component;
    component.Initialize(EntityId{1}, &eventBus);
    component.SetGuildId(GuildId{42});

    SUBCASE("Clear valid Emblem") {
        auto clear_result = component.ClearGuildEmblem();
        CHECK(clear_result.has_value());

        auto get_result = component.GetGuildId();
        CHECK(get_result.has_value());
        CHECK(get_result.value() == GuildId{0});
    }

    SUBCASE("Clear without Initialization") {
        ActorGuildEmblemComponent uninitComponent;
        auto result = uninitComponent.ClearGuildEmblem();
        CHECK(!result.has_value());
        CHECK(result.error() == GuildEmblemError::NotInitialized);
    }
}

TEST_CASE("ActorGuildEmblemComponent - Event Publishing") {
    // We cannot instantiate the full EterBase::EventBus easily without some mocking or using the real one if it's testable.
    // However, we can use the EterBase::EventBus implementation provided in the repo.
    
    // Reset singleton just in case
    // EterBase::EventBus is a singleton, so we use EventBus::GetInstance()
    
    ActorGuildEmblemComponent component;
    component.Initialize(EntityId{100}, &EventBus::GetInstance());

    bool eventFired = false;
    std::optional<GuildId> receivedGuildId;
    EntityId receivedEntityId{0};

    auto subId = EventBus::GetInstance().Subscribe<ActorGuildEmblemChangedEvent>(
        [&](const ActorGuildEmblemChangedEvent& event) {
            eventFired = true;
            receivedEntityId = event.entityId;
            receivedGuildId = event.guildId;
        }
    );

    SUBCASE("Event fired on SetGuildId") {
        component.SetGuildId(GuildId{55});
        CHECK(eventFired);
        CHECK(receivedEntityId == EntityId{100});
        CHECK(receivedGuildId.has_value());
        CHECK(receivedGuildId.value() == GuildId{55});
    }

    SUBCASE("Event not fired when setting same GuildId") {
        component.SetGuildId(GuildId{55});
        eventFired = false;
        
        component.SetGuildId(GuildId{55});
        CHECK_FALSE(eventFired);
    }

    SUBCASE("Event fired on ClearGuildEmblem") {
        component.SetGuildId(GuildId{55});
        eventFired = false;

        component.ClearGuildEmblem();
        CHECK(eventFired);
        CHECK(receivedEntityId == EntityId{100});
        CHECK(!receivedGuildId.has_value());
    }

    EventBus::GetInstance().Unsubscribe<ActorGuildEmblemChangedEvent>(subId);
}

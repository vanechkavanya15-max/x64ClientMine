#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Actors/ActorTitleComponent.h"
#include "../src/EterBase/EventBus.h"

using namespace Client::Actors;
using namespace EterBase;

TEST_CASE("ActorTitleComponent - Initialization") {
    ActorTitleComponent component;
    EventBus& eventBus = EventBus::Instance();
    EntityId entityId(123);

    SUBCASE("Cannot use without initialization") {
        auto result = component.GetAlignmentPoints();
        CHECK(!result.has_value());
        CHECK(result.error() == TitleError::NotInitialized);
    }

    SUBCASE("Successful initialization") {
        auto result = component.Initialize(entityId, &eventBus);
        CHECK(result.has_value());
        
        auto pointsResult = component.GetAlignmentPoints();
        CHECK(pointsResult.has_value());
        CHECK(pointsResult.value() == 0);
    }

    SUBCASE("Cannot initialize twice") {
        component.Initialize(entityId, &eventBus);
        auto result = component.Initialize(EntityId(456), &eventBus);
        CHECK(!result.has_value());
        CHECK(result.error() == TitleError::AlreadyInitialized);
    }
}

TEST_CASE("ActorTitleComponent - Alignment Points and Titles") {
    ActorTitleComponent component;
    EventBus& eventBus = EventBus::Instance();
    component.Initialize(EntityId(1), &eventBus);

    SUBCASE("Valid alignment points update") {
        auto result = component.SetAlignmentPoints(5000);
        CHECK(result.has_value());
        CHECK(component.GetAlignmentPoints().value() == 5000);
    }

    SUBCASE("Invalid alignment points bounds") {
        auto result = component.SetAlignmentPoints(25000);
        CHECK(!result.has_value());
        CHECK(result.error() == TitleError::InvalidPoints);
        
        result = component.SetAlignmentPoints(-25000);
        CHECK(!result.has_value());
        CHECK(result.error() == TitleError::InvalidPoints);
    }

    SUBCASE("Title calculation from points") {
        component.SetAlignmentPoints(15000);
        CHECK(component.GetAlignmentTitle().value() == AlignmentTitle::Chivalric);

        component.SetAlignmentPoints(8500);
        CHECK(component.GetAlignmentTitle().value() == AlignmentTitle::Noble);

        component.SetAlignmentPoints(0);
        CHECK(component.GetAlignmentTitle().value() == AlignmentTitle::Neutral);

        component.SetAlignmentPoints(-5000);
        CHECK(component.GetAlignmentTitle().value() == AlignmentTitle::Fraudulent);

        component.SetAlignmentPoints(-15000);
        CHECK(component.GetAlignmentTitle().value() == AlignmentTitle::Cruel);
    }

    SUBCASE("Formatted title string generation") {
        component.SetAlignmentPoints(15000);
        CHECK(component.GetFormattedTitle().value() == "Rycerski");

        component.SetAlignmentPoints(0);
        CHECK(component.GetFormattedTitle().value() == ""); // Neutral means empty title usually

        component.SetAlignmentPoints(-15000);
        CHECK(component.GetFormattedTitle().value() == "Okrutny");
    }
}

TEST_CASE("ActorTitleComponent - Colors") {
    ActorTitleComponent component;
    EventBus& eventBus = EventBus::Instance();
    component.Initialize(EntityId(1), &eventBus);

    SUBCASE("Color mapping") {
        component.SetAlignmentPoints(15000); // Chivalric
        TitleColor cColor = component.GetTitleColor().value();
        CHECK(cColor.b == 255); // Should be a blue-ish color
        
        component.SetAlignmentPoints(0); // Neutral
        TitleColor nColor = component.GetTitleColor().value();
        CHECK(nColor.r == 255);
        CHECK(nColor.g == 255);
        CHECK(nColor.b == 255); // White

        component.SetAlignmentPoints(-15000); // Cruel
        TitleColor crColor = component.GetTitleColor().value();
        CHECK(crColor.r == 150); // Dark red
        CHECK(crColor.g == 0);
        CHECK(crColor.b == 0);
    }
}

TEST_CASE("ActorTitleComponent - Custom Titles") {
    ActorTitleComponent component;
    EventBus& eventBus = EventBus::Instance();
    component.Initialize(EntityId(1), &eventBus);

    SUBCASE("Set and format custom title") {
        component.SetAlignmentPoints(15000); // Usually "Rycerski"
        
        auto result = component.SetCustomTitle("VIP");
        CHECK(result.has_value());
        
        // Custom title overrides alignment title in GetFormattedTitle
        CHECK(component.GetFormattedTitle().value() == "VIP");
    }

    SUBCASE("Clear custom title") {
        component.SetCustomTitle("Admin");
        component.SetAlignmentPoints(8000); // Noble -> "Szlachetny"
        
        auto result = component.ClearCustomTitle();
        CHECK(result.has_value());
        
        CHECK(component.GetFormattedTitle().value() == "Szlachetny");
    }
}

TEST_CASE("ActorTitleComponent - Event Broadcasting") {
    ActorTitleComponent component;
    EventBus& eventBus = EventBus::Instance();
    EntityId entityId(999);
    component.Initialize(entityId, &eventBus);

    std::string receivedTitle;
    bool eventReceived = false;
    EntityId receivedEntityId(0);

    auto subId = eventBus.Subscribe<ActorTitleChangedEvent>([&](const ActorTitleChangedEvent& ev) {
        eventReceived = true;
        receivedTitle = ev.newTitle;
        receivedEntityId = ev.entityId;
    });

    component.SetAlignmentPoints(12000); // Chivalric
    
    CHECK(eventReceived);
    CHECK(receivedEntityId == entityId);
    CHECK(receivedTitle == "Rycerski");

    eventReceived = false;
    component.SetCustomTitle("GameMaster");
    
    CHECK(eventReceived);
    CHECK(receivedTitle == "GameMaster");

    eventBus.Unsubscribe<ActorTitleChangedEvent>(subId);
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Events/SkillEvents.h"
#include "Client/Core/EventBus.h"

TEST_CASE("SkillEvents EventBus Publication") {
    using namespace Client::Events;
    auto& eventBus = Client::Core::EventBus::Instance();

    SUBCASE("SkillCannotUseEvent") {
        bool eventFired = false;
        uint32_t receivedCharIdx = 0;
        std::string receivedReason;

        auto subId = eventBus.Subscribe<SkillCannotUseEvent>([&](const SkillCannotUseEvent& event) {
            eventFired = true;
            receivedCharIdx = event.characterIndex;
            receivedReason = event.reason;
        });

        eventBus.Publish(SkillCannotUseEvent(123, "WAIT_COOLTIME"));

        CHECK(eventFired);
        CHECK(receivedCharIdx == 123);
        CHECK(receivedReason == "WAIT_COOLTIME");

        eventBus.Unsubscribe<SkillCannotUseEvent>(subId);
    }

    SUBCASE("SkillCooldownEvent") {
        bool eventFired = false;
        uint32_t receivedSlotIdx = 0;
        float receivedCooldown = 0.0f;

        auto subId = eventBus.Subscribe<SkillCooldownEvent>([&](const SkillCooldownEvent& event) {
            eventFired = true;
            receivedSlotIdx = event.skillSlotIndex;
            receivedCooldown = event.cooldownTime;
        });

        eventBus.Publish(SkillCooldownEvent(4, 15.5f));

        CHECK(eventFired);
        CHECK(receivedSlotIdx == 4);
        CHECK(receivedCooldown == 15.5f);

        eventBus.Unsubscribe<SkillCooldownEvent>(subId);
    }
}

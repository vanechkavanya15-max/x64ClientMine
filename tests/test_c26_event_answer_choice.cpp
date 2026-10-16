#include "doctest.h"
#include "Client/Events/EventAnswerChoiceManager.h"

using namespace Client::Events;

TEST_CASE("EventAnswerChoiceManager - Headless Isolated Testing") {
    EventAnswerChoiceManager manager;

    SUBCASE("Should start without an active event") {
        auto choices = manager.GetChoices();
        CHECK(choices.empty());

        auto result = manager.AddChoice(AnswerIndex(1), "Yes");
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == EventAnswerError::EventNotActive);
    }

    SUBCASE("Setting active event allows adding and retrieving choices") {
        auto setResult = manager.SetActiveEvent(EventIndex(100));
        REQUIRE(setResult.has_value());

        auto addResult = manager.AddChoice(AnswerIndex(1), "Accept");
        REQUIRE(addResult.has_value());
        
        auto addResult2 = manager.AddChoice(AnswerIndex(2), "Decline");
        REQUIRE(addResult2.has_value());

        auto choices = manager.GetChoices();
        REQUIRE(choices.size() == 2);
        CHECK(choices[0].index == AnswerIndex(1));
        CHECK(choices[0].text == "Accept");
        CHECK(choices[1].index == AnswerIndex(2));
        CHECK(choices[1].text == "Decline");
        
        auto textResult = manager.GetChoiceText(AnswerIndex(1));
        REQUIRE(textResult.has_value());
        CHECK(textResult.value() == "Accept");
    }

    SUBCASE("Should fail to add choice with empty text") {
        manager.SetActiveEvent(EventIndex(101));

        auto addResult = manager.AddChoice(AnswerIndex(1), "");
        REQUIRE_FALSE(addResult.has_value());
        CHECK(addResult.error() == EventAnswerError::EmptyChoiceText);
    }

    SUBCASE("Should fail to add choice if index already exists") {
        manager.SetActiveEvent(EventIndex(102));

        auto addResult1 = manager.AddChoice(AnswerIndex(1), "First");
        REQUIRE(addResult1.has_value());

        auto addResult2 = manager.AddChoice(AnswerIndex(1), "Duplicate");
        REQUIRE_FALSE(addResult2.has_value());
        CHECK(addResult2.error() == EventAnswerError::ChoiceAlreadyExists);
    }

    SUBCASE("Should fail to get choice text for invalid index") {
        manager.SetActiveEvent(EventIndex(103));
        manager.AddChoice(AnswerIndex(1), "Only choice");

        auto textResult = manager.GetChoiceText(AnswerIndex(2));
        REQUIRE_FALSE(textResult.has_value());
        CHECK(textResult.error() == EventAnswerError::InvalidAnswerIndex);
    }

    SUBCASE("Clearing active event resets state") {
        manager.SetActiveEvent(EventIndex(104));
        manager.AddChoice(AnswerIndex(1), "Test");
        REQUIRE(manager.GetChoices().size() == 1);

        manager.ClearActiveEvent();

        auto choices = manager.GetChoices();
        CHECK(choices.empty());

        auto addResult = manager.AddChoice(AnswerIndex(1), "Another test");
        REQUIRE_FALSE(addResult.has_value());
        CHECK(addResult.error() == EventAnswerError::EventNotActive);
    }

    SUBCASE("Setting a new active event clears previous choices") {
        manager.SetActiveEvent(EventIndex(105));
        manager.AddChoice(AnswerIndex(1), "Old choice");
        
        manager.SetActiveEvent(EventIndex(106));
        auto choices = manager.GetChoices();
        CHECK(choices.empty());
    }
}

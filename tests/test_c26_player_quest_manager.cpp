#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Player/PlayerQuestManager.h"
#include "Client/Player/IQuestNetworkContract.h"
#include <vector>

using namespace Client::Player;
using namespace Client::Core;

class MockQuestNetworkContract : public IQuestNetworkContract {
public:
    struct ScriptAnswer {
        int32_t answerIndex;
    };
    
    struct ConfirmAnswer {
        uint8_t answer;
        uint32_t requestPID;
    };

    std::vector<ScriptAnswer> scriptAnswers;
    std::vector<ConfirmAnswer> confirmAnswers;

    void SendScriptAnswer(int32_t answerIndex) override {
        scriptAnswers.push_back({answerIndex});
    }

    void SendQuestConfirmAnswer(uint8_t answer, uint32_t requestPID) override {
        confirmAnswers.push_back({answer, requestPID});
    }
};

TEST_CASE("PlayerQuestManager - Initial State") {
    MockQuestNetworkContract mockNetwork;
    PlayerQuestManager questManager(&mockNetwork);

    CHECK_FALSE(questManager.IsWaitingForConfirm());
    CHECK(questManager.GetCurrentConfirmRequestPID() == 0);
}

TEST_CASE("PlayerQuestManager - OnQuestConfirm Sets State") {
    MockQuestNetworkContract mockNetwork;
    PlayerQuestManager questManager(&mockNetwork);

    questManager.OnQuestConfirm("Wanna trade?", 30, 10543);

    CHECK(questManager.IsWaitingForConfirm());
    CHECK(questManager.GetCurrentConfirmRequestPID() == 10543);
}

TEST_CASE("PlayerQuestManager - AnswerQuestConfirm Valid State") {
    MockQuestNetworkContract mockNetwork;
    PlayerQuestManager questManager(&mockNetwork);

    questManager.OnQuestConfirm("Do you accept?", 60, 9999);
    
    auto result = questManager.AnswerQuestConfirm(1); // 1 for Yes
    
    CHECK(result.has_value());
    CHECK_FALSE(questManager.IsWaitingForConfirm());
    CHECK(questManager.GetCurrentConfirmRequestPID() == 0);
    
    REQUIRE(mockNetwork.confirmAnswers.size() == 1);
    CHECK(mockNetwork.confirmAnswers[0].answer == 1);
    CHECK(mockNetwork.confirmAnswers[0].requestPID == 9999);
}

TEST_CASE("PlayerQuestManager - AnswerQuestConfirm Not Waiting For Confirm Returns Error") {
    MockQuestNetworkContract mockNetwork;
    PlayerQuestManager questManager(&mockNetwork);

    auto result = questManager.AnswerQuestConfirm(1);
    
    CHECK_FALSE(result.has_value());
    CHECK(result.error() == QuestManagerError::NotWaitingForConfirm);
    CHECK(mockNetwork.confirmAnswers.empty());
}

TEST_CASE("PlayerQuestManager - AnswerScript Forwards To Network") {
    MockQuestNetworkContract mockNetwork;
    PlayerQuestManager questManager(&mockNetwork);

    auto result = questManager.AnswerScript(42);
    
    CHECK(result.has_value());
    REQUIRE(mockNetwork.scriptAnswers.size() == 1);
    CHECK(mockNetwork.scriptAnswers[0].answerIndex == 42);
}

TEST_CASE("PlayerQuestManager - Null Network Contract Returns Error") {
    PlayerQuestManager questManager(nullptr);
    
    SUBCASE("AnswerScript") {
        auto result = questManager.AnswerScript(10);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == QuestManagerError::NullNetworkContract);
    }
    
    SUBCASE("AnswerQuestConfirm") {
        questManager.OnQuestConfirm("Accept?", 30, 1234);
        auto result = questManager.AnswerQuestConfirm(1);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == QuestManagerError::NullNetworkContract);
    }
}

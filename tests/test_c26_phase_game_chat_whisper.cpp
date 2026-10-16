#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameChatWhisperBridge.h"

TEST_CASE("PhaseGameChatWhisperBridge: Chat message queue") {
    Client::Bridge::PhaseGameChatWhisperBridge bridge;
    bridge.AppendMessage(1, "PlayerA", "Hello!");
    bridge.AppendMessage(2, "PlayerB", "Hi there");
    CHECK(bridge.GetMessageCount() == 2);
    CHECK(bridge.GetMessages()[0].sender == "PlayerA");
}

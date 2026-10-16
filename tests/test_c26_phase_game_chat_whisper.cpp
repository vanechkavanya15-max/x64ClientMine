#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameChatWhisperBridge.h"

using namespace Client::Bridge;

TEST_CASE("PhaseGameChatWhisperBridge: Message queue and sanitization") {
    PhaseGameChatWhisperBridge bridge;
    bridge.AddBlockedWord("spam");
    bridge.AddBlockedWord("hack");

    SUBCASE("Append and retrieve messages") {
        bridge.AppendMessage(ChatType::Talk, "PlayerA", "Hello all");
        bridge.AppendMessage(ChatType::Whisper, "PlayerB", "Secret message");

        CHECK(bridge.GetMessageCount() == 2);
        CHECK(bridge.GetLastWhisperTarget() == "PlayerB");
        CHECK(bridge.GetMessages()[0].text == "Hello all");
    }

    SUBCASE("Word filtering and masking") {
        bridge.AppendMessage(ChatType::Shout, "Spammer", "Buy this spam now");
        CHECK(bridge.GetMessages()[0].text == "Buy this **** now");
    }
}

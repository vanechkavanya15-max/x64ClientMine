#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <Client/Gameplay/QuestCommandHandler.h>
#include <Client/Core/DomainCommands.h>
#include <Client/Network/Protocol/Protocol.h>
#include <memory>
#include <functional>
#include <span>
#include <cstring>

class MockNetworkPort : public Client::Core::INetworkPort {
public:
    bool connected{true};
    std::function<Client::Core::Result<void, Client::Core::PacketError>(uint8_t, std::span<const uint8_t>)> sendRawHandler;

    Client::Core::Result<void, Client::Core::PacketError> SendRaw(uint8_t opcode, std::span<const uint8_t> payload) override {
        if (sendRawHandler) {
            return sendRawHandler(opcode, payload);
        }
        return Client::Core::Result<void, Client::Core::PacketError>{};
    }

    bool IsConnected() const noexcept override {
        return connected;
    }
};

TEST_CASE("QuestCommandHandler - AnswerWithoutDialogReturnsError") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    Client::Core::QuestAnswerCommand cmd{0};
    auto result = handler->Execute(cmd);
    CHECK(!result.has_value());
    CHECK(result.error() == Client::Core::CommandError::InvalidParameter);
}

TEST_CASE("QuestCommandHandler - AnswerWithInvalidIndexReturnsError") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    handler->SetActiveScriptDialog(2); // Answers: 0, 1
    Client::Core::QuestAnswerCommand cmd{2};
    auto result = handler->Execute(cmd);
    CHECK(!result.has_value());
    CHECK(result.error() == Client::Core::CommandError::InvalidParameter);
}

TEST_CASE("QuestCommandHandler - AnswerDisconnectedReturnsError") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    handler->SetActiveScriptDialog(2);
    mockPort->connected = false;
    Client::Core::QuestAnswerCommand cmd{1};
    auto result = handler->Execute(cmd);
    CHECK(!result.has_value());
    CHECK(result.error() == Client::Core::CommandError::Disconnected);
}

TEST_CASE("QuestCommandHandler - AnswerValidSendsPacketAndClearsState") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    handler->SetActiveScriptDialog(2);
    Client::Core::QuestAnswerCommand cmd{1};

    bool packetSent = false;
    mockPort->sendRawHandler = [&](uint8_t opcode, std::span<const uint8_t> payload) {
        packetSent = true;
        CHECK(opcode == static_cast<uint8_t>(CG::SCRIPT_ANSWER));
        CHECK(payload.size() == sizeof(TPacketCGScriptAnswer));
        auto* packet = reinterpret_cast<const TPacketCGScriptAnswer*>(payload.data());
        CHECK(packet->header == CG::SCRIPT_ANSWER);
        CHECK(packet->answer == 1);
        return Client::Core::Result<void, Client::Core::PacketError>{};
    };

    auto result = handler->Execute(cmd);
    CHECK(result.has_value());
    CHECK(packetSent);
    CHECK(!handler->HasActiveScriptDialog());
}

TEST_CASE("QuestCommandHandler - ConfirmWithoutDialogReturnsError") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    Client::Core::QuestConfirmCommand cmd{1, 1234};
    auto result = handler->Execute(cmd);
    CHECK(!result.has_value());
    CHECK(result.error() == Client::Core::CommandError::InvalidParameter);
}

TEST_CASE("QuestCommandHandler - ConfirmWithMismatchedPIDReturnsError") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    handler->SetActiveConfirmDialog(1234);
    Client::Core::QuestConfirmCommand cmd{1, 9999};
    auto result = handler->Execute(cmd);
    CHECK(!result.has_value());
    CHECK(result.error() == Client::Core::CommandError::InvalidParameter);
}

TEST_CASE("QuestCommandHandler - ConfirmDisconnectedReturnsError") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    handler->SetActiveConfirmDialog(1234);
    mockPort->connected = false;
    Client::Core::QuestConfirmCommand cmd{1, 1234};
    auto result = handler->Execute(cmd);
    CHECK(!result.has_value());
    CHECK(result.error() == Client::Core::CommandError::Disconnected);
}

TEST_CASE("QuestCommandHandler - ConfirmValidSendsPacketAndClearsState") {
    auto mockPort = std::make_shared<MockNetworkPort>();
    auto handler = std::make_unique<Client::Gameplay::QuestCommandHandler>(mockPort);

    handler->SetActiveConfirmDialog(1234);
    Client::Core::QuestConfirmCommand cmd{1, 1234};

    bool packetSent = false;
    mockPort->sendRawHandler = [&](uint8_t opcode, std::span<const uint8_t> payload) {
        packetSent = true;
        CHECK(opcode == static_cast<uint8_t>(CG::QUEST_CONFIRM));
        CHECK(payload.size() == sizeof(TPacketCGQuestConfirm));
        auto* packet = reinterpret_cast<const TPacketCGQuestConfirm*>(payload.data());
        CHECK(packet->header == CG::QUEST_CONFIRM);
        CHECK(packet->answer == 1);
        CHECK(packet->requestPID == 1234);
        return Client::Core::Result<void, Client::Core::PacketError>{};
    };

    auto result = handler->Execute(cmd);
    CHECK(result.has_value());
    CHECK(packetSent);
    CHECK(!handler->HasActiveConfirmDialog());
}

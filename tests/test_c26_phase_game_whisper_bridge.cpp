#include "doctest.h"
#include "../src/Client/Bridge/PhaseGameWhisperBridge.h"
#include "../src/Client/Bridge/MockPythonRuntime.h"
#include <vector>
#include <cstring>

using namespace Client::Bridge;

TEST_CASE("PhaseGameWhisperBridge - Normal Whisper") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameWhisperBridge bridge(mockRuntime);

    std::vector<uint8_t> payload(sizeof(PacketWhisperHeader) + 5, 0);
    auto* header = reinterpret_cast<PacketWhisperHeader*>(payload.data());
    header->header = 0x0604;
    header->length = payload.size();
    header->type = 0; // WHISPER_TYPE_CHAT
    strncpy(header->senderName, "TestUser", sizeof(header->senderName));
    memcpy(payload.data() + sizeof(PacketWhisperHeader), "Hello", 5);

    auto result = bridge.Handle(payload);
    CHECK(result.has_value());
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].methodName == "OnRecvWhisper");
    CHECK(history[0].args.size() == 3);
    CHECK(history[0].args[1] == "TestUser");
    CHECK(history[0].args[2] == "TestUser : Hello");
}

TEST_CASE("PhaseGameWhisperBridge - System Message") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameWhisperBridge bridge(mockRuntime);

    std::vector<uint8_t> payload(sizeof(PacketWhisperHeader) + 6, 0);
    auto* header = reinterpret_cast<PacketWhisperHeader*>(payload.data());
    header->header = 0x0604;
    header->length = payload.size();
    header->type = 0xFF; // WHISPER_TYPE_SYSTEM
    strncpy(header->senderName, "System", sizeof(header->senderName));
    memcpy(payload.data() + sizeof(PacketWhisperHeader), "Notice", 6);

    auto result = bridge.Handle(payload);
    CHECK(result.has_value());
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].methodName == "OnRecvWhisperSystemMessage");
    CHECK(history[0].args.size() == 3);
    CHECK(history[0].args[1] == "System");
    CHECK(history[0].args[2] == "Notice");
}

TEST_CASE("PhaseGameWhisperBridge - Error Message") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameWhisperBridge bridge(mockRuntime);

    std::vector<uint8_t> payload(sizeof(PacketWhisperHeader) + 5, 0);
    auto* header = reinterpret_cast<PacketWhisperHeader*>(payload.data());
    header->header = 0x0604;
    header->length = payload.size();
    header->type = 1; // WHISPER_TYPE_NOT_EXIST
    strncpy(header->senderName, "Unknown", sizeof(header->senderName));
    memcpy(payload.data() + sizeof(PacketWhisperHeader), "Error", 5);

    auto result = bridge.Handle(payload);
    CHECK(result.has_value());
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].methodName == "OnRecvWhisperError");
    CHECK(history[0].args.size() == 3);
    CHECK(history[0].args[1] == "Unknown");
    CHECK(history[0].args[2] == "Error");
}

TEST_CASE("PhaseGameWhisperBridge - Buffer Underflow (Header)") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameWhisperBridge bridge(mockRuntime);

    std::vector<uint8_t> payload(sizeof(PacketWhisperHeader) - 1, 0);
    auto result = bridge.Handle(payload);
    CHECK(!result.has_value());
    CHECK(result.error() == EterBase::PacketError::BufferUnderflow);
}

TEST_CASE("PhaseGameWhisperBridge - Buffer Underflow (Payload)") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameWhisperBridge bridge(mockRuntime);

    std::vector<uint8_t> payload(sizeof(PacketWhisperHeader), 0);
    auto* header = reinterpret_cast<PacketWhisperHeader*>(payload.data());
    header->length = sizeof(PacketWhisperHeader) + 5;

    auto result = bridge.Handle(payload);
    CHECK(!result.has_value());
    CHECK(result.error() == EterBase::PacketError::BufferUnderflow);
}

TEST_CASE("PhaseGameWhisperBridge - Malformed Payload") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameWhisperBridge bridge(mockRuntime);

    std::vector<uint8_t> payload(sizeof(PacketWhisperHeader), 0);
    auto* header = reinterpret_cast<PacketWhisperHeader*>(payload.data());
    header->length = sizeof(PacketWhisperHeader) - 5;

    auto result = bridge.Handle(payload);
    CHECK(!result.has_value());
    CHECK(result.error() == EterBase::PacketError::MalformedPayload);
}

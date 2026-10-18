#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>
#include <span>
#include <cstring>

#include "../src/Client/Network/Handlers/DeadHandler.h"
#include "../src/Client/Network/CombatPacketCodec.h"
#include "../src/Client/Network/Protocol/Protocol.h"

using namespace Client::Network::Handlers;

static EterBase::EntityId s_capturedVid{0};
static ::World::MotionState s_capturedState{::World::MotionState::Count};
static bool s_callbackCalled = false;

static void TestCallback(EterBase::EntityId vid, ::World::MotionState state) {
    s_capturedVid = vid;
    s_capturedState = state;
    s_callbackCalled = true;
}

TEST_CASE("DeadHandler - ProcessDeadPacket_SuccessfulDecoding_CallsCallbackWithCorrectData") {
    s_capturedVid = EterBase::EntityId{0};
    s_capturedState = ::World::MotionState::Count;
    s_callbackCalled = false;

    DeadHandler handler(TestCallback);

    TPacketGCDead packet{};
    packet.header = 1;
    packet.length = sizeof(TPacketGCDead);
    packet.vid = 12345;

    std::vector<uint8_t> buffer(sizeof(TPacketGCDead));
    std::memcpy(buffer.data(), &packet, sizeof(TPacketGCDead));

    auto result = handler.ProcessDeadPacket(buffer);

    CHECK(result.has_value());
    CHECK(s_callbackCalled);
    CHECK(s_capturedVid.value() == 12345);
    CHECK(s_capturedState == ::World::MotionState::Dead);
}

TEST_CASE("DeadHandler - ProcessDeadPacket_BufferUnderflow_ReturnsError") {
    s_callbackCalled = false;
    DeadHandler handler(TestCallback);

    std::vector<uint8_t> buffer(sizeof(TPacketGCDead) - 1); // Too small

    auto result = handler.ProcessDeadPacket(buffer);

    CHECK(!result.has_value());
    CHECK(result.error() == EterBase::PacketError::BufferUnderflow);
    CHECK(!s_callbackCalled);
}

TEST_CASE("DeadHandler - ProcessDeadPacket_NoCallback_SucceedsWithoutCrashing") {
    DeadHandler handler(nullptr);

    TPacketGCDead packet{};
    packet.header = 1;
    packet.length = sizeof(TPacketGCDead);
    packet.vid = 54321;

    std::vector<uint8_t> buffer(sizeof(TPacketGCDead));
    std::memcpy(buffer.data(), &packet, sizeof(TPacketGCDead));

    auto result = handler.ProcessDeadPacket(buffer);

    CHECK(result.has_value());
}

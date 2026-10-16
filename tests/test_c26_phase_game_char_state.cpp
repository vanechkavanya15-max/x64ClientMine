#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameCharacterStateBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include <memory>
#include <cstring>

using namespace Client::Bridge;
using namespace Client::Core;

TEST_CASE("PhaseGameCharacterStateBridge: Point change recording (Legacy)") {
    PhaseGameCharacterStateBridge bridge;
    bridge.OnPointChange(1, 1500, 100);
    const auto& hist = bridge.GetHistory();
    REQUIRE(hist.size() == 1);
    CHECK(hist[0].type == 1);
    CHECK(hist[0].value == 1500);
    CHECK(hist[0].amount == 100);
}

TEST_CASE("PhaseGameCharacterStateBridge: Initialization and Clean up") {
    auto pyRuntime = std::make_shared<MockPythonRuntime>();
    pyRuntime->Initialize();
    
    PhaseGameCharacterStateBridge bridge(pyRuntime);
    EntityVid vid(42);
    
    // Default state check
    const ActorState* state = bridge.GetActorState(vid);
    CHECK(state == nullptr);
    
    // Trigger creation
    auto res = bridge.RecvPointChange(vid, PointType::HP, 100, 0);
    REQUIRE(res.has_value());
    
    state = bridge.GetActorState(vid);
    REQUIRE(state != nullptr);
    CHECK(state->vid == vid);
    
    // Clear check
    bridge.ClearState();
    CHECK(bridge.GetActorState(vid) == nullptr);
}

TEST_CASE("PhaseGameCharacterStateBridge: RecvPointChange and Python Delegation") {
    auto pyRuntime = std::make_shared<MockPythonRuntime>();
    pyRuntime->Initialize();
    PhaseGameCharacterStateBridge bridge(pyRuntime);
    EntityVid vid(42);

    SUBCASE("Update HP and limit check") {
        bridge.RecvPointChange(vid, PointType::MAX_HP, 1000, 0);
        bridge.RecvPointChange(vid, PointType::HP, 1200, 0); // Should clamp to max
        
        const ActorState* state = bridge.GetActorState(vid);
        REQUIRE(state != nullptr);
        CHECK(state->maxHP == 1000);
        CHECK(state->currentHP == 1000);
        
        // Verify Python calls
        auto history = pyRuntime->GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[0].methodName == "OnPointChange");
        CHECK(history[0].args.size() == 4);
    }
    
    SUBCASE("Update SP and limit check") {
        bridge.RecvPointChange(vid, PointType::MAX_SP, 500, 0);
        bridge.RecvPointChange(vid, PointType::SP, -50, 0); // Should clamp to 0
        
        const ActorState* state = bridge.GetActorState(vid);
        REQUIRE(state != nullptr);
        CHECK(state->currentSP == 0);
        
        bridge.RecvPointChange(vid, PointType::SP, 600, 0); // Should clamp to max
        CHECK(state->currentSP == 500);
    }
    
    SUBCASE("Death by HP reaching 0") {
        bridge.RecvPointChange(vid, PointType::MAX_HP, 1000, 0);
        bridge.RecvPointChange(vid, PointType::HP, 0, -1000);
        
        const ActorState* state = bridge.GetActorState(vid);
        REQUIRE(state != nullptr);
        CHECK(state->isDead == true);
        CHECK(state->actionState == ActorActionState::Dead);
    }
    
    SUBCASE("Invalid Point Type Payload") {
        auto res = bridge.RecvPointChange(vid, PointType::None, 0, 0);
        REQUIRE(!res.has_value());
        CHECK(res.error() == EterBase::PacketError::MalformedPayload);
    }
}

TEST_CASE("PhaseGameCharacterStateBridge: Stun and Death transitions") {
    auto pyRuntime = std::make_shared<MockPythonRuntime>();
    pyRuntime->Initialize();
    PhaseGameCharacterStateBridge bridge(pyRuntime);
    EntityVid vid(99);

    SUBCASE("Normal Stun") {
        auto res = bridge.RecvStun(vid);
        REQUIRE(res.has_value());
        
        const ActorState* state = bridge.GetActorState(vid);
        REQUIRE(state != nullptr);
        CHECK(state->isStunned == true);
        CHECK(state->actionState == ActorActionState::Stunned);
        
        auto history = pyRuntime->GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].methodName == "SetStatus");
        CHECK(history[0].args[1] == "STUN");
    }
    
    SUBCASE("Death cancels stun") {
        bridge.RecvStun(vid);
        auto res = bridge.RecvDead(vid);
        REQUIRE(res.has_value());
        
        const ActorState* state = bridge.GetActorState(vid);
        REQUIRE(state != nullptr);
        CHECK(state->isDead == true);
        CHECK(state->isStunned == false); // Stun is removed
        CHECK(state->actionState == ActorActionState::Dead);
        
        auto history = pyRuntime->GetHistory();
        REQUIRE(history.size() == 2);
        CHECK(history[1].methodName == "SetStatus");
        CHECK(history[1].args[1] == "DEAD");
    }
    
    SUBCASE("Cannot stun dead entity") {
        bridge.RecvDead(vid);
        pyRuntime->ClearHistory();
        
        auto res = bridge.RecvStun(vid);
        REQUIRE(!res.has_value());
        CHECK(res.error() == EterBase::PacketError::SequenceMismatch);
        
        CHECK(pyRuntime->GetHistory().size() == 0);
    }
}

TEST_CASE("PhaseGameCharacterStateBridge: Motion Handling") {
    auto pyRuntime = std::make_shared<MockPythonRuntime>();
    pyRuntime->Initialize();
    PhaseGameCharacterStateBridge bridge(pyRuntime);
    EntityVid vid(77);
    MapCoords targetPos{100.0f, 200.0f, 300.0f};
    
    SUBCASE("Normal Motion") {
        auto res = bridge.RecvMotion(vid, targetPos, 45.0f, 12345);
        REQUIRE(res.has_value());
        
        const ActorState* state = bridge.GetActorState(vid);
        REQUIRE(state != nullptr);
        CHECK(state->currentPosition.x == doctest::Approx(100.0f));
        CHECK(state->currentRotation == doctest::Approx(45.0f));
        CHECK(state->actionState == ActorActionState::Moving);
        
        auto history = pyRuntime->GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].methodName == "SetTargetPosition");
        CHECK(history[0].args.size() == 6); // vid, x, y, z, rot, time
    }
    
    SUBCASE("Motion prevented by Stun") {
        bridge.RecvStun(vid);
        auto res = bridge.RecvMotion(vid, targetPos, 0.0f, 0);
        REQUIRE(!res.has_value());
        CHECK(res.error() == EterBase::PacketError::SequenceMismatch);
    }
    
    SUBCASE("Motion prevented by Death") {
        bridge.RecvDead(vid);
        auto res = bridge.RecvMotion(vid, targetPos, 0.0f, 0);
        REQUIRE(!res.has_value());
        CHECK(res.error() == EterBase::PacketError::SequenceMismatch);
    }
}

TEST_CASE("PhaseGameCharacterStateBridge: Outgoing Packet Serialization") {
    PhaseGameCharacterStateBridge bridge;
    
    SUBCASE("SendCharacterStatePacket - valid") {
        CharacterStateData data;
        data.vid = EntityVid(0x12345678);
        data.actionState = ActorActionState::Moving;
        data.alignment = 5;
        data.empire = 2;
        data.guildId = 10;
        data.pkMode = 1;
        
        auto res = bridge.SendCharacterStatePacket(data);
        REQUIRE(res.has_value());
        const auto& buf = res.value();
        
        // Header(1) + VID(4) + Action(1) + Alignment(1) + Empire(1) + Guild(1) + PKMode(1) = 10 bytes
        REQUIRE(buf.size() == 10);
        CHECK(buf[0] == 0xA1); // Header
        
        uint32_t decodedVid;
        std::memcpy(&decodedVid, buf.data() + 1, 4);
        CHECK(decodedVid == 0x12345678);
        
        CHECK(buf[5] == static_cast<uint8_t>(ActorActionState::Moving));
        CHECK(buf[6] == 5);
        CHECK(buf[7] == 2);
        CHECK(buf[8] == 10);
        CHECK(buf[9] == 1);
    }
    
    SUBCASE("SendCharacterStatePacket - invalid VID") {
        CharacterStateData data;
        data.vid = EntityVid(0); // Invalid 0
        auto res = bridge.SendCharacterStatePacket(data);
        REQUIRE(!res.has_value());
        CHECK(res.error() == EterBase::PacketDispatchError::EncodingFailed);
    }
    
    SUBCASE("SendCharacterPosition - valid") {
        CharacterPositionData data;
        data.vid = EntityVid(0x87654321);
        data.position = {1.5f, 2.5f, 3.5f};
        data.rotation = 90.0f;
        data.destTime = 1000;
        
        auto res = bridge.SendCharacterPosition(data);
        REQUIRE(res.has_value());
        const auto& buf = res.value();
        
        // Header(1) + VID(4) + X(4) + Y(4) + Z(4) + Rot(4) + Time(4) = 25 bytes
        REQUIRE(buf.size() == 25);
        CHECK(buf[0] == 0xA2); // Header
        
        uint32_t decodedVid;
        std::memcpy(&decodedVid, buf.data() + 1, 4);
        CHECK(decodedVid == 0x87654321);
        
        float x, y, z, rot;
        std::memcpy(&x, buf.data() + 5, 4);
        std::memcpy(&y, buf.data() + 9, 4);
        std::memcpy(&z, buf.data() + 13, 4);
        std::memcpy(&rot, buf.data() + 17, 4);
        CHECK(x == doctest::Approx(1.5f));
        CHECK(y == doctest::Approx(2.5f));
        CHECK(z == doctest::Approx(3.5f));
        CHECK(rot == doctest::Approx(90.0f));
        
        uint32_t decodedTime;
        std::memcpy(&decodedTime, buf.data() + 21, 4);
        CHECK(decodedTime == 1000);
    }
}

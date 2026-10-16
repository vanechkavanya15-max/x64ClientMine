#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include <vector>
#include <cstdint>
#include <cstring>
#include <span>

#include "EterBase/Result.h"

// ============================================================================
// MOCKS
// ============================================================================


#pragma pack(push, 1)
struct TPacketGCDigMotion {
    uint16_t header;
    uint16_t length;
    uint32_t vid;
    uint32_t target_vid;
    uint8_t count;
};
#pragma pack(pop)

struct CRaceMotionData {
    static constexpr uint16_t NAME_DIG = 999;
};

class CInstanceBaseMock {
public:
    uint32_t vid = 0;
    int pushOnceMotionCalls = 0;
    uint16_t lastPushedMotion = 0;
    bool lookAtDestCalled = false;
    uint32_t lookAtDestVid = 0;

    void NEW_LookAtDestInstance(CInstanceBaseMock& rkInstDst) {
        lookAtDestCalled = true;
        lookAtDestVid = rkInstDst.vid;
    }

    void PushOnceMotion(uint16_t wMotion, float fBlendTime = 0.1f, float fSpeedRatio = 1.0f) {
        pushOnceMotionCalls++;
        lastPushedMotion = wMotion;
    }
};

#define CInstanceBase CInstanceBaseMock

class IAbstractCharacterManagerMock {
public:
    virtual ~IAbstractCharacterManagerMock() = default;
    virtual CInstanceBaseMock* GetInstancePtr(uint32_t dwVID) = 0;
};

#define IAbstractCharacterManager IAbstractCharacterManagerMock

class TestCharacterManager : public IAbstractCharacterManagerMock {
public:
    CInstanceBaseMock* mainActor = nullptr;
    CInstanceBaseMock* targetActor = nullptr;

    CInstanceBaseMock* GetInstancePtr(uint32_t dwVID) override {
        if (mainActor && mainActor->vid == dwVID) return mainActor;
        if (targetActor && targetActor->vid == dwVID) return targetActor;
        return nullptr;
    }
};

// ============================================================================
// INCLUSION OF SUT (System Under Test)
// ============================================================================
#ifndef TEST_MODE_DISABLE_STDAFX
#define TEST_MODE_DISABLE_STDAFX
#endif
#include "Client/Bridge/PhaseGameMiningBridge.cpp"

// ============================================================================
// TESTS
// ============================================================================

TEST_CASE("PhaseGameMiningBridge - Metadata") {
    TestCharacterManager mockMgr;
    Client::Bridge::PhaseGameMiningBridge bridge(mockMgr);

    CHECK(bridge.GetExpectedSize() == sizeof(TPacketGCDigMotion));
    CHECK(bridge.IsDynamicSize() == false);
}

TEST_CASE("PhaseGameMiningBridge - Handle Buffer Underflow") {
    TestCharacterManager mockMgr;
    Client::Bridge::PhaseGameMiningBridge bridge(mockMgr);

    std::vector<uint8_t> payload(sizeof(TPacketGCDigMotion) - 1, 0);

    auto result = bridge.Handle(payload);
    CHECK_FALSE(result.has_value());
    CHECK(result.error() == EterBase::PacketError::BufferUnderflow);
}

TEST_CASE("PhaseGameMiningBridge - Normal execution with target") {
    TestCharacterManager mockMgr;
    
    CInstanceBaseMock mainInst;
    mainInst.vid = 100;
    
    CInstanceBaseMock targetInst;
    targetInst.vid = 200;

    mockMgr.mainActor = &mainInst;
    mockMgr.targetActor = &targetInst;

    Client::Bridge::PhaseGameMiningBridge bridge(mockMgr);

    TPacketGCDigMotion pack{};
    pack.vid = 100;
    pack.target_vid = 200;
    pack.count = 3;

    std::vector<uint8_t> payload(sizeof(pack));
    std::memcpy(payload.data(), &pack, sizeof(pack));

    auto result = bridge.Handle(payload);
    CHECK(result.has_value());

    CHECK(mainInst.lookAtDestCalled == true);
    CHECK(mainInst.lookAtDestVid == 200);
    CHECK(mainInst.pushOnceMotionCalls == 3);
    CHECK(mainInst.lastPushedMotion == CRaceMotionData::NAME_DIG);
}

TEST_CASE("PhaseGameMiningBridge - Normal execution without target") {
    TestCharacterManager mockMgr;
    
    CInstanceBaseMock mainInst;
    mainInst.vid = 100;
    mockMgr.mainActor = &mainInst;
    mockMgr.targetActor = nullptr;

    Client::Bridge::PhaseGameMiningBridge bridge(mockMgr);

    TPacketGCDigMotion pack{};
    pack.vid = 100;
    pack.target_vid = 0; // No target
    pack.count = 2;

    std::vector<uint8_t> payload(sizeof(pack));
    std::memcpy(payload.data(), &pack, sizeof(pack));

    auto result = bridge.Handle(payload);
    CHECK(result.has_value());

    CHECK(mainInst.lookAtDestCalled == false);
    CHECK(mainInst.pushOnceMotionCalls == 2);
    CHECK(mainInst.lastPushedMotion == CRaceMotionData::NAME_DIG);
}

TEST_CASE("PhaseGameMiningBridge - Main actor not found") {
    TestCharacterManager mockMgr;
    mockMgr.mainActor = nullptr;
    mockMgr.targetActor = nullptr;

    Client::Bridge::PhaseGameMiningBridge bridge(mockMgr);

    TPacketGCDigMotion pack{};
    pack.vid = 100;
    pack.target_vid = 200;
    pack.count = 1;

    std::vector<uint8_t> payload(sizeof(pack));
    std::memcpy(payload.data(), &pack, sizeof(pack));

    // Even if actor is missing, the handler should gracefully succeed
    auto result = bridge.Handle(payload);
    CHECK(result.has_value());
}

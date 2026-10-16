#include "doctest/doctest.h"
#include "Client/IPC/HarnessEngineControlRouter.h"

using namespace Client::IPC;

TEST_CASE("HarnessEngineControlRouter: Basic Engine control and frame stepping") {
    HarnessEngineControlRouter engine;
    
    engine.SetWindowVisible(false);
    CHECK_FALSE(engine.IsWindowVisible());
    
    engine.SetGpuThrottle(true);
    CHECK(engine.IsGpuThrottled());

    auto setDriverResult = engine.SetProtocolDriver("beavium");
    CHECK(setDriverResult.has_value());
    CHECK(engine.GetProtocolDriver() == "beavium");

    // Default state is Initializing, tick should work after Transition to Running
    auto transitionResult = engine.TransitionState(EngineState::Running);
    CHECK(transitionResult.has_value());

    engine.TickFrame(0.016f);
    engine.TickFrame(0.016f);
    CHECK(engine.GetFrameCount() == 2);
    CHECK(engine.GetUpTime() == doctest::Approx(0.032f));
}

TEST_CASE("HarnessEngineControlRouter: State Machine Rules") {
    HarnessEngineControlRouter engine;
    
    // Initializing -> Paused should be invalid
    auto err1 = engine.TransitionState(EngineState::Paused);
    CHECK_FALSE(err1.has_value());
    CHECK(err1.error() == HarnessEngineControlRouter::ErrorCode::InvalidTransition);

    auto t1 = engine.TransitionState(EngineState::Running);
    CHECK(t1.has_value());
    
    auto t2 = engine.TransitionState(EngineState::Paused);
    CHECK(t2.has_value());
    
    // Force crash
    engine.ForceCrash(CrashReason::MemoryCorruption, "Segfault at 0x0");
    CHECK(engine.GetState() == EngineState::Crashed);
    
    // Transition out of crashed must be Initializing
    auto err2 = engine.TransitionState(EngineState::Running);
    CHECK_FALSE(err2.has_value());
    CHECK(err2.error() == HarnessEngineControlRouter::ErrorCode::InvalidTransition);
    
    // Valid reset transition
    auto t3 = engine.TransitionState(EngineState::Initializing);
    CHECK(t3.has_value());
    CHECK(engine.GetState() == EngineState::Initializing);
}

TEST_CASE("HarnessEngineControlRouter: Packet Injection and Queuing") {
    HarnessEngineControlRouter engine;
    engine.TransitionState(EngineState::Running);

    Packet p1{1, 100, {0x01, 0x02}, false};
    auto res1 = engine.InjectPacket(p1);
    CHECK(res1.has_value());
    CHECK(engine.GetPendingPacketCount() == 1);
    
    // Process queue
    engine.TickFrame(0.016f);
    CHECK(engine.GetPendingPacketCount() == 0);
    
    // After crash, injection is blocked
    engine.ForceCrash(CrashReason::Timeout, "Network timeout");
    auto res2 = engine.InjectPacket(p1);
    CHECK_FALSE(res2.has_value());
    CHECK(res2.error() == HarnessEngineControlRouter::ErrorCode::AlreadyCrashed);
}

TEST_CASE("HarnessEngineControlRouter: Fuzzer Capabilities") {
    FuzzerConfig cfg{
        .enabled = true,
        .dropRate = 1.0f, // 100% drop
        .duplicateRate = 0.0f,
        .corruptRate = 0.0f,
        .crashRate = 0.0f,
        .seed = 123
    };
    
    HarnessEngineControlRouter engine(cfg);
    engine.TransitionState(EngineState::Running);
    
    Packet p{1, 100, {0xFF}, false};
    
    // Drop test
    auto res1 = engine.InjectPacket(p);
    CHECK(res1.has_value());
    CHECK(engine.GetPendingPacketCount() == 0); // Dropped

    // Duplicate test
    cfg.dropRate = 0.0f;
    cfg.duplicateRate = 1.0f;
    engine.UpdateFuzzerConfig(cfg);
    
    auto res2 = engine.InjectPacket(p);
    CHECK(res2.has_value());
    CHECK(engine.GetPendingPacketCount() == 2); // Duplicated
    
    // Fuzzer Crash test
    cfg.duplicateRate = 0.0f;
    cfg.crashRate = 1.0f;
    engine.UpdateFuzzerConfig(cfg);
    
    engine.TickFrame(0.016f);
    CHECK(engine.GetState() == EngineState::Crashed);
    
    auto report = engine.GetCrashReport();
    REQUIRE(report.has_value());
    CHECK(report->reason == CrashReason::FuzzerInduced);
}

TEST_CASE("HarnessEngineControlRouter: Crash Report Formatting") {
    HarnessEngineControlRouter engine;
    engine.TransitionState(EngineState::Running);
    
    engine.TickFrame(1.0f);
    engine.TickFrame(1.0f);
    
    Packet p{1, 100, {0xAA}, false};
    engine.InjectPacket(p);
    engine.TickFrame(1.0f); // Process packet
    
    engine.ForceCrash(CrashReason::MemoryCorruption, "Null pointer dereference");
    
    auto report = engine.GetCrashReport();
    REQUIRE(report.has_value());
    CHECK(report->frameCount == 3);
    CHECK(report->upTime == doctest::Approx(3.0f));
    CHECK(report->packetsProcessed == 1);
    CHECK(report->reason == CrashReason::MemoryCorruption);
    
    CHECK(report->contextDump.find("Crash occurred at frame 3") != std::string::npos);
    CHECK(report->contextDump.find("Null pointer dereference") != std::string::npos);
}

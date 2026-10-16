#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/IPC/HarnessEngineControlRouter.h"

TEST_CASE("HarnessEngineControlRouter: Engine control and frame stepping") {
    Client::IPC::HarnessEngineControlRouter engine;
    engine.SetWindowVisible(false);
    CHECK_FALSE(engine.IsWindowVisible());
    engine.SetGpuThrottle(true);
    CHECK(engine.IsGpuThrottled());

    engine.SetProtocolDriver("beavium");
    CHECK(engine.GetProtocolDriver() == "beavium");

    engine.StepFrame(0.016f);
    engine.StepFrame(0.016f);
    CHECK(engine.GetFrameCount() == 2);
}

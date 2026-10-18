#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include <thread>
#include <vector>
#include <atomic>
#include "EterLib/RenderQueueManager.h"

TEST_CASE("RenderQueueManager AlignmentCheck")
{
    auto& mgr = CRenderQueueManager::Instance();
    mgr.BeginFrame();
    void* ptr1 = mgr.AllocateFrameData(10, 8); // Should align to 8
    void* ptr2 = mgr.AllocateFrameData(10, 16); // Should align to 16
    CHECK(reinterpret_cast<uintptr_t>(ptr1) % 8 == 0);
    CHECK(reinterpret_cast<uintptr_t>(ptr2) % 16 == 0);
}

#include <gtest/gtest.h>
#include <thread>
#include <vector>
#include <atomic>
#include "../src/EterLib/RenderQueueManager.h"

TEST(RenderQueueManager, AlignmentCheck)
{
    auto& mgr = CRenderQueueManager::Instance();
    mgr.BeginFrame();
    void* ptr1 = mgr.AllocateFrameData(10, 8); // Should align to 8
    void* ptr2 = mgr.AllocateFrameData(10, 16); // Should align to 16
    EXPECT_EQ(reinterpret_cast<uintptr_t>(ptr1) % 8, 0);
    EXPECT_EQ(reinterpret_cast<uintptr_t>(ptr2) % 16, 0);
}

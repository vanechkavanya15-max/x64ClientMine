#include <gtest/gtest.h>
#include <thread>
#include <vector>
#include "../src/GameLib/DungeonBlockRenderExtractor.h"

TEST(DungeonBlockRenderExtractorTest, IsActive_AtomicThreadSafety) {
    // Reset to false
    CDungeonBlockRenderExtractor::SetActive(false);
    
    const int num_threads = 10;
    const int iterations = 1000;
    std::vector<std::thread> threads;
    
    for (int i = 0; i < num_threads; ++i) {
        threads.emplace_back([i, iterations]() {
            for (int j = 0; j < iterations; ++j) {
                if (i % 2 == 0) {
                    CDungeonBlockRenderExtractor::SetActive(j % 2 == 0);
                } else {
                    bool state = CDungeonBlockRenderExtractor::IsActive();
                    // We don't assert on state because it changes concurrently,
                    // we just want to ensure it doesn't crash or trigger sanitizers.
                    (void)state;
                }
            }
        });
    }
    
    for (auto& t : threads) {
        t.join();
    }
    
    SUCCEED();
}

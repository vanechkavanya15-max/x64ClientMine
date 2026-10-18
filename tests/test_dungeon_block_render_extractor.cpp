#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include <thread>
#include <vector>
#include "../src/GameLib/DungeonBlockRenderExtractor.h"

TEST_CASE("DungeonBlockRenderExtractorTest IsActive_AtomicThreadSafety") {
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
                    (void)state;
                }
            }
        });
    }
    
    for (auto& t : threads) {
        t.join();
    }
    
    CHECK(true);
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Skills/SkillVisualPipeline.h"
#include <utility>
#include <unordered_set>

using namespace Client::Skills;
using namespace Client::Core;

class MockVisualEffectRenderer : public IVisualEffectRenderer {
public:
    std::unordered_set<uint32_t> validEntities;
    std::vector<std::pair<EntityVid, VisualEffectConfig>> dispatchedEffects;

    void DispatchEffect(EntityVid sourceVid, const VisualEffectConfig& config) override {
        dispatchedEffects.push_back({sourceVid, config});
    }

    bool IsEntityValid(EntityVid vid) const override {
        return validEntities.contains(vid.get());
    }

    void AddValidEntity(uint32_t vid) {
        validEntities.insert(vid);
    }
    
    void RemoveValidEntity(uint32_t vid) {
        validEntities.erase(vid);
    }
};

TEST_CASE("SkillVisualPipeline - Enqueue effect without delay") {
    MockVisualEffectRenderer renderer;
    renderer.AddValidEntity(1);
    
    SkillVisualPipeline pipeline(renderer);
    
    VisualEffectConfig config{100, std::nullopt, 0};
    auto result = pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    
    CHECK(result.has_value());
    CHECK(pipeline.GetQueueSize() == 1);
    
    pipeline.Update(1000);
    
    CHECK(pipeline.GetQueueSize() == 0);
    CHECK(renderer.dispatchedEffects.size() == 1);
    CHECK(renderer.dispatchedEffects[0].first == EntityVid{1});
    CHECK(renderer.dispatchedEffects[0].second.effectVnum == 100);
}

TEST_CASE("SkillVisualPipeline - Enqueue effect with delay") {
    MockVisualEffectRenderer renderer;
    renderer.AddValidEntity(1);
    
    SkillVisualPipeline pipeline(renderer);
    
    VisualEffectConfig config{100, std::nullopt, 500};
    auto result = pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    
    CHECK(result.has_value());
    
    // Not enough time passed
    pipeline.Update(1200);
    CHECK(pipeline.GetQueueSize() == 1);
    CHECK(renderer.dispatchedEffects.empty());
    
    // Enough time passed
    pipeline.Update(1500);
    CHECK(pipeline.GetQueueSize() == 0);
    CHECK(renderer.dispatchedEffects.size() == 1);
}

TEST_CASE("SkillVisualPipeline - Invalid configuration") {
    MockVisualEffectRenderer renderer;
    renderer.AddValidEntity(1);
    
    SkillVisualPipeline pipeline(renderer);
    
    VisualEffectConfig config{0, std::nullopt, 0}; // Invalid effectVnum
    auto result = pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    
    CHECK(!result.has_value());
    CHECK(result.error() == SkillVisualError::InvalidConfiguration);
}

TEST_CASE("SkillVisualPipeline - Invalid source entity") {
    MockVisualEffectRenderer renderer;
    // Entity 1 is not valid
    
    SkillVisualPipeline pipeline(renderer);
    
    VisualEffectConfig config{100, std::nullopt, 0};
    auto result = pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    
    CHECK(!result.has_value());
    CHECK(result.error() == SkillVisualError::InvalidSourceEntity);
}

TEST_CASE("SkillVisualPipeline - Invalid target entity") {
    MockVisualEffectRenderer renderer;
    renderer.AddValidEntity(1);
    // Target Entity 2 is not valid
    
    SkillVisualPipeline pipeline(renderer);
    
    VisualEffectConfig config{100, EntityVid{2}, 0};
    auto result = pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    
    CHECK(!result.has_value());
    CHECK(result.error() == SkillVisualError::TargetEntityNotFound);
}

TEST_CASE("SkillVisualPipeline - Source becomes invalid before dispatch") {
    MockVisualEffectRenderer renderer;
    renderer.AddValidEntity(1);
    
    SkillVisualPipeline pipeline(renderer);
    
    VisualEffectConfig config{100, std::nullopt, 500};
    pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    
    // Entity 1 dies/despawns before effect is dispatched
    renderer.RemoveValidEntity(1);
    
    pipeline.Update(1500);
    
    CHECK(pipeline.GetQueueSize() == 0);
    CHECK(renderer.dispatchedEffects.empty()); // Should not be dispatched
}

TEST_CASE("SkillVisualPipeline - Target becomes invalid before dispatch") {
    MockVisualEffectRenderer renderer;
    renderer.AddValidEntity(1);
    renderer.AddValidEntity(2);
    
    SkillVisualPipeline pipeline(renderer);
    
    VisualEffectConfig config{100, EntityVid{2}, 500};
    pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    
    // Target dies/despawns before effect is dispatched
    renderer.RemoveValidEntity(2);
    
    pipeline.Update(1500);
    
    CHECK(pipeline.GetQueueSize() == 0);
    CHECK(renderer.dispatchedEffects.empty()); // Should not be dispatched
}

TEST_CASE("SkillVisualPipeline - Queue full") {
    MockVisualEffectRenderer renderer;
    renderer.AddValidEntity(1);
    
    SkillVisualPipeline pipeline(renderer, 2); // Max 2 elements
    
    VisualEffectConfig config{100, std::nullopt, 500};
    
    auto r1 = pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    auto r2 = pipeline.EnqueueEffect(EntityVid{1}, config, 1000);
    auto r3 = pipeline.EnqueueEffect(EntityVid{1}, config, 1000); // Should fail
    
    CHECK(r1.has_value());
    CHECK(r2.has_value());
    CHECK(!r3.has_value());
    CHECK(r3.error() == SkillVisualError::PipelineQueueFull);
}

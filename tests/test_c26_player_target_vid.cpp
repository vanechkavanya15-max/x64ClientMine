#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerTargetVidController.h"
#include "EterBase/EventBus.h"

#include <unordered_map>
#include <vector>

namespace {

class MockActorQueryProvider : public Client::Player::IActorQueryProvider {
public:
    struct ActorState {
        bool alive{true};
        float distance{0.0f};
    };

    std::unordered_map<uint32_t, ActorState> actors;

    void SetActor(uint32_t vid, bool alive, float dist) {
        actors[vid] = ActorState{alive, dist};
    }

    void RemoveActor(uint32_t vid) {
        actors.erase(vid);
    }

    bool IsActorAlive(uint32_t vid) const override {
        if (auto it = actors.find(vid); it != actors.end()) {
            return it->second.alive;
        }
        return false;
    }

    float GetDistanceToActor(uint32_t vid) const override {
        if (auto it = actors.find(vid); it != actors.end()) {
            return it->second.distance;
        }
        return 999999.0f;
    }

    std::vector<uint32_t> GetAllActorVids() const override {
        std::vector<uint32_t> vids;
        for (const auto& [vid, _] : actors) {
            vids.push_back(vid);
        }
        return vids;
    }
};

} // namespace

TEST_CASE("PlayerTargetVidController: Manual Targeting") {
    MockActorQueryProvider provider;
    Client::Player::PlayerTargetVidController controller(&provider);

    CHECK_FALSE(controller.HasTarget());
    CHECK(controller.GetMode() == Client::Player::TargetMode::None);

    SUBCASE("SetTargetVid - valid target") {
        provider.SetActor(100, true, 50.0f);
        auto result = controller.SetTargetVid(100);
        CHECK(result.has_value());
        CHECK(controller.HasTarget());
        CHECK(controller.GetTargetVid() == 100);
        CHECK(controller.GetMode() == Client::Player::TargetMode::Manual);
    }

    SUBCASE("SetTargetVid - zero clears target") {
        provider.SetActor(100, true, 50.0f);
        controller.SetTargetVid(100);
        CHECK(controller.HasTarget());

        auto result = controller.SetTargetVid(0);
        CHECK(result.has_value());
        CHECK_FALSE(controller.HasTarget());
        CHECK(controller.GetMode() == Client::Player::TargetMode::None);
    }

    SUBCASE("SetTargetVid - dead target") {
        provider.SetActor(200, false, 10.0f);
        auto result = controller.SetTargetVid(200);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == Client::Core::EntityError::Dead);
        CHECK_FALSE(controller.HasTarget());
    }

    SUBCASE("SetTargetVid - non-existent target") {
        auto result = controller.SetTargetVid(999);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == Client::Core::EntityError::Dead); // IsActorAlive returns false
    }
}

TEST_CASE("PlayerTargetVidController: Auto Targeting") {
    MockActorQueryProvider provider;
    Client::Player::PlayerTargetVidController controller(&provider);

    SUBCASE("Find closest valid actor") {
        provider.SetActor(10, true, 1500.0f);
        provider.SetActor(11, true, 500.0f); // closest
        provider.SetActor(12, true, 800.0f);
        provider.SetActor(13, false, 100.0f); // dead, ignore

        controller.UpdateAutoTarget(1000.0f);
        CHECK(controller.HasTarget());
        CHECK(controller.GetTargetVid() == 11);
        CHECK(controller.GetMode() == Client::Player::TargetMode::Auto);
    }

    SUBCASE("No actors within radius") {
        provider.SetActor(10, true, 1500.0f);
        controller.UpdateAutoTarget(1000.0f);
        CHECK_FALSE(controller.HasTarget());
    }

    SUBCASE("Keep manual target if still valid and within radius") {
        provider.SetActor(20, true, 300.0f);
        provider.SetActor(21, true, 100.0f); // Closer, but 20 is already targeted

        controller.SetTargetVid(20);
        controller.UpdateAutoTarget(1000.0f);
        
        CHECK(controller.GetTargetVid() == 20); // Didn't switch to 21
        CHECK(controller.GetMode() == Client::Player::TargetMode::Manual);
    }
    
    SUBCASE("Switch auto target if current target is out of radius") {
        provider.SetActor(30, true, 1200.0f); // Currently targeted but out of range
        provider.SetActor(31, true, 200.0f);

        controller.SetTargetVid(30);
        controller.UpdateAutoTarget(1000.0f);
        
        CHECK(controller.GetTargetVid() == 31); // Switched to 31
        CHECK(controller.GetMode() == Client::Player::TargetMode::Auto);
    }
}

TEST_CASE("PlayerTargetVidController: Locking Mechanism") {
    MockActorQueryProvider provider;
    Client::Player::PlayerTargetVidController controller(&provider);

    provider.SetActor(1, true, 500.0f);
    provider.SetActor(2, true, 200.0f);

    controller.SetTargetVid(1);
    controller.LockTarget();
    CHECK(controller.GetMode() == Client::Player::TargetMode::Locked);

    // UpdateAutoTarget should not switch even if actor 2 is closer
    controller.UpdateAutoTarget(1000.0f);
    CHECK(controller.GetTargetVid() == 1);
    CHECK(controller.GetMode() == Client::Player::TargetMode::Locked);

    controller.UnlockTarget();
    CHECK(controller.GetMode() == Client::Player::TargetMode::Manual);
}

TEST_CASE("PlayerTargetVidController: ActorDeadEvent handling") {
    MockActorQueryProvider provider;
    Client::Player::PlayerTargetVidController controller(&provider);

    provider.SetActor(50, true, 300.0f);
    controller.SetTargetVid(50);
    CHECK(controller.GetTargetVid() == 50);

    // Publish death event
    EterBase::EventBus::GetInstance().Publish(Client::Core::ActorDeadEvent{50});

    // Target should be cleared automatically
    CHECK_FALSE(controller.HasTarget());
    CHECK(controller.GetMode() == Client::Player::TargetMode::None);

    // Death of another actor shouldn't affect current target
    provider.SetActor(60, true, 200.0f);
    controller.SetTargetVid(60);
    EterBase::EventBus::GetInstance().Publish(Client::Core::ActorDeadEvent{99});
    CHECK(controller.GetTargetVid() == 60);
}

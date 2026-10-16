#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/IPC/HarnessPlayerRpcRouter.h"

using namespace Client::IPC;
using namespace Client::Core;

TEST_CASE("HarnessPlayerRpcRouter: Routing player RPC actions") {
    HarnessPlayerRpcRouter router;
    router.SetCurrentTime(1000); // Start at 1000ms

    SUBCASE("RouteMoveToPosition successful") {
        router.UpdateCurrentPosition({0.0f, 0.0f, 0.0f});
        auto res = router.RouteMoveToPosition({150.0f, 300.0f, 0.0f}, true);
        REQUIRE(res.has_value());

        const auto& hist = router.GetCommandHistory();
        REQUIRE(hist.size() == 1);
        auto* cmd = std::get_if<CommandMoveToPosition>(&hist[0].payload);
        REQUIRE(cmd != nullptr);
        CHECK(cmd->destination.x == 150.0f);
        CHECK(cmd->destination.y == 300.0f);
        CHECK(cmd->isRunning == true);
        
        CHECK(router.GetState().isMoving == true);
    }

    SUBCASE("RouteMoveToPosition out of bounds") {
        auto res = router.RouteMoveToPosition({2000000.0f, 300.0f, 0.0f});
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::OutOfBounds);
    }

    SUBCASE("RouteMoveToPosition same position") {
        router.UpdateCurrentPosition({150.0f, 300.0f, 0.0f});
        auto res = router.RouteMoveToPosition({150.0f, 300.0f, 0.0f});
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::InvalidState);
    }

    SUBCASE("RouteStop successful") {
        router.RouteMoveToPosition({150.0f, 300.0f, 0.0f});
        auto res = router.RouteStop();
        REQUIRE(res.has_value());
        CHECK(router.GetState().isMoving == false);
        
        const auto& hist = router.GetCommandHistory();
        REQUIRE(hist.size() == 2);
        CHECK(std::holds_alternative<CommandStop>(hist[1].payload));
    }

    SUBCASE("RouteStop invalid state (not moving)") {
        auto res = router.RouteStop();
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::InvalidState);
    }

    SUBCASE("RouteActionAttack successful") {
        auto res = router.RouteActionAttack(EntityVid{5001});
        REQUIRE(res.has_value());
        
        const auto& hist = router.GetCommandHistory();
        REQUIRE(hist.size() == 1);
        auto* cmd = std::get_if<CommandActionAttack>(&hist[0].payload);
        REQUIRE(cmd != nullptr);
        CHECK(cmd->targetVid == EntityVid{5001});
        CHECK(router.GetState().currentTarget == EntityVid{5001});
    }

    SUBCASE("RouteActionAttack invalid target") {
        auto res = router.RouteActionAttack(EntityVid{0});
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::InvalidTarget);
    }

    SUBCASE("RouteActionAttack cooldown spam") {
        router.RouteActionAttack(EntityVid{5001});
        router.SetCurrentTime(1100); // 100ms later
        auto res = router.RouteActionAttack(EntityVid{5002});
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::AntiSpamCooldown);
        
        router.SetCurrentTime(1301); // >300ms later
        auto res2 = router.RouteActionAttack(EntityVid{5002});
        REQUIRE(res2.has_value());
    }

    SUBCASE("RouteActionUseItem successful") {
        auto res = router.RouteActionUseItem(ItemSlot{3});
        REQUIRE(res.has_value());
        
        const auto& hist = router.GetCommandHistory();
        REQUIRE(hist.size() == 1);
        auto* cmd = std::get_if<CommandActionUseItem>(&hist[0].payload);
        REQUIRE(cmd != nullptr);
        CHECK(cmd->slot == ItemSlot{3});
    }

    SUBCASE("RouteActionUseItem invalid slot") {
        auto res = router.RouteActionUseItem(ItemSlot{0xFFFF}); // Invalid
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::ItemSlotInvalid);
        
        auto res2 = router.RouteActionUseItem(ItemSlot{600}); // Over limit
        REQUIRE_FALSE(res2.has_value());
        CHECK(res2.error() == RpcRoutingError::ItemSlotInvalid);
    }

    SUBCASE("RouteSetTarget and RouteClearTarget") {
        auto res = router.RouteSetTarget(EntityVid{123});
        REQUIRE(res.has_value());
        CHECK(router.GetState().currentTarget == EntityVid{123});
        
        auto resClear = router.RouteClearTarget();
        REQUIRE(resClear.has_value());
        CHECK(router.GetState().currentTarget == EntityVid{0});
    }

    SUBCASE("RouteClearTarget without target") {
        auto res = router.RouteClearTarget();
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::NoTarget);
    }
    
    SUBCASE("RouteUseSkill with cooldown") {
        auto res = router.RouteUseSkill(SkillIndex{1}, EntityVid{123});
        REQUIRE(res.has_value());
        
        router.SetCurrentTime(1500); // 500ms later
        auto res2 = router.RouteUseSkill(SkillIndex{1}, EntityVid{123});
        REQUIRE_FALSE(res2.has_value());
        CHECK(res2.error() == RpcRoutingError::SkillOnCooldown);
        
        router.SetCurrentTime(2001); // >1000ms later
        auto res3 = router.RouteUseSkill(SkillIndex{1}, EntityVid{123});
        REQUIRE(res3.has_value());
    }

    SUBCASE("RoutePickCloseItem spam prevention") {
        auto res = router.RoutePickCloseItem();
        REQUIRE(res.has_value());
        
        router.SetCurrentTime(1200); // 200ms later
        auto res2 = router.RoutePickCloseItem();
        REQUIRE_FALSE(res2.has_value());
        CHECK(res2.error() == RpcRoutingError::AntiSpamCooldown);
        
        router.SetCurrentTime(1501); // >500ms later
        auto res3 = router.RoutePickCloseItem();
        REQUIRE(res3.has_value());
    }

    SUBCASE("RouteSendChat successful") {
        auto res = router.RouteSendChat("Hello World");
        REQUIRE(res.has_value());
        
        const auto& hist = router.GetCommandHistory();
        REQUIRE(hist.size() == 1);
        auto* cmd = std::get_if<CommandSendChat>(&hist[0].payload);
        REQUIRE(cmd != nullptr);
        CHECK(cmd->message == "Hello World");
        CHECK(cmd->chatType == 1);
    }

    SUBCASE("RouteSendChat too long") {
        std::string longMsg(600, 'A');
        auto res = router.RouteSendChat(longMsg);
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::MessageTooLong);
    }

    SUBCASE("RouteSendChat anti-spam") {
        for(int i=0; i<5; ++i) {
            router.SetCurrentTime(1000 + i * 10);
            auto res = router.RouteSendChat("Spam");
            REQUIRE(res.has_value());
        }
        
        router.SetCurrentTime(1050);
        auto res = router.RouteSendChat("Spam again");
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == RpcRoutingError::AntiSpamCooldown);
        
        router.SetCurrentTime(3001); // > 2000ms from start
        auto res2 = router.RouteSendChat("Allowed now");
        REQUIRE(res2.has_value());
    }
}

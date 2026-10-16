#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerEffectRegistryBridge.h"

TEST_CASE("PlayerEffectRegistryBridge: Effect registration") {
    Client::Player::PlayerEffectRegistryBridge reg;
    reg.RegisterEffect(1, "d:/ymir work/effect/level_up.mse");
    CHECK(reg.GetEffectPath(1) == "d:/ymir work/effect/level_up.mse");
    CHECK(reg.GetEffectPath(99) == "");
}

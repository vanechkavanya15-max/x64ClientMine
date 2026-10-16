#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerTargetVidController.h"

TEST_CASE("PlayerTargetVidController: Target selection and clear") {
    Client::Player::PlayerTargetVidController target;
    CHECK_FALSE(target.HasTarget());
    target.SetTargetVid(999);
    CHECK(target.HasTarget());
    CHECK(target.GetTargetVid() == 999);
    target.ClearTarget();
    CHECK_FALSE(target.HasTarget());
}

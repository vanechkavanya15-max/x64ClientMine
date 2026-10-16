#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerEntityPicker.h"

TEST_CASE("PlayerEntityPicker: Pick actor and item") {
    Client::Player::PlayerEntityPicker picker;
    CHECK_FALSE(picker.GetPickedActor().has_value());
    picker.SetPickedActor(1234);
    CHECK(picker.GetPickedActor() == 1234);
    picker.ClearPickedActor();
    CHECK_FALSE(picker.GetPickedActor().has_value());
}

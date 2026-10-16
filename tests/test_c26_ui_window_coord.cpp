#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/UIWindowCoordinateTransform.h"

TEST_CASE("UIWindowCoordinateTransform: Transforms and bounding checks") {
    Client::UI::UIWindowCoordinateTransform transform(10, 20, 100, 50);
    auto global = transform.LocalToGlobal({5, 5}, {100, 100});
    CHECK(global.x == 115);
    CHECK(global.y == 125);
    CHECK(transform.ContainsPoint(50, 25));
    CHECK_FALSE(transform.ContainsPoint(105, 25));
}

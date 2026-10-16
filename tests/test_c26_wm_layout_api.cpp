#include <doctest.h>
#include "Client/UI/WindowManagerLayoutApiBridge.h"

using namespace Client::UI;

TEST_CASE("WindowManagerLayoutApiBridge: Lifetime and Hierarchy") {
    WindowManagerLayoutApiBridge bridge;
    
    CHECK(bridge.CreateWindow(1).has_value());
    CHECK(bridge.CreateWindow(2).has_value());
    CHECK(bridge.CreateWindow(3).has_value());
    
    CHECK_FALSE(bridge.CreateWindow(1).has_value()); // Already exists

    // Hierarchy
    CHECK(bridge.AttachWindow(2, 1).has_value());
    CHECK(bridge.AttachWindow(3, 2).has_value());
    
    // Circular dependency check
    auto err = bridge.AttachWindow(1, 3);
    CHECK_FALSE(err.has_value());
    CHECK(err.error() == LayoutError::CircularDependencyDetected);

    // Render Order should be 1 -> 2 -> 3
    auto order = bridge.GetRenderOrder();
    REQUIRE(order.size() == 3);
    CHECK(order[0] == 1);
    CHECK(order[1] == 2);
    CHECK(order[2] == 3);

    // Destroy middle node, child 3 becomes root
    CHECK(bridge.DestroyWindow(2).has_value());
    order = bridge.GetRenderOrder();
    REQUIRE(order.size() == 2);
    // order might be 1, 3 or 3, 1 depending on zOrder (both are 0)
}

TEST_CASE("WindowManagerLayoutApiBridge: Coordinate calculations and alignment") {
    WindowManagerLayoutApiBridge bridge;
    bridge.CreateWindow(1);
    bridge.SetWindowPosition(1, 10, 20);
    bridge.SetWindowSize(1, 100, 50);

    SUBCASE("Left-Top alignment") {
        auto r = bridge.CalculateGlobalRect(1, 800, 600).value();
        CHECK(r.left == 10);
        CHECK(r.top == 20);
        CHECK(r.Width() == 100);
        CHECK(r.Height() == 50);
    }
    
    SUBCASE("Center-Center alignment") {
        bridge.SetHorizontalAlign(1, HorizontalAlign::Center);
        bridge.SetVerticalAlign(1, VerticalAlign::Center);
        auto r = bridge.CalculateGlobalRect(1, 800, 600).value();
        CHECK(r.left == 360); // 800-100 / 2 + 10
        CHECK(r.top == 295);  // 600-50 / 2 + 20
    }
    
    SUBCASE("Right-Bottom alignment") {
        bridge.SetHorizontalAlign(1, HorizontalAlign::Right);
        bridge.SetVerticalAlign(1, VerticalAlign::Bottom);
        auto r = bridge.CalculateGlobalRect(1, 800, 600).value();
        CHECK(r.left == 690); // 800 - 100 - 10
        CHECK(r.top == 530);  // 600 - 50 - 20
    }

    SUBCASE("Stretch alignment with margins") {
        bridge.SetHorizontalAlign(1, HorizontalAlign::Stretch);
        bridge.SetVerticalAlign(1, VerticalAlign::Stretch);
        bridge.SetMargin(1, Margin{5, 10, 15, 20});
        auto r = bridge.CalculateGlobalRect(1, 800, 600).value();
        CHECK(r.left == 15); // 0 + 5(margin) + 10(x)
        CHECK(r.Width() == 770); // 800 - 5(margin.l) - 15(margin.r) - 10(x) -> 770
        CHECK(r.top == 30);  // 0 + 10(margin) + 20(y)
        CHECK(r.Height() == 550); // 600 - 10 - 20 - 20 -> 550
    }
}

TEST_CASE("WindowManagerLayoutApiBridge: Deep hierarchy math") {
    WindowManagerLayoutApiBridge bridge;
    bridge.CreateWindow(1);
    bridge.CreateWindow(2);
    bridge.CreateWindow(3);
    
    bridge.AttachWindow(2, 1);
    bridge.AttachWindow(3, 2);

    bridge.SetWindowPosition(1, 100, 100);
    bridge.SetWindowSize(1, 400, 400);

    bridge.SetWindowPosition(2, 50, 50);
    bridge.SetWindowSize(2, 200, 200);
    
    // Child 3 is centered within Child 2
    bridge.SetWindowSize(3, 50, 50);
    bridge.SetHorizontalAlign(3, HorizontalAlign::Center);
    bridge.SetVerticalAlign(3, VerticalAlign::Center);

    auto r = bridge.CalculateGlobalRect(3, 800, 600).value();
    // Root = (100, 100, 500, 500)
    // Child 2 = (150, 150, 350, 350) - 200x200
    // Child 3 = parentX + (parentW - w)/2 + x => 150 + (200 - 50)/2 + 0 = 225
    CHECK(r.left == 225);
    CHECK(r.top == 225);
    CHECK(r.Width() == 50);
}

TEST_CASE("WindowManagerLayoutApiBridge: Z-Order and HitTest") {
    WindowManagerLayoutApiBridge bridge;
    bridge.CreateWindow(1);
    bridge.CreateWindow(2);
    
    bridge.SetWindowPosition(1, 0, 0);
    bridge.SetWindowSize(1, 100, 100);
    
    bridge.SetWindowPosition(2, 50, 50);
    bridge.SetWindowSize(2, 100, 100);

    // Defaults: 1 and 2 are siblings, order depends on internal but we can set it
    bridge.SetZOrder(1, 10);
    bridge.SetZOrder(2, 20);

    // Overlapping point
    CHECK(bridge.HitTest(75, 75, 800, 600) == 2);

    // Bring 1 to front
    bridge.BringToFront(1);
    CHECK(bridge.HitTest(75, 75, 800, 600) == 1);
    
    // Hit 2 where 1 doesn't overlap
    CHECK(bridge.HitTest(125, 125, 800, 600) == 2);
}

TEST_CASE("WindowManagerLayoutApiBridge: Scissor Rect Intersection") {
    WindowManagerLayoutApiBridge bridge;
    bridge.CreateWindow(1);
    bridge.CreateWindow(2);
    bridge.AttachWindow(2, 1);
    
    bridge.SetWindowPosition(1, 100, 100);
    bridge.SetWindowSize(1, 400, 400);
    bridge.EnableScissorRect(1, true);
    bridge.SetScissorRect(1, Rect2D{0, 0, 200, 200}); // Global: 100,100 -> 300,300
    
    bridge.SetWindowPosition(2, 150, 150);
    bridge.SetWindowSize(2, 400, 400);
    bridge.EnableScissorRect(2, true);
    bridge.SetScissorRect(2, Rect2D{0, 0, 200, 200}); // Global: 250,250 -> 450,450

    auto sr = bridge.CalculateEffectiveScissorRect(2, 800, 600).value();
    REQUIRE(sr.has_value());
    // Intersection of (100,100,300,300) and (250,250,450,450)
    CHECK(sr->left == 250);
    CHECK(sr->top == 250);
    CHECK(sr->right == 300);
    CHECK(sr->bottom == 300);

    // Hit test inside intersection
    CHECK(bridge.HitTest(275, 275, 800, 600) == 2);
    // Hit test outside intersection but inside 2's bounds
    CHECK(bridge.HitTest(350, 350, 800, 600) == std::nullopt); // Root scissor clips it
}

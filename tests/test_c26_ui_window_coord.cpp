#include "doctest/doctest.h"
#include "Client/UI/UIWindowCoordinateTransform.h"

using namespace Client::UI;

TEST_CASE("UIWindowCoordinateTransform: Rect2D intersections and basics") {
    Rect2D r1{10, 10, 50, 50}; // w:40, h:40
    CHECK(r1.GetWidth() == 40);
    CHECK(r1.GetHeight() == 40);
    CHECK(r1.Contains(20, 20));
    CHECK_FALSE(r1.Contains(5, 5));
    CHECK_FALSE(r1.Contains(50, 50)); // Right/Bottom are exclusive

    Rect2D r2{20, 20, 100, 100};
    Rect2D intersect = r1.Intersect(r2);
    CHECK(intersect.left == 20);
    CHECK(intersect.top == 20);
    CHECK(intersect.right == 50);
    CHECK(intersect.bottom == 50);
    CHECK(intersect.IsValid());

    Rect2D r3{60, 60, 100, 100};
    Rect2D noIntersect = r1.Intersect(r3);
    CHECK_FALSE(noIntersect.IsValid());
}

TEST_CASE("UIWindowCoordinateTransform: Basic Anchoring (TopLeft and StretchAll)") {
    TransformContext parentCtx{
        {0, 0, 800, 600}, // parentGlobalRect
        {0, 0, 800, 600}, // parentScissorRect
        {1.0f, 1.0f},     // globalDpiScale
        true              // enableScissorClip
    };

    UIWindowCoordinateTransform transform(0, 0, 100, 50);
    
    SUBCASE("TopLeft Anchor (Default)") {
        transform.SetAnchorType(AnchorType::TopLeft);
        auto state = transform.UpdateTransform(parentCtx);
        
        CHECK(state.globalRect.left == 0);
        CHECK(state.globalRect.top == 0);
        CHECK(state.globalRect.right == 100);
        CHECK(state.globalRect.bottom == 50);
    }

    SUBCASE("TopLeft with Margins") {
        transform.SetAnchorType(AnchorType::TopLeft);
        transform.SetMargins({10, 20, 0, 0});
        auto state = transform.UpdateTransform(parentCtx);
        
        CHECK(state.globalRect.left == 10);
        CHECK(state.globalRect.top == 20);
        CHECK(state.globalRect.right == 110);
        CHECK(state.globalRect.bottom == 70);
    }

    SUBCASE("StretchAll Anchor") {
        transform.SetAnchorType(AnchorType::StretchAll);
        transform.SetMargins({10, 10, 10, 10}); // Left, Top, Right, Bottom
        auto state = transform.UpdateTransform(parentCtx);
        
        CHECK(state.globalRect.left == 10);
        CHECK(state.globalRect.top == 10);
        CHECK(state.globalRect.right == 790); // 800 - 10
        CHECK(state.globalRect.bottom == 590); // 600 - 10
    }
}

TEST_CASE("UIWindowCoordinateTransform: Alignment Anchors") {
    TransformContext parentCtx{
        {100, 100, 500, 500}, // parentGlobalRect (400x400), starts at 100,100
        {100, 100, 500, 500}, // parentScissorRect
        {1.0f, 1.0f},
        true
    };

    UIWindowCoordinateTransform transform;
    transform.SetLocalRect(0, 0, 50, 50);

    SUBCASE("MiddleCenter") {
        transform.SetAnchorType(AnchorType::MiddleCenter);
        auto state = transform.UpdateTransform(parentCtx);
        
        // Parent center is 100 + 400/2 = 300, so half width (25) left is 275
        CHECK(state.globalRect.left == 275);
        CHECK(state.globalRect.top == 275);
        CHECK(state.globalRect.right == 325);
        CHECK(state.globalRect.bottom == 325);
    }

    SUBCASE("BottomRight") {
        transform.SetAnchorType(AnchorType::BottomRight);
        transform.SetMargins({0, 0, 10, 10}); // 10 from right/bottom
        auto state = transform.UpdateTransform(parentCtx);
        
        CHECK(state.globalRect.right == 490); // 500 - 10
        CHECK(state.globalRect.bottom == 490); // 500 - 10
        CHECK(state.globalRect.left == 440); // 490 - 50
        CHECK(state.globalRect.top == 440); // 490 - 50
    }
}

TEST_CASE("UIWindowCoordinateTransform: DPI Scaling") {
    TransformContext parentCtx{
        {0, 0, 1920, 1080},
        {0, 0, 1920, 1080},
        {2.0f, 2.0f}, // 2x DPI scale
        true
    };

    UIWindowCoordinateTransform transform(10, 10, 100, 50);
    transform.SetAnchorType(AnchorType::TopLeft);
    transform.SetMargins({5, 5, 0, 0});
    
    auto state = transform.UpdateTransform(parentCtx);
    
    // Scale applied to local pos (10), size (100x50), margins (5)
    // Left: ml (5*2) + local.x (10*2) = 10 + 20 = 30
    // Top: mt (5*2) + local.y (10*2) = 10 + 20 = 30
    // Right: Left (30) + scaledWidth (100*2) = 230
    // Bottom: Top (30) + scaledHeight (50*2) = 130
    
    CHECK(state.globalRect.left == 30);
    CHECK(state.globalRect.top == 30);
    CHECK(state.globalRect.right == 230);
    CHECK(state.globalRect.bottom == 130);

    // GlobalToLocal test
    Point2D local = transform.GlobalToLocal({130, 80}, state);
    // (130 - 30) / 2 = 50
    // (80 - 30) / 2 = 25
    CHECK(local.x == 50);
    CHECK(local.y == 25);
    
    // LocalToGlobal test
    Point2D global = transform.LocalToGlobal({50, 25}, state);
    CHECK(global.x == 130);
    CHECK(global.y == 80);
}

TEST_CASE("UIWindowCoordinateTransform: Scissor Rect Clipping") {
    TransformContext parentCtx{
        {0, 0, 1000, 1000},
        {100, 100, 400, 400}, // Clip area
        {1.0f, 1.0f},
        true
    };

    UIWindowCoordinateTransform transform(350, 350, 100, 100); // 350->450
    transform.SetAnchorType(AnchorType::TopLeft);
    auto state = transform.UpdateTransform(parentCtx);

    // Should be clipped to parent scissor right/bottom (400)
    CHECK(state.globalRect.right == 450);
    CHECK(state.scissorRect.right == 400);
    CHECK(state.scissorRect.bottom == 400);
    CHECK(state.isVisible);

    // Test hit testing against scissor rect
    CHECK(transform.ContainsGlobalPoint({390, 390}, state)); // Inside scissor
    CHECK_FALSE(transform.ContainsGlobalPoint({410, 410}, state)); // Outside scissor, inside globalRect

    // Completely clipped
    UIWindowCoordinateTransform t2(500, 500, 100, 100);
    auto state2 = t2.UpdateTransform(parentCtx);
    CHECK_FALSE(state2.scissorRect.IsValid());
    CHECK_FALSE(state2.isVisible);
    CHECK_FALSE(t2.ContainsGlobalPoint({550, 550}, state2));
}

TEST_CASE("UIWindowCoordinateTransform: Context Generation for Children (Padding)") {
    TransformContext parentCtx{
        {0, 0, 1000, 1000},
        {0, 0, 1000, 1000},
        {2.0f, 2.0f},
        true
    };

    UIWindowCoordinateTransform transform(0, 0, 100, 100); // Scaled to 200x200
    transform.SetPadding({10, 10, 10, 10}); // Scaled to 20 padding

    auto state = transform.UpdateTransform(parentCtx);
    auto childCtx = transform.GetContentContext(state);

    // Parent global rect was 0,0,200,200
    // With padding of 20, new rect should be 20,20,180,180
    CHECK(childCtx.parentGlobalRect.left == 20);
    CHECK(childCtx.parentGlobalRect.top == 20);
    CHECK(childCtx.parentGlobalRect.right == 180);
    CHECK(childCtx.parentGlobalRect.bottom == 180);
    CHECK(childCtx.globalDpiScale.x == doctest::Approx(2.0f));
}

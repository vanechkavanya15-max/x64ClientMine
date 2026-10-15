#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include "Client/UI/UIWindowLayoutManager.h"
#include <memory>

using namespace Client::UI;

TEST_CASE("UIWindowLayoutManager - ArrangeLinear Horizontal") {
    auto container = std::make_shared<Window>("Container");
    container->SetSize(500, 500);

    auto child1 = std::make_shared<Window>("Child1");
    child1->SetSize(100, 50);
    container->AddChild(child1);

    auto child2 = std::make_shared<Window>("Child2");
    child2->SetSize(120, 60);
    container->AddChild(child2);

    auto child3 = std::make_shared<Window>("Child3");
    child3->SetSize(80, 40);
    child3->SetVisible(false); // Should be ignored
    container->AddChild(child3);

    LayoutConfig config;
    config.padding = 10;
    config.spacing = 5;
    config.alignment = LayoutAlignment::Start;

    auto result = UIWindowLayoutManager::ArrangeLinear(container.get(), LayoutDirection::Horizontal, config);
    CHECK(result.has_value() == true);

    auto pos1 = child1->GetLocalPosition();
    CHECK(pos1.x == 10);
    CHECK(pos1.y == 10);

    auto pos2 = child2->GetLocalPosition();
    CHECK(pos2.x == 10 + 100 + 5); // padding + width + spacing
    CHECK(pos2.y == 10);
}

TEST_CASE("UIWindowLayoutManager - ArrangeLinear Vertical Center") {
    auto container = std::make_shared<Window>("Container");
    container->SetSize(200, 500);

    auto child1 = std::make_shared<Window>("Child1");
    child1->SetSize(100, 50);
    container->AddChild(child1);

    auto child2 = std::make_shared<Window>("Child2");
    child2->SetSize(120, 60);
    container->AddChild(child2);

    LayoutConfig config;
    config.padding = 15;
    config.spacing = 10;
    config.alignment = LayoutAlignment::Center;

    auto result = UIWindowLayoutManager::ArrangeLinear(container.get(), LayoutDirection::Vertical, config);
    CHECK(result.has_value() == true);

    auto pos1 = child1->GetLocalPosition();
    CHECK(pos1.x == (200 - 100) / 2); // Centered
    CHECK(pos1.y == 15);

    auto pos2 = child2->GetLocalPosition();
    CHECK(pos2.x == (200 - 120) / 2); // Centered
    CHECK(pos2.y == 15 + 50 + 10);
}

TEST_CASE("UIWindowLayoutManager - ArrangeGrid") {
    auto container = std::make_shared<Window>("Container");
    
    // Add 5 children for a 2-column grid
    for (int i = 0; i < 5; ++i) {
        auto child = std::make_shared<Window>("Child" + std::to_string(i));
        child->SetSize(100, 50);
        container->AddChild(child);
    }

    LayoutConfig config;
    config.padding = 10;
    config.spacing = 5;

    auto result = UIWindowLayoutManager::ArrangeGrid(container.get(), 2, config);
    CHECK(result.has_value() == true);

    auto children = container->GetChildren();
    
    // Child 0 (row 0, col 0)
    auto pos0 = children[0]->GetLocalPosition();
    CHECK(pos0.x == 10);
    CHECK(pos0.y == 10);

    // Child 1 (row 0, col 1)
    auto pos1 = children[1]->GetLocalPosition();
    CHECK(pos1.x == 10 + 100 + 5);
    CHECK(pos1.y == 10);

    // Child 2 (row 1, col 0)
    auto pos2 = children[2]->GetLocalPosition();
    CHECK(pos2.x == 10);
    CHECK(pos2.y == 10 + 50 + 5);

    // Child 4 (row 2, col 0)
    auto pos4 = children[4]->GetLocalPosition();
    CHECK(pos4.x == 10);
    CHECK(pos4.y == 10 + 50 + 5 + 50 + 5);
}

TEST_CASE("UIWindowLayoutManager - ArrangeGrid Errors") {
    auto container = std::make_shared<Window>("Container");
    
    LayoutConfig config;
    auto res1 = UIWindowLayoutManager::ArrangeGrid(container.get(), 2, config);
    CHECK(res1.has_value() == false);
    CHECK(res1.error() == LayoutError::NoChildren);

    auto child = std::make_shared<Window>("Child");
    container->AddChild(child);

    auto res2 = UIWindowLayoutManager::ArrangeGrid(container.get(), 0, config);
    CHECK(res2.has_value() == false);
    CHECK(res2.error() == LayoutError::InvalidGridColumns);

    auto res3 = UIWindowLayoutManager::ArrangeGrid(nullptr, 2, config);
    CHECK(res3.has_value() == false);
    CHECK(res3.error() == LayoutError::NullContainer);
}

TEST_CASE("UIWindowLayoutManager - CenterInParent") {
    auto container = std::make_shared<Window>("Container");
    container->SetSize(800, 600);

    auto child = std::make_shared<Window>("Child");
    child->SetSize(200, 100);
    container->AddChild(child);

    auto result = UIWindowLayoutManager::CenterInParent(child.get());
    CHECK(result.has_value() == true);

    auto pos = child->GetLocalPosition();
    CHECK(pos.x == (800 - 200) / 2);
    CHECK(pos.y == (600 - 100) / 2);
}

TEST_CASE("UIWindowLayoutManager - FillParent") {
    auto container = std::make_shared<Window>("Container");
    container->SetSize(800, 600);

    auto child = std::make_shared<Window>("Child");
    container->AddChild(child);

    auto result = UIWindowLayoutManager::FillParent(child.get(), 20); // 20 padding
    CHECK(result.has_value() == true);

    auto pos = child->GetLocalPosition();
    CHECK(pos.x == 20);
    CHECK(pos.y == 20);

    auto rect = child->GetLocalRect();
    CHECK(rect.width == 800 - 40);
    CHECK(rect.height == 600 - 40);
}

TEST_CASE("UIWindowLayoutManager - Empty Container Errors") {
    auto container = std::make_shared<Window>("Container");
    auto res = UIWindowLayoutManager::ArrangeLinear(container.get(), LayoutDirection::Horizontal);
    CHECK(res.has_value() == false);
    CHECK(res.error() == LayoutError::NoChildren);

    auto res2 = UIWindowLayoutManager::CenterInParent(nullptr);
    CHECK(res2.has_value() == false);
    CHECK(res2.error() == LayoutError::NullContainer);
}

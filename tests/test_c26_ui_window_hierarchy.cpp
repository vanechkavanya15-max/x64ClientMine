#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/UIWindowHierarchyController.h"

TEST_CASE("UIWindowHierarchyController: Window tree management") {
    auto root = std::make_shared<Client::UI::UIWindowHierarchyController>(1);
    auto child1 = std::make_shared<Client::UI::UIWindowHierarchyController>(2);
    auto child2 = std::make_shared<Client::UI::UIWindowHierarchyController>(3);

    root->AddChild(child1);
    root->AddChild(child2);
    CHECK(root->GetChildCount() == 2);

    root->RemoveChild(2);
    CHECK(root->GetChildCount() == 1);
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/UIWindowHierarchyController.h"

using namespace Client::UI;

TEST_CASE("UIWindowHierarchyController: Basic Parent-Child Management") {
    auto root = std::make_shared<UIWindowHierarchyController>(1);
    auto child1 = std::make_shared<UIWindowHierarchyController>(2);
    auto child2 = std::make_shared<UIWindowHierarchyController>(3);

    SUBCASE("Add and Remove children successfully") {
        auto addRes1 = root->AddChild(child1);
        auto addRes2 = root->AddChild(child2);
        
        CHECK(addRes1.has_value() == true);
        CHECK(addRes2.has_value() == true);
        CHECK(root->GetChildCount() == 2);
        CHECK(child1->GetParent() == root);

        auto rmRes = root->RemoveChild(WindowId{2});
        CHECK(rmRes.has_value() == true);
        CHECK(root->GetChildCount() == 1);
        CHECK(child1->GetParent() == nullptr);
    }
    
    SUBCASE("Remove non-existent child returns NodeNotFound") {
        auto rmRes = root->RemoveChild(WindowId{99});
        CHECK(rmRes.has_value() == false);
        CHECK(rmRes.error() == HierarchyError::NodeNotFound);
    }
}

TEST_CASE("UIWindowHierarchyController: Cycle Detection") {
    auto nodeA = std::make_shared<UIWindowHierarchyController>(1);
    auto nodeB = std::make_shared<UIWindowHierarchyController>(2);
    auto nodeC = std::make_shared<UIWindowHierarchyController>(3);

    nodeA->AddChild(nodeB);
    nodeB->AddChild(nodeC);

    SUBCASE("Adding self as child fails") {
        auto res = nodeA->AddChild(nodeA);
        CHECK(res.has_value() == false);
        CHECK(res.error() == HierarchyError::CycleDetected);
    }

    SUBCASE("Setting ancestor as child fails") {
        auto res = nodeC->AddChild(nodeA);
        CHECK(res.has_value() == false);
        CHECK(res.error() == HierarchyError::CycleDetected);
    }

    SUBCASE("Setting parent to descendant fails") {
        auto res = nodeA->SetParent(nodeC);
        CHECK(res.has_value() == false);
        CHECK(res.error() == HierarchyError::CycleDetected);
    }
}

TEST_CASE("UIWindowHierarchyController: Z-Order and Ordering") {
    auto root = std::make_shared<UIWindowHierarchyController>(1);
    auto child1 = std::make_shared<UIWindowHierarchyController>(2);
    auto child2 = std::make_shared<UIWindowHierarchyController>(3);
    auto child3 = std::make_shared<UIWindowHierarchyController>(4);

    root->AddChild(child1);
    root->AddChild(child2);
    root->AddChild(child3);

    SUBCASE("Sort Z-Order manually via SetZIndex") {
        child3->SetZIndex(-1); // Should be first
        child1->SetZIndex(5);  // Should be last
        child2->SetZIndex(0);  // Should be middle
        
        // Z-Index change auto triggers parent SortZOrder
        auto children = root->GetChildren();
        CHECK(children[0]->GetId() == WindowId{4});
        CHECK(children[1]->GetId() == WindowId{3});
        CHECK(children[2]->GetId() == WindowId{2});
    }

    SUBCASE("BringToFront and SendToBack") {
        root->BringToFront(WindowId{2});
        auto children = root->GetChildren();
        CHECK(children.back()->GetId() == WindowId{2});

        root->SendToBack(WindowId{2});
        children = root->GetChildren();
        CHECK(children.front()->GetId() == WindowId{2});
    }

    SUBCASE("GetRenderOrder PreOrder flattening") {
        auto subChild = std::make_shared<UIWindowHierarchyController>(5);
        child1->AddChild(subChild);

        auto order = root->GetRenderOrder();
        // Expected PreOrder: Root, Child1, SubChild, Child2, Child3
        REQUIRE(order.size() == 5);
        CHECK(order[0]->GetId() == WindowId{1});
        CHECK(order[1]->GetId() == WindowId{2});
        CHECK(order[2]->GetId() == WindowId{5});
        CHECK(order[3]->GetId() == WindowId{3});
        CHECK(order[4]->GetId() == WindowId{4});
    }
}

TEST_CASE("UIWindowHierarchyController: RemoveDescendantsIf") {
    auto root = std::make_shared<UIWindowHierarchyController>(1);
    auto child1 = std::make_shared<UIWindowHierarchyController>(2);
    auto child2 = std::make_shared<UIWindowHierarchyController>(3);
    auto subChild = std::make_shared<UIWindowHierarchyController>(4);

    root->AddChild(child1);
    root->AddChild(child2);
    child1->AddChild(subChild);

    SUBCASE("Predicate removes specific descendants") {
        // Remove nodes with ID > 2
        auto res = root->RemoveDescendantsIf([](const UIWindowHierarchyController& node) {
            return node.GetId().get() > 2;
        });

        CHECK(res.has_value() == true);
        CHECK(root->GetChildCount() == 1);
        CHECK(root->GetChildren()[0]->GetId() == WindowId{2});
        CHECK(child1->GetChildCount() == 0); // subChild removed
    }
}

TEST_CASE("UIWindowHierarchyController: Reservation System") {
    auto root = std::make_shared<UIWindowHierarchyController>(1);
    auto child = std::make_shared<UIWindowHierarchyController>(2);
    auto targetParent = std::make_shared<UIWindowHierarchyController>(3);

    SUBCASE("Add and Remove reservations committed") {
        CHECK(root->ReserveAddChild(child).has_value());
        CHECK(root->GetChildCount() == 0); // Not added yet

        CHECK(root->CommitReservations().has_value());
        CHECK(root->GetChildCount() == 1); // Now added

        CHECK(root->ReserveRemoveChild(WindowId{2}).has_value());
        CHECK(root->CommitReservations().has_value());
        CHECK(root->GetChildCount() == 0);
    }
    
    SUBCASE("SetParent reservation committed") {
        CHECK(child->ReserveSetParent(targetParent).has_value());
        CHECK(child->GetParent() == nullptr);
        
        CHECK(child->CommitReservations().has_value());
        CHECK(child->GetParent() == targetParent);
        CHECK(targetParent->GetChildCount() == 1);
    }

    SUBCASE("Clear reservations aborts action") {
        CHECK(root->ReserveAddChild(child).has_value());
        root->ClearReservations();
        CHECK(root->CommitReservations().has_value());
        CHECK(root->GetChildCount() == 0);
    }
}

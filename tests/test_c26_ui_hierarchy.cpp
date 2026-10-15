#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#define TEST_MODE_DISABLE_STDAFX
#include "mock_includes/gtest/gtest.h"
#include "../src/Client/UI/UIWindowHierarchyTree.h"

using namespace Client::UI;
using Client::Core::StrongType;

TEST(UIWindowHierarchyTreeTest, BasicAddAndRemove) {
    UIWindowHierarchyTree tree;
    WindowId id1{1};
    WindowId id2{2};
    WindowId id3{0}; // Invalid

    // Invalid ID
    auto resInvalid = tree.AddWindow(id3);
    EXPECT_FALSE(resInvalid.has_value());
    EXPECT_TRUE(resInvalid.error() == HierarchyError::InvalidWindowId);

    // Normal Add
    auto resAdd = tree.AddWindow(id1);
    EXPECT_TRUE(resAdd.has_value());

    // Add existing
    auto resExisting = tree.AddWindow(id1);
    EXPECT_FALSE(resExisting.has_value());
    EXPECT_TRUE(resExisting.error() == HierarchyError::WindowAlreadyExists);

    // Remove
    auto resRemove = tree.RemoveWindow(id1);
    EXPECT_TRUE(resRemove.has_value());

    // Remove not existing
    auto resRemoveNot = tree.RemoveWindow(id2);
    EXPECT_FALSE(resRemoveNot.has_value());
    EXPECT_TRUE(resRemoveNot.error() == HierarchyError::WindowNotFound);
}

TEST(UIWindowHierarchyTreeTest, HierarchyAndCycle) {
    UIWindowHierarchyTree tree;
    WindowId root{1};
    WindowId child1{2};
    WindowId child2{3};

    tree.AddWindow(root);
    tree.AddWindow(child1);
    tree.AddWindow(child2);

    auto resSet = tree.SetParent(child1, root);
    EXPECT_TRUE(resSet.has_value());

    auto resSet2 = tree.SetParent(child2, child1);
    EXPECT_TRUE(resSet2.has_value());

    // Check ancestor
    auto isAncestor1 = tree.IsAncestor(root, child2);
    EXPECT_TRUE(isAncestor1.has_value());
    EXPECT_TRUE(isAncestor1.value());

    auto isAncestor2 = tree.IsAncestor(child2, root);
    EXPECT_TRUE(isAncestor2.has_value());
    EXPECT_FALSE(isAncestor2.value());

    // Cycle detection: trying to set root's parent to child2
    auto resCycle = tree.SetParent(root, child2);
    EXPECT_FALSE(resCycle.has_value());
    EXPECT_TRUE(resCycle.error() == HierarchyError::CycleDetected);

    // Cycle detection: trying to set child1's parent to child1
    auto resCycleSelf = tree.SetParent(child1, child1);
    EXPECT_FALSE(resCycleSelf.has_value());
    EXPECT_TRUE(resCycleSelf.error() == HierarchyError::CycleDetected);
}

TEST(UIWindowHierarchyTreeTest, Ordering) {
    UIWindowHierarchyTree tree;
    WindowId root{1};
    WindowId child1{2};
    WindowId child2{3};
    WindowId child3{4};

    tree.AddWindow(root);
    tree.AddWindow(child1);
    tree.AddWindow(child2);
    tree.AddWindow(child3);

    tree.SetParent(child1, root);
    tree.SetParent(child2, root);
    tree.SetParent(child3, root);

    auto children = tree.GetChildren(root);
    EXPECT_TRUE(children.has_value());
    EXPECT_TRUE(children.value().size() == 3);
    EXPECT_TRUE(children.value()[0] == child1);
    EXPECT_TRUE(children.value()[1] == child2);
    EXPECT_TRUE(children.value()[2] == child3);

    // Bring to front
    auto resFront = tree.BringToFront(child1);
    EXPECT_TRUE(resFront.has_value());

    children = tree.GetChildren(root);
    EXPECT_TRUE(children.value()[0] == child2);
    EXPECT_TRUE(children.value()[1] == child3);
    EXPECT_TRUE(children.value()[2] == child1);

    // Send to back
    auto resBack = tree.SendToBack(child1);
    EXPECT_TRUE(resBack.has_value());

    children = tree.GetChildren(root);
    EXPECT_TRUE(children.value()[0] == child1);
    EXPECT_TRUE(children.value()[1] == child2);
    EXPECT_TRUE(children.value()[2] == child3);
}

TEST(UIWindowHierarchyTreeTest, CascadeRemove) {
    UIWindowHierarchyTree tree;
    WindowId root{1};
    WindowId child1{2};
    
    tree.AddWindow(root);
    tree.AddWindow(child1);
    tree.SetParent(child1, root);
    
    tree.RemoveWindow(root);
    
    // Child should be orphaned but still exist
    auto res = tree.GetParent(child1);
    EXPECT_FALSE(res.has_value());
    
    // Root should not exist
    auto children = tree.GetChildren(root);
    EXPECT_FALSE(children.has_value());
    EXPECT_TRUE(children.error() == HierarchyError::WindowNotFound);
}

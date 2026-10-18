#include "UiWindowHierarchy.hpp"
#include <iostream>
#include <cassert>
#include <vector>
#include <memory_resource>

class TrackingMemoryResource : public std::pmr::memory_resource {
public:
    size_t allocatedBytes{0};
    size_t deallocatedBytes{0};
    size_t allocationCount{0};
    size_t deallocationCount{0};

protected:
    void* do_allocate(size_t bytes, size_t alignment) override {
        allocatedBytes += bytes;
        allocationCount++;
        return ::operator new(bytes, std::align_val_t{alignment});
    }

    void do_deallocate(void* p, size_t bytes, size_t alignment) override {
        deallocatedBytes += bytes;
        deallocationCount++;
        ::operator delete(p, std::align_val_t{alignment});
    }

    bool do_is_equal(const std::pmr::memory_resource& other) const noexcept override {
        return this == &other;
    }
};

void Test_NodeLifecycleAndPmr() {
    TrackingMemoryResource tracker;
    {
        Client::UI::UiWindowHierarchy hierarchy(&tracker);
        assert(hierarchy.GetNodeCount() == 0);

        auto id1 = hierarchy.CreateNode("RootWindow", Client::UI::WindowFlags::Movable);
        assert(hierarchy.GetNodeCount() == 1);
        assert(hierarchy.ContainsNode(id1));

        auto* node1 = hierarchy.GetNode(id1);
        assert(node1 != nullptr);
        assert(node1->name == "RootWindow");
        assert(Client::UI::HasFlag(node1->flags, Client::UI::WindowFlags::Movable));

        auto id2 = hierarchy.CreateChildNode(id1, "Child1");
        assert(hierarchy.GetNodeCount() == 2);
        assert(hierarchy.GetParent(id2) == id1);
        assert(hierarchy.GetChildCount(id1) == 1);

        bool destroyed = hierarchy.DestroyNode(id1, true);
        assert(destroyed);
        assert(hierarchy.GetNodeCount() == 0);
    }
    // Verify RAII & PMR memory integrity: zero leaks
    assert(tracker.allocatedBytes == tracker.deallocatedBytes);
    assert(tracker.allocationCount == tracker.deallocationCount);
    std::cout << "[OK] Test_NodeLifecycleAndPmr (Allocations: " << tracker.allocationCount << ", Bytes: " << tracker.allocatedBytes << ")\n";
}

void Test_TreeInsertionAndParentChild() {
    Client::UI::UiWindowHierarchy hierarchy;

    auto root = hierarchy.CreateNode("Root");
    auto layerGame = hierarchy.CreateChildNode(root, "GAME");
    auto layerUi = hierarchy.CreateChildNode(root, "UI");
    auto dialog = hierarchy.CreateChildNode(layerUi, "InventoryDialog");
    auto btnClose = hierarchy.CreateChildNode(dialog, "CloseButton");

    assert(hierarchy.GetRoot(btnClose) == root);
    assert(hierarchy.GetRoot(dialog) == root);
    assert(hierarchy.GetRoot(root) == root);

    assert(hierarchy.GetParent(btnClose) == dialog);
    assert(hierarchy.GetParent(dialog) == layerUi);

    assert(hierarchy.IsChildOf(btnClose, dialog));
    assert(!hierarchy.IsChildOf(btnClose, layerUi));

    assert(hierarchy.IsDescendantOf(btnClose, root));
    assert(hierarchy.IsDescendantOf(btnClose, layerUi));
    assert(hierarchy.IsDescendantOf(btnClose, dialog));
    assert(!hierarchy.IsDescendantOf(dialog, btnClose));

    assert(hierarchy.IsAncestorOf(root, btnClose));
    assert(hierarchy.IsAncestorOf(layerUi, btnClose));

    auto childrenOfUi = hierarchy.GetChildren(layerUi);
    assert(childrenOfUi.size() == 1);
    assert(childrenOfUi[0] == dialog);

    std::cout << "[OK] Test_TreeInsertionAndParentChild\n";
}

void Test_ReparentingAndCyclePrevention() {
    Client::UI::UiWindowHierarchy hierarchy;

    auto nodeA = hierarchy.CreateNode("NodeA");
    auto nodeB = hierarchy.CreateChildNode(nodeA, "NodeB");
    auto nodeC = hierarchy.CreateChildNode(nodeB, "NodeC");

    // Valid reparenting
    assert(hierarchy.GetParent(nodeC) == nodeB);
    bool reparentOk = hierarchy.SetParent(nodeC, nodeA);
    assert(reparentOk);
    assert(hierarchy.GetParent(nodeC) == nodeA);
    assert(hierarchy.GetChildCount(nodeB) == 0);
    assert(hierarchy.GetChildCount(nodeA) == 2);

    // Detach from parent
    hierarchy.DetachFromParent(nodeC);
    assert(hierarchy.GetParent(nodeC) == Client::UI::WindowId{0});
    assert(hierarchy.GetChildCount(nodeA) == 1);

    // Restore hierarchy for cycle check: NodeA -> NodeB -> NodeC
    hierarchy.SetParent(nodeC, nodeB);
    assert(hierarchy.GetParent(nodeC) == nodeB);

    // Cycle check: attempt to make NodeA a child of NodeC
    bool cycleDetected = hierarchy.SetParent(nodeA, nodeC);
    assert(!cycleDetected && "Cycle must be rejected");
    assert(hierarchy.GetParent(nodeA) == Client::UI::WindowId{0});

    // Self-parenting check
    assert(!hierarchy.SetParent(nodeA, nodeA));

    std::cout << "[OK] Test_ReparentingAndCyclePrevention\n";
}

void Test_ZOrderSortingAndBringToFront() {
    Client::UI::UiWindowHierarchy hierarchy;

    auto root = hierarchy.CreateNode("Root");
    auto win1 = hierarchy.CreateChildNode(root, "Win1", Client::UI::WindowFlags::Float, 10);
    auto win2 = hierarchy.CreateChildNode(root, "Win2", Client::UI::WindowFlags::Float, 20);
    auto win3 = hierarchy.CreateChildNode(root, "Win3", Client::UI::WindowFlags::Float, 30);

    auto children = hierarchy.GetChildren(root);
    assert(children[0] == win1);
    assert(children[1] == win2);
    assert(children[2] == win3);

    // Bring win1 to front (moves to back of container)
    hierarchy.BringToFront(win1);
    children = hierarchy.GetChildren(root);
    assert(children[0] == win2);
    assert(children[1] == win3);
    assert(children[2] == win1);

    // Send win1 to back (moves to front of container)
    hierarchy.SendToBack(win1);
    children = hierarchy.GetChildren(root);
    assert(children[0] == win1);
    assert(children[1] == win2);
    assert(children[2] == win3);

    // Custom z-index sort
    hierarchy.SetZIndex(win1, 100);
    hierarchy.SetZIndex(win2, 5);
    hierarchy.SetZIndex(win3, 50);
    hierarchy.SortChildrenByZOrder(root);

    children = hierarchy.GetChildren(root);
    assert(children[0] == win2); // z=5
    assert(children[1] == win3); // z=50
    assert(children[2] == win1); // z=100

    std::cout << "[OK] Test_ZOrderSortingAndBringToFront\n";
}

void Test_FocusStackAndModalLock() {
    Client::UI::UiWindowHierarchy hierarchy;

    auto w1 = hierarchy.CreateNode("Window1");
    auto w2 = hierarchy.CreateNode("Window2");
    auto w3 = hierarchy.CreateNode("Window3");

    // Active window focus transitions
    hierarchy.SetActiveWindow(w1);
    assert(hierarchy.GetActiveWindow() == w1);

    hierarchy.SetActiveWindow(w2);
    assert(hierarchy.GetActiveWindow() == w2);
    assert(hierarchy.GetActiveWindowHistory().size() == 1);
    assert(hierarchy.GetActiveWindowHistory()[0] == w1);

    hierarchy.SetActiveWindow(w3);
    assert(hierarchy.GetActiveWindow() == w3);
    assert(hierarchy.GetActiveWindowHistory().size() == 2);

    // Deactivate pops back
    hierarchy.DeactivateWindow();
    assert(hierarchy.GetActiveWindow() == w2);

    hierarchy.DeactivateWindow();
    assert(hierarchy.GetActiveWindow() == w1);

    hierarchy.DeactivateWindow();
    assert(hierarchy.GetActiveWindow() == Client::UI::WindowId{0});

    // Modal lock stack
    auto modal1 = hierarchy.CreateNode("Modal1", Client::UI::WindowFlags::Modal);
    auto modal2 = hierarchy.CreateNode("Modal2", Client::UI::WindowFlags::Modal);

    assert(!hierarchy.IsModalLocked());
    hierarchy.LockWindow(modal1);
    assert(hierarchy.IsModalLocked());
    assert(hierarchy.GetLockWindow() == modal1);

    hierarchy.LockWindow(modal2);
    assert(hierarchy.GetLockWindow() == modal2);

    hierarchy.UnlockWindow();
    assert(hierarchy.GetLockWindow() == modal1);

    hierarchy.UnlockWindow();
    assert(!hierarchy.IsModalLocked());
    assert(hierarchy.GetLockWindow() == Client::UI::WindowId{0});

    std::cout << "[OK] Test_FocusStackAndModalLock\n";
}

void Test_FocusTraversal() {
    Client::UI::UiWindowHierarchy hierarchy;

    auto root = hierarchy.CreateNode("Root");
    auto btn1 = hierarchy.CreateChildNode(root, "Btn1", Client::UI::WindowFlags::Focusable);
    auto label = hierarchy.CreateChildNode(root, "Label"); // Not focusable
    auto btn2 = hierarchy.CreateChildNode(root, "Btn2", Client::UI::WindowFlags::Focusable);
    auto btn3 = hierarchy.CreateChildNode(root, "Btn3", Client::UI::WindowFlags::Focusable);

    auto next = hierarchy.GetNextFocusable(btn1);
    assert(next.has_value() && *next == btn2);

    next = hierarchy.GetNextFocusable(btn2);
    assert(next.has_value() && *next == btn3);

    // Wrap-around
    next = hierarchy.GetNextFocusable(btn3);
    assert(next.has_value() && *next == btn1);

    // Reverse (prev) wrap-around
    auto prev = hierarchy.GetPrevFocusable(btn1);
    assert(prev.has_value() && *prev == btn3);

    prev = hierarchy.GetPrevFocusable(btn3);
    assert(prev.has_value() && *prev == btn2);

    // Visibility toggle hides from tab navigation
    hierarchy.SetVisible(btn2, false);
    next = hierarchy.GetNextFocusable(btn1);
    assert(next.has_value() && *next == btn3);

    std::cout << "[OK] Test_FocusTraversal\n";
}

void Test_CoordinateLayoutAndScissor() {
    Client::UI::UiWindowHierarchy hierarchy;

    auto root = hierarchy.CreateNode("Root");
    hierarchy.SetPosition(root, 0, 0);
    hierarchy.SetSize(root, 1024, 768);

    auto parent = hierarchy.CreateChildNode(root, "ParentPanel");
    hierarchy.SetPosition(parent, 100, 50);
    hierarchy.SetSize(parent, 400, 300);

    // Top-Left aligned child
    auto childTL = hierarchy.CreateChildNode(parent, "ChildTL");
    hierarchy.SetPosition(childTL, 10, 20);
    hierarchy.SetSize(childTL, 50, 50);
    hierarchy.SetAlign(childTL, Client::UI::HorizontalAlign::Left, Client::UI::VerticalAlign::Top);

    auto* nTL = hierarchy.GetNode(childTL);
    assert(nTL->globalRect.left == 110);
    assert(nTL->globalRect.top == 70);
    assert(nTL->globalRect.right == 160);
    assert(nTL->globalRect.bottom == 120);

    // Center-Center aligned child
    auto childCenter = hierarchy.CreateChildNode(parent, "ChildCenter");
    hierarchy.SetPosition(childCenter, 0, 0);
    hierarchy.SetSize(childCenter, 100, 100);
    hierarchy.SetAlign(childCenter, Client::UI::HorizontalAlign::Center, Client::UI::VerticalAlign::Center);

    auto* nC = hierarchy.GetNode(childCenter);
    // Left: 100 + (400 - 100) / 2 = 250
    // Top: 50 + (300 - 100) / 2 = 150
    assert(nC->globalRect.left == 250);
    assert(nC->globalRect.top == 150);

    // Scissor hierarchy test
    hierarchy.SetEnableScissor(parent, true);
    auto scissor = hierarchy.ComputeActiveScissor(childTL);
    assert(scissor.has_value());
    assert(scissor->left == 100);
    assert(scissor->top == 50);
    assert(scissor->right == 500);
    assert(scissor->bottom == 350);

    std::cout << "[OK] Test_CoordinateLayoutAndScissor\n";
}

void Test_PickingAndHitTesting() {
    Client::UI::UiWindowHierarchy hierarchy;

    auto root = hierarchy.CreateNode("Root");
    hierarchy.SetPosition(root, 0, 0);
    hierarchy.SetSize(root, 800, 600);

    auto panelA = hierarchy.CreateChildNode(root, "PanelA");
    hierarchy.SetPosition(panelA, 50, 50);
    hierarchy.SetSize(panelA, 200, 200);

    auto panelB = hierarchy.CreateChildNode(root, "PanelB");
    hierarchy.SetPosition(panelB, 100, 100);
    hierarchy.SetSize(panelB, 200, 200);

    // Overlapping point (150, 150) hits both. PanelB is top-most child of root
    auto picked = hierarchy.PickWindow(150, 150, root);
    assert(picked.has_value() && *picked == panelB);

    // NotPick flag
    hierarchy.AddFlags(panelB, Client::UI::WindowFlags::NotPick);
    picked = hierarchy.PickWindow(150, 150, root);
    assert(picked.has_value() && *picked == panelA);

    // Modal picking
    auto modal = hierarchy.CreateChildNode(root, "ModalDlg");
    hierarchy.SetPosition(modal, 400, 400);
    hierarchy.SetSize(modal, 100, 100);
    hierarchy.LockWindow(modal);

    // When modal locked, picking outside modal returns nullopt
    picked = hierarchy.PickWindow(50, 50, root);
    assert(!picked.has_value());

    // Picking inside modal succeeds
    picked = hierarchy.PickWindow(450, 450, root);
    assert(picked.has_value() && *picked == modal);

    std::cout << "[OK] Test_PickingAndHitTesting\n";
}

int main() {
    std::cout << "Running test_c26_ui_window_hierarchy (C++23 std::pmr / modern ranges)...\n";

    Test_NodeLifecycleAndPmr();
    Test_TreeInsertionAndParentChild();
    Test_ReparentingAndCyclePrevention();
    Test_ZOrderSortingAndBringToFront();
    Test_FocusStackAndModalLock();
    Test_FocusTraversal();
    Test_CoordinateLayoutAndScissor();
    Test_PickingAndHitTesting();

    std::cout << "\n==================================================\n";
    std::cout << "test_c26_ui_window_hierarchy: ALL TESTS PASSED (100%)\n";
    std::cout << "==================================================\n";
    return 0;
}

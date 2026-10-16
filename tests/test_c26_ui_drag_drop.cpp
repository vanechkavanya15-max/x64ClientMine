#include "doctest.h"
#include "Client/UI/UIDragDropController.h"
#include "Client/UI/WindowHierarchy.h"

using namespace Client::UI;
using namespace Client::Core;

class MockDropTargetWindow : public Window {
public:
    MockDropTargetWindow(std::string name) : Window(std::move(name)) {}

    bool OnDragDrop(Window* source, const DragDropPayload& payload) override {
        if (payload.dataType == "item") {
            receivedPayload = payload;
            return true;
        }
        return false;
    }

    std::optional<DragDropPayload> receivedPayload;
};

TEST_CASE("UIDragDropController functionality") {
    UIDragDropController controller;

    SUBCASE("Initial state") {
        CHECK_FALSE(controller.IsDragging());
        CHECK(controller.GetDragSource() == nullptr);
    }

    SUBCASE("StartDrag handles null source") {
        auto result = controller.StartDrag(nullptr);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == DragDropError::NotDraggable);
    }

    SUBCASE("StartDrag handles not draggable source") {
        Window source("source");
        source.SetDraggable(false);
        auto result = controller.StartDrag(&source);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == DragDropError::NotDraggable);
    }

    SUBCASE("StartDrag handles missing payload") {
        Window source("source");
        source.SetDraggable(true);
        // No payload set
        auto result = controller.StartDrag(&source);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == DragDropError::NoPayload);
    }

    SUBCASE("StartDrag success") {
        Window source("source");
        source.SetDraggable(true);
        source.SetDragPayload({"item", 123});

        auto result = controller.StartDrag(&source);
        CHECK(result.has_value());
        CHECK(controller.IsDragging());
        CHECK(controller.GetDragSource() == &source);
        
        SUBCASE("Already dragging") {
            Window source2("source2");
            source2.SetDraggable(true);
            source2.SetDragPayload({"item", 456});
            
            auto result2 = controller.StartDrag(&source2);
            CHECK_FALSE(result2.has_value());
            CHECK(result2.error() == DragDropError::AlreadyDragging);
        }
        
        SUBCASE("Cancel drag") {
            auto cancelResult = controller.CancelDrag();
            CHECK(cancelResult.has_value());
            CHECK_FALSE(controller.IsDragging());
            CHECK(controller.GetDragSource() == nullptr);
        }

        SUBCASE("Drop on valid target") {
            MockDropTargetWindow target("target");
            auto dropResult = controller.Drop(&target);
            
            CHECK(dropResult.has_value());
            CHECK_FALSE(controller.IsDragging());
            CHECK(controller.GetDragSource() == nullptr);
            CHECK(target.receivedPayload.has_value());
            CHECK(target.receivedPayload->dataType == "item");
            CHECK(target.receivedPayload->dataId == 123);
        }

        SUBCASE("Drop on target that rejects") {
            Window source2("source2");
            source2.SetDraggable(true);
            source2.SetDragPayload({"unknown", 999});
            
            // Cancel previous drag and start new one
            controller.CancelDrag();
            controller.StartDrag(&source2);
            
            MockDropTargetWindow target("target");
            auto dropResult = controller.Drop(&target);
            
            CHECK_FALSE(dropResult.has_value());
            CHECK(dropResult.error() == DragDropError::DropRejected);
            CHECK_FALSE(controller.IsDragging()); // Should still clear state on rejection
        }

        SUBCASE("Drop on null target") {
            auto dropResult = controller.Drop(nullptr);
            CHECK_FALSE(dropResult.has_value());
            CHECK(dropResult.error() == DragDropError::DropTargetNotFound);
            CHECK_FALSE(controller.IsDragging());
        }
    }

    SUBCASE("CancelDrag when not dragging") {
        auto result = controller.CancelDrag();
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == DragDropError::NotDragging);
    }
    
    SUBCASE("Drop when not dragging") {
        MockDropTargetWindow target("target");
        auto dropResult = controller.Drop(&target);
        
        CHECK_FALSE(dropResult.has_value());
        CHECK(dropResult.error() == DragDropError::NotDragging);
    }
}

#pragma once

#include "Client/UI/WindowHierarchy.h"
#include "Client/Core/Result.h"
#include <optional>
#include <string_view>
#include <format>
#include <expected>

namespace Client::UI {

enum class DragDropError : uint8_t {
    NotDraggable,
    NoPayload,
    AlreadyDragging,
    NotDragging,
    DropTargetNotFound,
    DropRejected
};

[[nodiscard]] constexpr std::string_view to_string(DragDropError err) noexcept {
    switch (err) {
        case DragDropError::NotDraggable: return "NotDraggable";
        case DragDropError::NoPayload: return "NoPayload";
        case DragDropError::AlreadyDragging: return "AlreadyDragging";
        case DragDropError::NotDragging: return "NotDragging";
        case DragDropError::DropTargetNotFound: return "DropTargetNotFound";
        case DragDropError::DropRejected: return "DropRejected";
        default: return "Unknown";
    }
}

class UIDragDropController {
public:
    UIDragDropController() = default;
    ~UIDragDropController() = default;

    Client::Core::Result<void, DragDropError> StartDrag(Window* source);
    Client::Core::Result<void, DragDropError> Drop(Window* target);
    Client::Core::Result<void, DragDropError> CancelDrag();

    [[nodiscard]] bool IsDragging() const;
    [[nodiscard]] Window* GetDragSource() const;

private:
    Window* m_dragSource = nullptr;
};

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::DragDropError> : std::formatter<std::string_view> {
    auto format(Client::UI::DragDropError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::to_string(err), ctx);
    }
};

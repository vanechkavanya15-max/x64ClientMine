#include "UiEventDispatcher.hpp"
#include <cmath>

namespace Client::UI {

    void UiEventDispatcher::SetScreenSize(int32_t width, int32_t height) noexcept {
        screen_width_ = width;
        screen_height_ = height;
    }

    void UiEventDispatcher::SetResolution(int32_t hres, int32_t vres) noexcept {
        if (hres > 0 && vres > 0) {
            virtual_hres_ = hres;
            virtual_vres_ = vres;
        }
    }

    Point UiEventDispatcher::TransformCoords(int32_t screen_x, int32_t screen_y) const noexcept {
        if (virtual_hres_ <= 0 || virtual_vres_ <= 0) {
            return Point{screen_x, screen_y};
        }
        return Point{
            .x = screen_width_ * screen_x / virtual_hres_,
            .y = screen_height_ * screen_y / virtual_vres_
        };
    }

    bool UiEventDispatcher::IsDragging() const noexcept {
        const int32_t ldx = std::abs(current_mouse_pos_.x - picked_mouse_pos_.x);
        const int32_t ldy = std::abs(current_mouse_pos_.y - picked_mouse_pos_.y);
        return (ldx + ldy) >= DragThresholdDistance;
    }

    bool UiEventDispatcher::AddLayer(IUiWindow* layer) noexcept {
        if (!layer || layers_.contains(layer)) return false;
        return layers_.push_back(layer);
    }

    bool UiEventDispatcher::RemoveLayer(IUiWindow* layer) noexcept {
        if (!layer) return false;
        return layers_.remove(layer);
    }

    bool UiEventDispatcher::AddPickAlways(IUiWindow* window) noexcept {
        if (!window || pick_always_.contains(window)) return false;
        return pick_always_.push_back(window);
    }

    bool UiEventDispatcher::RemovePickAlways(IUiWindow* window) noexcept {
        if (!window) return false;
        return pick_always_.remove(window);
    }

    void UiEventDispatcher::ActivateWindow(IUiWindow* window) noexcept {
        if (window == active_window_) return;

        if (active_window_) {
            focus_stack_.remove(window);
            focus_stack_.push_back(active_window_);
            active_window_->OnKillFocus();
        }

        active_window_ = window;
        if (active_window_) {
            active_window_->OnSetFocus();
        }
    }

    void UiEventDispatcher::DeactivateWindow() noexcept {
        if (!active_window_) return;

        active_window_->OnKillFocus();
        active_window_ = nullptr;

        while (!focus_stack_.empty()) {
            IUiWindow* candidate = focus_stack_.back();
            focus_stack_.pop_back();
            if (candidate && candidate->IsVisible()) {
                active_window_ = candidate;
                active_window_->OnSetFocus();
                break;
            }
        }
    }

    void UiEventDispatcher::PushModalLock(IUiWindow* window) noexcept {
        if (!window || window == lock_window_) return;

        if (active_window_) {
            active_window_->OnKillFocus();
        }

        if (lock_window_) {
            modal_stack_.push_back(lock_window_);
        }
        lock_window_ = window;
    }

    void UiEventDispatcher::PopModalLock() noexcept {
        if (!lock_window_) return;

        if (!modal_stack_.empty()) {
            lock_window_ = modal_stack_.back();
            modal_stack_.pop_back();
        } else {
            lock_window_ = nullptr;
            if (active_window_) {
                active_window_->OnSetFocus();
            }
        }
    }

    void UiEventDispatcher::ResetCapture() noexcept {
        left_capture_window_ = nullptr;
        right_capture_window_ = nullptr;
        middle_capture_window_ = nullptr;
    }

    void UiEventDispatcher::NotifyDestroyWindow(IUiWindow* window) noexcept {
        if (!window) return;

        if (window == active_window_) active_window_ = nullptr;
        if (window == point_window_) point_window_ = nullptr;
        if (window == left_capture_window_) left_capture_window_ = nullptr;
        if (window == right_capture_window_) right_capture_window_ = nullptr;
        if (window == middle_capture_window_) middle_capture_window_ = nullptr;
        if (window == lock_window_) lock_window_ = nullptr;

        focus_stack_.remove(window);
        modal_stack_.remove(window);
        pick_always_.remove(window);
        layers_.remove(window);

        for (size_t i = 0; i < key_captures_.size();) {
            if (key_captures_[i].window == window) {
                key_captures_.erase(i);
            } else {
                ++i;
            }
        }
    }

    void UiEventDispatcher::OnceIgnoreMouseLeftButtonUpEvent(uint32_t duration_ms) noexcept {
        ignore_next_left_up_ = true;
        ignore_end_time_ms_ = duration_ms;
    }

    IUiWindow* UiEventDispatcher::PickWindowRecursive(IUiWindow* root, int32_t x, int32_t y) noexcept {
        if (!root || !root->IsVisible()) return nullptr;

        auto children = root->GetChildren();
        for (auto it = children.rbegin(); it != children.rend(); ++it) {
            IUiWindow* child = *it;
            if (!child || !child->IsVisible()) continue;

            if (!child->HasFlag(WindowFlags::IgnoreSize)) {
                if (!child->GetRect().Contains(x, y) && child->GetRect().Width() >= 0) {
                    continue;
                }
            }

            IUiWindow* picked = PickWindowRecursive(child, x, y);
            if (picked) return picked;
        }

        if (root->HasFlag(WindowFlags::NotPick)) {
            return nullptr;
        }

        if (root->GetRect().Contains(x, y)) {
            return root;
        }

        return nullptr;
    }

    IUiWindow* UiEventDispatcher::PickWindow(int32_t x, int32_t y) noexcept {
        ++metrics_.pick_queries;

        if (lock_window_) {
            return PickWindowRecursive(lock_window_, x, y);
        }

        for (IUiWindow* win : pick_always_) {
            if (win && win->IsRendering() && win->GetRect().Contains(x, y)) {
                return win;
            }
        }

        for (auto it = layers_.rbegin(); it != layers_.rend(); ++it) {
            IUiWindow* layer = *it;
            if (!layer) continue;
            IUiWindow* picked = PickWindowRecursive(layer, x, y);
            if (picked && picked != layer) {
                return picked;
            }
        }

        return nullptr;
    }

    IUiWindow* UiEventDispatcher::PickTopWindow(int32_t x, int32_t y) noexcept {
        ++metrics_.pick_queries;
        for (auto it = layers_.rbegin(); it != layers_.rend(); ++it) {
            IUiWindow* layer = *it;
            if (!layer || !layer->IsVisible()) continue;

            auto children = layer->GetChildren();
            for (auto cit = children.rbegin(); cit != children.rend(); ++cit) {
                IUiWindow* child = *cit;
                if (child && child->IsVisible() && child->GetRect().Contains(x, y)) {
                    if (!child->HasFlag(WindowFlags::NotPick)) {
                        return child;
                    }
                }
            }
        }
        return nullptr;
    }

    void UiEventDispatcher::BringToTop(IUiWindow* window) noexcept {
        if (!window || !window->GetParent()) return;
        window->OnTop();
    }

    EventResult UiEventDispatcher::DispatchMouseMove(int32_t screen_x, int32_t screen_y) noexcept {
        ++metrics_.total_mouse_events;
        const Point new_pos = TransformCoords(screen_x, screen_y);
        const Point delta{new_pos.x - current_mouse_pos_.x, new_pos.y - current_mouse_pos_.y};
        current_mouse_pos_ = new_pos;

        const bool dragging = IsDragging();
        UiMouseMoveEvent move_event{
            .position = current_mouse_pos_,
            .delta = delta,
            .button_mask = (left_capture_window_ ? 1u : 0u) | (right_capture_window_ ? 2u : 0u) | (middle_capture_window_ ? 4u : 0u),
            .is_dragging = dragging
        };

        if (event_filter_ && event_filter_->FilterMouseMove(move_event)) {
            return EventResult::ConsumedAndStop;
        }

        // Obsluga aktywnego przeciagania (drag/movable)
        if (left_capture_window_) {
            IUiWindow* win = left_capture_window_;
            if (win->HasFlag(WindowFlags::Movable)) {
                int32_t nx = current_mouse_pos_.x - drag_start_offset_.x;
                int32_t ny = current_mouse_pos_.y - drag_start_offset_.y;

                if (win->GetParent()) {
                    nx -= win->GetParent()->GetRect().left;
                    ny -= win->GetParent()->GetRect().top;
                }

                if (win->HasFlag(WindowFlags::RestrictX)) {
                    nx = win->GetRect().left - (win->GetParent() ? win->GetParent()->GetRect().left : 0);
                }
                if (win->HasFlag(WindowFlags::RestrictY)) {
                    ny = win->GetRect().top - (win->GetParent() ? win->GetParent()->GetRect().top : 0);
                }

                if (win->HasFlag(WindowFlags::Limit)) {
                    const Rect& bias = win->GetLimitBias();
                    const int32_t limit_left = bias.left;
                    const int32_t limit_top = bias.top;
                    const int32_t limit_right = screen_width_ - bias.right;
                    const int32_t limit_bottom = screen_height_ - bias.bottom;

                    if (nx < limit_left) nx = limit_left;
                    else if (nx + win->GetRect().Width() >= limit_right) nx = limit_right - win->GetRect().Width();

                    if (ny < limit_top) ny = limit_top;
                    else if (ny + win->GetRect().Height() >= limit_bottom) ny = limit_bottom - win->GetRect().Height();
                }

                win->SetPosition(nx, ny);
                win->OnMoveWindow(nx, ny);
                ++metrics_.events_handled;
            } else if (win->HasFlag(WindowFlags::Dragable)) {
                UiDragEvent drag_ev{
                    .local_pos = Point{current_mouse_pos_.x - drag_start_offset_.x, current_mouse_pos_.y - drag_start_offset_.y},
                    .drag_start_offset = drag_start_offset_,
                    .screen_pos = current_mouse_pos_
                };
                win->OnMouseDrag(drag_ev);
                ++metrics_.events_handled;
            }
        }

        // Hover tracking
        IUiWindow* hovered = PickWindow(current_mouse_pos_.x, current_mouse_pos_.y);
        if (point_window_ != hovered) {
            if (point_window_) {
                point_window_->OnMouseLeave();
            }
            point_window_ = hovered;
            if (point_window_) {
                point_window_->OnMouseEnter();
            }
        } else if (point_window_) {
            point_window_->OnMouseHover();
        }

        if (point_window_) {
            point_window_->OnMouseMove(move_event);
            return EventResult::Handled;
        }

        return EventResult::Ignored;
    }

    EventResult UiEventDispatcher::DispatchMouseButtonDown(MouseButton button, int32_t screen_x, int32_t screen_y) noexcept {
        ++metrics_.total_mouse_events;
        current_mouse_pos_ = TransformCoords(screen_x, screen_y);

        IUiWindow* target = PickWindow(current_mouse_pos_.x, current_mouse_pos_.y);
        if (!target) return EventResult::Ignored;

        if (target->HasFlag(WindowFlags::Attach)) {
            target = target->GetRoot();
        }

        if (!target->HasFlag(WindowFlags::NotCapture)) {
            if (button == MouseButton::Left) left_capture_window_ = target;
            else if (button == MouseButton::Right) right_capture_window_ = target;
            else if (button == MouseButton::Middle) middle_capture_window_ = target;
        }

        drag_start_offset_.x = current_mouse_pos_.x - target->GetRect().left;
        drag_start_offset_.y = current_mouse_pos_.y - target->GetRect().top;
        picked_mouse_pos_ = current_mouse_pos_;

        UiMouseButtonEvent btn_event{
            .button = button,
            .state = ButtonState::Down,
            .position = current_mouse_pos_,
            .local_position = Point{current_mouse_pos_.x - target->GetRect().left, current_mouse_pos_.y - target->GetRect().top}
        };

        if (event_filter_ && event_filter_->FilterMouseButton(btn_event)) {
            return EventResult::ConsumedAndStop;
        }

        ActivateWindow(target);
        BringToTop(target);

        EventResult res = target->OnMouseDown(btn_event);
        if (res != EventResult::Ignored) {
            ++metrics_.events_handled;
        }
        return res;
    }

    EventResult UiEventDispatcher::DispatchMouseButtonUp(MouseButton button, int32_t screen_x, int32_t screen_y, uint32_t current_time_ms) noexcept {
        ++metrics_.total_mouse_events;
        current_mouse_pos_ = TransformCoords(screen_x, screen_y);

        if (button == MouseButton::Left && ignore_next_left_up_) {
            ignore_next_left_up_ = false;
            if (current_time_ms < ignore_end_time_ms_) {
                return EventResult::Ignored;
            }
        }

        IUiWindow* target = nullptr;
        if (button == MouseButton::Left && left_capture_window_) {
            target = left_capture_window_;
            left_capture_window_ = nullptr;
        } else if (button == MouseButton::Right && right_capture_window_) {
            target = right_capture_window_;
            right_capture_window_ = nullptr;
        } else if (button == MouseButton::Middle && middle_capture_window_) {
            target = middle_capture_window_;
            middle_capture_window_ = nullptr;
        } else {
            target = PickWindow(current_mouse_pos_.x, current_mouse_pos_.y);
        }

        if (!target) return EventResult::Ignored;

        UiMouseButtonEvent btn_event{
            .button = button,
            .state = ButtonState::Up,
            .position = current_mouse_pos_,
            .local_position = Point{current_mouse_pos_.x - target->GetRect().left, current_mouse_pos_.y - target->GetRect().top}
        };

        if (event_filter_ && event_filter_->FilterMouseButton(btn_event)) {
            return EventResult::ConsumedAndStop;
        }

        EventResult res = target->OnMouseUp(btn_event);
        ++metrics_.events_handled;
        return res;
    }

    EventResult UiEventDispatcher::DispatchMouseButtonDoubleClick(MouseButton button, int32_t screen_x, int32_t screen_y) noexcept {
        ++metrics_.total_mouse_events;
        current_mouse_pos_ = TransformCoords(screen_x, screen_y);

        IUiWindow* target = PickWindow(current_mouse_pos_.x, current_mouse_pos_.y);
        if (!target) return EventResult::Ignored;

        UiMouseButtonEvent btn_event{
            .button = button,
            .state = ButtonState::DoubleClick,
            .position = current_mouse_pos_,
            .local_position = Point{current_mouse_pos_.x - target->GetRect().left, current_mouse_pos_.y - target->GetRect().top}
        };

        if (event_filter_ && event_filter_->FilterMouseButton(btn_event)) {
            return EventResult::ConsumedAndStop;
        }

        EventResult res = target->OnMouseDoubleClick(btn_event);
        if (res != EventResult::Ignored) {
            ++metrics_.events_handled;
        }
        return res;
    }

    EventResult UiEventDispatcher::DispatchMouseWheel(int32_t delta, int32_t screen_x, int32_t screen_y) noexcept {
        ++metrics_.total_mouse_events;
        current_mouse_pos_ = TransformCoords(screen_x, screen_y);

        IUiWindow* target = PickWindow(current_mouse_pos_.x, current_mouse_pos_.y);
        if (!target) target = active_window_;
        if (!target) return EventResult::Ignored;

        UiMouseWheelEvent wheel_event{
            .delta = delta,
            .position = current_mouse_pos_
        };

        if (event_filter_ && event_filter_->FilterMouseWheel(wheel_event)) {
            return EventResult::ConsumedAndStop;
        }

        EventResult res = target->OnMouseWheel(wheel_event);
        if (res != EventResult::Ignored) {
            ++metrics_.events_handled;
        }
        return res;
    }

    EventResult UiEventDispatcher::DispatchDrop(uint32_t item_vnum, uint32_t source_slot) noexcept {
        IUiWindow* target = PickWindow(current_mouse_pos_.x, current_mouse_pos_.y);
        if (!target) return EventResult::Ignored;

        UiDropEvent drop_event{
            .position = current_mouse_pos_,
            .item_vnum = item_vnum,
            .source_slot = source_slot
        };

        EventResult res = target->OnDrop(drop_event);
        if (res != EventResult::Ignored) {
            ++metrics_.events_handled;
        }
        return res;
    }

    EventResult UiEventDispatcher::DispatchKeyDown(int32_t key_code, bool repeat, bool alt, bool ctrl, bool shift) noexcept {
        ++metrics_.total_key_events;
        UiKeyEvent event{
            .key_code = key_code,
            .is_repeat = repeat,
            .alt = alt,
            .ctrl = ctrl,
            .shift = shift
        };

        if (event_filter_ && event_filter_->FilterKey(event)) {
            return EventResult::ConsumedAndStop;
        }

        if (lock_window_) {
            EventResult res = lock_window_->OnKeyDown(event);
            if (res != EventResult::Ignored) {
                key_captures_.push_back(KeyCaptureEntry{.key_code = key_code, .window = lock_window_});
                ++metrics_.events_handled;
                return res;
            }
            return EventResult::Ignored;
        }

        if (active_window_ && active_window_->IsVisible()) {
            EventResult res = active_window_->OnKeyDown(event);
            if (res != EventResult::Ignored) {
                key_captures_.push_back(KeyCaptureEntry{.key_code = key_code, .window = active_window_});
                ++metrics_.events_handled;
                return res;
            }
        }

        // Bubbling po warstwach
        for (auto it = layers_.rbegin(); it != layers_.rend(); ++it) {
            IUiWindow* layer = *it;
            if (layer && layer->IsVisible()) {
                EventResult res = layer->OnKeyDown(event);
                if (res != EventResult::Ignored) {
                    key_captures_.push_back(KeyCaptureEntry{.key_code = key_code, .window = layer});
                    ++metrics_.events_handled;
                    return res;
                }
            }
        }

        return EventResult::Ignored;
    }

    EventResult UiEventDispatcher::DispatchKeyUp(int32_t key_code) noexcept {
        ++metrics_.total_key_events;
        UiKeyEvent event{.key_code = key_code};

        if (event_filter_ && event_filter_->FilterKey(event)) {
            return EventResult::ConsumedAndStop;
        }

        // Sprawdz najpierw przechwycone okna klawisza (KeyCaptureMap)
        for (size_t i = 0; i < key_captures_.size(); ++i) {
            if (key_captures_[i].key_code == key_code) {
                IUiWindow* captured = key_captures_[i].window;
                key_captures_.erase(i);
                if (captured && captured->IsVisible()) {
                    EventResult res = captured->OnKeyUp(event);
                    ++metrics_.events_handled;
                    return res;
                }
                return EventResult::Handled;
            }
        }

        if (lock_window_) {
            EventResult res = lock_window_->OnKeyUp(event);
            if (res != EventResult::Ignored) ++metrics_.events_handled;
            return res;
        }

        if (active_window_ && active_window_->IsVisible()) {
            EventResult res = active_window_->OnKeyUp(event);
            if (res != EventResult::Ignored) {
                ++metrics_.events_handled;
                return res;
            }
        }

        for (auto it = layers_.rbegin(); it != layers_.rend(); ++it) {
            IUiWindow* layer = *it;
            if (layer && layer->IsVisible()) {
                EventResult res = layer->OnKeyUp(event);
                if (res != EventResult::Ignored) {
                    ++metrics_.events_handled;
                    return res;
                }
            }
        }

        return EventResult::Ignored;
    }

    EventResult UiEventDispatcher::DispatchSpecialKey(SpecialKey key) noexcept {
        ++metrics_.total_key_events;
        UiSpecialKeyEvent event{.key = key};

        if (event_filter_ && event_filter_->FilterSpecialKey(event)) {
            return EventResult::ConsumedAndStop;
        }

        if (lock_window_) {
            EventResult res = lock_window_->OnSpecialKey(event);
            if (res != EventResult::Ignored) ++metrics_.events_handled;
            return res;
        }

        if (active_window_ && active_window_->IsVisible()) {
            EventResult res = active_window_->OnSpecialKey(event);
            if (res != EventResult::Ignored) {
                ++metrics_.events_handled;
                return res;
            }
        }

        for (auto it = layers_.rbegin(); it != layers_.rend(); ++it) {
            IUiWindow* layer = *it;
            if (layer && layer->IsVisible()) {
                EventResult res = BubbleSpecialKey(layer, event);
                if (res != EventResult::Ignored) {
                    ++metrics_.events_handled;
                    return res;
                }
            }
        }

        return EventResult::Ignored;
    }

    EventResult UiEventDispatcher::BubbleSpecialKey(IUiWindow* start_window, const UiSpecialKeyEvent& event) noexcept {
        if (!start_window || !start_window->IsVisible()) return EventResult::Ignored;

        auto children = start_window->GetChildren();
        for (auto it = children.rbegin(); it != children.rend(); ++it) {
            IUiWindow* child = *it;
            if (child && child->IsVisible()) {
                EventResult res = BubbleSpecialKey(child, event);
                if (res != EventResult::Ignored) return res;
            }
        }

        return start_window->OnSpecialKey(event);
    }

} // namespace Client::UI

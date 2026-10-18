#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "UiEventDispatcher.hpp"

#include <vector>
#include <string>
#include <atomic>

using namespace Client::UI;

namespace {

    std::atomic<size_t> g_allocated_bytes{0};
    std::atomic<size_t> g_allocation_calls{0};

    class MockWindow : public IUiWindow {
    public:
        MockWindow(uint32_t id, std::string_view name, Rect rect, uint32_t flags = 0)
            : id_(id), name_(name), rect_(rect), flags_(flags) {}

        void AddChild(MockWindow* child) {
            if (!child) return;
            child->parent_ = this;
            children_.push_back(child);
            child_ptrs_.push_back(child);
        }

        [[nodiscard]] uint32_t GetId() const noexcept override { return id_; }
        [[nodiscard]] std::string_view GetName() const noexcept override { return name_; }
        [[nodiscard]] const Rect& GetRect() const noexcept override { return rect_; }
        void SetPosition(int32_t x, int32_t y) noexcept override {
            int32_t w = rect_.Width();
            int32_t h = rect_.Height();
            rect_.left = x;
            rect_.top = y;
            rect_.right = x + w;
            rect_.bottom = y + h;
            move_window_calls_++;
        }

        [[nodiscard]] bool IsVisible() const noexcept override { return visible_; }
        void SetVisible(bool v) noexcept { visible_ = v; }

        [[nodiscard]] bool IsRendering() const noexcept override {
            if (!visible_) return false;
            if (parent_) return parent_->IsRendering();
            return true;
        }

        [[nodiscard]] uint32_t GetFlags() const noexcept override { return flags_; }
        [[nodiscard]] bool HasFlag(uint32_t flag) const noexcept override { return (flags_ & flag) != 0; }
        void SetFlags(uint32_t flags) noexcept { flags_ = flags; }

        [[nodiscard]] IUiWindow* GetParent() const noexcept override { return parent_; }
        [[nodiscard]] IUiWindow* GetRoot() noexcept override {
            if (parent_) return parent_->GetRoot();
            return this;
        }

        [[nodiscard]] std::span<IUiWindow* const> GetChildren() const noexcept override {
            return std::span<IUiWindow* const>(child_ptrs_.data(), child_ptrs_.size());
        }

        [[nodiscard]] const Rect& GetLimitBias() const noexcept override { return limit_bias_; }
        void SetLimitBias(const Rect& bias) noexcept { limit_bias_ = bias; }

        EventResult OnMouseMove(const UiMouseMoveEvent& event) noexcept override {
            mouse_move_count_++;
            return EventResult::Handled;
        }

        EventResult OnMouseDown(const UiMouseButtonEvent& event) noexcept override {
            mouse_down_count_++;
            last_mouse_button_ = event.button;
            return EventResult::Handled;
        }

        EventResult OnMouseUp(const UiMouseButtonEvent& event) noexcept override {
            mouse_up_count_++;
            return EventResult::Handled;
        }

        EventResult OnMouseDoubleClick(const UiMouseButtonEvent& event) noexcept override {
            mouse_dclick_count_++;
            return EventResult::Handled;
        }

        EventResult OnMouseWheel(const UiMouseWheelEvent& event) noexcept override {
            mouse_wheel_count_++;
            last_wheel_delta_ = event.delta;
            return EventResult::Handled;
        }

        void OnMouseEnter() noexcept override { mouse_enter_count_++; is_hovered_ = true; }
        void OnMouseLeave() noexcept override { mouse_leave_count_++; is_hovered_ = false; }
        void OnMouseHover() noexcept override { mouse_hover_count_++; }

        EventResult OnMouseDrag(const UiDragEvent& event) noexcept override {
            drag_count_++;
            return EventResult::Handled;
        }

        EventResult OnDrop(const UiDropEvent& event) noexcept override {
            drop_count_++;
            dropped_item_vnum_ = event.item_vnum;
            return EventResult::Handled;
        }

        void OnMoveWindow(int32_t x, int32_t y) noexcept override {
            move_window_calls_++;
        }

        EventResult OnKeyDown(const UiKeyEvent& event) noexcept override {
            key_down_count_++;
            last_key_code_ = event.key_code;
            return accept_keys_ ? EventResult::Handled : EventResult::Ignored;
        }

        EventResult OnKeyUp(const UiKeyEvent& event) noexcept override {
            key_up_count_++;
            last_key_code_ = event.key_code;
            return accept_keys_ ? EventResult::Handled : EventResult::Ignored;
        }

        EventResult OnSpecialKey(const UiSpecialKeyEvent& event) noexcept override {
            special_key_count_++;
            last_special_key_ = event.key;
            return accept_special_keys_ ? EventResult::Handled : EventResult::Ignored;
        }

        void OnSetFocus() noexcept override { has_focus_ = true; focus_gained_count_++; }
        void OnKillFocus() noexcept override { has_focus_ = false; focus_lost_count_++; }
        void OnTop() noexcept override { on_top_count_++; }

        uint32_t id_{0};
        std::string name_;
        Rect rect_{};
        uint32_t flags_{0};
        bool visible_{true};
        Rect limit_bias_{0, 0, 0, 0};

        MockWindow* parent_{nullptr};
        std::vector<MockWindow*> children_{};
        std::vector<IUiWindow*> child_ptrs_{};

        // Telemetria testowa
        uint32_t mouse_move_count_{0};
        uint32_t mouse_down_count_{0};
        uint32_t mouse_up_count_{0};
        uint32_t mouse_dclick_count_{0};
        uint32_t mouse_wheel_count_{0};
        uint32_t mouse_enter_count_{0};
        uint32_t mouse_leave_count_{0};
        uint32_t mouse_hover_count_{0};
        uint32_t drag_count_{0};
        uint32_t drop_count_{0};
        uint32_t move_window_calls_{0};
        uint32_t key_down_count_{0};
        uint32_t key_up_count_{0};
        uint32_t special_key_count_{0};
        uint32_t focus_gained_count_{0};
        uint32_t focus_lost_count_{0};
        uint32_t on_top_count_{0};

        bool is_hovered_{false};
        bool has_focus_{false};
        bool accept_keys_{true};
        bool accept_special_keys_{true};

        MouseButton last_mouse_button_{MouseButton::None};
        int32_t last_wheel_delta_{0};
        int32_t last_key_code_{0};
        SpecialKey last_special_key_{SpecialKey::Escape};
        uint32_t dropped_item_vnum_{0};
    };

    class PassthroughFilter : public IUiEventFilter {
    public:
        bool block_mouse_{false};
        bool block_keys_{false};

        bool FilterMouseMove(const UiMouseMoveEvent&) noexcept override { return block_mouse_; }
        bool FilterMouseButton(const UiMouseButtonEvent&) noexcept override { return block_mouse_; }
        bool FilterKey(const UiKeyEvent&) noexcept override { return block_keys_; }
    };

} // namespace

TEST_CASE("UiEventDispatcher: Window picking and hierarchy reverse ordering") {
    UiEventDispatcher dispatcher;
    dispatcher.SetScreenSize(1024, 768);

    MockWindow root(1, "Root", Rect{0, 0, 1024, 768});
    MockWindow child1(2, "Button1", Rect{50, 50, 150, 100});
    MockWindow child2(3, "Button2Overlapping", Rect{100, 75, 200, 150});

    root.AddChild(&child1);
    root.AddChild(&child2);

    dispatcher.AddLayer(&root);

    SUBCASE("Pick Child2 when clicking inside overlapping region") {
        IUiWindow* picked = dispatcher.PickWindow(120, 80);
        REQUIRE(picked != nullptr);
        CHECK(picked->GetId() == 3);
        CHECK(picked->GetName() == "Button2Overlapping");
    }

    SUBCASE("Pick Child1 when clicking outside overlapping region") {
        IUiWindow* picked = dispatcher.PickWindow(60, 60);
        REQUIRE(picked != nullptr);
        CHECK(picked->GetId() == 2);
        CHECK(picked->GetName() == "Button1");
    }

    SUBCASE("Pick Root when clicking empty area") {
        IUiWindow* picked = dispatcher.PickWindow(500, 500);
        CHECK(picked == nullptr); // warstwa rootowa jest pomijana przy PickWindow
    }
}

TEST_CASE("UiEventDispatcher: Flags NOT_PICK and IGNORE_SIZE") {
    UiEventDispatcher dispatcher;
    dispatcher.SetScreenSize(800, 600);

    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow overlay(2, "Overlay", Rect{100, 100, 300, 300}, WindowFlags::NotPick);
    MockWindow under(3, "UnderWindow", Rect{150, 150, 250, 250});

    root.AddChild(&under);
    root.AddChild(&overlay);
    dispatcher.AddLayer(&root);

    SUBCASE("Overlay has FLAG_NOT_PICK: picks under window") {
        IUiWindow* picked = dispatcher.PickWindow(180, 180);
        REQUIRE(picked != nullptr);
        CHECK(picked->GetId() == 3);
    }

    SUBCASE("Parent has IGNORE_SIZE: child outside parent rect is picked") {
        MockWindow parentIgnoreSize(4, "ParentContainer", Rect{0, 0, 10, 10}, WindowFlags::IgnoreSize);
        MockWindow detachedChild(5, "Detached", Rect{200, 200, 250, 250});
        parentIgnoreSize.AddChild(&detachedChild);
        root.AddChild(&parentIgnoreSize);

        IUiWindow* picked = dispatcher.PickWindow(220, 220);
        REQUIRE(picked != nullptr);
        CHECK(picked->GetId() == 5);
    }
}

TEST_CASE("UiEventDispatcher: PickAlways priority") {
    UiEventDispatcher dispatcher;
    MockWindow regular(1, "Regular", Rect{100, 100, 200, 200});
    MockWindow pickAlways(2, "TooltipPickAlways", Rect{120, 120, 180, 180});

    dispatcher.AddLayer(&regular);
    dispatcher.AddPickAlways(&pickAlways);

    IUiWindow* picked = dispatcher.PickWindow(150, 150);
    REQUIRE(picked != nullptr);
    CHECK(picked->GetId() == 2);
}

TEST_CASE("UiEventDispatcher: Mouse hover transitions (Enter, Leave, Hover)") {
    UiEventDispatcher dispatcher;
    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow btnA(2, "BtnA", Rect{50, 50, 100, 100});
    MockWindow btnB(3, "BtnB", Rect{200, 50, 250, 100});

    root.AddChild(&btnA);
    root.AddChild(&btnB);
    dispatcher.AddLayer(&root);

    dispatcher.DispatchMouseMove(60, 60);
    CHECK(btnA.mouse_enter_count_ == 1);
    CHECK(btnA.is_hovered_ == true);
    CHECK(btnB.is_hovered_ == false);

    dispatcher.DispatchMouseMove(65, 65);
    CHECK(btnA.mouse_hover_count_ == 1);
    CHECK(btnA.mouse_enter_count_ == 1);

    dispatcher.DispatchMouseMove(210, 60);
    CHECK(btnA.mouse_leave_count_ == 1);
    CHECK(btnA.is_hovered_ == false);
    CHECK(btnB.mouse_enter_count_ == 1);
    CHECK(btnB.is_hovered_ == true);
}

TEST_CASE("UiEventDispatcher: Mouse Dragging & Movable with Limit clamping") {
    UiEventDispatcher dispatcher;
    dispatcher.SetScreenSize(800, 600);

    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow movable(2, "MovableWnd", Rect{100, 100, 200, 200}, WindowFlags::Movable | WindowFlags::Limit);
    movable.SetLimitBias(Rect{10, 10, 10, 10});

    root.AddChild(&movable);
    dispatcher.AddLayer(&root);

    dispatcher.DispatchMouseButtonDown(MouseButton::Left, 110, 110);
    CHECK(dispatcher.GetActiveWindow() == &movable);

    // Przesuniecie ponizej progu drag (10px) - brak dragu
    dispatcher.DispatchMouseMove(112, 112);
    CHECK_FALSE(dispatcher.IsDragging());

    // Przesuniecie powyzej progu
    dispatcher.DispatchMouseMove(150, 150);
    CHECK(dispatcher.IsDragging());
    CHECK(movable.move_window_calls_ > 0);

    // Proba wyjechania poza ekran w lewo (LimitBias left = 10)
    dispatcher.DispatchMouseMove(0, 150);
    CHECK(movable.GetRect().left >= 10);

    dispatcher.DispatchMouseButtonUp(MouseButton::Left, 150, 150);
    CHECK(movable.mouse_up_count_ == 1);
}

TEST_CASE("UiEventDispatcher: Axis restriction (RestrictX / RestrictY)") {
    UiEventDispatcher dispatcher;
    dispatcher.SetScreenSize(800, 600);

    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow sliderX(2, "SliderHorizontal", Rect{50, 50, 100, 80}, WindowFlags::Movable | WindowFlags::RestrictY);

    root.AddChild(&sliderX);
    dispatcher.AddLayer(&root);

    dispatcher.DispatchMouseButtonDown(MouseButton::Left, 60, 60);
    dispatcher.DispatchMouseMove(90, 150); // Proba ruchu po osi Y
    dispatcher.DispatchMouseButtonUp(MouseButton::Left, 90, 150);

    CHECK(sliderX.GetRect().top == 50); // Osi Y nie mozna zmienic
    CHECK(sliderX.GetRect().left == 80); // Os X zmieniona
}

TEST_CASE("UiEventDispatcher: Keyboard Focus & KeyCapture Map") {
    UiEventDispatcher dispatcher;

    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow editA(2, "EditBoxA", Rect{10, 10, 100, 30});
    MockWindow editB(3, "EditBoxB", Rect{10, 50, 100, 80});

    root.AddChild(&editA);
    root.AddChild(&editB);
    dispatcher.AddLayer(&root);

    dispatcher.ActivateWindow(&editA);
    CHECK(editA.has_focus_ == true);

    dispatcher.DispatchKeyDown(65); // Klawisz 'A'
    CHECK(editA.key_down_count_ == 1);
    CHECK(editA.last_key_code_ == 65);

    // Zmiana fokusu na editB
    dispatcher.ActivateWindow(&editB);
    CHECK(editA.has_focus_ == false);
    CHECK(editB.has_focus_ == true);

    // Klawisz KeyUp dla 'A' powinnien powrocic do editA ze wzgledu na KeyCapture!
    dispatcher.DispatchKeyUp(65);
    CHECK(editA.key_up_count_ == 1);
    CHECK(editB.key_up_count_ == 0);
}

TEST_CASE("UiEventDispatcher: Modal lock exclusivity") {
    UiEventDispatcher dispatcher;

    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow backgroundBtn(2, "BgBtn", Rect{20, 20, 60, 60});
    MockWindow modalDialog(3, "ModalDialog", Rect{200, 200, 400, 400});
    MockWindow modalBtn(4, "ModalBtn", Rect{220, 220, 280, 250});

    modalDialog.AddChild(&modalBtn);
    root.AddChild(&backgroundBtn);
    root.AddChild(&modalDialog);
    dispatcher.AddLayer(&root);

    dispatcher.PushModalLock(&modalDialog);
    CHECK(dispatcher.GetModalLockWindow() == &modalDialog);

    SUBCASE("Clicking background button during modal lock is blocked") {
        IUiWindow* picked = dispatcher.PickWindow(30, 30);
        CHECK(picked == nullptr);
    }

    SUBCASE("Clicking modal button succeeds") {
        IUiWindow* picked = dispatcher.PickWindow(230, 230);
        REQUIRE(picked != nullptr);
        CHECK(picked->GetId() == 4);
    }

    SUBCASE("Popping modal lock restores interaction") {
        dispatcher.PopModalLock();
        CHECK(dispatcher.GetModalLockWindow() == nullptr);
        IUiWindow* picked = dispatcher.PickWindow(30, 30);
        REQUIRE(picked != nullptr);
        CHECK(picked->GetId() == 2);
    }
}

TEST_CASE("UiEventDispatcher: Special key routing (Escape bubbling)") {
    UiEventDispatcher dispatcher;
    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow parentWnd(2, "Dialog", Rect{100, 100, 300, 300});
    MockWindow childWnd(3, "SubChild", Rect{120, 120, 150, 150});

    childWnd.accept_special_keys_ = false; // Nie obsluguje Escape
    parentWnd.accept_special_keys_ = true;  // Dialog zamyka sie na Escape

    parentWnd.AddChild(&childWnd);
    root.AddChild(&parentWnd);
    dispatcher.AddLayer(&root);

    dispatcher.ActivateWindow(&childWnd);
    EventResult res = dispatcher.DispatchSpecialKey(SpecialKey::Escape);

    CHECK(res == EventResult::Handled);
    CHECK(parentWnd.special_key_count_ == 1);
    CHECK(parentWnd.last_special_key_ == SpecialKey::Escape);
}

TEST_CASE("UiEventDispatcher: Event filtering interceptor") {
    UiEventDispatcher dispatcher;
    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow btn(2, "Btn", Rect{50, 50, 100, 100});
    root.AddChild(&btn);
    dispatcher.AddLayer(&root);

    PassthroughFilter filter;
    dispatcher.SetEventFilter(&filter);

    filter.block_mouse_ = true;
    EventResult res = dispatcher.DispatchMouseButtonDown(MouseButton::Left, 60, 60);
    CHECK(res == EventResult::ConsumedAndStop);
    CHECK(btn.mouse_down_count_ == 0);

    filter.block_mouse_ = false;
    res = dispatcher.DispatchMouseButtonDown(MouseButton::Left, 60, 60);
    CHECK(res == EventResult::Handled);
    CHECK(btn.mouse_down_count_ == 1);
}

TEST_CASE("UiEventDispatcher: OnceIgnoreMouseLeftButtonUpEvent") {
    UiEventDispatcher dispatcher;
    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow btn(2, "Btn", Rect{50, 50, 100, 100});
    root.AddChild(&btn);
    dispatcher.AddLayer(&root);

    dispatcher.OnceIgnoreMouseLeftButtonUpEvent(500);
    EventResult res = dispatcher.DispatchMouseButtonUp(MouseButton::Left, 60, 60, 100);
    CHECK(res == EventResult::Ignored);
    CHECK(btn.mouse_up_count_ == 0);

    // Kolejny przycisk Up juz nie jest ignorowany
    res = dispatcher.DispatchMouseButtonUp(MouseButton::Left, 60, 60, 200);
    CHECK(res == EventResult::Handled);
    CHECK(btn.mouse_up_count_ == 1);
}

TEST_CASE("UiEventDispatcher: Dispatch metrics validation") {
    UiEventDispatcher dispatcher;
    MockWindow root(1, "Root", Rect{0, 0, 800, 600});
    MockWindow btn(2, "Btn", Rect{50, 50, 100, 100});
    root.AddChild(&btn);
    dispatcher.AddLayer(&root);

    dispatcher.DispatchMouseMove(60, 60);
    dispatcher.DispatchMouseButtonDown(MouseButton::Left, 60, 60);
    dispatcher.DispatchMouseButtonUp(MouseButton::Left, 60, 60);
    dispatcher.DispatchKeyDown(13);
    dispatcher.DispatchKeyUp(13);

    const auto& metrics = dispatcher.GetMetrics();
    CHECK(metrics.total_mouse_events >= 3);
    CHECK(metrics.total_key_events == 2);
    CHECK(metrics.events_handled > 0);
}

#pragma once

#include <cstdint>
#include <cstddef>
#include <string_view>
#include <optional>
#include <span>
#include <array>
#include <algorithm>
#include <utility>
#include <concepts>
#include <expected>

namespace Client::UI {

    /**
     * Prosta struktura punktu 2D
     */
    struct Point {
        int32_t x{0};
        int32_t y{0};

        constexpr auto operator<=>(const Point&) const = default;
    };

    /**
     * Prostokat 2D uzywany do hit-testingu i geometrii okien
     */
    struct Rect {
        int32_t left{0};
        int32_t top{0};
        int32_t right{0};
        int32_t bottom{0};

        [[nodiscard]] constexpr int32_t Width() const noexcept { return right - left; }
        [[nodiscard]] constexpr int32_t Height() const noexcept { return bottom - top; }

        [[nodiscard]] constexpr bool Contains(int32_t px, int32_t py) const noexcept {
            return px >= left && px <= right && py >= top && py <= bottom;
        }

        [[nodiscard]] constexpr bool Contains(Point pt) const noexcept {
            return Contains(pt.x, pt.y);
        }

        [[nodiscard]] constexpr bool IsEmpty() const noexcept {
            return right <= left || bottom <= top;
        }

        [[nodiscard]] constexpr Rect Intersect(const Rect& other) const noexcept {
            return Rect{
                .left = (std::max)(left, other.left),
                .top = (std::max)(top, other.top),
                .right = (std::min)(right, other.right),
                .bottom = (std::min)(bottom, other.bottom)
            };
        }

        constexpr auto operator<=>(const Rect&) const = default;
    };

    /**
     * Flagi okna zachowujace pelna zgodnosc z legacy CWindow::EFlags
     */
    namespace WindowFlags {
        inline constexpr uint32_t Movable       = (1 << 0);
        inline constexpr uint32_t Limit         = (1 << 1);
        inline constexpr uint32_t Snap          = (1 << 2);
        inline constexpr uint32_t Dragable      = (1 << 3);
        inline constexpr uint32_t Attach        = (1 << 4);
        inline constexpr uint32_t RestrictX     = (1 << 5);
        inline constexpr uint32_t RestrictY     = (1 << 6);
        inline constexpr uint32_t NotCapture    = (1 << 7);
        inline constexpr uint32_t Float         = (1 << 8);
        inline constexpr uint32_t NotPick       = (1 << 9);
        inline constexpr uint32_t IgnoreSize    = (1 << 10);
        inline constexpr uint32_t RTL           = (1 << 11);
    }

    enum class MouseButton : uint8_t {
        Left = 0,
        Right = 1,
        Middle = 2,
        None = 255
    };

    enum class ButtonState : uint8_t {
        Down = 0,
        Up = 1,
        DoubleClick = 2
    };

    enum class SpecialKey : uint8_t {
        Escape = 0,
        Exit = 1,
        Tab = 2,
        Return = 3
    };

    enum class EventResult : uint8_t {
        Ignored = 0,
        Handled = 1,
        ConsumedAndStop = 2
    };

    enum class RoutingStrategy : uint8_t {
        Direct = 0,
        Bubble = 1,
        Tunnel = 2
    };

    struct UiMouseMoveEvent {
        Point position;
        Point delta;
        uint32_t button_mask{0};
        bool is_dragging{false};
    };

    struct UiMouseButtonEvent {
        MouseButton button{MouseButton::None};
        ButtonState state{ButtonState::Down};
        Point position;
        Point local_position;
    };

    struct UiMouseWheelEvent {
        int32_t delta{0};
        Point position;
    };

    struct UiDragEvent {
        Point local_pos;
        Point drag_start_offset;
        Point screen_pos;
    };

    struct UiDropEvent {
        Point position;
        uint32_t item_vnum{0};
        uint32_t source_slot{0};
    };

    struct UiKeyEvent {
        int32_t key_code{0};
        bool is_repeat{false};
        bool alt{false};
        bool ctrl{false};
        bool shift{false};
    };

    struct UiSpecialKeyEvent {
        SpecialKey key{SpecialKey::Escape};
    };

    class IUiWindow;

    /**
     * Czysty interfejs okna UI niezalezny od Pythona i renderera D3D9
     */
    class IUiWindow {
    public:
        virtual ~IUiWindow() = default;

        [[nodiscard]] virtual uint32_t GetId() const noexcept = 0;
        [[nodiscard]] virtual std::string_view GetName() const noexcept = 0;
        [[nodiscard]] virtual const Rect& GetRect() const noexcept = 0;
        virtual void SetPosition(int32_t x, int32_t y) noexcept = 0;

        [[nodiscard]] virtual bool IsVisible() const noexcept = 0;
        [[nodiscard]] virtual bool IsRendering() const noexcept = 0;
        [[nodiscard]] virtual uint32_t GetFlags() const noexcept = 0;
        [[nodiscard]] virtual bool HasFlag(uint32_t flag) const noexcept = 0;

        [[nodiscard]] virtual IUiWindow* GetParent() const noexcept = 0;
        [[nodiscard]] virtual IUiWindow* GetRoot() noexcept = 0;
        [[nodiscard]] virtual std::span<IUiWindow* const> GetChildren() const noexcept = 0;
        [[nodiscard]] virtual const Rect& GetLimitBias() const noexcept = 0;

        // Callbacki obslugi zdarzen
        virtual EventResult OnMouseMove(const UiMouseMoveEvent& event) noexcept { return EventResult::Ignored; }
        virtual EventResult OnMouseDown(const UiMouseButtonEvent& event) noexcept { return EventResult::Ignored; }
        virtual EventResult OnMouseUp(const UiMouseButtonEvent& event) noexcept { return EventResult::Ignored; }
        virtual EventResult OnMouseDoubleClick(const UiMouseButtonEvent& event) noexcept { return EventResult::Ignored; }
        virtual EventResult OnMouseWheel(const UiMouseWheelEvent& event) noexcept { return EventResult::Ignored; }

        virtual void OnMouseEnter() noexcept {}
        virtual void OnMouseLeave() noexcept {}
        virtual void OnMouseHover() noexcept {}

        virtual EventResult OnMouseDrag(const UiDragEvent& event) noexcept { return EventResult::Ignored; }
        virtual EventResult OnDrop(const UiDropEvent& event) noexcept { return EventResult::Ignored; }
        virtual void OnMoveWindow(int32_t x, int32_t y) noexcept {}

        virtual EventResult OnKeyDown(const UiKeyEvent& event) noexcept { return EventResult::Ignored; }
        virtual EventResult OnKeyUp(const UiKeyEvent& event) noexcept { return EventResult::Ignored; }
        virtual EventResult OnSpecialKey(const UiSpecialKeyEvent& event) noexcept { return EventResult::Ignored; }

        virtual void OnSetFocus() noexcept {}
        virtual void OnKillFocus() noexcept {}
        virtual void OnTop() noexcept {}
    };

    /**
     * Interfejs filtru zdarzen do globalnego przechwytywania / blokowania wejscia
     */
    class IUiEventFilter {
    public:
        virtual ~IUiEventFilter() = default;
        virtual bool FilterMouseMove(const UiMouseMoveEvent& event) noexcept { return false; }
        virtual bool FilterMouseButton(const UiMouseButtonEvent& event) noexcept { return false; }
        virtual bool FilterMouseWheel(const UiMouseWheelEvent& event) noexcept { return false; }
        virtual bool FilterKey(const UiKeyEvent& event) noexcept { return false; }
        virtual bool FilterSpecialKey(const UiSpecialKeyEvent& event) noexcept { return false; }
    };

    /**
     * Zero-allocation bufor o stalej pojemnosci z pelna semantyka tablicowa
     */
    template <typename T, size_t Capacity>
    class FixedVector {
    public:
        constexpr FixedVector() noexcept : size_(0) {}

        [[nodiscard]] constexpr size_t size() const noexcept { return size_; }
        [[nodiscard]] constexpr size_t capacity() const noexcept { return Capacity; }
        [[nodiscard]] constexpr bool empty() const noexcept { return size_ == 0; }
        [[nodiscard]] constexpr bool full() const noexcept { return size_ >= Capacity; }

        constexpr void clear() noexcept { size_ = 0; }

        constexpr bool push_back(const T& val) noexcept {
            if (size_ >= Capacity) return false;
            data_[size_++] = val;
            return true;
        }

        constexpr bool pop_back() noexcept {
            if (size_ == 0) return false;
            --size_;
            return true;
        }

        constexpr T& back() noexcept { return data_[size_ - 1]; }
        constexpr const T& back() const noexcept { return data_[size_ - 1]; }

        constexpr T& operator[](size_t idx) noexcept { return data_[idx]; }
        constexpr const T& operator[](size_t idx) const noexcept { return data_[idx]; }

        constexpr T* begin() noexcept { return data_.data(); }
        constexpr const T* begin() const noexcept { return data_.data(); }
        constexpr T* end() noexcept { return data_.data() + size_; }
        constexpr const T* end() const noexcept { return data_.data() + size_; }

        constexpr auto rbegin() noexcept { return std::reverse_iterator(end()); }
        constexpr auto rbegin() const noexcept { return std::reverse_iterator(end()); }
        constexpr auto rend() noexcept { return std::reverse_iterator(begin()); }
        constexpr auto rend() const noexcept { return std::reverse_iterator(begin()); }

        constexpr bool erase(size_t index) noexcept {
            if (index >= size_) return false;
            for (size_t i = index; i + 1 < size_; ++i) {
                data_[i] = std::move(data_[i + 1]);
            }
            --size_;
            return true;
        }

        constexpr bool remove(const T& val) noexcept {
            for (size_t i = 0; i < size_; ++i) {
                if (data_[i] == val) {
                    return erase(i);
                }
            }
            return false;
        }

        [[nodiscard]] constexpr bool contains(const T& val) const noexcept {
            for (size_t i = 0; i < size_; ++i) {
                if (data_[i] == val) return true;
            }
            return false;
        }

    private:
        std::array<T, Capacity> data_{};
        size_t size_{0};
    };

    struct DispatcherMetrics {
        uint64_t total_mouse_events{0};
        uint64_t total_key_events{0};
        uint64_t events_handled{0};
        uint64_t pick_queries{0};
    };

    /**
     * UiEventDispatcher - C++23 bezalokacyjny zarzadca routingu zdarzen UI
     */
    class UiEventDispatcher {
    public:
        static constexpr size_t MaxLayers = 16;
        static constexpr size_t MaxPickAlways = 32;
        static constexpr size_t MaxFocusStack = 16;
        static constexpr size_t MaxModalStack = 16;
        static constexpr size_t MaxKeyCaptures = 32;
        static constexpr size_t MaxHierarchyDepth = 64;
        static constexpr int32_t DragThresholdDistance = 10;

        UiEventDispatcher() noexcept = default;
        ~UiEventDispatcher() noexcept = default;

        UiEventDispatcher(const UiEventDispatcher&) = delete;
        UiEventDispatcher& operator=(const UiEventDispatcher&) = delete;
        UiEventDispatcher(UiEventDispatcher&&) noexcept = default;
        UiEventDispatcher& operator=(UiEventDispatcher&&) noexcept = default;

        // Konfiguracja rozdzielczosci i ekranu
        void SetScreenSize(int32_t width, int32_t height) noexcept;
        void SetResolution(int32_t hres, int32_t vres) noexcept;
        [[nodiscard]] Point GetMousePosition() const noexcept { return current_mouse_pos_; }
        [[nodiscard]] bool IsDragging() const noexcept;

        // Rejestracja warstw i okien zawsze pickowanych
        bool AddLayer(IUiWindow* layer) noexcept;
        bool RemoveLayer(IUiWindow* layer) noexcept;
        bool AddPickAlways(IUiWindow* window) noexcept;
        bool RemovePickAlways(IUiWindow* window) noexcept;

        // Zarzadzanie fokusem i trybem modalnym
        void ActivateWindow(IUiWindow* window) noexcept;
        void DeactivateWindow() noexcept;
        [[nodiscard]] IUiWindow* GetActiveWindow() const noexcept { return active_window_; }

        void PushModalLock(IUiWindow* window) noexcept;
        void PopModalLock() noexcept;
        [[nodiscard]] IUiWindow* GetModalLockWindow() const noexcept { return lock_window_; }

        void ResetCapture() noexcept;
        void NotifyDestroyWindow(IUiWindow* window) noexcept;
        void OnceIgnoreMouseLeftButtonUpEvent(uint32_t duration_ms = 500) noexcept;

        void SetEventFilter(IUiEventFilter* filter) noexcept { event_filter_ = filter; }

        // Glowny potok dispatchera
        EventResult DispatchMouseMove(int32_t screen_x, int32_t screen_y) noexcept;
        EventResult DispatchMouseButtonDown(MouseButton button, int32_t screen_x, int32_t screen_y) noexcept;
        EventResult DispatchMouseButtonUp(MouseButton button, int32_t screen_x, int32_t screen_y, uint32_t current_time_ms = 0) noexcept;
        EventResult DispatchMouseButtonDoubleClick(MouseButton button, int32_t screen_x, int32_t screen_y) noexcept;
        EventResult DispatchMouseWheel(int32_t delta, int32_t screen_x, int32_t screen_y) noexcept;

        EventResult DispatchKeyDown(int32_t key_code, bool repeat = false, bool alt = false, bool ctrl = false, bool shift = false) noexcept;
        EventResult DispatchKeyUp(int32_t key_code) noexcept;
        EventResult DispatchSpecialKey(SpecialKey key) noexcept;
        EventResult DispatchDrop(uint32_t item_vnum, uint32_t source_slot) noexcept;

        // Hit-testing / picking
        [[nodiscard]] IUiWindow* PickWindow(int32_t x, int32_t y) noexcept;
        [[nodiscard]] IUiWindow* PickTopWindow(int32_t x, int32_t y) noexcept;
        void BringToTop(IUiWindow* window) noexcept;

        // Metryki
        [[nodiscard]] const DispatcherMetrics& GetMetrics() const noexcept { return metrics_; }
        void ResetMetrics() noexcept { metrics_ = {}; }

    private:
        struct KeyCaptureEntry {
            int32_t key_code{0};
            IUiWindow* window{nullptr};
            constexpr auto operator<=>(const KeyCaptureEntry&) const = default;
            constexpr bool operator==(const KeyCaptureEntry& other) const noexcept {
                return key_code == other.key_code && window == other.window;
            }
        };

        [[nodiscard]] Point TransformCoords(int32_t screen_x, int32_t screen_y) const noexcept;
        [[nodiscard]] IUiWindow* PickWindowRecursive(IUiWindow* root, int32_t x, int32_t y) noexcept;
        EventResult RouteKeyEvent(const UiKeyEvent& event, bool is_down) noexcept;
        EventResult BubbleSpecialKey(IUiWindow* start_window, const UiSpecialKeyEvent& event) noexcept;

        int32_t screen_width_{800};
        int32_t screen_height_{600};
        int32_t virtual_hres_{0};
        int32_t virtual_vres_{0};

        Point current_mouse_pos_{0, 0};
        Point picked_mouse_pos_{0, 0};
        Point drag_start_offset_{0, 0};

        bool ignore_next_left_up_{false};
        uint32_t ignore_end_time_ms_{0};

        IUiWindow* point_window_{nullptr};
        IUiWindow* active_window_{nullptr};
        IUiWindow* lock_window_{nullptr};

        IUiWindow* left_capture_window_{nullptr};
        IUiWindow* right_capture_window_{nullptr};
        IUiWindow* middle_capture_window_{nullptr};

        IUiEventFilter* event_filter_{nullptr};

        FixedVector<IUiWindow*, MaxLayers> layers_{};
        FixedVector<IUiWindow*, MaxPickAlways> pick_always_{};
        FixedVector<IUiWindow*, MaxFocusStack> focus_stack_{};
        FixedVector<IUiWindow*, MaxModalStack> modal_stack_{};
        FixedVector<KeyCaptureEntry, MaxKeyCaptures> key_captures_{};

        DispatcherMetrics metrics_{};
    };

} // namespace Client::UI

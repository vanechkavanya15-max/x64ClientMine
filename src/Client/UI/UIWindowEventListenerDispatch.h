#pragma once
#include <cstdint>
#include <functional>
#include <unordered_map>

namespace Client::UI {

enum class UIEventType : uint8_t { Click = 0, DoubleClick = 1, MouseOver = 2, MouseLeave = 3 };

class UIWindowEventListenerDispatch {
public:
    using EventHandler = std::function<void(uint32_t)>;

    UIWindowEventListenerDispatch() = default;
    ~UIWindowEventListenerDispatch() = default;

    void RegisterHandler(UIEventType evt, EventHandler handler) {
        m_handlers[evt] = handler;
    }
    bool DispatchEvent(UIEventType evt, uint32_t windowId) {
        auto it = m_handlers.find(evt);
        if (it != m_handlers.end() && it->second) {
            it->second(windowId);
            return true;
        }
        return false;
    }

private:
    std::unordered_map<UIEventType, EventHandler> m_handlers;
};

} // namespace Client::UI

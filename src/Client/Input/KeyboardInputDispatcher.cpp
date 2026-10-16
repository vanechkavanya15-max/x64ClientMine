#include "KeyboardInputDispatcher.h"
#include <algorithm>
#include <new>

namespace Client::Input {

std::expected<void, KeyboardError> KeyboardInputDispatcher::RegisterHandler(KeyCode key, KeyHandler handler) noexcept {
    if (!handler) {
        return std::unexpected(KeyboardError::InvalidKey);
    }
    
    auto it = std::find_if(handlers_.begin(), handlers_.end(), 
        [key](const Registration& reg) { return reg.key == key; });
        
    if (it != handlers_.end()) {
        return std::unexpected(KeyboardError::AlreadyRegistered);
    }
    
    try {
        handlers_.push_back({key, std::move(handler)});
    } catch (const std::bad_alloc&) {
        return std::unexpected(KeyboardError::AllocationFailed);
    }
    return {};
}

std::expected<void, KeyboardError> KeyboardInputDispatcher::UnregisterHandler(KeyCode key) noexcept {
    auto it = std::find_if(handlers_.begin(), handlers_.end(), 
        [key](const Registration& reg) { return reg.key == key; });
        
    if (it == handlers_.end()) {
        return std::unexpected(KeyboardError::HandlerNotFound);
    }
    
    handlers_.erase(it);
    return {};
}

std::expected<void, KeyboardError> KeyboardInputDispatcher::Dispatch(KeyCode key, KeyState state) const {
    auto it = std::find_if(handlers_.begin(), handlers_.end(), 
        [key](const Registration& reg) { return reg.key == key; });
        
    if (it == handlers_.end()) {
        return std::unexpected(KeyboardError::HandlerNotFound);
    }
    
    it->handler(state);
    return {};
}

void KeyboardInputDispatcher::Clear() noexcept {
    handlers_.clear();
}

} // namespace Client::Input

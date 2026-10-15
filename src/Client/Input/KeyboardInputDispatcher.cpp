#include "KeyboardInputDispatcher.h"
#include <algorithm>

namespace Client::Input {

std::expected<void, KeyboardError> KeyboardInputDispatcher::RegisterHandler(KeyCode key, KeyHandler handler) {
    if (!handler) {
        return std::unexpected(KeyboardError::InvalidKey);
    }
    
    auto it = std::find_if(handlers_.begin(), handlers_.end(), 
        [key](const Registration& reg) { return reg.key == key; });
        
    if (it != handlers_.end()) {
        return std::unexpected(KeyboardError::AlreadyRegistered);
    }
    
    handlers_.push_back({key, std::move(handler)});
    return {};
}

std::expected<void, KeyboardError> KeyboardInputDispatcher::UnregisterHandler(KeyCode key) {
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

void KeyboardInputDispatcher::Clear() {
    handlers_.clear();
}

} // namespace Client::Input

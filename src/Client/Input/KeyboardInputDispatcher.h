#pragma once

#include <cstdint>
#include <functional>
#include <optional>
#include <expected>
#include <vector>
#include <algorithm>
#include "Client/Core/StrongTypes.h"

namespace Client::Input {

enum class KeyboardError {
    InvalidKey,
    HandlerNotFound,
    AlreadyRegistered,
    AllocationFailed
};

enum class KeyState {
    Pressed,
    Released
};

struct KeyCodeTag {};
using KeyCode = Client::Core::StrongType<KeyCodeTag, uint32_t, 0>;

using KeyHandler = std::function<void(KeyState)>;

class KeyboardInputDispatcher {
public:
    KeyboardInputDispatcher() = default;
    ~KeyboardInputDispatcher() = default;
    KeyboardInputDispatcher(const KeyboardInputDispatcher&) = delete;
    KeyboardInputDispatcher& operator=(const KeyboardInputDispatcher&) = delete;
    KeyboardInputDispatcher(KeyboardInputDispatcher&&) = delete;
    KeyboardInputDispatcher& operator=(KeyboardInputDispatcher&&) = delete;

    [[nodiscard]] std::expected<void, KeyboardError> RegisterHandler(KeyCode key, KeyHandler handler) noexcept;
    [[nodiscard]] std::expected<void, KeyboardError> UnregisterHandler(KeyCode key) noexcept;
    [[nodiscard]] std::expected<void, KeyboardError> Dispatch(KeyCode key, KeyState state) const;
    void Clear() noexcept;

private:
    struct Registration {
        KeyCode key;
        KeyHandler handler;
    };
    std::vector<Registration> handlers_;
};

} // namespace Client::Input

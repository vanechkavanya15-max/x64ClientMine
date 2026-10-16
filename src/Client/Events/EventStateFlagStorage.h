#pragma once

#include <string>
#include <unordered_map>
#include <optional>
#include <expected>
#include "Client/Core/ExpectedResult.h"

namespace Client::Events {

    enum class EventStateError {
        NotFound,
        InvalidKey
    };

    class EventStateFlagStorage {
    public:
        EventStateFlagStorage() = default;
        ~EventStateFlagStorage() = default;

        // Disallow copying and assignment for a single source of truth locally
        EventStateFlagStorage(const EventStateFlagStorage&) = delete;
        EventStateFlagStorage& operator=(const EventStateFlagStorage&) = delete;

        void SetFlag(const std::string& key, int value) noexcept;
        Client::Core::ExpectedResult<int, EventStateError> GetFlag(const std::string& key) const noexcept;
        void ClearFlag(const std::string& key) noexcept;
        void ClearAll() noexcept;

    private:
        std::unordered_map<std::string, int> m_flags;
    };

} // namespace Client::Events

#include "EventStateFlagStorage.h"

namespace Client::Events {

    void EventStateFlagStorage::SetFlag(const std::string& key, int value) noexcept {
        if (key.empty()) {
            return; // Or we could log it / return expected, but let's just ignore for now as requested by typical simple set
        }
        m_flags[key] = value;
    }

    Client::Core::ExpectedResult<int, EventStateError> EventStateFlagStorage::GetFlag(const std::string& key) const noexcept {
        if (key.empty()) {
            return Client::Core::MakeUnexpected(EventStateError::InvalidKey);
        }
        
        auto it = m_flags.find(key);
        if (it == m_flags.end()) {
            return Client::Core::MakeUnexpected(EventStateError::NotFound);
        }
        
        return it->second;
    }

    void EventStateFlagStorage::ClearFlag(const std::string& key) noexcept {
        m_flags.erase(key);
    }

    void EventStateFlagStorage::ClearAll() noexcept {
        m_flags.clear();
    }

} // namespace Client::Events

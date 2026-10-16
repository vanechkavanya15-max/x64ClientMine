#pragma once

#include <chrono>
#include <unordered_map>
#include <optional>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/DomainErrors.h"

namespace Client::Skills {

class SkillBuffManager {
public:
    SkillBuffManager() = default;
    ~SkillBuffManager() = default;

    SkillBuffManager(const SkillBuffManager&) = delete;
    SkillBuffManager& operator=(const SkillBuffManager&) = delete;

    // Dodaje nowy buff lub odswieza jego czas trwania
    Client::Core::Result<void, Client::Core::SkillError> AddBuff(Client::Core::SkillId skillId, std::chrono::milliseconds duration);

    // Usuwa buff manualnie
    Client::Core::Result<void, Client::Core::SkillError> RemoveBuff(Client::Core::SkillId skillId);

    // Aktualizuje stan buffow na podstawie czasu jaki uplynal
    void Update(std::chrono::milliseconds deltaTime);

    // Sprawdza czy dany buff jest aktywny
    bool HasBuff(Client::Core::SkillId skillId) const;

    // Zwraca pozostaly czas trwania buffa (jesli aktywny)
    std::optional<std::chrono::milliseconds> GetBuffRemainingTime(Client::Core::SkillId skillId) const;

    // Usuwa wszystkie aktywne buffy
    void ClearAll();

private:
    std::unordered_map<Client::Core::SkillId, std::chrono::milliseconds> m_activeBuffs;
};

} // namespace Client::Skills

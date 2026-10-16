#pragma once

#include <optional>
#include "Core/StrongTypes.h"
#include "Core/ExpectedResult.h"

namespace Client::Player {

    enum class TargetError {
        InvalidEntity,
        TargetAlreadySetToSameEntity,
        NoTargetSet
    };

    class PlayerTargetManager {
    public:
        PlayerTargetManager() = default;
        ~PlayerTargetManager() = default;

        Client::Core::ExpectedResult<void, TargetError> SetTarget(Client::Core::EntityVid targetVid) noexcept;
        Client::Core::ExpectedResult<void, TargetError> ClearTarget() noexcept;

        [[nodiscard]] std::optional<Client::Core::EntityVid> GetTarget() const noexcept;

        void OnEntityDespawned(Client::Core::EntityVid despawnedVid) noexcept;
        void OnEntityDied(Client::Core::EntityVid deadVid) noexcept;

    private:
        std::optional<Client::Core::EntityVid> currentTarget_;
    };

} // namespace Client::Player

#include "PlayerTargetManager.h"

namespace Client::Player {

    Client::Core::ExpectedResult<void, TargetError> PlayerTargetManager::SetTarget(Client::Core::EntityVid targetVid) noexcept {
        if (!targetVid) {
            return Client::Core::MakeUnexpected(TargetError::InvalidEntity);
        }

        if (currentTarget_ && currentTarget_.value() == targetVid) {
            return Client::Core::MakeUnexpected(TargetError::TargetAlreadySetToSameEntity);
        }

        currentTarget_ = targetVid;
        return {};
    }

    Client::Core::ExpectedResult<void, TargetError> PlayerTargetManager::ClearTarget() noexcept {
        if (!currentTarget_) {
            return Client::Core::MakeUnexpected(TargetError::NoTargetSet);
        }

        currentTarget_.reset();
        return {};
    }

    std::optional<Client::Core::EntityVid> PlayerTargetManager::GetTarget() const noexcept {
        return currentTarget_;
    }

    void PlayerTargetManager::OnEntityDespawned(Client::Core::EntityVid despawnedVid) noexcept {
        if (currentTarget_ && currentTarget_.value() == despawnedVid) {
            ClearTarget();
        }
    }

    void PlayerTargetManager::OnEntityDied(Client::Core::EntityVid deadVid) noexcept {
        if (currentTarget_ && currentTarget_.value() == deadVid) {
            ClearTarget();
        }
    }

} // namespace Client::Player

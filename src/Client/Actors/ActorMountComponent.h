#pragma once

#include <expected>
#include <optional>
#include "Client/Core/StrongTypes.h"

namespace Client::Actors {

enum class MountError {
    None,
    AlreadyMounted,
    NotMounted,
    InvalidVnum
};

class ActorMountComponent {
public:
    ActorMountComponent() = default;
    ~ActorMountComponent() = default;

    ActorMountComponent(const ActorMountComponent&) = delete;
    ActorMountComponent& operator=(const ActorMountComponent&) = delete;

    ActorMountComponent(ActorMountComponent&&) noexcept = default;
    ActorMountComponent& operator=(ActorMountComponent&&) noexcept = default;

    std::expected<void, MountError> Mount(Core::ItemVnum mountVnum);
    std::expected<void, MountError> Unmount();

    std::optional<Core::ItemVnum> GetMountVnum() const noexcept;
    bool IsMounted() const noexcept;

private:
    std::optional<Core::ItemVnum> m_mountVnum;
};

} // namespace Client::Actors

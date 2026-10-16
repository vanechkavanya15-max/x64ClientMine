#pragma once

#include <expected>
#include <optional>
#include <cstdint>
#include "../../EterBase/StrongTypes.h"
#include "../../EterBase/EventBus.h"

namespace Client::Actors {

    enum class GuildEmblemError {
        NotInitialized,
        AlreadyInitialized
    };

    struct ActorGuildEmblemChangedEvent : public EterBase::IEvent {
        EterBase::EntityId entityId;
        std::optional<EterBase::GuildId> guildId;

        ActorGuildEmblemChangedEvent(EterBase::EntityId id, std::optional<EterBase::GuildId> gId)
            : entityId(id), guildId(gId) {}
    };

    class ActorGuildEmblemComponent {
    public:
        ActorGuildEmblemComponent();
        ~ActorGuildEmblemComponent() = default;

        ActorGuildEmblemComponent(const ActorGuildEmblemComponent&) = delete;
        ActorGuildEmblemComponent& operator=(const ActorGuildEmblemComponent&) = delete;
        ActorGuildEmblemComponent(ActorGuildEmblemComponent&&) noexcept = default;
        ActorGuildEmblemComponent& operator=(ActorGuildEmblemComponent&&) noexcept = default;

        std::expected<void, GuildEmblemError> Initialize(EterBase::EntityId id, EterBase::EventBus* bus);
        
        std::expected<void, GuildEmblemError> SetGuildId(EterBase::GuildId guildId);
        std::expected<EterBase::GuildId, GuildEmblemError> GetGuildId() const noexcept;
        
        std::expected<void, GuildEmblemError> ClearGuildEmblem();

    private:
        void BroadcastGuildEmblemChange();

        std::optional<EterBase::EntityId> m_entityId;
        EterBase::EventBus* m_eventBus{nullptr};
        
        std::optional<EterBase::GuildId> m_guildId;
    };

} // namespace Client::Actors

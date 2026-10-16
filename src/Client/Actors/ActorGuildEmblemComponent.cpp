#include "ActorGuildEmblemComponent.h"

namespace Client::Actors {

    ActorGuildEmblemComponent::ActorGuildEmblemComponent() = default;

    std::expected<void, GuildEmblemError> ActorGuildEmblemComponent::Initialize(EterBase::EntityId id, EterBase::EventBus* bus) {
        if (m_entityId.has_value()) {
            return std::unexpected(GuildEmblemError::AlreadyInitialized);
        }

        m_entityId = id;
        m_eventBus = bus;
        return {};
    }
    
    std::expected<void, GuildEmblemError> ActorGuildEmblemComponent::SetGuildId(EterBase::GuildId guildId) {
        if (!m_entityId.has_value()) {
            return std::unexpected(GuildEmblemError::NotInitialized);
        }

        if (m_guildId != guildId) {
            m_guildId = guildId;
            BroadcastGuildEmblemChange();
        }

        return {};
    }
    
    std::expected<EterBase::GuildId, GuildEmblemError> ActorGuildEmblemComponent::GetGuildId() const noexcept {
        if (!m_entityId.has_value()) {
            return std::unexpected(GuildEmblemError::NotInitialized);
        }

        if (m_guildId.has_value()) {
            return m_guildId.value();
        }

        // Return a default GuildId (value 0) to represent no guild
        return EterBase::GuildId{0};
    }
    
    std::expected<void, GuildEmblemError> ActorGuildEmblemComponent::ClearGuildEmblem() {
        if (!m_entityId.has_value()) {
            return std::unexpected(GuildEmblemError::NotInitialized);
        }

        if (m_guildId.has_value()) {
            m_guildId.reset();
            BroadcastGuildEmblemChange();
        }

        return {};
    }

    void ActorGuildEmblemComponent::BroadcastGuildEmblemChange() {
        if (m_eventBus && m_entityId.has_value()) {
            m_eventBus->Publish(ActorGuildEmblemChangedEvent(m_entityId.value(), m_guildId));
        }
    }

} // namespace Client::Actors

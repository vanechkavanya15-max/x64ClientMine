#pragma once

#include <string>
#include <expected>
#include <optional>
#include <cstdint>
#include <format>
#include "../../EterBase/StrongTypes.h"
#include "../../EterBase/EventBus.h"

namespace Client::Actors {

    enum class TitleError {
        NotInitialized,
        InvalidPoints,
        AlreadyInitialized
    };

    enum class AlignmentTitle {
        Chivalric,
        Noble,
        Good,
        Friendly,
        Neutral,
        Aggressive,
        Fraudulent,
        Malicious,
        Cruel
    };

    struct TitleColor {
        uint8_t r;
        uint8_t g;
        uint8_t b;

        bool operator==(const TitleColor&) const = default;
    };

    struct ActorTitleChangedEvent {
        EterBase::EntityId entityId;
        std::string newTitle;
    };

    class ActorTitleComponent {
    public:
        ActorTitleComponent();
        ~ActorTitleComponent();

        ActorTitleComponent(const ActorTitleComponent&) = delete;
        ActorTitleComponent& operator=(const ActorTitleComponent&) = delete;
        ActorTitleComponent(ActorTitleComponent&&) noexcept = default;
        ActorTitleComponent& operator=(ActorTitleComponent&&) noexcept = default;

        std::expected<void, TitleError> Initialize(EterBase::EntityId id, EterBase::EventBus* bus);
        
        std::expected<void, TitleError> SetAlignmentPoints(int points);
        std::expected<int, TitleError> GetAlignmentPoints() const noexcept;
        
        std::expected<AlignmentTitle, TitleError> GetAlignmentTitle() const noexcept;
        std::expected<TitleColor, TitleError> GetTitleColor() const noexcept;
        
        std::expected<void, TitleError> SetCustomTitle(const std::string& title);
        std::expected<void, TitleError> ClearCustomTitle();
        
        std::expected<std::string, TitleError> GetFormattedTitle() const;

    private:
        AlignmentTitle CalculateTitleFromPoints(int points) const noexcept;
        TitleColor GetColorForTitle(AlignmentTitle title) const noexcept;
        void BroadcastTitleChange();

        std::optional<EterBase::EntityId> m_entityId;
        EterBase::EventBus* m_eventBus;
        
        int m_alignmentPoints{0};
        std::optional<std::string> m_customTitle;
    };

} // namespace Client::Actors

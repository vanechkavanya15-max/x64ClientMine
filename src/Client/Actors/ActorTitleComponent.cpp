#include "ActorTitleComponent.h"

namespace Client::Actors {

    ActorTitleComponent::ActorTitleComponent() : m_eventBus(nullptr) {}
    
    ActorTitleComponent::~ActorTitleComponent() = default;

    std::expected<void, TitleError> ActorTitleComponent::Initialize(EterBase::EntityId id, EterBase::EventBus* bus) {
        if (m_entityId.has_value()) {
            return std::unexpected(TitleError::AlreadyInitialized);
        }
        
        m_entityId = id;
        m_eventBus = bus;
        m_alignmentPoints = 0;
        m_customTitle.reset();
        
        return {};
    }

    std::expected<void, TitleError> ActorTitleComponent::SetAlignmentPoints(int points) {
        if (!m_entityId.has_value()) {
            return std::unexpected(TitleError::NotInitialized);
        }
        
        // Typowy zakres to od -20000 do 20000
        if (points < -20000 || points > 20000) {
            return std::unexpected(TitleError::InvalidPoints);
        }
        
        m_alignmentPoints = points;
        BroadcastTitleChange();
        return {};
    }

    std::expected<int, TitleError> ActorTitleComponent::GetAlignmentPoints() const noexcept {
        if (!m_entityId.has_value()) {
            return std::unexpected(TitleError::NotInitialized);
        }
        return m_alignmentPoints;
    }

    std::expected<AlignmentTitle, TitleError> ActorTitleComponent::GetAlignmentTitle() const noexcept {
        if (!m_entityId.has_value()) {
            return std::unexpected(TitleError::NotInitialized);
        }
        return CalculateTitleFromPoints(m_alignmentPoints);
    }

    std::expected<TitleColor, TitleError> ActorTitleComponent::GetTitleColor() const noexcept {
        if (!m_entityId.has_value()) {
            return std::unexpected(TitleError::NotInitialized);
        }
        AlignmentTitle title = CalculateTitleFromPoints(m_alignmentPoints);
        return GetColorForTitle(title);
    }

    std::expected<void, TitleError> ActorTitleComponent::SetCustomTitle(const std::string& title) {
        if (!m_entityId.has_value()) {
            return std::unexpected(TitleError::NotInitialized);
        }
        
        m_customTitle = title;
        BroadcastTitleChange();
        return {};
    }

    std::expected<void, TitleError> ActorTitleComponent::ClearCustomTitle() {
        if (!m_entityId.has_value()) {
            return std::unexpected(TitleError::NotInitialized);
        }
        
        m_customTitle.reset();
        BroadcastTitleChange();
        return {};
    }

    std::expected<std::string, TitleError> ActorTitleComponent::GetFormattedTitle() const {
        if (!m_entityId.has_value()) {
            return std::unexpected(TitleError::NotInitialized);
        }
        
        if (m_customTitle.has_value()) {
            return m_customTitle.value();
        }
        
        AlignmentTitle title = CalculateTitleFromPoints(m_alignmentPoints);
        
        switch (title) {
            case AlignmentTitle::Chivalric: return "Rycerski";
            case AlignmentTitle::Noble: return "Szlachetny";
            case AlignmentTitle::Good: return "Dobry";
            case AlignmentTitle::Friendly: return "Przyjazny";
            case AlignmentTitle::Neutral: return "";
            case AlignmentTitle::Aggressive: return "Agresywny";
            case AlignmentTitle::Fraudulent: return "Nieuczciwy";
            case AlignmentTitle::Malicious: return "Zlosliwy";
            case AlignmentTitle::Cruel: return "Okrutny";
            default: return "";
        }
    }

    AlignmentTitle ActorTitleComponent::CalculateTitleFromPoints(int points) const noexcept {
        if (points >= 12000) return AlignmentTitle::Chivalric;
        if (points >= 8000) return AlignmentTitle::Noble;
        if (points >= 4000) return AlignmentTitle::Good;
        if (points >= 1000) return AlignmentTitle::Friendly;
        if (points >= 0) return AlignmentTitle::Neutral;
        if (points > -4000) return AlignmentTitle::Aggressive;
        if (points > -8000) return AlignmentTitle::Fraudulent;
        if (points > -12000) return AlignmentTitle::Malicious;
        return AlignmentTitle::Cruel;
    }

    TitleColor ActorTitleComponent::GetColorForTitle(AlignmentTitle title) const noexcept {
        switch (title) {
            case AlignmentTitle::Chivalric: return {0, 204, 255};   // Niebieski
            case AlignmentTitle::Noble: return {0, 144, 255};       // Ciemniejszy niebieski
            case AlignmentTitle::Good: return {0, 100, 255};        // Jeszcze ciemniejszy
            case AlignmentTitle::Friendly: return {0, 0, 255};      // Ciemny niebieski
            case AlignmentTitle::Neutral: return {255, 255, 255};   // Bialy
            case AlignmentTitle::Aggressive: return {255, 100, 0};  // Pomaranczowy
            case AlignmentTitle::Fraudulent: return {255, 50, 0};   // Ciemnopomaranczowy
            case AlignmentTitle::Malicious: return {255, 0, 0};     // Czerwony
            case AlignmentTitle::Cruel: return {150, 0, 0};         // Ciemnoczerwony
            default: return {255, 255, 255};
        }
    }

    void ActorTitleComponent::BroadcastTitleChange() {
        if (m_eventBus && m_entityId.has_value()) {
            auto formattedTitleResult = GetFormattedTitle();
            std::string newTitle = formattedTitleResult.has_value() ? formattedTitleResult.value() : "";
            
            ActorTitleChangedEvent event;
            event.entityId = m_entityId.value();
            event.newTitle = newTitle;
            
            m_eventBus->Publish(event);
        }
    }

} // namespace Client::Actors

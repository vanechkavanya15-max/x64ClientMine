#include "ActorDespawnCleaner.h"
#include "EterBase/LogModern.h"

namespace Client::Actor {

ActorDespawnCleaner::ActorDespawnCleaner(
    UserInterface::PlayerControllers::PlayerTargetController* targetController,
    Client::Gameplay::ExchangeCommandHandler* exchangeHandler
) noexcept
    : m_targetController(targetController)
    , m_exchangeHandler(exchangeHandler)
{
}

EterBase::Result<void, CleanerError> ActorDespawnCleaner::CleanReferences(
    EterBase::EntityId deadActorId, 
    CleanDomain domains) const noexcept
{
    if (deadActorId.get() == 0) {
        return EterBase::MakeError(CleanerError::InvalidEntityId);
    }

    if (domains & CleanDomain::TargetSelection) {
        CleanTargetSelection(deadActorId);
    }

    if (domains & CleanDomain::ExchangeWindow) {
        CleanExchangeWindow(deadActorId);
    }

    return {};
}

void ActorDespawnCleaner::CleanTargetSelection(EterBase::EntityId deadActorId) const noexcept
{
    if (!m_targetController) {
        return;
    }

    if (m_targetController->IsSameTargetVID(deadActorId.get())) {
        m_targetController->ClearTarget();
        EterBase::ModernLogger::Info("ActorDespawnCleaner: Wyczyszczono zaznaczenie celu dla VID {}", deadActorId.get());
    }
}

void ActorDespawnCleaner::CleanExchangeWindow(EterBase::EntityId deadActorId) const noexcept
{
    if (!m_exchangeHandler) {
        return;
    }

    if (m_exchangeHandler->IsActive()) {
        const auto* exchange = m_exchangeHandler->GetExchange();
        if (exchange) {
            const auto* participant = exchange->GetParticipant(deadActorId);
            if (participant != nullptr) {
                // Jesli znikajacy aktor uczestniczy w handlu, anulujemy go.
                // Rzutowanie na void w kontekscie noexcept w celu zignorowania ewentualnych bledow domeny.
                (void)m_exchangeHandler->Cancel(); 
                EterBase::ModernLogger::Info("ActorDespawnCleaner: Anulowano okno handlu z powodu despawnu aktora VID {}", deadActorId.get());
            }
        }
    }
}

} // namespace Client::Actor

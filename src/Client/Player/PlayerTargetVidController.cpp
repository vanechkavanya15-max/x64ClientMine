#include "PlayerTargetVidController.h"
#include <limits>
#include <algorithm>

namespace Client::Player {

PlayerTargetVidController::PlayerTargetVidController(IActorQueryProvider* provider)
    : m_provider(provider)
{
    // Subskrypcja zdarzenia smierci aktora. Uzywamy formatu wymagającego wskaznika na metode i instancje this.
    (void)m_eventSubscriber.Subscribe<Client::Core::ActorDeadEvent>(this, &PlayerTargetVidController::OnActorDead);
}

Client::Core::Result<void, Client::Core::EntityError> PlayerTargetVidController::SetTargetVid(uint32_t vid) {
    if (!m_provider) {
        return std::unexpected(Client::Core::EntityError::NotFound);
    }

    if (vid == 0) {
        ClearTarget();
        return {};
    }

    if (!m_provider->IsActorAlive(vid)) {
        return std::unexpected(Client::Core::EntityError::Dead);
    }

    m_targetVid = vid;
    m_mode = TargetMode::Manual;

    return {};
}

void PlayerTargetVidController::ClearTarget() noexcept {
    m_targetVid = 0;
    m_mode = TargetMode::None;
}

void PlayerTargetVidController::LockTarget() noexcept {
    if (HasTarget() && m_mode != TargetMode::Locked) {
        m_mode = TargetMode::Locked;
    }
}

void PlayerTargetVidController::UnlockTarget() noexcept {
    if (m_mode == TargetMode::Locked) {
        // Jezeli odblokowujemy cel recznie zablokowany, traktujemy go nadal jako cel reczny
        m_mode = TargetMode::Manual;
    }
}

void PlayerTargetVidController::UpdateAutoTarget(float maxRadius) {
    if (!m_provider) {
        return;
    }

    // Blokada zapobiega automatycznym zmianom celu
    if (m_mode == TargetMode::Locked) {
        // Jezeli cel nagle umrze bedac zablokowanym, powinno to byc wylapane przez OnActorDead
        return;
    }

    // Jezeli mamy manualny cel ktory wciaz zyje i jest w miare blisko, nie przelaczamy go agresywnie
    if ((m_mode == TargetMode::Manual || m_mode == TargetMode::Auto) && HasTarget()) {
        if (m_provider->IsActorAlive(m_targetVid)) {
            float dist = m_provider->GetDistanceToActor(m_targetVid);
            if (dist <= maxRadius) {
                return; // Aktualny cel jest wciaz sensowny
            }
        }
    }

    // Auto-znajdowanie najblizszego
    uint32_t bestVid = 0;
    float minDistance = std::numeric_limits<float>::max();

    const auto allVids = m_provider->GetAllActorVids();
    for (const uint32_t vid : allVids) {
        if (!m_provider->IsActorAlive(vid)) {
            continue;
        }

        const float distance = m_provider->GetDistanceToActor(vid);
        if (distance <= maxRadius && distance < minDistance) {
            minDistance = distance;
            bestVid = vid;
        }
    }

    if (bestVid != 0) {
        m_targetVid = bestVid;
        m_mode = TargetMode::Auto;
    } else {
        // Nic nie znalezlismy w zasiegu, wiec zerujemy (jesli stary uciekl itp.)
        ClearTarget();
    }
}

void PlayerTargetVidController::OnActorDead(const Client::Core::ActorDeadEvent& event) {
    if (m_targetVid != 0 && event.vid == m_targetVid) {
        ClearTarget();
    }
}

} // namespace Client::Player

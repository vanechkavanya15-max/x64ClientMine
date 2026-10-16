#include "PhaseGameObserverBridge.h"
#include <cmath>

namespace Client::Bridge {

PhaseGameObserverBridge::PhaseGameObserverBridge() {
    m_state = ObserverState::Inactive;
    m_isObserver = false;
}

ObserverResult<void> PhaseGameObserverBridge::EnableObserverMode() noexcept {
    if (m_isObserver) {
        return Client::Core::MakeUnexpected(ObserverError::AlreadyInState);
    }
    
    m_isObserver = true;
    return TransitionTo(ObserverState::FreeCamera);
}

ObserverResult<void> PhaseGameObserverBridge::DisableObserverMode() noexcept {
    if (!m_isObserver) {
        return Client::Core::MakeUnexpected(ObserverError::NotInObserverMode);
    }

    m_isObserver = false;
    m_context.targetVid = Client::Core::EntityVid{0};
    return TransitionTo(ObserverState::Inactive);
}

bool PhaseGameObserverBridge::IsObserverMode() const noexcept {
    return m_isObserver;
}

ObserverState PhaseGameObserverBridge::GetCurrentState() const noexcept {
    return m_state;
}

ObserverResult<void> PhaseGameObserverBridge::SetTrackingTarget(Client::Core::EntityVid vid) noexcept {
    if (!m_isObserver) {
        return Client::Core::MakeUnexpected(ObserverError::NotInObserverMode);
    }

    if (vid == Client::Core::EntityVid{0}) {
        return Client::Core::MakeUnexpected(ObserverError::InvalidTargetVid);
    }

    m_context.targetVid = vid;
    return TransitionTo(ObserverState::TargetTracking);
}

ObserverResult<void> PhaseGameObserverBridge::ClearTrackingTarget() noexcept {
    if (!m_isObserver) {
        return Client::Core::MakeUnexpected(ObserverError::NotInObserverMode);
    }

    if (m_state != ObserverState::TargetTracking) {
        return Client::Core::MakeUnexpected(ObserverError::AlreadyInState);
    }

    m_context.targetVid = Client::Core::EntityVid{0};
    return TransitionTo(ObserverState::FreeCamera);
}

Client::Core::EntityVid PhaseGameObserverBridge::GetTrackingTarget() const noexcept {
    return m_context.targetVid;
}

ObserverResult<void> PhaseGameObserverBridge::SetFreeCameraPosition(const Client::Core::MapCoords& position) noexcept {
    if (!m_isObserver) {
        return Client::Core::MakeUnexpected(ObserverError::NotInObserverMode);
    }

    if (m_state != ObserverState::FreeCamera) {
        // Zamiast zwracac blad, przelaczamy na FreeCamera i ustawiamy pozycje, lub mozna rzucic blad zaleznosci od wymagan.
        // Tutaj dla wygody wymusimy FreeCamera jesli ustawiamy pozycje swobodna.
        auto transRes = TransitionTo(ObserverState::FreeCamera);
        if (!transRes.has_value()) {
            return transRes;
        }
    }

    auto valRes = ValidatePosition(position);
    if (!valRes.has_value()) {
        return valRes;
    }

    m_context.currentPosition = position;
    return ObserverResult<void>{};
}

ObserverResult<void> PhaseGameObserverBridge::MoveFreeCamera(const Client::Core::MapCoords& delta) noexcept {
    if (!m_isObserver) {
        return Client::Core::MakeUnexpected(ObserverError::NotInObserverMode);
    }
    
    if (m_state != ObserverState::FreeCamera) {
        return Client::Core::MakeUnexpected(ObserverError::PermissionDenied);
    }

    Client::Core::MapCoords newPosition = m_context.currentPosition + delta;
    
    auto valRes = ValidatePosition(newPosition);
    if (!valRes.has_value()) {
        return valRes;
    }

    m_context.currentPosition = newPosition;
    return ObserverResult<void>{};
}

Client::Core::MapCoords PhaseGameObserverBridge::GetCurrentPosition() const noexcept {
    return m_context.currentPosition;
}

void PhaseGameObserverBridge::SetConfig(const ObserverConfig& config) noexcept {
    m_config = config;
}

const ObserverConfig& PhaseGameObserverBridge::GetConfig() const noexcept {
    return m_config;
}

ObserverResult<void> PhaseGameObserverBridge::Update(uint64_t currentTimeMs) noexcept {
    if (!m_isObserver || m_state == ObserverState::Inactive) {
        return ObserverResult<void>{}; // Nic do roboty
    }

    // Obliczamy delte czasu do ewentualnych interpolacji.
    // uint64_t dt = currentTimeMs - m_context.lastUpdateTimeMs;
    m_context.lastUpdateTimeMs = currentTimeMs;

    if (m_state == ObserverState::TargetTracking) {
        // W trybie sledzenia upewniamy sie ze VID jest nadal ustawiony
        if (m_context.targetVid == Client::Core::EntityVid{0}) {
            // Cel zgubiony, przechodzimy na FreeCamera
            return TransitionTo(ObserverState::FreeCamera);
        }

        // W rzeczywistosci tutaj pobieralibysmy pozycje entity.
        // Poniewaz to jest most i jestesmy 'headless', na razie z symulujemy zachowanie lub polegamy na zewnętrznym mechanizmie aktualizacji pozcyji obsewatora z innej klasy.
        // Dla maszyny stanow po prostu trzymamy stan
    }

    return ObserverResult<void>{};
}

ObserverResult<PhaseGameObserverBridge::TelemetryData> PhaseGameObserverBridge::GetTelemetry() const noexcept {
    if (!m_isObserver) {
        return Client::Core::MakeUnexpected(ObserverError::NotInObserverMode);
    }

    TelemetryData tdata{};
    tdata.state = m_state;
    tdata.position = m_context.currentPosition;
    tdata.targetVid = m_context.targetVid.get();
    tdata.distanceToTarget = 0.0f; // W przyszlosci obliczane, jesli sledzimy target

    return ObserverResult<TelemetryData>{tdata};
}

ObserverResult<void> PhaseGameObserverBridge::ValidatePosition(const Client::Core::MapCoords& pos) const noexcept {
    if (m_config.restrictToMapBounds) {
        if (std::abs(pos.x) > m_config.maxWanderDistance || std::abs(pos.y) > m_config.maxWanderDistance) {
            return Client::Core::MakeUnexpected(ObserverError::OutOfMapBounds);
        }
        
        if (pos.z < m_config.minHeight || pos.z > m_config.maxHeight) {
            return Client::Core::MakeUnexpected(ObserverError::OutOfMapBounds);
        }
    }
    
    // Zapobieganie calkowitemu NaN itp
    if (std::isnan(pos.x) || std::isnan(pos.y) || std::isnan(pos.z)) {
         return Client::Core::MakeUnexpected(ObserverError::InvalidPosition);
    }

    return ObserverResult<void>{};
}

ObserverResult<void> PhaseGameObserverBridge::TransitionTo(ObserverState newState) noexcept {
    if (m_state == newState) {
        return ObserverResult<void>{};
    }

    // Dodatkowa logika wyjscia z poprzedniego stanu
    if (m_state == ObserverState::TargetTracking && newState == ObserverState::FreeCamera) {
        // Zwolnienie targetVid (teoretycznie powinno byc robione przez caller, ale zrobimy to zapobiegawczo)
        m_context.targetVid = Client::Core::EntityVid{0};
    }

    // Przejscie
    m_state = newState;
    return ObserverResult<void>{};
}

} // namespace Client::Bridge

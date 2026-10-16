#pragma once

#include <cstdint>
#include <expected>
#include <string_view>
#include <vector>
#include <chrono>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/ExpectedResult.h"

namespace Client::Bridge {

enum class ObserverState : uint8_t {
    Inactive = 0,
    FreeCamera,
    TargetTracking
};

enum class ObserverError : uint8_t {
    None = 0,
    AlreadyInState,
    NotInObserverMode,
    InvalidTargetVid,
    InvalidPosition,
    OutOfMapBounds,
    StateTransitionFailed,
    PermissionDenied
};

[[nodiscard]] constexpr std::string_view to_string(ObserverError error) noexcept {
    switch (error) {
        case ObserverError::None: return "ObserverError::None";
        case ObserverError::AlreadyInState: return "ObserverError::AlreadyInState - The observer is already in the requested state";
        case ObserverError::NotInObserverMode: return "ObserverError::NotInObserverMode - Operation requires active observer mode";
        case ObserverError::InvalidTargetVid: return "ObserverError::InvalidTargetVid - The target VID is invalid or missing";
        case ObserverError::InvalidPosition: return "ObserverError::InvalidPosition - The requested coordinates are invalid";
        case ObserverError::OutOfMapBounds: return "ObserverError::OutOfMapBounds - The coordinates are outside of allowed map bounds";
        case ObserverError::StateTransitionFailed: return "ObserverError::StateTransitionFailed - Failed to transition to the new state";
        case ObserverError::PermissionDenied: return "ObserverError::PermissionDenied - Permission denied for this operation";
        default: return "ObserverError::Unknown";
    }
}

template <typename T>
using ObserverResult = Client::Core::ExpectedResult<T, ObserverError>;

struct ObserverConfig {
    float maxWanderDistance{100000.0f};
    float minHeight{0.0f};
    float maxHeight{50000.0f};
    bool restrictToMapBounds{true};
    uint32_t trackingTimeoutMs{5000};
};

struct ObserverContext {
    Client::Core::MapCoords currentPosition{};
    Client::Core::EntityVid targetVid{0};
    uint64_t lastUpdateTimeMs{0};
    float cameraZoom{1.0f};
    float cameraPitch{0.0f};
    float cameraYaw{0.0f};
};

class PhaseGameObserverBridge {
public:
    PhaseGameObserverBridge();
    ~PhaseGameObserverBridge() = default;

    // Podstawowa obsluga trybu
    ObserverResult<void> EnableObserverMode() noexcept;
    ObserverResult<void> DisableObserverMode() noexcept;
    
    [[nodiscard]] bool IsObserverMode() const noexcept;
    [[nodiscard]] ObserverState GetCurrentState() const noexcept;

    // Sterowanie trybem kibica / podazania
    ObserverResult<void> SetTrackingTarget(Client::Core::EntityVid vid) noexcept;
    ObserverResult<void> ClearTrackingTarget() noexcept;
    [[nodiscard]] Client::Core::EntityVid GetTrackingTarget() const noexcept;

    // Sterowanie kamera swobodna
    ObserverResult<void> SetFreeCameraPosition(const Client::Core::MapCoords& position) noexcept;
    ObserverResult<void> MoveFreeCamera(const Client::Core::MapCoords& delta) noexcept;
    [[nodiscard]] Client::Core::MapCoords GetCurrentPosition() const noexcept;

    // Konfiguracja
    void SetConfig(const ObserverConfig& config) noexcept;
    [[nodiscard]] const ObserverConfig& GetConfig() const noexcept;

    // Aktualizacja w petli gry (Update)
    ObserverResult<void> Update(uint64_t currentTimeMs) noexcept;

    // Helper: narzedzia telemetryczne
    struct TelemetryData {
        ObserverState state;
        Client::Core::MapCoords position;
        uint32_t targetVid;
        float distanceToTarget;
    };
    [[nodiscard]] ObserverResult<TelemetryData> GetTelemetry() const noexcept;

private:
    ObserverResult<void> ValidatePosition(const Client::Core::MapCoords& pos) const noexcept;
    ObserverResult<void> TransitionTo(ObserverState newState) noexcept;

    ObserverState m_state{ObserverState::Inactive};
    ObserverConfig m_config{};
    ObserverContext m_context{};
    bool m_isObserver{false};
};

} // namespace Client::Bridge

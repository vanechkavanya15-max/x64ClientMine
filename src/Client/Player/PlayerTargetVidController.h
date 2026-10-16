#pragma once

#include <cstdint>
#include <vector>
#include <optional>
#include "Client/Core/EventSubscriber.h"
#include "Client/Core/Result.h"
#include "Client/Core/DomainEvents.h"

namespace Client::Player {

/**
 * @brief Tryby namierzania celu.
 */
enum class TargetMode : uint8_t {
    None = 0,
    Manual,
    Auto,
    Locked
};

/**
 * @brief Interfejs dostarczajacy informacje o aktorach, pozwalajacy zachowac
 * modul PlayerTargetVidController calkowicie odizolowany (100% headless).
 */
class IActorQueryProvider {
public:
    virtual ~IActorQueryProvider() = default;

    [[nodiscard]] virtual bool IsActorAlive(uint32_t vid) const = 0;
    [[nodiscard]] virtual float GetDistanceToActor(uint32_t vid) const = 0;
    [[nodiscard]] virtual std::vector<uint32_t> GetAllActorVids() const = 0;
};

/**
 * @brief C++23 SRP Kontroler namierzania i biezacego celu ataku.
 * Odpowiada wylacznie za zarzadzanie stanem i algorytmy wyboru (FindActor), 
 * ignorujac kompletnie warstwe graficzna.
 */
class PlayerTargetVidController {
public:
    explicit PlayerTargetVidController(IActorQueryProvider* provider);
    ~PlayerTargetVidController() = default;

    // Usuniecie konstruktorow kopiujacych/przenoszacych z powodu rejestracji eventow i trzymania this
    PlayerTargetVidController(const PlayerTargetVidController&) = delete;
    PlayerTargetVidController& operator=(const PlayerTargetVidController&) = delete;
    PlayerTargetVidController(PlayerTargetVidController&&) = delete;
    PlayerTargetVidController& operator=(PlayerTargetVidController&&) = delete;

    /**
     * @brief Reczne ustawienie celu (np. kliknieciem myszy).
     */
    Client::Core::Result<void, Client::Core::EntityError> SetTargetVid(uint32_t vid);

    /**
     * @brief Wyczyszczenie biezacego celu.
     */
    void ClearTarget() noexcept;

    /**
     * @brief Blokuje aktualny cel. W tym trybie auto-namierzanie nie przelaczy na inny.
     */
    void LockTarget() noexcept;

    /**
     * @brief Odblokowuje cel.
     */
    void UnlockTarget() noexcept;

    /**
     * @brief Automatyczne znalezienie i ustawienie najblizszego celu w zadanym promieniu.
     * Jezeli cel jest zablokowany (Locked) lub obecny cel jest w zasiegu, nie zmienia go.
     */
    void UpdateAutoTarget(float maxRadius);

    // Gettery
    [[nodiscard]] uint32_t GetTargetVid() const noexcept { return m_targetVid; }
    [[nodiscard]] bool HasTarget() const noexcept { return m_targetVid != 0; }
    [[nodiscard]] TargetMode GetMode() const noexcept { return m_mode; }

private:
    void OnActorDead(const Client::Core::ActorDeadEvent& event);

private:
    IActorQueryProvider* m_provider{nullptr};
    TargetMode m_mode{TargetMode::None};
    uint32_t m_targetVid{0};
    
    Client::Core::EventSubscriber m_eventSubscriber;
};

} // namespace Client::Player

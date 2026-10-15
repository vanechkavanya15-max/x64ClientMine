#pragma once

#include <optional>
#include <EterBase/Result.h>
#include <Client/Core/StrongTypes.h>

namespace Client::Actor {

/**
 * @brief Komponent odpowiedzialny za logike przemieszczania sie aktora w przestrzeni 3D.
 * Czysty C++23, zero wyciekow RAII, brak zaleznosci od Windows API/DirectX.
 */
class ActorMovementComponent {
public:
    ActorMovementComponent() noexcept = default;
    explicit ActorMovementComponent(const Client::Core::MapCoords& initialPos) noexcept;

    /**
     * @brief Zleca przemieszczenie do podanego punktu docelowego z okreslona predkoscia.
     * @param targetPos Punkt docelowy.
     * @param speed Predkosc ruchu (jednostki / sekunda). Musi byc > 0.
     * @return Sukces lub blad walidacji (np. niewlasciwa predkosc).
     */
    [[nodiscard]] EterBase::Result<void, EterBase::EntityError> MoveTo(const Client::Core::MapCoords& targetPos, float speed) noexcept;

    /**
     * @brief Zatrzymuje aktora w aktualnej pozycji.
     */
    void Stop() noexcept;

    /**
     * @brief Aktualizuje pozycje aktora na podstawie uplynietego czasu.
     * @param deltaTime Uplyniety czas (w sekundach).
     * @return Opcjonalna nowa pozycja, jesli aktor zmienil polozenie. nullopt jesli nie wykonano ruchu.
     */
    std::optional<Client::Core::MapCoords> Update(float deltaTime) noexcept;

    /**
     * @brief Zwraca aktualna pozycje aktora.
     */
    [[nodiscard]] const Client::Core::MapCoords& GetCurrentPosition() const noexcept;

    /**
     * @brief Zwraca punkt docelowy ruchu.
     */
    [[nodiscard]] const Client::Core::MapCoords& GetTargetPosition() const noexcept;

    /**
     * @brief Sprawdza, czy aktor aktualnie sie porusza.
     */
    [[nodiscard]] bool IsMoving() const noexcept;

    /**
     * @brief Ustawia natychmiastowo nowa pozycje, zatrzymujac ewentualny ruch.
     */
    void SetPosition(const Client::Core::MapCoords& pos) noexcept;

private:
    Client::Core::MapCoords m_currentPosition{};
    Client::Core::MapCoords m_targetPosition{};
    float m_speed{0.0f};
    bool m_isMoving{false};
};

} // namespace Client::Actor

#pragma once
#include <cstdint>
#include <optional>

namespace Client::Player {

class PlayerEntityPicker {
public:
    PlayerEntityPicker() = default;
    ~PlayerEntityPicker() = default;

    void SetPickedActor(uint32_t vid) noexcept { m_pickedActorVid = vid; }
    void ClearPickedActor() noexcept { m_pickedActorVid.reset(); }
    [[nodiscard]] std::optional<uint32_t> GetPickedActor() const noexcept { return m_pickedActorVid; }

    void SetPickedItem(uint32_t vid) noexcept { m_pickedItemVid = vid; }
    void ClearPickedItem() noexcept { m_pickedItemVid.reset(); }
    [[nodiscard]] std::optional<uint32_t> GetPickedItem() const noexcept { return m_pickedItemVid; }

private:
    std::optional<uint32_t> m_pickedActorVid;
    std::optional<uint32_t> m_pickedItemVid;
};

} // namespace Client::Player

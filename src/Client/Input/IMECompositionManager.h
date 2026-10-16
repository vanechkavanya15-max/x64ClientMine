#pragma once
#include <cstdint>
#include <string>
#include <vector>

namespace Client::Input {

class IMECompositionManager {
public:
    IMECompositionManager() = default;
    ~IMECompositionManager() = default;

    void SetCompositionString(const std::string& str, int32_t caretPos);
    void Clear() noexcept;

    [[nodiscard]] const std::string& GetString() const noexcept { return m_composition; }
    [[nodiscard]] int32_t GetCaretPos() const noexcept { return m_caretPos; }
    [[nodiscard]] bool HasComposition() const noexcept { return !m_composition.empty(); }

private:
    std::string m_composition{};
    int32_t m_caretPos{0};
};

} // namespace Client::Input

#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include <expected>
#include <string_view>
#include <format>
#include "Client/Core/Result.h"

namespace Client::Input {

enum class IMEError : uint8_t {
    None = 0,
    InvalidCaretPosition,
    ClauseIndexOutOfBounds,
    AttributeIndexOutOfBounds,
    InvalidUTF8Sequence
};

[[nodiscard]] inline std::string_view to_string(IMEError error) noexcept {
    switch (error) {
        case IMEError::None: return "IMEError::None";
        case IMEError::InvalidCaretPosition: return "IMEError::InvalidCaretPosition";
        case IMEError::ClauseIndexOutOfBounds: return "IMEError::ClauseIndexOutOfBounds";
        case IMEError::AttributeIndexOutOfBounds: return "IMEError::AttributeIndexOutOfBounds";
        case IMEError::InvalidUTF8Sequence: return "IMEError::InvalidUTF8Sequence";
        default: return "IMEError::Unknown";
    }
}

enum class IMECompositionAttribute : uint8_t {
    Input = 0,
    TargetConverted = 1,
    Converted = 2,
    TargetNotConverted = 3,
    InputError = 4,
    FixedConverted = 5
};

struct IMEClause {
    uint32_t startPos;
    uint32_t endPos; // Exclusive
};

class IMECompositionManager {
public:
    IMECompositionManager() = default;
    ~IMECompositionManager() = default;

    // Composition string
    void SetCompositionString(const std::string& str, int32_t caretPos);
    [[nodiscard]] const std::string& GetString() const noexcept { return m_composition; }
    [[nodiscard]] int32_t GetCaretPos() const noexcept { return m_caretPos; }
    [[nodiscard]] bool HasComposition() const noexcept { return !m_composition.empty(); }

    // Reading string
    void SetReadingString(const std::string& str);
    [[nodiscard]] const std::string& GetReadingString() const noexcept { return m_reading; }
    [[nodiscard]] bool HasReading() const noexcept { return !m_reading.empty(); }

    // Clauses
    void SetClauses(const std::vector<uint32_t>& clauseLengths);
    [[nodiscard]] const std::vector<IMEClause>& GetClauses() const noexcept { return m_clauses; }
    [[nodiscard]] Client::Core::Result<std::string, IMEError> GetClauseText(size_t clauseIndex) const;
    [[nodiscard]] size_t GetClauseCount() const noexcept { return m_clauses.size(); }

    // Attributes
    void SetAttributes(const std::vector<IMECompositionAttribute>& attributes);
    [[nodiscard]] const std::vector<IMECompositionAttribute>& GetAttributes() const noexcept { return m_attributes; }
    [[nodiscard]] Client::Core::Result<IMECompositionAttribute, IMEError> GetAttribute(size_t byteIndex) const;

    // Utilities
    void Clear() noexcept;
    
    // UTF-8 processing
    [[nodiscard]] static Client::Core::Result<size_t, IMEError> GetCodepointLength(char leadByte) noexcept;
    [[nodiscard]] Client::Core::Result<size_t, IMEError> CodepointOffsetToByteOffset(size_t codepointOffset) const noexcept;
    [[nodiscard]] Client::Core::Result<size_t, IMEError> ByteOffsetToCodepointOffset(size_t byteOffset) const noexcept;

private:
    std::string m_composition{};
    std::string m_reading{};
    int32_t m_caretPos{0};
    
    std::vector<IMEClause> m_clauses{};
    std::vector<IMECompositionAttribute> m_attributes{};
    
    bool IsValidByteOffset(size_t offset) const noexcept { return offset <= m_composition.length(); }
};

} // namespace Client::Input

template <>
struct std::formatter<Client::Input::IMEError> : std::formatter<std::string_view> {
    auto format(Client::Input::IMEError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Input::to_string(err), ctx);
    }
};

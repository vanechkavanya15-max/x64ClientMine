#pragma once

#include "Client/Core/ExpectedResult.h"
#include <string_view>
#include <charconv>
#include <cstdint>

namespace Client::Text {

enum class ColorParseError {
    InvalidFormat,
    TooShort
};

class TextTagColorParser {
public:
    static Client::Core::ExpectedResult<uint32_t, ColorParseError> ParseColor(std::string_view hexColor) noexcept {
        if (hexColor.length() < 8) {
            return Client::Core::MakeUnexpected(ColorParseError::TooShort);
        }
        
        uint32_t color = 0;
        auto [ptr, ec] = std::from_chars(hexColor.data(), hexColor.data() + 8, color, 16);
        if (ec != std::errc{} || ptr != hexColor.data() + 8) {
            return Client::Core::MakeUnexpected(ColorParseError::InvalidFormat);
        }
        return Client::Core::ExpectedResult<uint32_t, ColorParseError>(color);
    }
    
    static Client::Core::ExpectedResult<uint32_t, ColorParseError> ParseColor(std::wstring_view hexColor) noexcept {
        if (hexColor.length() < 8) {
            return Client::Core::MakeUnexpected(ColorParseError::TooShort);
        }
        
        uint32_t color = 0;
        for (size_t i = 0; i < 8; ++i) {
            wchar_t c = hexColor[i];
            uint32_t val = 0;
            if (c >= L'0' && c <= L'9') {
                val = c - L'0';
            } else if (c >= L'a' && c <= L'f') {
                val = c - L'a' + 10;
            } else if (c >= L'A' && c <= L'F') {
                val = c - L'A' + 10;
            } else {
                return Client::Core::MakeUnexpected(ColorParseError::InvalidFormat);
            }
            color = (color << 4) | val;
        }
        return Client::Core::ExpectedResult<uint32_t, ColorParseError>(color);
    }
};

} // namespace Client::Text

#include <iostream>
#include <string_view>
#include <charconv>
#include <cstdint>
#include <string>
#include <cwctype>
#include <algorithm>
#include <expected>

namespace Client::Core {
    enum class ParseError {
        InvalidFormat,
        TooShort
    };
    template <typename T, typename E>
    using ExpectedResult = std::expected<T, E>;
    template <typename T, typename E>
    constexpr auto MakeExpected(T&& value) noexcept {
        return std::expected<std::decay_t<T>, E>(std::forward<T>(value));
    }
    template <typename E>
    constexpr auto MakeUnexpected(E&& error) noexcept {
        return std::unexpected<std::decay_t<E>>(std::forward<E>(error));
    }
}

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
            return color;
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
            return color;
        }
    };
}

int main() {
    auto res1 = Client::Text::TextTagColorParser::ParseColor("ffff0000");
    if (res1) {
        std::cout << "res1: " << *res1 << std::endl;
    }
    
    auto res2 = Client::Text::TextTagColorParser::ParseColor(L"FFff0000");
    if (res2) {
        std::cout << "res2: " << *res2 << std::endl;
    }
}

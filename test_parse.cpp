#include <iostream>
#include <string_view>
#include <charconv>

int main() {
    std::string_view s = "ffff0000";
    uint32_t val;
    auto [ptr, ec] = std::from_chars(s.data(), s.data() + s.size(), val, 16);
    if (ec == std::errc{}) {
        std::cout << val << std::endl;
    } else {
        std::cout << "error" << std::endl;
    }
}

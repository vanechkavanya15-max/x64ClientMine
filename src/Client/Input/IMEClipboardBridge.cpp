#include "IMEClipboardBridge.h"

namespace Client::Input {

std::string IMEClipboardBridge::SanitizeText(const std::string& input) {
    std::string out;
    out.reserve(input.size());
    for (char c : input) {
        if (c >= 32 || c == '\t' || c == '\n') {
            out.push_back(c);
        }
    }
    return out;
}

} // namespace Client::Input

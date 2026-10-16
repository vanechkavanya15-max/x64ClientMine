#include "IMECompositionManager.h"

namespace Client::Input {

void IMECompositionManager::SetCompositionString(const std::string& str, int32_t caretPos) {
    m_composition = str;
    m_caretPos = caretPos;
}

void IMECompositionManager::Clear() noexcept {
    m_composition.clear();
    m_caretPos = 0;
}

} // namespace Client::Input

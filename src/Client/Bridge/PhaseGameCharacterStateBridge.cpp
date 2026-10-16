#include "PhaseGameCharacterStateBridge.h"

namespace Client::Bridge {

void PhaseGameCharacterStateBridge::OnPointChange(uint8_t type, int64_t value, int64_t amount) {
    m_history.push_back({type, value, amount});
}

} // namespace Client::Bridge

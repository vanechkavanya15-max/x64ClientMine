#include "HarnessWorldQueryRouter.h"
#include <sstream>

namespace Client::IPC {

std::string HarnessWorldQueryRouter::SerializeWorldSnapshot(uint32_t playerVid) const {
    std::ostringstream ss;
    ss << "{\"player_vid\":" << playerVid << ",\"entity_count\":" << m_entities.size() << ",\"entities\":[";
    for (size_t i = 0; i < m_entities.size(); ++i) {
        const auto& e = m_entities[i];
        ss << "{\"vid\":" << e.vid << ",\"race\":" << e.race << ",\"x\":" << e.x << ",\"y\":" << e.y << "}";
        if (i + 1 < m_entities.size()) ss << ",";
    }
    ss << "]}";
    return ss.str();
}

} // namespace Client::IPC

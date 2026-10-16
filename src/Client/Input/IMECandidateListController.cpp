#include "IMECandidateListController.h"

namespace Client::Input {

void IMECandidateListController::SetCandidates(const std::vector<std::string>& candidates) {
    m_candidates = candidates;
    m_selectedIndex = 0;
}

void IMECandidateListController::NextCandidate() {
    if (!m_candidates.empty() && m_selectedIndex + 1 < m_candidates.size()) {
        m_selectedIndex++;
    }
}

void IMECandidateListController::PrevCandidate() {
    if (!m_candidates.empty() && m_selectedIndex > 0) {
        m_selectedIndex--;
    }
}

std::string IMECandidateListController::GetSelectedCandidate() const {
    if (m_candidates.empty() || m_selectedIndex >= m_candidates.size()) return "";
    return m_candidates[m_selectedIndex];
}

} // namespace Client::Input

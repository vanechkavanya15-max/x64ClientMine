#pragma once
#include <cstdint>
#include <string>
#include <vector>

namespace Client::Input {

class IMECandidateListController {
public:
    IMECandidateListController() = default;
    ~IMECandidateListController() = default;

    void SetCandidates(const std::vector<std::string>& candidates);
    void SetPageSize(uint32_t pageSize) noexcept { m_pageSize = pageSize > 0 ? pageSize : 1; }
    void NextCandidate();
    void PrevCandidate();
    [[nodiscard]] std::string GetSelectedCandidate() const;
    [[nodiscard]] size_t GetCandidateCount() const noexcept { return m_candidates.size(); }

private:
    std::vector<std::string> m_candidates;
    size_t m_selectedIndex{0};
    uint32_t m_pageSize{9};
};

} // namespace Client::Input

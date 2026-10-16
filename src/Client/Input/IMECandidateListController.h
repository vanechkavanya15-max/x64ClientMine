#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <memory>
#include <expected>
#include <string_view>

namespace Client::Input {

enum class CandidateListError : uint8_t {
    None = 0,
    InvalidPageSize,
    ListEmpty,
    IndexOutOfBounds,
    FilterPatternInvalid
};

enum class LayoutMode : uint8_t {
    Vertical = 0,
    Horizontal
};

struct CandidateEntry {
    std::string text;
    uint32_t frequency;
    bool isUserDefined;
};

class IMECandidateListController {
public:
    IMECandidateListController();
    ~IMECandidateListController();
    
    // Copy/Move semantics
    IMECandidateListController(const IMECandidateListController&) = delete;
    IMECandidateListController& operator=(const IMECandidateListController&) = delete;
    IMECandidateListController(IMECandidateListController&&) noexcept;
    IMECandidateListController& operator=(IMECandidateListController&&) noexcept;

    // Core Data Methods
    void SetCandidates(const std::vector<std::string>& candidates);
    void SetCandidatesDetailed(const std::vector<CandidateEntry>& candidates);
    void ClearCandidates();
    [[nodiscard]] size_t GetCandidateCount() const noexcept;
    
    // Paging and Navigation
    std::expected<void, CandidateListError> SetPageSize(uint32_t pageSize) noexcept;
    [[nodiscard]] uint32_t GetPageSize() const noexcept;
    [[nodiscard]] uint32_t GetTotalPages() const noexcept;
    [[nodiscard]] uint32_t GetCurrentPage() const noexcept;
    
    std::expected<void, CandidateListError> NextPage();
    std::expected<void, CandidateListError> PrevPage();
    
    std::expected<void, CandidateListError> NextCandidate();
    std::expected<void, CandidateListError> PrevCandidate();
    
    // Selection state
    [[nodiscard]] std::string GetSelectedCandidate() const;
    [[nodiscard]] size_t GetSelectedIndex() const noexcept;
    std::expected<void, CandidateListError> SelectIndex(size_t index);
    
    // Layout and View
    void SetLayoutMode(LayoutMode mode) noexcept;
    [[nodiscard]] LayoutMode GetLayoutMode() const noexcept;
    [[nodiscard]] std::vector<std::string> GetCurrentPageCandidates() const;

    // Filtering and Sorting
    std::expected<void, CandidateListError> ApplyFilter(std::string_view prefix);
    void ClearFilter();
    void SortByFrequency();

private:
    struct Impl;
    std::unique_ptr<Impl> m_impl;
};

} // namespace Client::Input

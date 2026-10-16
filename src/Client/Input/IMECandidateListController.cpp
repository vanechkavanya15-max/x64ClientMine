#include "IMECandidateListController.h"
#include <algorithm>
#include <iterator>

namespace Client::Input {

struct IMECandidateListController::Impl {
    std::vector<CandidateEntry> originalCandidates;
    std::vector<CandidateEntry> filteredCandidates;
    
    size_t selectedIndex{0};
    uint32_t pageSize{9};
    LayoutMode layoutMode{LayoutMode::Vertical};
    std::string currentFilter;
    bool isFiltered{false};
    
    [[nodiscard]] const std::vector<CandidateEntry>& GetActiveCandidates() const noexcept {
        return isFiltered ? filteredCandidates : originalCandidates;
    }
    
    void ClampSelection() noexcept {
        const auto& active = GetActiveCandidates();
        if (active.empty()) {
            selectedIndex = 0;
            return;
        }
        if (selectedIndex >= active.size()) {
            selectedIndex = active.size() - 1;
        }
    }
};

IMECandidateListController::IMECandidateListController() : m_impl(std::make_unique<Impl>()) {}
IMECandidateListController::~IMECandidateListController() = default;

IMECandidateListController::IMECandidateListController(IMECandidateListController&&) noexcept = default;
IMECandidateListController& IMECandidateListController::operator=(IMECandidateListController&&) noexcept = default;

void IMECandidateListController::SetCandidates(const std::vector<std::string>& candidates) {
    std::vector<CandidateEntry> entries;
    entries.reserve(candidates.size());
    for (const auto& c : candidates) {
        entries.push_back(CandidateEntry{c, 0, false});
    }
    SetCandidatesDetailed(entries);
}

void IMECandidateListController::SetCandidatesDetailed(const std::vector<CandidateEntry>& candidates) {
    m_impl->originalCandidates = candidates;
    if (m_impl->isFiltered) {
        // Re-apply filter on new candidates
        ApplyFilter(m_impl->currentFilter);
    } else {
        m_impl->selectedIndex = 0;
    }
}

void IMECandidateListController::ClearCandidates() {
    m_impl->originalCandidates.clear();
    m_impl->filteredCandidates.clear();
    m_impl->selectedIndex = 0;
    m_impl->isFiltered = false;
    m_impl->currentFilter.clear();
}

size_t IMECandidateListController::GetCandidateCount() const noexcept {
    return m_impl->GetActiveCandidates().size();
}

std::expected<void, CandidateListError> IMECandidateListController::SetPageSize(uint32_t pageSize) noexcept {
    if (pageSize == 0) {
        return std::unexpected(CandidateListError::InvalidPageSize);
    }
    m_impl->pageSize = pageSize;
    return {};
}

uint32_t IMECandidateListController::GetPageSize() const noexcept {
    return m_impl->pageSize;
}

uint32_t IMECandidateListController::GetTotalPages() const noexcept {
    size_t count = GetCandidateCount();
    if (count == 0) return 0;
    return static_cast<uint32_t>((count + m_impl->pageSize - 1) / m_impl->pageSize);
}

uint32_t IMECandidateListController::GetCurrentPage() const noexcept {
    if (GetCandidateCount() == 0) return 0;
    return static_cast<uint32_t>(m_impl->selectedIndex / m_impl->pageSize);
}

std::expected<void, CandidateListError> IMECandidateListController::NextPage() {
    if (GetCandidateCount() == 0) {
        return std::unexpected(CandidateListError::ListEmpty);
    }
    
    uint32_t currentPage = GetCurrentPage();
    uint32_t totalPages = GetTotalPages();
    
    if (currentPage + 1 < totalPages) {
        m_impl->selectedIndex = (currentPage + 1) * m_impl->pageSize;
        m_impl->ClampSelection();
    } else {
        // Optional wrap to first page, or stay on last page. Here we wrap to start.
        m_impl->selectedIndex = 0;
    }
    return {};
}

std::expected<void, CandidateListError> IMECandidateListController::PrevPage() {
    if (GetCandidateCount() == 0) {
        return std::unexpected(CandidateListError::ListEmpty);
    }
    
    uint32_t currentPage = GetCurrentPage();
    uint32_t totalPages = GetTotalPages();
    
    if (currentPage > 0) {
        m_impl->selectedIndex = (currentPage - 1) * m_impl->pageSize;
    } else {
        // Wrap to last page
        if (totalPages > 0) {
            m_impl->selectedIndex = (totalPages - 1) * m_impl->pageSize;
        }
    }
    m_impl->ClampSelection();
    return {};
}

std::expected<void, CandidateListError> IMECandidateListController::NextCandidate() {
    if (GetCandidateCount() == 0) {
        return std::unexpected(CandidateListError::ListEmpty);
    }
    
    if (m_impl->selectedIndex + 1 < GetCandidateCount()) {
        m_impl->selectedIndex++;
    } else {
        // Wrap to first
        m_impl->selectedIndex = 0;
    }
    return {};
}

std::expected<void, CandidateListError> IMECandidateListController::PrevCandidate() {
    if (GetCandidateCount() == 0) {
        return std::unexpected(CandidateListError::ListEmpty);
    }
    
    if (m_impl->selectedIndex > 0) {
        m_impl->selectedIndex--;
    } else {
        // Wrap to last
        m_impl->selectedIndex = GetCandidateCount() - 1;
    }
    return {};
}

std::string IMECandidateListController::GetSelectedCandidate() const {
    const auto& active = m_impl->GetActiveCandidates();
    if (active.empty() || m_impl->selectedIndex >= active.size()) {
        return "";
    }
    return active[m_impl->selectedIndex].text;
}

size_t IMECandidateListController::GetSelectedIndex() const noexcept {
    return m_impl->selectedIndex;
}

std::expected<void, CandidateListError> IMECandidateListController::SelectIndex(size_t index) {
    if (index >= GetCandidateCount()) {
        return std::unexpected(CandidateListError::IndexOutOfBounds);
    }
    m_impl->selectedIndex = index;
    return {};
}

void IMECandidateListController::SetLayoutMode(LayoutMode mode) noexcept {
    m_impl->layoutMode = mode;
}

LayoutMode IMECandidateListController::GetLayoutMode() const noexcept {
    return m_impl->layoutMode;
}

std::vector<std::string> IMECandidateListController::GetCurrentPageCandidates() const {
    std::vector<std::string> pageCandidates;
    const auto& active = m_impl->GetActiveCandidates();
    if (active.empty()) return pageCandidates;
    
    uint32_t currentPage = GetCurrentPage();
    size_t startIndex = currentPage * m_impl->pageSize;
    size_t endIndex = std::min(startIndex + m_impl->pageSize, active.size());
    
    pageCandidates.reserve(endIndex - startIndex);
    for (size_t i = startIndex; i < endIndex; ++i) {
        pageCandidates.push_back(active[i].text);
    }
    return pageCandidates;
}

std::expected<void, CandidateListError> IMECandidateListController::ApplyFilter(std::string_view prefix) {
    m_impl->currentFilter = std::string(prefix);
    m_impl->isFiltered = true;
    m_impl->filteredCandidates.clear();
    
    for (const auto& entry : m_impl->originalCandidates) {
        // Simple prefix match
        if (entry.text.starts_with(prefix)) {
            m_impl->filteredCandidates.push_back(entry);
        }
    }
    
    m_impl->selectedIndex = 0;
    return {};
}

void IMECandidateListController::ClearFilter() {
    m_impl->isFiltered = false;
    m_impl->currentFilter.clear();
    m_impl->filteredCandidates.clear();
    m_impl->ClampSelection();
}

void IMECandidateListController::SortByFrequency() {
    auto& target = m_impl->isFiltered ? m_impl->filteredCandidates : m_impl->originalCandidates;
    std::stable_sort(target.begin(), target.end(), [](const CandidateEntry& a, const CandidateEntry& b) {
        return a.frequency > b.frequency;
    });
    // Optional: preserve selection based on previous text, but here we just reset for simplicity
    m_impl->selectedIndex = 0;
}

} // namespace Client::Input

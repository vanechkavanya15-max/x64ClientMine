#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Input/IMECandidateListController.h"

using namespace Client::Input;

TEST_CASE("IMECandidateListController: Basic Navigation and Wrapping") {
    IMECandidateListController controller;
    
    // Empty list tests
    CHECK(!controller.NextCandidate().has_value());
    CHECK(controller.NextCandidate().error() == CandidateListError::ListEmpty);
    CHECK(controller.GetSelectedCandidate() == "");
    
    controller.SetCandidates({"cand1", "cand2", "cand3"});
    CHECK(controller.GetSelectedCandidate() == "cand1");
    CHECK(controller.GetSelectedIndex() == 0);
    
    CHECK(controller.NextCandidate().has_value());
    CHECK(controller.GetSelectedCandidate() == "cand2");
    
    CHECK(controller.NextCandidate().has_value());
    CHECK(controller.GetSelectedCandidate() == "cand3");
    
    // Wrap to first
    CHECK(controller.NextCandidate().has_value());
    CHECK(controller.GetSelectedCandidate() == "cand1");
    
    // Wrap to last
    CHECK(controller.PrevCandidate().has_value());
    CHECK(controller.GetSelectedCandidate() == "cand3");
}

TEST_CASE("IMECandidateListController: Paging Logic") {
    IMECandidateListController controller;
    
    std::vector<std::string> cands;
    for (int i = 0; i < 20; ++i) cands.push_back("c" + std::to_string(i));
    controller.SetCandidates(cands);
    
    CHECK(controller.SetPageSize(9).has_value());
    CHECK(controller.GetPageSize() == 9);
    CHECK(controller.GetTotalPages() == 3); // 9 + 9 + 2
    CHECK(controller.GetCurrentPage() == 0);
    
    // Verify first page items
    auto pageCands = controller.GetCurrentPageCandidates();
    CHECK(pageCands.size() == 9);
    CHECK(pageCands[0] == "c0");
    CHECK(pageCands[8] == "c8");
    
    CHECK(controller.NextPage().has_value());
    CHECK(controller.GetCurrentPage() == 1);
    CHECK(controller.GetSelectedCandidate() == "c9");
    
    CHECK(controller.NextPage().has_value());
    CHECK(controller.GetCurrentPage() == 2);
    CHECK(controller.GetSelectedCandidate() == "c18");
    
    // Last page size
    pageCands = controller.GetCurrentPageCandidates();
    CHECK(pageCands.size() == 2);
    CHECK(pageCands[0] == "c18");
    CHECK(pageCands[1] == "c19");
    
    // Wrap next page to start
    CHECK(controller.NextPage().has_value());
    CHECK(controller.GetCurrentPage() == 0);
    CHECK(controller.GetSelectedCandidate() == "c0");
    
    // Prev page wraps to last
    CHECK(controller.PrevPage().has_value());
    CHECK(controller.GetCurrentPage() == 2);
    CHECK(controller.GetSelectedCandidate() == "c18");
}

TEST_CASE("IMECandidateListController: Filtering and Sorting") {
    IMECandidateListController controller;
    
    std::vector<CandidateEntry> entries = {
        {"apple", 10, false},
        {"application", 50, true},
        {"banana", 5, false},
        {"apartment", 20, false}
    };
    
    controller.SetCandidatesDetailed(entries);
    CHECK(controller.GetCandidateCount() == 4);
    
    // Filter
    CHECK(controller.ApplyFilter("ap").has_value());
    CHECK(controller.GetCandidateCount() == 3); // apple, application, apartment
    
    // Sort
    controller.SortByFrequency();
    auto pageCands = controller.GetCurrentPageCandidates();
    CHECK(pageCands.size() == 3);
    CHECK(pageCands[0] == "application"); // 50
    CHECK(pageCands[1] == "apartment");   // 20
    CHECK(pageCands[2] == "apple");       // 10
    
    // Clear filter
    controller.ClearFilter();
    CHECK(controller.GetCandidateCount() == 4);
}

TEST_CASE("IMECandidateListController: LayoutMode Validation") {
    IMECandidateListController controller;
    CHECK(controller.GetLayoutMode() == LayoutMode::Vertical); // default
    
    controller.SetLayoutMode(LayoutMode::Horizontal);
    CHECK(controller.GetLayoutMode() == LayoutMode::Horizontal);
}

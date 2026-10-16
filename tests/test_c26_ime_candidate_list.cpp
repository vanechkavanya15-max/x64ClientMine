#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Input/IMECandidateListController.h"

TEST_CASE("IMECandidateListController: Candidates navigation") {
    Client::Input::IMECandidateListController controller;
    controller.SetCandidates({"cand1", "cand2", "cand3"});
    CHECK(controller.GetSelectedCandidate() == "cand1");
    controller.NextCandidate();
    CHECK(controller.GetSelectedCandidate() == "cand2");
    controller.PrevCandidate();
    CHECK(controller.GetSelectedCandidate() == "cand1");
}

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Input/IMEOsHookController.h"

using namespace Client::Input;
using namespace Client::Core;

class MockImeOsBackend : public IImeOsBackend {
public:
    Result<void, ImeHookError> GetContext() override {
        if (failContext) return std::unexpected(ImeHookError::NoContext);
        contextAcquired = true;
        return {};
    }
    
    Result<void, ImeHookError> ReleaseContext() override {
        contextAcquired = false;
        return {};
    }

    Result<std::string, ImeHookError> GetCompositionString(int64_t flag) override {
        if (flag == GCS_COMPSTR) return compString;
        if (flag == GCS_RESULTSTR) return resultString;
        return std::unexpected(ImeHookError::InvalidParameter);
    }
    
    Result<int32_t, ImeHookError> GetCompositionCursorPos() override {
        return cursorPos;
    }
    
    Result<std::vector<std::string>, ImeHookError> GetCandidateList() override {
        return candidates;
    }
    
    Result<uint32_t, ImeHookError> GetCandidateSelection() override {
        return 0;
    }
    
    Result<uint32_t, ImeHookError> GetCandidatePageSize() override {
        return pageSize;
    }

    Result<bool, ImeHookError> IsOpen() override {
        return isOpen;
    }
    
    Result<uint32_t, ImeHookError> GetConversionMode() override {
        return conversionMode;
    }

    void SendDefaultMessage(uint32_t msg, uint64_t wparam, int64_t lparam) override {
        lastDefaultMsg = msg;
        lastWparam = wparam;
        lastLparam = lparam;
    }

    // Mock states
    bool failContext{false};
    bool contextAcquired{false};
    std::string compString{"comp"};
    std::string resultString{"result"};
    int32_t cursorPos{2};
    std::vector<std::string> candidates{"cand1", "cand2"};
    uint32_t pageSize{5};
    bool isOpen{true};
    uint32_t conversionMode{42};
    
    uint32_t lastDefaultMsg{0};
    uint64_t lastWparam{0};
    int64_t lastLparam{0};
};

TEST_CASE("IMEOsHookController: Basic capture state") {
    MockImeOsBackend backend;
    IMECompositionManager compMgr;
    IMECandidateListController candCtrl;
    IMEOsHookController hook(&backend, &compMgr, &candCtrl);
    
    CHECK(hook.IsCapturing());
    hook.EnableInputCapture(false);
    CHECK_FALSE(hook.IsCapturing());
    
    auto res = hook.HandleOsMessage(WM_IME_SETCONTEXT, 1, 0);
    CHECK_FALSE(res.value()); // capturing is false, returns false
    
    hook.EnableInputCapture(true);
    hook.SetReadingInformation(true);
    CHECK(hook.HasReadingInformation());
}

TEST_CASE("IMEOsHookController: Missing backend") {
    IMEOsHookController hook(nullptr, nullptr, nullptr);
    auto res = hook.HandleOsMessage(WM_IME_SETCONTEXT, 1, 0);
    CHECK_FALSE(res.has_value());
    CHECK(res.error() == ImeHookError::BackendFailure);
}

TEST_CASE("IMEOsHookController: Handle SetContext") {
    MockImeOsBackend backend;
    IMECompositionManager compMgr;
    IMECandidateListController candCtrl;
    IMEOsHookController hook(&backend, &compMgr, &candCtrl);

    SUBCASE("Active context") {
        auto res = hook.HandleOsMessage(WM_IME_SETCONTEXT, 1, 0x80000000);
        CHECK(res.has_value());
        CHECK(res.value());
        CHECK(hook.IsContextActive());
        CHECK(hook.IsImeOpen());
        CHECK(hook.GetCurrentConversionMode() == 42);
        CHECK(backend.lastDefaultMsg == WM_IME_SETCONTEXT);
        CHECK(backend.lastLparam == 0); // 0x80000000 should be cleared
    }

    SUBCASE("Inactive context") {
        auto res = hook.HandleOsMessage(WM_IME_SETCONTEXT, 0, 0x123);
        CHECK(res.has_value());
        CHECK(res.value());
        CHECK_FALSE(hook.IsContextActive());
        CHECK(backend.lastLparam == 0x123); // Not cleared when inactive
    }
}

TEST_CASE("IMEOsHookController: Start and End Composition") {
    MockImeOsBackend backend;
    IMECompositionManager compMgr;
    IMECandidateListController candCtrl;
    IMEOsHookController hook(&backend, &compMgr, &candCtrl);

    compMgr.SetCompositionString("test", 4);
    CHECK(compMgr.HasComposition());

    auto res = hook.HandleOsMessage(WM_IME_STARTCOMPOSITION, 0, 0);
    CHECK(res.has_value());
    CHECK_FALSE(compMgr.HasComposition());

    compMgr.SetCompositionString("test2", 5);
    res = hook.HandleOsMessage(WM_IME_ENDCOMPOSITION, 0, 0);
    CHECK(res.has_value());
    CHECK_FALSE(compMgr.HasComposition());
}

TEST_CASE("IMEOsHookController: Handle Composition") {
    MockImeOsBackend backend;
    IMECompositionManager compMgr;
    IMECandidateListController candCtrl;
    IMEOsHookController hook(&backend, &compMgr, &candCtrl);

    SUBCASE("Fails if context cannot be acquired") {
        backend.failContext = true;
        auto res = hook.HandleOsMessage(WM_IME_COMPOSITION, 0, GCS_COMPSTR);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == ImeHookError::NoContext);
    }

    SUBCASE("Result string") {
        auto res = hook.HandleOsMessage(WM_IME_COMPOSITION, 0, GCS_RESULTSTR);
        CHECK(res.has_value());
        // Result string sets it, but immediately clears it as it is finalized
        CHECK_FALSE(compMgr.HasComposition());
    }

    SUBCASE("Composition string") {
        auto res = hook.HandleOsMessage(WM_IME_COMPOSITION, 0, GCS_COMPSTR);
        CHECK(res.has_value());
        CHECK(compMgr.HasComposition());
        CHECK(compMgr.GetString() == "comp");
        CHECK(compMgr.GetCaretPos() == 2);
    }
}

TEST_CASE("IMEOsHookController: Handle Notify") {
    MockImeOsBackend backend;
    IMECompositionManager compMgr;
    IMECandidateListController candCtrl;
    IMEOsHookController hook(&backend, &compMgr, &candCtrl);

    SUBCASE("Set Open Status") {
        backend.isOpen = false;
        hook.HandleOsMessage(WM_IME_NOTIFY, IMN_SETOPENSTATUS, 0);
        CHECK_FALSE(hook.IsImeOpen());
    }

    SUBCASE("Set Conversion Mode") {
        backend.conversionMode = 99;
        hook.HandleOsMessage(WM_IME_NOTIFY, IMN_SETCONVERSIONMODE, 0);
        CHECK(hook.GetCurrentConversionMode() == 99);
    }

    SUBCASE("Open / Change Candidate") {
        hook.HandleOsMessage(WM_IME_NOTIFY, IMN_OPENCANDIDATE, 0);
        CHECK(candCtrl.GetCandidateCount() == 2);
    }

    SUBCASE("Close Candidate") {
        hook.HandleOsMessage(WM_IME_NOTIFY, IMN_OPENCANDIDATE, 0);
        CHECK(candCtrl.GetCandidateCount() == 2);
        hook.HandleOsMessage(WM_IME_NOTIFY, IMN_CLOSECANDIDATE, 0);
        CHECK(candCtrl.GetCandidateCount() == 0);
    }
}

TEST_CASE("IMEOsHookController: Ignored Messages") {
    MockImeOsBackend backend;
    IMECompositionManager compMgr;
    IMECandidateListController candCtrl;
    IMEOsHookController hook(&backend, &compMgr, &candCtrl);

    auto res = hook.HandleOsMessage(WM_IME_KEYDOWN, 0, 0);
    CHECK(res.has_value());
    CHECK_FALSE(res.value()); // Returns false to let default proc handle
}

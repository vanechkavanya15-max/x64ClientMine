#include "Client/Input/IMEOsHookController.h"
#include <iostream>

namespace Client::Input {

IMEOsHookController::IMEOsHookController(IImeOsBackend* backend, IMECompositionManager* compManager, IMECandidateListController* candController)
    : m_backend(backend), m_compManager(compManager), m_candController(candController) {
}

Client::Core::Result<bool, ImeHookError> IMEOsHookController::HandleOsMessage(uint32_t msg, uint64_t wparam, int64_t lparam) {
    if (!m_capturing) {
        return false;
    }

    if (!m_backend) {
        return std::unexpected(ImeHookError::BackendFailure);
    }

    switch (msg) {
        case WM_IME_SETCONTEXT: {
            auto res = HandleSetContext(wparam, lparam);
            if (!res) return std::unexpected(res.error());
            return true;
        }
        case WM_IME_STARTCOMPOSITION: {
            auto res = HandleStartComposition(wparam, lparam);
            if (!res) return std::unexpected(res.error());
            return true;
        }
        case WM_IME_COMPOSITION: {
            auto res = HandleComposition(wparam, lparam);
            if (!res) return std::unexpected(res.error());
            return true;
        }
        case WM_IME_ENDCOMPOSITION: {
            auto res = HandleEndComposition(wparam, lparam);
            if (!res) return std::unexpected(res.error());
            return true;
        }
        case WM_IME_NOTIFY: {
            auto res = HandleNotify(wparam, lparam);
            if (!res) return std::unexpected(res.error());
            return true;
        }
        case WM_IME_REQUEST: {
            auto res = HandleRequest(wparam, lparam);
            if (!res) return std::unexpected(res.error());
            return true;
        }
        case WM_IME_CONTROL:
        case WM_IME_COMPOSITIONFULL:
        case WM_IME_SELECT:
        case WM_IME_CHAR:
        case WM_IME_KEYDOWN:
        case WM_IME_KEYUP:
            // Just let the default window proc handle these for now.
            return false;
        default:
            return false;
    }
}

Client::Core::Result<void, ImeHookError> IMEOsHookController::HandleSetContext(uint64_t wparam, int64_t lparam) {
    bool active = (wparam != 0);
    m_contextActive = active;

    if (active) {
        // Clear ISC_SHOWUICOMPOSITIONWINDOW (0x80000000) so we draw it ourselves
        lparam &= ~0x80000000;
        m_backend->SendDefaultMessage(WM_IME_SETCONTEXT, wparam, lparam);
        
        if (auto openRes = m_backend->IsOpen()) {
            m_isImeOpen = *openRes;
        }
        if (auto modeRes = m_backend->GetConversionMode()) {
            m_currentConversionMode = *modeRes;
        }
    } else {
        m_backend->SendDefaultMessage(WM_IME_SETCONTEXT, wparam, lparam);
    }
    return {};
}

Client::Core::Result<void, ImeHookError> IMEOsHookController::HandleStartComposition(uint64_t wparam, int64_t lparam) {
    (void)wparam;
    (void)lparam;
    if (m_compManager) {
        m_compManager->Clear();
    }
    return {};
}

Client::Core::Result<void, ImeHookError> IMEOsHookController::HandleComposition(uint64_t wparam, int64_t lparam) {
    if (!m_backend) {
        return std::unexpected(ImeHookError::BackendFailure);
    }

    auto ctxRes = m_backend->GetContext();
    if (!ctxRes) {
        return std::unexpected(ctxRes.error());
    }

    if (lparam & GCS_RESULTSTR) {
        auto resultStr = m_backend->GetCompositionString(GCS_RESULTSTR);
        if (resultStr && m_compManager) {
            // Note: In a real integration, this might be sent as a finalized string
            // For now, we update the comp manager which might just consume it
            m_compManager->SetCompositionString(*resultStr, resultStr->length());
            // Immediately clear after sending result since it's finalized
            m_compManager->Clear();
        }
    }

    if (lparam & GCS_COMPSTR) {
        auto compStr = m_backend->GetCompositionString(GCS_COMPSTR);
        auto cursorPos = m_backend->GetCompositionCursorPos();
        
        if (compStr && m_compManager) {
            int32_t pos = cursorPos ? *cursorPos : compStr->length();
            m_compManager->SetCompositionString(*compStr, pos);
        }
    }

    m_backend->ReleaseContext();
    return {};
}

Client::Core::Result<void, ImeHookError> IMEOsHookController::HandleEndComposition(uint64_t wparam, int64_t lparam) {
    (void)wparam;
    (void)lparam;
    if (m_compManager) {
        m_compManager->Clear();
    }
    return {};
}

Client::Core::Result<void, ImeHookError> IMEOsHookController::HandleNotify(uint64_t wparam, int64_t lparam) {
    (void)lparam;
    switch (static_cast<OsImeNotify>(wparam)) {
        case IMN_SETOPENSTATUS: {
            auto ctxRes = m_backend->GetContext();
            if (ctxRes) {
                if (auto openRes = m_backend->IsOpen()) {
                    m_isImeOpen = *openRes;
                }
                m_backend->ReleaseContext();
            }
            break;
        }
        case IMN_SETCONVERSIONMODE: {
            auto ctxRes = m_backend->GetContext();
            if (ctxRes) {
                if (auto modeRes = m_backend->GetConversionMode()) {
                    m_currentConversionMode = *modeRes;
                }
                m_backend->ReleaseContext();
            }
            break;
        }
        case IMN_OPENCANDIDATE:
        case IMN_CHANGECANDIDATE: {
            UpdateCandidateList();
            break;
        }
        case IMN_CLOSECANDIDATE: {
            if (m_candController) {
                m_candController->SetCandidates({});
            }
            break;
        }
        default:
            break;
    }
    return {};
}

Client::Core::Result<void, ImeHookError> IMEOsHookController::HandleRequest(uint64_t wparam, int64_t lparam) {
    (void)lparam;
    switch (static_cast<OsImeRequest>(wparam)) {
        case IMR_COMPOSITIONWINDOW:
        case IMR_CANDIDATEWINDOW:
        case IMR_COMPOSITIONFONT:
        case IMR_QUERYCHARPOSITION:
            // Custom handling requires specific lparam pointer casts which we can't do purely headlessly,
            // so we typically let the backend answer these if they are forwarded, or just ignore.
            break;
        default:
            break;
    }
    return {};
}

void IMEOsHookController::UpdateCandidateList() {
    if (!m_backend || !m_candController) return;

    auto ctxRes = m_backend->GetContext();
    if (!ctxRes) return;

    auto candidates = m_backend->GetCandidateList();
    if (candidates) {
        m_candController->SetCandidates(*candidates);
        if (auto pageSize = m_backend->GetCandidatePageSize()) {
            m_candController->SetPageSize(*pageSize);
        }
    }

    m_backend->ReleaseContext();
}

} // namespace Client::Input

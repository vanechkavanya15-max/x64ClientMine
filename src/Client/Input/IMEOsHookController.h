#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include <string_view>
#include <expected>
#include <format>
#include "Client/Core/Result.h"
#include "Client/Input/IMECompositionManager.h"
#include "Client/Input/IMECandidateListController.h"

namespace Client::Input {

enum class ImeHookError : uint8_t {
    None = 0,
    InvalidMessage,
    NoContext,
    BackendFailure,
    InvalidParameter
};

constexpr std::string_view to_string(ImeHookError err) noexcept {
    switch (err) {
        case ImeHookError::None: return "None";
        case ImeHookError::InvalidMessage: return "InvalidMessage";
        case ImeHookError::NoContext: return "NoContext";
        case ImeHookError::BackendFailure: return "BackendFailure";
        case ImeHookError::InvalidParameter: return "InvalidParameter";
        default: return "Unknown";
    }
}

} // namespace Client::Input

template <>
struct std::formatter<Client::Input::ImeHookError> : std::formatter<std::string_view> {
    auto format(Client::Input::ImeHookError err, format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Input::to_string(err), ctx);
    }
};

namespace Client::Input {

enum OsImeMessage : uint32_t {
    WM_IME_SETCONTEXT = 0x0281,
    WM_IME_NOTIFY = 0x0282,
    WM_IME_CONTROL = 0x0283,
    WM_IME_COMPOSITIONFULL = 0x0284,
    WM_IME_SELECT = 0x0285,
    WM_IME_CHAR = 0x0286,
    WM_IME_REQUEST = 0x0288,
    WM_IME_KEYDOWN = 0x0290,
    WM_IME_KEYUP = 0x0291,
    WM_IME_STARTCOMPOSITION = 0x010D,
    WM_IME_ENDCOMPOSITION = 0x010E,
    WM_IME_COMPOSITION = 0x010F
};

enum OsImeNotify : uint32_t {
    IMN_CLOSESTATUSWINDOW = 0x0001,
    IMN_OPENSTATUSWINDOW = 0x0002,
    IMN_CHANGECANDIDATE = 0x0003,
    IMN_CLOSECANDIDATE = 0x0004,
    IMN_OPENCANDIDATE = 0x0005,
    IMN_SETCONVERSIONMODE = 0x0006,
    IMN_SETSENTENCEMODE = 0x0007,
    IMN_SETOPENSTATUS = 0x0008,
    IMN_SETCANDIDATEPOS = 0x0009,
    IMN_SETCOMPOSITIONFONT = 0x000A,
    IMN_SETCOMPOSITIONWINDOW = 0x000B,
    IMN_SETSTATUSWINDOWPOS = 0x000C,
    IMN_GUIDELINE = 0x000D,
    IMN_PRIVATE = 0x000E
};

enum OsImeCompositionParam : int64_t {
    GCS_COMPREADSTR = 0x0001,
    GCS_COMPREADATTR = 0x0002,
    GCS_COMPREADCLAUSE = 0x0004,
    GCS_COMPSTR = 0x0008,
    GCS_COMPATTR = 0x0010,
    GCS_COMPCLAUSE = 0x0020,
    GCS_CURSORPOS = 0x0080,
    GCS_DELTASTART = 0x0100,
    GCS_RESULTREADSTR = 0x0200,
    GCS_RESULTREADCLAUSE = 0x0400,
    GCS_RESULTSTR = 0x0800,
    GCS_RESULTCLAUSE = 0x1000
};

enum OsImeRequest : uint64_t {
    IMR_COMPOSITIONWINDOW = 0x0001,
    IMR_CANDIDATEWINDOW = 0x0002,
    IMR_COMPOSITIONFONT = 0x0003,
    IMR_RECONVERTSTRING = 0x0004,
    IMR_CONFIRMRECONVERTSTRING = 0x0005,
    IMR_QUERYCHARPOSITION = 0x0006,
    IMR_DOCUMENTFEED = 0x0007
};

class IImeOsBackend {
public:
    virtual ~IImeOsBackend() = default;

    virtual Client::Core::Result<void, ImeHookError> GetContext() = 0;
    virtual Client::Core::Result<void, ImeHookError> ReleaseContext() = 0;

    virtual Client::Core::Result<std::string, ImeHookError> GetCompositionString(int64_t flag) = 0;
    virtual Client::Core::Result<int32_t, ImeHookError> GetCompositionCursorPos() = 0;
    virtual Client::Core::Result<std::vector<std::string>, ImeHookError> GetCandidateList() = 0;
    virtual Client::Core::Result<uint32_t, ImeHookError> GetCandidateSelection() = 0;
    virtual Client::Core::Result<uint32_t, ImeHookError> GetCandidatePageSize() = 0;

    virtual Client::Core::Result<bool, ImeHookError> IsOpen() = 0;
    virtual Client::Core::Result<uint32_t, ImeHookError> GetConversionMode() = 0;
    
    // Send a message back to the application or default window procedure
    virtual void SendDefaultMessage(uint32_t msg, uint64_t wparam, int64_t lparam) = 0;
};

class IMEOsHookController {
public:
    IMEOsHookController(IImeOsBackend* backend, IMECompositionManager* compManager, IMECandidateListController* candController);
    ~IMEOsHookController() = default;

    Client::Core::Result<bool, ImeHookError> HandleOsMessage(uint32_t msg, uint64_t wparam, int64_t lparam);

    void EnableInputCapture(bool enable) noexcept { m_capturing = enable; }
    [[nodiscard]] bool IsCapturing() const noexcept { return m_capturing; }

    void SetReadingInformation(bool active) noexcept { m_reading = active; }
    [[nodiscard]] bool HasReadingInformation() const noexcept { return m_reading; }

    [[nodiscard]] bool IsContextActive() const noexcept { return m_contextActive; }
    [[nodiscard]] uint32_t GetCurrentConversionMode() const noexcept { return m_currentConversionMode; }
    [[nodiscard]] bool IsImeOpen() const noexcept { return m_isImeOpen; }

private:
    Client::Core::Result<void, ImeHookError> HandleSetContext(uint64_t wparam, int64_t lparam);
    Client::Core::Result<void, ImeHookError> HandleStartComposition(uint64_t wparam, int64_t lparam);
    Client::Core::Result<void, ImeHookError> HandleComposition(uint64_t wparam, int64_t lparam);
    Client::Core::Result<void, ImeHookError> HandleEndComposition(uint64_t wparam, int64_t lparam);
    Client::Core::Result<void, ImeHookError> HandleNotify(uint64_t wparam, int64_t lparam);
    Client::Core::Result<void, ImeHookError> HandleRequest(uint64_t wparam, int64_t lparam);

    void UpdateCandidateList();

    IImeOsBackend* m_backend{nullptr};
    IMECompositionManager* m_compManager{nullptr};
    IMECandidateListController* m_candController{nullptr};

    bool m_capturing{true};
    bool m_reading{false};
    bool m_contextActive{false};
    bool m_isImeOpen{false};
    uint32_t m_currentConversionMode{0};
};

} // namespace Client::Input

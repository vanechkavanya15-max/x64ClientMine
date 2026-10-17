#ifndef __INC_ETERLIB_DEBUG_H__
#define __INC_ETERLIB_DEBUG_H__

#include <windows.h>
#include <format>
#include <string_view>
#include <string>
#include <source_location>
#include <atomic>
#include <cstdint>

#if defined(_DEBUG) && !defined(_PACKETDUMP)
#define _PACKETDUMP
#endif

// ============================================================================
// LOG LEVELS I KONTROLA POZIOMU LOGOWANIA
// ============================================================================
extern void SetLogLevel(UINT uLevel);
extern UINT GetLogLevel();

// ============================================================================
// HEADLESS MODE I KONTROLA DIALOGOW WIN32 (FILAR B & C)
// ============================================================================
extern bool g_bHeadlessMode;
extern void SetHeadlessMode(bool bEnable) noexcept;
[[nodiscard]] extern bool IsHeadlessMode() noexcept;

// ============================================================================
// BEZPIECZNE FUNKCJE NISKOPOZIOMOWE (std::string_view, ZERO BUFFER OVERFLOW)
// ============================================================================
extern void TraceRaw(std::string_view msg, bool appendNewline = false);
extern void TraceErrorRaw(std::string_view msg, bool appendNewline = true);
extern void LogRaw(UINT uLevel, std::string_view msg, bool appendNewline = false);
extern void LogFileRaw(std::string_view msg);
extern void LogBoxRaw(std::string_view msg, std::string_view caption = "LOG", HWND hWnd = NULL);
extern void PacketDumpRaw(std::string_view msg);
extern void TempTraceRaw(std::string_view msg, bool errType = false, bool appendNewline = false);

// ============================================================================
// NOWOCZESNE SZABLONY std::format (C++20/C++23) Z WALIDACJA W CZASIE KOMPILACJI
// ============================================================================

template <typename... Args>
void TraceErrorFmt(std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        TraceErrorRaw(s, true);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[TraceErrorFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[TraceErrorFmt unknown exception]", true);
    }
}

template <typename... Args>
void TraceErrorWithoutEnterFmt(std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        TraceErrorRaw(s, false);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[TraceErrorWithoutEnterFmt exception: {}]", e.what()), false);
    }
    catch (...)
    {
        TraceErrorRaw("[TraceErrorWithoutEnterFmt unknown exception]", false);
    }
}

template <typename... Args>
void TraceFmt(std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        TraceRaw(s, false);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[TraceFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[TraceFmt unknown exception]", true);
    }
}

template <typename... Args>
void TracenFmt(std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        TraceRaw(s, true);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[TracenFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[TracenFmt unknown exception]", true);
    }
}

template <typename... Args>
void LogFmt(UINT uLevel, std::format_string<Args...> fmt, Args&&... args)
{
    if (uLevel < GetLogLevel())
        return;

    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        LogRaw(uLevel, s, false);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[LogFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[LogFmt unknown exception]", true);
    }
}

template <typename... Args>
void LognFmt(UINT uLevel, std::format_string<Args...> fmt, Args&&... args)
{
    if (uLevel < GetLogLevel())
        return;

    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        LogRaw(uLevel, s, true);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[LognFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[LognFmt unknown exception]", true);
    }
}

template <typename... Args>
void LogFileFmt(std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        LogFileRaw(s);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[LogFileFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[LogFileFmt unknown exception]", true);
    }
}

template <typename... Args>
void LogBoxFmt(std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        LogBoxRaw(s);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[LogBoxFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[LogBoxFmt unknown exception]", true);
    }
}

template <typename... Args>
void PacketDumpFmt(std::format_string<Args...> fmt, Args&&... args)
{
#ifdef _PACKETDUMP
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        PacketDumpRaw(s);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[PacketDumpFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[PacketDumpFmt unknown exception]", true);
    }
#else
    (void)sizeof...(args);
#endif
}

template <typename... Args>
void TempTraceFmt(bool errType, std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        TempTraceRaw(s, errType, false);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[TempTraceFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[TempTraceFmt unknown exception]", true);
    }
}

template <typename... Args>
void TempTracenFmt(bool errType, std::format_string<Args...> fmt, Args&&... args)
{
    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        TempTraceRaw(s, errType, true);
    }
    catch (const std::exception& e)
    {
        TraceErrorRaw(std::format("[TempTracenFmt exception: {}]", e.what()), true);
    }
    catch (...)
    {
        TraceErrorRaw("[TempTracenFmt unknown exception]", true);
    }
}

// ============================================================================
// 100% KOMPATYBILNOSC WSTECZNA DLA ISTNIEJACYCH WYWOLAN C-STYLE (printf / va_list)
// ============================================================================
extern void Log(UINT uLevel, const char* c_szMsg);
extern void Logn(UINT uLevel, const char* c_szMsg);
extern void Logf(UINT uLevel, const char* c_szFormat, ...);
extern void Lognf(UINT uLevel, const char* c_szFormat, ...);

extern void Trace(const char* c_szMsg);
extern void Tracen(const char* c_szMsg);
extern void Tracenf(const char* c_szFormat, ...);
extern void Tracef(const char* c_szFormat, ...);
extern void TraceError(const char* c_szFormat, ...);
extern void TraceErrorWithoutEnter(const char* c_szFormat, ...);

// MR-11: Separate packet dump log from the main log file
extern void TempTrace(const char* c_szMsg, bool errType = false);
extern void TempTracef(const char* c_szFormat, bool errType = false, ...);
extern void TempTracen(const char* c_szMsg, bool errType = false);
extern void TempTracenf(const char* c_szFormat, bool errType = false, ...);

extern void PacketDump(const char* c_szMsg);
extern void PacketDumpf(const char* c_szFormat, ...);
// MR-11: -- END OF -- Separate packet dump log from the main log file

extern void LogBox(const char* c_szMsg, const char* c_szCaption = NULL, HWND hWnd = NULL);
extern void LogBoxf(const char* c_szMsg, ...);

extern void LogFile(const char* c_szMsg);
extern void LogFilef(const char* c_szMessage, ...);
extern void OpenConsoleWindow(void);
extern void CloseConsoleWindow();
extern void SetupLog(void);

extern void OpenLogFile(bool bUseLogFile = true);
extern void CloseLogFile();

extern HWND g_PopupHwnd;

// ============================================================================
// LOCK-FREE DIAGNOSTYKA AI (FAZA 3: TraceErrorOnce / TraceErrorOnceKeyed)
// ============================================================================
namespace EterBase::Diagnostics
{
    inline constexpr uint64_t kFnvOffsetBasis = 14695981039346656037ULL;
    inline constexpr uint64_t kFnvPrime       = 1099511628211ULL;

    [[nodiscard]] constexpr uint64_t HashBytesFnv1a(const void* data, size_t len, uint64_t hash = kFnvOffsetBasis) noexcept
    {
        const auto* bytes = static_cast<const uint8_t*>(data);
        for (size_t i = 0; i < len; ++i)
        {
            hash ^= static_cast<uint64_t>(bytes[i]);
            hash *= kFnvPrime;
        }
        return hash;
    }

    [[nodiscard]] constexpr uint64_t HashStringFnv1a(std::string_view sv, uint64_t hash = kFnvOffsetBasis) noexcept
    {
        for (char c : sv)
        {
            hash ^= static_cast<uint64_t>(static_cast<unsigned char>(c));
            hash *= kFnvPrime;
        }
        return hash;
    }

    [[nodiscard]] constexpr uint64_t HashCombine(uint64_t seed, uint64_t value) noexcept
    {
        seed ^= value + 0x9e3779b97f4a7c15ULL + (seed << 6) + (seed >> 2);
        return seed;
    }

    [[nodiscard]] constexpr uint64_t HashSourceLocation(const std::source_location& loc) noexcept
    {
        uint64_t h = HashStringFnv1a(loc.file_name());
        h = HashCombine(h, static_cast<uint64_t>(loc.line()));
        h = HashCombine(h, static_cast<uint64_t>(loc.column()));
        return h;
    }

    class CLockFreeOnceRegistry
    {
    public:
        static constexpr size_t kCapacity = 8192; // Potega dwojki (64 KB w BSS)
        static constexpr size_t kMask     = kCapacity - 1;
        static constexpr size_t kMaxProbe = 16;

        [[nodiscard]] static bool TryRegister(uint64_t keyHash) noexcept
        {
            if (keyHash == 0)
                keyHash = 1;

            const size_t startIdx = static_cast<size_t>(keyHash) & kMask;

            for (size_t probe = 0; probe < kMaxProbe; ++probe)
            {
                const size_t idx = (startIdx + probe) & kMask;
                uint64_t existing = ms_slots[idx].load(std::memory_order_relaxed);

                if (existing == keyHash)
                    return false;

                if (existing == 0)
                {
                    uint64_t expected = 0;
                    if (ms_slots[idx].compare_exchange_strong(
                            expected, keyHash,
                            std::memory_order_acq_rel,
                            std::memory_order_relaxed))
                    {
                        return true;
                    }
                    if (expected == keyHash)
                        return false;
                }
            }
            return false;
        }

        static void ResetForTesting() noexcept
        {
            for (size_t i = 0; i < kCapacity; ++i)
                ms_slots[i].store(0, std::memory_order_relaxed);
        }

    private:
        inline static std::atomic<uint64_t> ms_slots[kCapacity]{};
    };

    inline void TraceErrorOnceImpl(const std::source_location& loc, const char* c_szFormat, ...) noexcept
    {
        if (!c_szFormat)
            return;

        const uint64_t locHash = HashSourceLocation(loc);
        if (!CLockFreeOnceRegistry::TryRegister(locHash))
            return;

        char szBuffer[1024];
        va_list args;
        va_start(args, c_szFormat);
        const int written = vsnprintf(szBuffer, sizeof(szBuffer), c_szFormat, args);
        va_end(args);

        if (written <= 0)
            return;

        char szFinal[1280];
        _snprintf_s(szFinal, sizeof(szFinal), _TRUNCATE, "[ONCE][%s:%u] %s",
            loc.function_name(), loc.line(), szBuffer);
        TraceErrorRaw(szFinal, true);
    }
} // namespace EterBase::Diagnostics

#define TRACE_ERROR_ONCE(fmt, ...) \
    do { \
        static std::atomic_flag s_onceFlag = ATOMIC_FLAG_INIT; \
        if (!s_onceFlag.test(std::memory_order_relaxed) && !s_onceFlag.test_and_set(std::memory_order_relaxed)) { \
            ::EterBase::Diagnostics::TraceErrorOnceImpl(::std::source_location::current(), fmt, ##__VA_ARGS__); \
        } \
    } while (0)

#ifndef TraceErrorOnce
#define TraceErrorOnce(fmt, ...) TRACE_ERROR_ONCE(fmt, ##__VA_ARGS__)
#endif

template <typename... Args>
inline void TraceErrorOnceKeyed(
    uint64_t runtimeKey,
    const std::source_location& loc,
    std::format_string<Args...> fmt,
    Args&&... args) noexcept
{
    const uint64_t compositeKey = ::EterBase::Diagnostics::HashCombine(
        ::EterBase::Diagnostics::HashSourceLocation(loc), runtimeKey);

    if (!::EterBase::Diagnostics::CLockFreeOnceRegistry::TryRegister(compositeKey))
        return;

    try
    {
        std::string s = std::format(fmt, std::forward<Args>(args)...);
        TraceErrorRaw(std::format("[ONCE][{}:{}] {}", loc.function_name(), loc.line(), s), true);
    }
    catch (...)
    {
        TraceErrorRaw("[TraceErrorOnceKeyed formatting exception]", true);
    }
}

#define TRACE_ERROR_ONCE_KEYED(key, fmt, ...) \
    TraceErrorOnceKeyed(static_cast<uint64_t>(key), ::std::source_location::current(), fmt, ##__VA_ARGS__)

#define CHECK_RETURN(flag, string)          \
    if (flag)                               \
    {                                       \
        LogBox(string);                     \
        return;                             \
    }

// ============================================================================
// NOWOCZESNA ASERCJA (std::source_location, CZYSTY ZRZUT BLEDU, BEZ throw "ffs")
// ============================================================================
namespace EterBase {
    bool HandleAssertFailure(const char* expr, const std::source_location& loc);
    bool HandleAssertFailure(const char* expr, const char* file, int line, const char* function = "");
}

#define MD_ASSERT(expr) ((expr) ? true : (::EterBase::HandleAssertFailure(#expr, ::std::source_location::current()), false))

#endif // __INC_ETERLIB_DEBUG_H__

#pragma once

#include "EterBase/EventBus.h"
#include <expected>
#include <cstdint>
#include <string_view>
#include <string>

namespace Client::Events {

/**
 * @brief Reprezentuje blad walidacji zdarzen interfejsu uzytkownika.
 */
enum class UIEventError : uint8_t {
    InvalidResolution,
    InvalidWindowId
};

/**
 * @brief Zdarzenie wywolywane, gdy zmianie ulega rozdzielczosc okna gry (lub przejscie w tryb pelnoekranowy).
 */
struct ScreenResolutionChangedEvent final : public ::EterBase::IEvent {
    uint32_t width;
    uint32_t height;
    bool isFullscreen;

    constexpr ScreenResolutionChangedEvent(uint32_t w, uint32_t h, bool full) noexcept
        : width(w), height(h), isFullscreen(full) {}

    constexpr std::expected<void, UIEventError> Validate() const noexcept {
        if (width == 0 || height == 0) {
            return std::unexpected(UIEventError::InvalidResolution);
        }
        return {};
    }
};

/**
 * @brief Zdarzenie wywolywane przy zmianie fokusu wewnetrznego okienka UI (np. okienko ekwipunku, statusu).
 */
struct UIWindowFocusChangedEvent final : public ::EterBase::IEvent {
    uint32_t windowId;
    bool hasFocus;

    constexpr UIWindowFocusChangedEvent(uint32_t id, bool focus) noexcept
        : windowId(id), hasFocus(focus) {}
        
    constexpr std::expected<void, UIEventError> Validate() const noexcept {
        // Identyfikator 0 traktujemy jako nieprawidlowy
        if (windowId == 0) {
            return std::unexpected(UIEventError::InvalidWindowId);
        }
        return {};
    }
};


/**
 * @brief Reprezentuje kategorie bledu przyczyny niemoznosci wykonania akcji.
 */
struct UICannotUseSkillEvent final : public ::EterBase::IEvent {
    uint32_t mainCharIndex;
    std::string reason;

    UICannotUseSkillEvent(uint32_t index, std::string_view r)
        : mainCharIndex(index), reason(r) {}
};

struct UICannotShotEvent final : public ::EterBase::IEvent {
    uint32_t mainCharIndex;
    std::string reason;

    UICannotShotEvent(uint32_t index, std::string_view r)
        : mainCharIndex(index), reason(r) {}
};

struct UICannotAttackEvent final : public ::EterBase::IEvent {
    uint32_t mainCharIndex;
    std::string reason;

    UICannotAttackEvent(uint32_t index, std::string_view r)
        : mainCharIndex(index), reason(r) {}
};

struct UIFishingWrongPlaceEvent final : public ::EterBase::IEvent {
    void* uiContext;
    constexpr UIFishingWrongPlaceEvent(void* ctx) noexcept : uiContext(ctx) {}
};

struct UIPCTargetBoardSetEvent final : public ::EterBase::IEvent {
    void* uiContext;
    uint32_t virtualID;
    std::string name;

    UIPCTargetBoardSetEvent(void* ctx, uint32_t vid, std::string_view n)
        : uiContext(ctx), virtualID(vid), name(n) {}
};

/**
 * Zdarzenia skryptowe UI dla PythonEventManager
 */

struct UIDoneEvent final : public ::EterBase::IEvent {
    void* uiContext;
    constexpr UIDoneEvent(void* ctx) noexcept : uiContext(ctx) {}
};

struct UILeftImageEvent final : public ::EterBase::IEvent {
    void* uiContext;
    std::string src;
    UILeftImageEvent(void* ctx, std::string_view s) : uiContext(ctx), src(s) {}
};

struct UITopImageEvent final : public ::EterBase::IEvent {
    void* uiContext;
    std::string src;
    UITopImageEvent(void* ctx, std::string_view s) : uiContext(ctx), src(s) {}
};

struct UIBackgroundImageEvent final : public ::EterBase::IEvent {
    void* uiContext;
    std::string src;
    UIBackgroundImageEvent(void* ctx, std::string_view s) : uiContext(ctx), src(s) {}
};

struct UIImageEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t x;
    int32_t y;
    std::string src;
    UIImageEvent(void* ctx, int32_t xPos, int32_t yPos, std::string_view s) 
        : uiContext(ctx), x(xPos), y(yPos), src(s) {}
};

struct UIInsertItemIconEvent final : public ::EterBase::IEvent {
    void* uiContext;
    std::string imageType;
    int32_t itemIndex;
    std::string title;
    std::string desc;
    int32_t index;
    int32_t total;
    UIInsertItemIconEvent(void* ctx, std::string_view t, int32_t iIdx, std::string_view tit, std::string_view d, int32_t idx, int32_t tot)
        : uiContext(ctx), imageType(t), itemIndex(iIdx), title(tit), desc(d), index(idx), total(tot) {}
};

struct UIInsertImageEvent final : public ::EterBase::IEvent {
    void* uiContext;
    std::string imageFile;
    std::string title1;
    std::string title2;
    std::string desc;
    int32_t index;
    int32_t total;
    UIInsertImageEvent(void* ctx, std::string_view f, std::string_view t1, std::string_view t2, std::string_view d, int32_t idx, int32_t tot)
        : uiContext(ctx), imageFile(f), title1(t1), title2(t2), desc(d), index(idx), total(tot) {}
};

struct UIClearQuestEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t index;
    UIClearQuestEvent(void* ctx, int32_t idx) : uiContext(ctx), index(idx) {}
};

struct UIRecvQuestEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t index;
    std::string name;
    UIRecvQuestEvent(void* ctx, int32_t idx, std::string_view n) : uiContext(ctx), index(idx), name(n) {}
};

struct UIBinaryRecvQuestEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t index;
    std::string name;
    std::string type;
    std::string file;
    UIBinaryRecvQuestEvent(void* ctx, int32_t idx, std::string_view n, std::string_view t, std::string_view f)
        : uiContext(ctx), index(idx), name(n), type(t), file(f) {}
};

struct UIFadeOutEvent final : public ::EterBase::IEvent {
    void* uiContext;
    float speed;
    constexpr UIFadeOutEvent(void* ctx, float s) noexcept : uiContext(ctx), speed(s) {}
};

struct UIFadeInEvent final : public ::EterBase::IEvent {
    void* uiContext;
    float speed;
    constexpr UIFadeInEvent(void* ctx, float s) noexcept : uiContext(ctx), speed(s) {}
};

struct UIWhiteOutEvent final : public ::EterBase::IEvent {
    void* uiContext;
    float speed;
    constexpr UIWhiteOutEvent(void* ctx, float s) noexcept : uiContext(ctx), speed(s) {}
};

struct UIWhiteInEvent final : public ::EterBase::IEvent {
    void* uiContext;
    float speed;
    constexpr UIWhiteInEvent(void* ctx, float s) noexcept : uiContext(ctx), speed(s) {}
};

struct UITitleImageEvent final : public ::EterBase::IEvent {
    void* uiContext;
    std::string src;
    UITitleImageEvent(void* ctx, std::string_view s) : uiContext(ctx), src(s) {}
};

struct UIShowDungeonResultEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t args[9];
    UIShowDungeonResultEvent(void* ctx, int32_t a0, int32_t a1, int32_t a2, int32_t a3, int32_t a4, int32_t a5, int32_t a6, int32_t a7, int32_t a8)
        : uiContext(ctx), args{a0, a1, a2, a3, a4, a5, a6, a7, a8} {}
};

struct UISizeEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t width;
    int32_t height;
    constexpr UISizeEvent(void* ctx, int32_t w, int32_t h) noexcept : uiContext(ctx), width(w), height(h) {}
};

struct UIInputEvent final : public ::EterBase::IEvent {
    void* uiContext;
    constexpr UIInputEvent(void* ctx) noexcept : uiContext(ctx) {}
};

struct UICloseSelfEvent final : public ::EterBase::IEvent {
    void* uiContext;
    constexpr UICloseSelfEvent(void* ctx) noexcept : uiContext(ctx) {}
};

struct UIOpenSelectItemWindowEvent final : public ::EterBase::IEvent {
    void* uiContext;
    constexpr UIOpenSelectItemWindowEvent(void* ctx) noexcept : uiContext(ctx) {}
};

struct UIMakeNextButtonEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t buttonType;
    constexpr UIMakeNextButtonEvent(void* ctx, int32_t type) noexcept : uiContext(ctx), buttonType(type) {}
};

struct UIMakeQuestionEvent final : public ::EterBase::IEvent {
    void* uiContext;
    int32_t count;
    constexpr UIMakeQuestionEvent(void* ctx, int32_t c) noexcept : uiContext(ctx), count(c) {}
};

struct UIAppendQuestionEvent final : public ::EterBase::IEvent {
    void* uiContext;
    std::string text;
    int32_t index;
    UIAppendQuestionEvent(void* ctx, std::string_view t, int32_t idx) : uiContext(ctx), text(t), index(idx) {}
};

} // namespace Client::Events

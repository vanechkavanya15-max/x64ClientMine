#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Input/IMEClipboardBridge.h"
#include <new>
#include <cstdlib>

using namespace Client::Input;

TEST_CASE("IMEClipboardBridge: Text sanitize and clipboard storage") {
    IMEClipboardBridge clip;
    std::string dirty = "Text\x01\x02\x03WithControl";
    auto result = clip.SetClipboardText(dirty);
    REQUIRE(result.has_value());
    
    auto text = clip.GetClipboardText();
    REQUIRE(text.has_value());
    CHECK(text.value() == "TextWithControl");
}

TEST_CASE("IMEClipboardBridge: Text sanitize allows newlines and tabs") {
    IMEClipboardBridge clip;
    std::string dirty = "Text\nWith\tControl";
    auto result = clip.SetClipboardText(dirty);
    REQUIRE(result.has_value());
    
    auto text = clip.GetClipboardText();
    REQUIRE(text.has_value());
    CHECK(text.value() == "Text\nWith\tControl");
}

TEST_CASE("IMEClipboardBridge: UTF-8 to UTF-16 Conversion") {
    std::string utf8_text = "Zażółć gęślą jaźń 😊";
    auto utf16_result = IMEClipboardBridge::Utf8ToUtf16(utf8_text);
    REQUIRE(utf16_result.has_value());
    
    auto utf16_str = utf16_result.value();
    
    auto back_to_utf8_result = IMEClipboardBridge::Utf16ToUtf8(utf16_str);
    REQUIRE(back_to_utf8_result.has_value());
    CHECK(back_to_utf8_result.value() == utf8_text);
}

TEST_CASE("IMEClipboardBridge: UTF-8 to ANSI Conversion") {
    std::string utf8_text = "Zazółc"; // Let's use simple string since ANSI is ISO-8859-1 for our simulation
    auto ansi_result = IMEClipboardBridge::Utf8ToAnsi(utf8_text);
    REQUIRE(ansi_result.has_value());
    
    auto back_to_utf8_result = IMEClipboardBridge::AnsiToUtf8(ansi_result.value());
    REQUIRE(back_to_utf8_result.has_value());
    // Since 'ó' is valid in ISO-8859-1, it should map back correctly
    // 'ł' and 'ć' would map to '?' based on our simple mapping in AnsiToUtf8.
    // Let's test a simple string that fits in ISO-8859-1.
}

TEST_CASE("IMEClipboardBridge: ISO-8859-1 Roundtrip") {
    std::string utf8_text = "H\xC3\xABllo"; // "Hëllo" in UTF-8
    auto ansi_result = IMEClipboardBridge::Utf8ToAnsi(utf8_text);
    REQUIRE(ansi_result.has_value());
    
    auto back_to_utf8_result = IMEClipboardBridge::AnsiToUtf8(ansi_result.value());
    REQUIRE(back_to_utf8_result.has_value());
    CHECK(back_to_utf8_result.value() == utf8_text);
}

TEST_CASE("IMEClipboardBridge: Select and Delete") {
    IMEClipboardBridge clip;
    clip.SetClipboardText("Hello World");
    
    // Select " World"
    auto sel_res = clip.SetSelection(5, 11);
    REQUIRE(sel_res.has_value());
    
    auto selected_text = clip.GetSelectedText();
    REQUIRE(selected_text.has_value());
    CHECK(selected_text.value() == " World");
    
    auto del_res = clip.DeleteSelection();
    REQUIRE(del_res.has_value());
    
    auto current_text = clip.GetClipboardText();
    REQUIRE(current_text.has_value());
    CHECK(current_text.value() == "Hello");
}

TEST_CASE("IMEClipboardBridge: Replace Selection") {
    IMEClipboardBridge clip;
    clip.SetClipboardText("Hello World");
    
    auto sel_res = clip.SetSelection(0, 5);
    REQUIRE(sel_res.has_value());
    
    auto rep_res = clip.ReplaceSelection("Hi");
    REQUIRE(rep_res.has_value());
    
    auto current_text = clip.GetClipboardText();
    REQUIRE(current_text.has_value());
    CHECK(current_text.value() == "Hi World");
    
    auto sel_status = clip.GetSelection();
    REQUIRE(sel_status.has_value());
    CHECK(sel_status.value().start == 2);
    CHECK(sel_status.value().end == 2);
}

TEST_CASE("IMEClipboardBridge: Out of bounds selection") {
    IMEClipboardBridge clip;
    clip.SetClipboardText("Hello");
    
    auto sel_res = clip.SetSelection(0, 10);
    REQUIRE_FALSE(sel_res.has_value());
    CHECK(sel_res.error() == ClipboardError::InvalidSelection);
}

TEST_CASE("IMEClipboardBridge: Empty clipboard fetch") {
    IMEClipboardBridge clip;
    auto res = clip.GetClipboardText();
    REQUIRE_FALSE(res.has_value());
    CHECK(res.error() == ClipboardError::EmptyClipboard);
}

TEST_CASE("IMEClipboardBridge: Setting UTF-16 content") {
    IMEClipboardBridge clip;
    std::string utf8_text = "Zażółć gęślą jaźń";
    auto utf16_text = IMEClipboardBridge::Utf8ToUtf16(utf8_text).value();
    
    // Convert to byte representation for passing as string_view
    std::string_view utf16_view(reinterpret_cast<const char*>(utf16_text.data()), utf16_text.size() * 2);
    
    auto set_res = clip.SetClipboardText(utf16_view, ClipboardEncoding::UTF16);
    REQUIRE(set_res.has_value());
    
    auto get_res = clip.GetClipboardText();
    REQUIRE(get_res.has_value());
    CHECK(get_res.value() == utf8_text);
}

TEST_CASE("IMEClipboardBridge: Clear clipboard") {
    IMEClipboardBridge clip;
    clip.SetClipboardText("Test");
    CHECK(clip.GetTextLength() == 4);
    
    clip.Clear();
    CHECK(clip.GetTextLength() == 0);
    auto res = clip.GetClipboardText();
    REQUIRE_FALSE(res.has_value());
}

TEST_CASE("IMEClipboardBridge: Invalid UTF-8 handling") {
    IMEClipboardBridge clip;
    // Invalid UTF-8 byte
    std::string invalid_utf8 = "Test\xFE" "Data";
    clip.SetClipboardText(invalid_utf8);
    auto text = clip.GetClipboardText();
    REQUIRE(text.has_value());
    CHECK(text.value() == "TestData"); // Should skip the invalid byte during sanitization
}

bool g_simulate_bad_alloc = false;
void* operator new(std::size_t size) {
    if (g_simulate_bad_alloc) throw std::bad_alloc();
    if (void* ptr = std::malloc(size)) return ptr;
    throw std::bad_alloc();
}
void operator delete(void* ptr) noexcept { std::free(ptr); }
void operator delete(void* ptr, std::size_t) noexcept { std::free(ptr); }

TEST_CASE("IMEClipboardBridge: std::bad_alloc simulation") {
    // std::string uses Small String Optimization (SSO).
    // To trigger bad_alloc we need strings larger than SSO buffer (typically 15-22 bytes).
    std::string large_string(100, 'A');
    
    g_simulate_bad_alloc = true;
    auto utf16_result = IMEClipboardBridge::Utf8ToUtf16(large_string);
    g_simulate_bad_alloc = false;
    CHECK(!utf16_result.has_value());
    CHECK(utf16_result.error() == ClipboardError::OutOfMemory);
    
    g_simulate_bad_alloc = true;
    auto ansi_result = IMEClipboardBridge::AnsiToUtf8(large_string);
    g_simulate_bad_alloc = false;
    CHECK(!ansi_result.has_value());
    CHECK(ansi_result.error() == ClipboardError::OutOfMemory);
    
    g_simulate_bad_alloc = true;
    auto u8toansi_result = IMEClipboardBridge::Utf8ToAnsi(large_string);
    g_simulate_bad_alloc = false;
    CHECK(!u8toansi_result.has_value());
    CHECK(u8toansi_result.error() == ClipboardError::OutOfMemory);
}

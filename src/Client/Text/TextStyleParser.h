#pragma once

#include <string>
#include <string_view>
#include <expected>
#include <tuple>
#include <cstdint>

namespace Client::Text {

enum class TagType {
    Plain,
    Tag,             // ||
    Color,           // |cffffffff
    HyperlinkStart,  // |H
    HyperlinkEnd,    // |h
    RestoreColor     // |r
};

struct TagParseResult {
    TagType type;
    size_t tagLength;
    std::wstring extraInfo;
};

// Nowy, samodzielny parser tekstow UI eliminujacy zaleznosc od EterLib.
// Zapewnia czyste parsowanie kodow sterujacych w napisach C++.
class TextStyleParser {
public:
    static TagParseResult ParseTag(std::wstring_view src);
    
    static std::wstring GetOutputString(std::wstring_view src);
    
    static size_t GetOutputLen(std::wstring_view src);
    
    static size_t GetInternalPosFromRenderPos(std::wstring_view src, size_t offset);
    
    static size_t FindColorTagStartPosition(std::wstring_view src);
    
    static size_t FindColorTagEndPosition(std::wstring_view src);
};

} // namespace Client::Text

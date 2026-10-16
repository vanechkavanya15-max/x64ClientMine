#include "TextLineBreakFormatter.h"
#include <charconv>
#include <cctype>

namespace Client::Text {

namespace {

    char32_t DecodeNextUtf8Codepoint(std::string_view text, size_t& offset) {
        if (offset >= text.length()) return 0;
        
        unsigned char c = text[offset];
        char32_t cp = 0;
        
        if (c < 0x80) {
            cp = c;
            offset += 1;
        } else if ((c & 0xE0) == 0xC0) {
            if (offset + 1 >= text.length()) { offset++; return c; }
            cp = ((c & 0x1F) << 6) | (text[offset + 1] & 0x3F);
            offset += 2;
        } else if ((c & 0xF0) == 0xE0) {
            if (offset + 2 >= text.length()) { offset++; return c; }
            cp = ((c & 0x0F) << 12) | ((text[offset + 1] & 0x3F) << 6) | (text[offset + 2] & 0x3F);
            offset += 3;
        } else if ((c & 0xF8) == 0xF0) {
            if (offset + 3 >= text.length()) { offset++; return c; }
            cp = ((c & 0x07) << 18) | ((text[offset + 1] & 0x3F) << 12) | ((text[offset + 2] & 0x3F) << 6) | (text[offset + 3] & 0x3F);
            offset += 4;
        } else {
            cp = c;
            offset += 1;
        }
        return cp;
    }
    
    struct ParsedToken {
        enum class Type { Text, Tag, Space, Newline };
        Type type;
        std::string_view content;
        float width = 0.0f;
    };
    
    std::vector<ParsedToken> TokenizeText(std::string_view text, const ILineBreakMetricsProvider& metrics) {
        std::vector<ParsedToken> tokens;
        size_t i = 0;
        
        while (i < text.length()) {
            if (text[i] == '|' && i + 1 < text.length()) {
                char next = text[i + 1];
                if (next == 'c' && i + 9 < text.length()) {
                    tokens.push_back({ParsedToken::Type::Tag, text.substr(i, 10), 0.0f});
                    i += 10;
                    continue;
                } else if (next == 'r') {
                    tokens.push_back({ParsedToken::Type::Tag, text.substr(i, 2), 0.0f});
                    i += 2;
                    continue;
                } else if (next == 'H') {
                    size_t endH = text.find("|h", i + 2);
                    if (endH != std::string_view::npos) {
                        tokens.push_back({ParsedToken::Type::Tag, text.substr(i, endH + 2 - i), 0.0f});
                        i = endH + 2;
                        continue;
                    }
                } else if (next == 'h') {
                    tokens.push_back({ParsedToken::Type::Tag, text.substr(i, 2), 0.0f});
                    i += 2;
                    continue;
                }
            }
            
            size_t startOffset = i;
            char32_t cp = DecodeNextUtf8Codepoint(text, i);
            
            if (cp == '\n') {
                tokens.push_back({ParsedToken::Type::Newline, text.substr(startOffset, i - startOffset), 0.0f});
            } else if (cp == ' ' || cp == '\t') {
                float w = metrics.GetGlyphAdvance(cp);
                tokens.push_back({ParsedToken::Type::Space, text.substr(startOffset, i - startOffset), w});
            } else {
                float w = metrics.GetGlyphAdvance(cp);
                if (!tokens.empty() && tokens.back().type == ParsedToken::Type::Text) {
                    tokens.back().content = std::string_view(tokens.back().content.data(), tokens.back().content.length() + (i - startOffset));
                    tokens.back().width += w;
                } else {
                    tokens.push_back({ParsedToken::Type::Text, text.substr(startOffset, i - startOffset), w});
                }
            }
        }
        return tokens;
    }

} // anonymous namespace

std::expected<std::vector<TextLineInfo>, LineBreakError> TextLineBreakFormatter::BreakLines(
    std::string_view text,
    const ILineBreakMetricsProvider& metrics,
    float maxLineWidth)
{
    if (maxLineWidth <= 0.0f) {
        return std::unexpected(LineBreakError::InvalidInput);
    }
    
    std::vector<TextLineInfo> lines;
    if (text.empty()) {
        lines.push_back({text, 0.0f});
        return lines;
    }

    auto tokens = TokenizeText(text, metrics);
    
    size_t lineStartTokenIdx = 0;
    float currentLineWidth = 0.0f;
    
    auto createLine = [&](size_t startToken, size_t endToken) {
        if (startToken >= tokens.size() || startToken > endToken) {
            lines.push_back({std::string_view(text.data() + text.length(), 0), 0.0f});
            return;
        }
        
        // Trim trailing spaces for the line
        size_t actualEndToken = endToken;
        while (actualEndToken > startToken && tokens[actualEndToken - 1].type == ParsedToken::Type::Space) {
            actualEndToken--;
        }
        
        float w = 0.0f;
        const char* startPtr = tokens[startToken].content.data();
        const char* endPtr = startPtr;
        
        if (actualEndToken > startToken) {
            for (size_t i = startToken; i < actualEndToken; ++i) {
                w += tokens[i].width;
            }
            endPtr = tokens[actualEndToken - 1].content.data() + tokens[actualEndToken - 1].content.length();
        }
        
        std::string_view lineContent(startPtr, endPtr - startPtr);
        lines.push_back({lineContent, w});
    };
    
    size_t currentTokenIdx = 0;
    while (currentTokenIdx < tokens.size()) {
        const auto& token = tokens[currentTokenIdx];
        
        if (token.type == ParsedToken::Type::Newline) {
            createLine(lineStartTokenIdx, currentTokenIdx);
            lineStartTokenIdx = currentTokenIdx + 1;
            currentLineWidth = 0.0f;
            currentTokenIdx++;
            continue;
        }
        
        if (currentLineWidth + token.width > maxLineWidth) {
            if (token.type == ParsedToken::Type::Space) {
                // Ignore spaces that exceed max width (they just wrap the line)
                createLine(lineStartTokenIdx, currentTokenIdx);
                lineStartTokenIdx = currentTokenIdx + 1;
                currentLineWidth = 0.0f;
                currentTokenIdx++;
                continue;
            } else if (token.type == ParsedToken::Type::Text) {
                if (currentLineWidth == 0.0f) {
                    // Mid-word break: A single word is longer than max width.
                    // Need to split the token.
                    
                    size_t offset = 0;
                    float w = 0.0f;
                    size_t splitOffset = 0;
                    float splitW = 0.0f;
                    
                    while (offset < token.content.length()) {
                        size_t prevOffset = offset;
                        char32_t cp = DecodeNextUtf8Codepoint(token.content, offset);
                        float cpW = metrics.GetGlyphAdvance(cp);
                        
                        if (w + cpW > maxLineWidth && w > 0.0f) {
                            splitOffset = prevOffset;
                            splitW = w;
                            break;
                        }
                        w += cpW;
                    }
                    
                    if (splitOffset > 0) {
                        // Split the text token
                        ParsedToken part1 = {ParsedToken::Type::Text, token.content.substr(0, splitOffset), splitW};
                        ParsedToken part2 = {ParsedToken::Type::Text, token.content.substr(splitOffset), token.width - splitW};
                        
                        // Replace current token with part2, and insert part1 before it
                        tokens[currentTokenIdx] = part2;
                        tokens.insert(tokens.begin() + currentTokenIdx, part1);
                        
                        createLine(lineStartTokenIdx, currentTokenIdx + 1);
                        lineStartTokenIdx = currentTokenIdx + 1;
                        currentLineWidth = 0.0f;
                        currentTokenIdx++; // Move to part2
                        continue;
                    }
                } else {
                    // Wrap at word boundary
                    // Backtrack to remove trailing spaces from the wrapped line
                    size_t backtrackEndIdx = currentTokenIdx;
                    while (backtrackEndIdx > lineStartTokenIdx && tokens[backtrackEndIdx - 1].type == ParsedToken::Type::Space) {
                        backtrackEndIdx--;
                    }
                    
                    createLine(lineStartTokenIdx, backtrackEndIdx);
                    
                    // The new line starts at the word that didn't fit
                    lineStartTokenIdx = currentTokenIdx;
                    currentLineWidth = token.width;
                    currentTokenIdx++;
                    continue;
                }
            }
        }
        
        currentLineWidth += token.width;
        currentTokenIdx++;
    }
    
    if (lineStartTokenIdx < tokens.size() || (!tokens.empty() && tokens.back().type == ParsedToken::Type::Newline)) {
        createLine(lineStartTokenIdx, tokens.size());
    }
    
    return lines;
}

} // namespace Client::Text

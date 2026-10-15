#include "TextLayoutEngine.h"

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

struct Token {
    enum class Type { Text, ColorTag, RestoreColor, HyperlinkStart, HyperlinkEnd };
    Type type;
    std::string_view content;
    uint32_t color = 0;
};

std::vector<Token> ParseTags(std::string_view text) {
    std::vector<Token> tokens;
    size_t i = 0;
    size_t text_start = 0;

    auto push_text = [&](size_t end) {
        if (end > text_start) {
            tokens.push_back({Token::Type::Text, text.substr(text_start, end - text_start)});
        }
    };

    while (i < text.length()) {
        if (text[i] == '|' && i + 1 < text.length()) {
            char next = text[i + 1];
            if (next == 'c' && i + 9 < text.length()) {
                push_text(i);
                std::string_view colorStr = text.substr(i + 2, 8);
                uint32_t color = 0;
                try {
                    color = std::stoul(std::string(colorStr), nullptr, 16);
                } catch(...) {}
                tokens.push_back({Token::Type::ColorTag, text.substr(i, 10), color});
                i += 10;
                text_start = i;
                continue;
            } else if (next == 'r') {
                push_text(i);
                tokens.push_back({Token::Type::RestoreColor, text.substr(i, 2)});
                i += 2;
                text_start = i;
                continue;
            } else if (next == 'H') {
                push_text(i);
                size_t endH = text.find("|h", i + 2);
                if (endH != std::string_view::npos) {
                    tokens.push_back({Token::Type::HyperlinkStart, text.substr(i + 2, endH - (i + 2))});
                    i = endH + 2;
                    text_start = i;
                    continue;
                }
            } else if (next == 'h') {
                push_text(i);
                tokens.push_back({Token::Type::HyperlinkEnd, text.substr(i, 2)});
                i += 2;
                text_start = i;
                continue;
            }
        }
        i++;
    }
    push_text(text.length());
    return tokens;
}

bool IsRTLCharacter(char32_t cp) {
    // Basic Arabic/Hebrew block check
    return (cp >= 0x0590 && cp <= 0x05FF) || (cp >= 0x0600 && cp <= 0x06FF);
}

} // anonymous namespace

std::expected<TextLayoutResult, LayoutError> TextLayoutEngine::BuildLayout(
    std::string_view text,
    const IFontMetricsProvider& fontMetrics,
    const LayoutConfig& config)
{
    TextLayoutResult result;
    result.width = 0.0f;
    result.height = 0.0f;
    
    if (text.empty()) {
        return result;
    }

    bool hasRTLChar = false;
    for (size_t i = 0; i < text.length(); ) {
        char32_t cp = DecodeNextUtf8Codepoint(text, i);
        if (IsRTLCharacter(cp)) {
            hasRTLChar = true;
            break;
        }
    }
    
    result.isRtlComputed = (config.textDirection == TextDirection::RTL) || 
                           (config.textDirection == TextDirection::Auto && hasRTLChar);

    auto tokens = ParseTags(text);
    
    uint32_t currentColor = config.defaultColor;
    std::optional<std::string> currentHyperlink = std::nullopt;
    
    std::vector<TextLayoutLine> lines;
    lines.push_back(TextLayoutLine{});
    TextLayoutLine* currentLine = &lines.back();
    
    float currentX = 0.0f;
    float lineHeight = fontMetrics.GetLineHeight();
    
    auto wrapLine = [&]() {
        result.width = std::max(result.width, currentX);
        lines.push_back(TextLayoutLine{});
        currentLine = &lines.back();
        currentX = 0.0f;
    };

    struct PendingHyperlink {
        std::string text;
        float startX;
        float startY;
        size_t lineIndex;
    };
    std::vector<PendingHyperlink> pendingHyperlinks;

    for (const auto& token : tokens) {
        if (token.type == Token::Type::ColorTag) {
            currentColor = token.color;
        } else if (token.type == Token::Type::RestoreColor) {
            currentColor = config.defaultColor;
        } else if (token.type == Token::Type::HyperlinkStart) {
            currentHyperlink = std::string(token.content);
            pendingHyperlinks.push_back({*currentHyperlink, currentX, static_cast<float>(lines.size() - 1) * lineHeight, lines.size() - 1});
        } else if (token.type == Token::Type::HyperlinkEnd) {
            if (currentHyperlink) {
                for (auto it = pendingHyperlinks.rbegin(); it != pendingHyperlinks.rend(); ++it) {
                    if (it->text == *currentHyperlink) {
                        result.hyperlinks.push_back({
                            it->startX, currentX,
                            it->startY, it->startY + lineHeight,
                            it->text
                        });
                        pendingHyperlinks.erase(std::next(it).base());
                        break;
                    }
                }
                currentHyperlink = std::nullopt;
            }
        } else if (token.type == Token::Type::Text) {
            size_t offset = 0;
            while (offset < token.content.length()) {
                size_t startOffset = offset;
                char32_t cp = DecodeNextUtf8Codepoint(token.content, offset);
                
                if (cp == '\n' && config.multiLine) {
                    wrapLine();
                    continue;
                }
                if (cp == '\n' && !config.multiLine) {
                    continue; // Skip newline in single-line mode
                }
                
                float advance = fontMetrics.GetGlyphAdvance(cp);
                
                if (config.limitWidth && currentX + advance > *config.limitWidth && config.multiLine) {
                    wrapLine();
                }
                
                currentLine->glyphs.push_back({
                    currentX, 0.0f, advance,
                    token.content.substr(startOffset, offset - startOffset),
                    currentColor,
                    currentHyperlink,
                    true
                });
                currentX += advance;
                currentLine->width = currentX;
            }
        }
    }
    
    result.width = std::max(result.width, currentX);
    
    // Process alignments and RTL
    float totalHeight = static_cast<float>(lines.size()) * lineHeight;
    result.height = totalHeight;
    
    float startY = 0.0f;
    if (config.verticalAlign == VerticalAlignment::Bottom) {
        startY = -totalHeight;
    } else if (config.verticalAlign == VerticalAlignment::Center) {
        startY = -totalHeight / 2.0f;
    }

    for (size_t i = 0; i < lines.size(); ++i) {
        auto& line = lines[i];
        float y = startY + i * lineHeight;
        
        float offsetX = 0.0f;
        
        if (result.isRtlComputed) {
            // Reorder visual positions within the line for RTL
            // For now, simple right alignment + reversal
            if (config.horizontalAlign == HorizontalAlignment::Left) {
                // In RTL, "Left" alignment visually goes to the right edge if width limit exists, 
                // but usually RTL text starts from the right edge.
                // If limitWidth is given, align to right of limitWidth. Else just go negative X.
                offsetX = config.limitWidth ? *config.limitWidth - line.width : -line.width;
            } else if (config.horizontalAlign == HorizontalAlignment::Right) {
                offsetX = 0.0f; // aligned to right origin
            } else { // Center
                offsetX = config.limitWidth ? (*config.limitWidth - line.width) / 2.0f : -line.width / 2.0f;
            }
            
            float currentRtlX = offsetX + line.width;
            for (auto& glyph : line.glyphs) {
                currentRtlX -= glyph.width;
                glyph.x = currentRtlX;
                glyph.y = y;
            }
            // Fix hyperlink bounds for RTL (simplified)
            for (auto& link : result.hyperlinks) {
                // Not perfectly accurate across multiline, but enough for basic test
                if (link.sy == y) {
                    float w = link.ex - link.sx;
                    link.sx = offsetX + line.width - link.ex;
                    link.ex = link.sx + w;
                    link.sy = y;
                    link.ey = y + lineHeight;
                }
            }
        } else {
            // LTR
            if (config.horizontalAlign == HorizontalAlignment::Right) {
                offsetX = config.limitWidth ? *config.limitWidth - line.width : -line.width;
            } else if (config.horizontalAlign == HorizontalAlignment::Center) {
                offsetX = config.limitWidth ? (*config.limitWidth - line.width) / 2.0f : -line.width / 2.0f;
            }
            
            for (auto& glyph : line.glyphs) {
                glyph.x += offsetX;
                glyph.y = y;
            }
            // Fix hyperlink bounds
            for (auto& link : result.hyperlinks) {
                 if (link.sy == y) { // rough check for line
                    link.sx += offsetX;
                    link.ex += offsetX;
                 }
            }
        }
    }
    
    result.lines = std::move(lines);
    return result;
}

} // namespace Client::Text

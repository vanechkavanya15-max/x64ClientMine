#include "TextFontGlyphMetrics.h"
#include <algorithm>

namespace Client::Text {

TextFontGlyphMetrics::TextFontGlyphMetrics(float lineHeight, const std::vector<GlyphAdvanceMapping>& mappings)
    : m_lineHeight(lineHeight)
{
    // Inicjalizacja tablicy ASCII wartoscia 0.0f
    m_asciiAdvances.fill(0.0f);

    for (const auto& mapping : mappings) {
        if (mapping.codepoint < 128) {
            m_asciiAdvances[mapping.codepoint] = mapping.advance;
        } else {
            m_unicodeAdvances[mapping.codepoint] = mapping.advance;
        }
    }
}

float TextFontGlyphMetrics::GetGlyphAdvance(char32_t codepoint) const {
    if (codepoint < 128) {
        return m_asciiAdvances[codepoint];
    }
    
    auto it = m_unicodeAdvances.find(codepoint);
    if (it != m_unicodeAdvances.end()) {
        return it->second;
    }
    
    // Zwracamy 0 jezeli znak nie zostal znaleziony
    return 0.0f;
}

float TextFontGlyphMetrics::GetLineHeight() const {
    return m_lineHeight;
}

} // namespace Client::Text

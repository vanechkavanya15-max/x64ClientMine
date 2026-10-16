#pragma once

#include "TextLayoutEngine.h"
#include <array>
#include <unordered_map>
#include <vector>

namespace Client::Text {

// Struktura sluzaca do inicjalizacji
struct GlyphAdvanceMapping {
    char32_t codepoint;
    float advance;
};

// Klasa dostarczajaca metryki czcionek dla silnika ukladu tekstu
class TextFontGlyphMetrics : public IFontMetricsProvider {
public:
    // Inicjalizuje metryki podanymi wartosciami
    TextFontGlyphMetrics(float lineHeight, const std::vector<GlyphAdvanceMapping>& mappings);
    ~TextFontGlyphMetrics() override = default;

    // Pobiera szerokosc znaku
    float GetGlyphAdvance(char32_t codepoint) const override;
    
    // Pobiera wysokosc linii
    float GetLineHeight() const override;

private:
    float m_lineHeight;
    std::array<float, 128> m_asciiAdvances;
    std::unordered_map<char32_t, float> m_unicodeAdvances;
};

} // namespace Client::Text

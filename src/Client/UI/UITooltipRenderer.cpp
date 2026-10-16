#include "UITooltipRenderer.h"

namespace Client::UI {

void UITooltipRenderer::AddTextLine(std::string_view text, Color color) {
    m_elements.push_back(TooltipTextLine{std::string(text), color});
}

void UITooltipRenderer::AddSeparator(Color color) {
    m_elements.push_back(TooltipSeparator{color});
}

void UITooltipRenderer::Clear() {
    m_elements.clear();
}

bool UITooltipRenderer::IsEmpty() const {
    return m_elements.empty();
}

const std::vector<TooltipElement>& UITooltipRenderer::GetElements() const {
    return m_elements;
}

std::expected<void, TooltipRenderError> UITooltipRenderer::Render(UIRenderBatcher& batcher, const TooltipLayoutConfig& config) const {
    if (m_elements.empty()) {
        return std::unexpected(TooltipRenderError::EmptyContent);
    }
    if (!config.fontProvider) {
        return std::unexpected(TooltipRenderError::InvalidGlyph);
    }

    // Pass 1: Calculate layout
    float currentY = config.y + config.padding;
    float maxWidth = 0.0f;

    for (const auto& element : m_elements) {
        if (std::holds_alternative<TooltipTextLine>(element)) {
            const auto& line = std::get<TooltipTextLine>(element);
            float lineWidth = 0.0f;
            float maxLineHeight = 0.0f;
            for (char c : line.text) {
                auto glyphRes = config.fontProvider->GetGlyphMetrics(c);
                if (!glyphRes) return std::unexpected(glyphRes.error());
                lineWidth += glyphRes->width;
                if (glyphRes->height > maxLineHeight) maxLineHeight = glyphRes->height;
            }
            if (lineWidth > maxWidth) {
                maxWidth = lineWidth;
            }
            currentY += maxLineHeight + config.lineSpacing;
        } else if (std::holds_alternative<TooltipSeparator>(element)) {
            currentY += config.separatorHeight + config.lineSpacing;
        }
    }

    float totalHeight = currentY - config.y + config.padding - config.lineSpacing;
    float totalWidth = maxWidth + (config.padding * 2.0f);

    // Pass 2: Render background
    if (auto res = batcher.DrawRect(config.x, config.y, totalWidth, totalHeight, config.backgroundColor); !res) {
        return std::unexpected(res.error() == UIBatchError::BufferOverflow ? TooltipRenderError::BatcherOverflow : TooltipRenderError::InvalidGeometry);
    }

    // Pass 3: Render elements
    float drawY = config.y + config.padding;
    for (const auto& element : m_elements) {
        if (std::holds_alternative<TooltipTextLine>(element)) {
            const auto& line = std::get<TooltipTextLine>(element);
            float drawX = config.x + config.padding;
            float maxLineHeight = 0.0f;
            for (char c : line.text) {
                auto glyphRes = config.fontProvider->GetGlyphMetrics(c);
                if (!glyphRes) return std::unexpected(glyphRes.error());
                if (auto res = batcher.DrawTexturedRect(drawX, drawY, glyphRes->width, glyphRes->height, glyphRes->u, glyphRes->v, glyphRes->uw, glyphRes->vh, config.fontTextureId, line.color); !res) {
                    return std::unexpected(res.error() == UIBatchError::BufferOverflow ? TooltipRenderError::BatcherOverflow : TooltipRenderError::InvalidGeometry);
                }
                drawX += glyphRes->width;
                if (glyphRes->height > maxLineHeight) maxLineHeight = glyphRes->height;
            }
            drawY += maxLineHeight + config.lineSpacing;
        } else if (std::holds_alternative<TooltipSeparator>(element)) {
            const auto& sep = std::get<TooltipSeparator>(element);
            if (auto res = batcher.DrawLine(config.x + config.padding, drawY + config.separatorHeight / 2.0f, config.x + totalWidth - config.padding, drawY + config.separatorHeight / 2.0f, sep.color); !res) {
                 return std::unexpected(res.error() == UIBatchError::BufferOverflow ? TooltipRenderError::BatcherOverflow : TooltipRenderError::InvalidGeometry);
            }
            drawY += config.separatorHeight + config.lineSpacing;
        }
    }

    return {};
}

} // namespace Client::UI

#include "UIWindowCoordinateTransform.h"

namespace Client::UI {

TransformState UIWindowCoordinateTransform::UpdateTransform(const TransformContext& parentCtx) const noexcept {
    TransformState state;
    state.localRect = m_baseLocalRect;
    state.currentScale = {
        parentCtx.globalDpiScale.x * m_localScale.x,
        parentCtx.globalDpiScale.y * m_localScale.y
    };

    // Calculate anchored offset logic based on parent's global rect
    Rect2D anchoredRect = CalculateAnchoredRect(parentCtx, state.currentScale);
    
    // Apply final rect to state
    state.globalRect = anchoredRect;
    
    // Scissor logic (Intersection with parent's scissor rect)
    if (parentCtx.enableScissorClip && parentCtx.parentScissorRect.IsValid()) {
        state.scissorRect = anchoredRect.Intersect(parentCtx.parentScissorRect);
        state.isVisible = state.scissorRect.IsValid();
    } else {
        state.scissorRect = anchoredRect;
        state.isVisible = anchoredRect.IsValid(); // Valid implies width > 0 and height > 0
    }

    state.anchorOffset = {
        anchoredRect.left - parentCtx.parentGlobalRect.left,
        anchoredRect.top - parentCtx.parentGlobalRect.top
    };

    return state;
}

TransformContext UIWindowCoordinateTransform::GetContentContext(const TransformState& currentState) const noexcept {
    TransformContext ctx;
    ctx.globalDpiScale = currentState.currentScale;
    
    // Apply padding to global rect for children
    ctx.parentGlobalRect = {
        currentState.globalRect.left + static_cast<int32_t>(m_padding.left * currentState.currentScale.x),
        currentState.globalRect.top + static_cast<int32_t>(m_padding.top * currentState.currentScale.y),
        currentState.globalRect.right - static_cast<int32_t>(m_padding.right * currentState.currentScale.x),
        currentState.globalRect.bottom - static_cast<int32_t>(m_padding.bottom * currentState.currentScale.y)
    };

    ctx.parentScissorRect = currentState.scissorRect;
    ctx.enableScissorClip = true; // Children generally inherit clipping

    return ctx;
}

Point2D UIWindowCoordinateTransform::LocalToGlobal(const Point2D& local, const TransformState& state) const noexcept {
    return {
        state.globalRect.left + static_cast<int32_t>(local.x * state.currentScale.x),
        state.globalRect.top + static_cast<int32_t>(local.y * state.currentScale.y)
    };
}

Point2D UIWindowCoordinateTransform::GlobalToLocal(const Point2D& global, const TransformState& state) const noexcept {
    // Prevent divide by zero if scale is somehow 0
    float scaleX = state.currentScale.x != 0.0f ? state.currentScale.x : 1.0f;
    float scaleY = state.currentScale.y != 0.0f ? state.currentScale.y : 1.0f;

    return {
        static_cast<int32_t>((global.x - state.globalRect.left) / scaleX),
        static_cast<int32_t>((global.y - state.globalRect.top) / scaleY)
    };
}

bool UIWindowCoordinateTransform::ContainsGlobalPoint(const Point2D& global, const TransformState& state) const noexcept {
    if (!state.isVisible) return false;
    
    // Check against scissor rect since if it's clipped, it shouldn't receive hits outside the clip region
    return state.scissorRect.Contains(global);
}

Rect2D UIWindowCoordinateTransform::CalculateAnchoredRect(const TransformContext& parentCtx, const Scale2D& finalScale) const noexcept {
    int32_t parentWidth = parentCtx.parentGlobalRect.GetWidth();
    int32_t parentHeight = parentCtx.parentGlobalRect.GetHeight();
    
    int32_t scaledWidth = static_cast<int32_t>(m_baseLocalRect.GetWidth() * finalScale.x);
    int32_t scaledHeight = static_cast<int32_t>(m_baseLocalRect.GetHeight() * finalScale.y);

    int32_t ml = static_cast<int32_t>(m_margins.left * finalScale.x);
    int32_t mr = static_cast<int32_t>(m_margins.right * finalScale.x);
    int32_t mt = static_cast<int32_t>(m_margins.top * finalScale.y);
    int32_t mb = static_cast<int32_t>(m_margins.bottom * finalScale.y);

    int32_t offsetX = 0;
    int32_t offsetY = 0;

    switch (m_anchorType) {
        case AnchorType::TopLeft:
            offsetX = ml;
            offsetY = mt;
            break;
        case AnchorType::TopCenter:
            offsetX = (parentWidth - scaledWidth) / 2 + ml - mr;
            offsetY = mt;
            break;
        case AnchorType::TopRight:
            offsetX = parentWidth - scaledWidth - mr;
            offsetY = mt;
            break;
        case AnchorType::MiddleLeft:
            offsetX = ml;
            offsetY = (parentHeight - scaledHeight) / 2 + mt - mb;
            break;
        case AnchorType::MiddleCenter:
            offsetX = (parentWidth - scaledWidth) / 2 + ml - mr;
            offsetY = (parentHeight - scaledHeight) / 2 + mt - mb;
            break;
        case AnchorType::MiddleRight:
            offsetX = parentWidth - scaledWidth - mr;
            offsetY = (parentHeight - scaledHeight) / 2 + mt - mb;
            break;
        case AnchorType::BottomLeft:
            offsetX = ml;
            offsetY = parentHeight - scaledHeight - mb;
            break;
        case AnchorType::BottomCenter:
            offsetX = (parentWidth - scaledWidth) / 2 + ml - mr;
            offsetY = parentHeight - scaledHeight - mb;
            break;
        case AnchorType::BottomRight:
            offsetX = parentWidth - scaledWidth - mr;
            offsetY = parentHeight - scaledHeight - mb;
            break;
        case AnchorType::StretchAll:
            offsetX = ml;
            offsetY = mt;
            scaledWidth = std::max(0, parentWidth - ml - mr);
            scaledHeight = std::max(0, parentHeight - mt - mb);
            break;
        case AnchorType::StretchHorizontal:
            offsetX = ml;
            offsetY = mt; // Assuming Top alignment for stretch H
            scaledWidth = std::max(0, parentWidth - ml - mr);
            break;
        case AnchorType::StretchVertical:
            offsetX = ml; // Assuming Left alignment for stretch V
            offsetY = mt;
            scaledHeight = std::max(0, parentHeight - mt - mb);
            break;
    }

    // Apply explicit local base rect position for non-stretching anchors if we want local offsets to still apply
    // (In many UI frameworks, anchors override local X/Y or combine with them. Let's combine them here scaled.)
    int32_t localOffsetX = static_cast<int32_t>(m_baseLocalRect.left * finalScale.x);
    int32_t localOffsetY = static_cast<int32_t>(m_baseLocalRect.top * finalScale.y);
    
    if (m_anchorType == AnchorType::StretchAll) {
        localOffsetX = 0;
        localOffsetY = 0;
    } else if (m_anchorType == AnchorType::StretchHorizontal) {
        localOffsetX = 0;
    } else if (m_anchorType == AnchorType::StretchVertical) {
        localOffsetY = 0;
    }

    int32_t finalLeft = parentCtx.parentGlobalRect.left + offsetX + localOffsetX;
    int32_t finalTop = parentCtx.parentGlobalRect.top + offsetY + localOffsetY;

    return {
        finalLeft,
        finalTop,
        finalLeft + scaledWidth,
        finalTop + scaledHeight
    };
}

} // namespace Client::UI

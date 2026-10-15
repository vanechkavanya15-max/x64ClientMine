#include "UIWindowLayoutManager.h"
#include <vector>
#include <algorithm>

namespace Client::UI {

EterBase::Result<void, LayoutError> UIWindowLayoutManager::ArrangeLinear(Window* container, LayoutDirection direction, const LayoutConfig& config)
{
    if (!container) {
        return EterBase::MakeError(LayoutError::NullContainer);
    }

    const auto& allChildren = container->GetChildren();
    std::vector<Window*> children;
    for (const auto& child : allChildren) {
        if (child && child->IsVisible()) {
            children.push_back(child.get());
        }
    }

    if (children.empty()) {
        return EterBase::MakeError(LayoutError::NoChildren);
    }

    int currentPos = config.padding;
    const Rect containerRect = container->GetLocalRect();

    for (Window* child : children) {
        Rect childRect = child->GetLocalRect();
        int childWidth = childRect.width;
        int childHeight = childRect.height;

        int x = 0;
        int y = 0;

        if (direction == LayoutDirection::Horizontal) {
            x = currentPos;
            if (config.alignment == LayoutAlignment::Start) {
                y = config.padding;
            } else if (config.alignment == LayoutAlignment::Center) {
                y = (containerRect.height - childHeight) / 2;
            } else if (config.alignment == LayoutAlignment::End) {
                y = containerRect.height - childHeight - config.padding;
            }
            currentPos += childWidth + config.spacing;
        } else { // Vertical
            y = currentPos;
            if (config.alignment == LayoutAlignment::Start) {
                x = config.padding;
            } else if (config.alignment == LayoutAlignment::Center) {
                x = (containerRect.width - childWidth) / 2;
            } else if (config.alignment == LayoutAlignment::End) {
                x = containerRect.width - childWidth - config.padding;
            }
            currentPos += childHeight + config.spacing;
        }

        child->SetLocalPosition(x, y);
    }

    return {};
}

EterBase::Result<void, LayoutError> UIWindowLayoutManager::ArrangeGrid(Window* container, uint32_t columns, const LayoutConfig& config)
{
    if (!container) {
        return EterBase::MakeError(LayoutError::NullContainer);
    }

    if (columns == 0) {
        return EterBase::MakeError(LayoutError::InvalidGridColumns);
    }

    const auto& allChildren = container->GetChildren();
    std::vector<Window*> children;
    for (const auto& child : allChildren) {
        if (child && child->IsVisible()) {
            children.push_back(child.get());
        }
    }

    if (children.empty()) {
        return EterBase::MakeError(LayoutError::NoChildren);
    }

    // Grid layout computes positions based on rows and columns
    // We'll figure out max width/height per column/row or assume uniform grid
    // For simplicity, we'll calculate row heights and column widths.
    
    std::vector<int> colWidths(columns, 0);
    std::vector<int> rowHeights;
    
    int numRows = (children.size() + columns - 1) / columns;
    rowHeights.resize(numRows, 0);

    for (size_t i = 0; i < children.size(); ++i) {
        Window* child = children[i];
        Rect rect = child->GetLocalRect();
        
        size_t col = i % columns;
        size_t row = i / columns;

        colWidths[col] = std::max(colWidths[col], rect.width);
        rowHeights[row] = std::max(rowHeights[row], rect.height);
    }

    int currentY = config.padding;
    for (size_t row = 0; row < numRows; ++row) {
        int currentX = config.padding;
        
        for (size_t col = 0; col < columns; ++col) {
            size_t childIdx = row * columns + col;
            if (childIdx >= children.size()) break;

            Window* child = children[childIdx];
            Rect childRect = child->GetLocalRect();
            
            int x = currentX;
            int y = currentY;

            // Apply alignment within the cell
            if (config.alignment == LayoutAlignment::Center) {
                x += (colWidths[col] - childRect.width) / 2;
                y += (rowHeights[row] - childRect.height) / 2;
            } else if (config.alignment == LayoutAlignment::End) {
                x += colWidths[col] - childRect.width;
                y += rowHeights[row] - childRect.height;
            }

            child->SetLocalPosition(x, y);
            
            currentX += colWidths[col] + config.spacing;
        }
        
        currentY += rowHeights[row] + config.spacing;
    }

    return {};
}

EterBase::Result<void, LayoutError> UIWindowLayoutManager::CenterInParent(Window* child)
{
    if (!child) {
        return EterBase::MakeError(LayoutError::NullContainer); // Using this as NullArg basically
    }

    Window* parent = child->GetParent();
    if (!parent) {
        return EterBase::MakeError(LayoutError::NullContainer);
    }

    Rect parentRect = parent->GetLocalRect();
    Rect childRect = child->GetLocalRect();

    int x = (parentRect.width - childRect.width) / 2;
    int y = (parentRect.height - childRect.height) / 2;

    child->SetLocalPosition(x, y);

    return {};
}

EterBase::Result<void, LayoutError> UIWindowLayoutManager::FillParent(Window* child, int padding)
{
    if (!child) {
        return EterBase::MakeError(LayoutError::NullContainer);
    }

    Window* parent = child->GetParent();
    if (!parent) {
        return EterBase::MakeError(LayoutError::NullContainer);
    }

    Rect parentRect = parent->GetLocalRect();
    
    int newWidth = std::max(0, parentRect.width - padding * 2);
    int newHeight = std::max(0, parentRect.height - padding * 2);

    child->SetLocalPosition(padding, padding);
    child->SetSize(newWidth, newHeight);

    return {};
}

} // namespace Client::UI

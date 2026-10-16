#include "SlotCoolDownAnimationRenderer.h"
#include <cmath>
#include <numbers>
#include <algorithm>

namespace Client::UI {

std::expected<std::vector<UIVertex>, UIBatchError> SlotCoolDownAnimationRenderer::GenerateRadialWipe(
    float x, float y, float w, float h, float progress, Color color) 
{
    if (progress >= 1.0f) {
        return std::vector<UIVertex>{};
    }
    
    float clampedProgress = std::max(0.0f, progress);
    
    struct Vec2 { float x, y; };
    
    auto get_angle = [](float px, float py) {
        float a = std::atan2(px, -py);
        if (a < 0.0f) {
            a += 2.0f * std::numbers::pi_v<float>;
        }
        return a;
    };

    float start_angle = clampedProgress * 2.0f * std::numbers::pi_v<float>;
    float end_angle = 2.0f * std::numbers::pi_v<float>;
    
    std::vector<Vec2> polygon_points;
    
    if (clampedProgress <= 0.0f) {
        polygon_points = {
            {0.0f, -h/2.0f}, {w/2.0f, -h/2.0f}, {w/2.0f, h/2.0f}, {-w/2.0f, h/2.0f}, {-w/2.0f, -h/2.0f}, {0.0f, -h/2.0f}
        };
    } else {
        float sin_a = std::sin(start_angle);
        float cos_a = -std::cos(start_angle);
        
        float t_min = 1e9f;
        Vec2 p_start = {0.0f, 0.0f};
        
        if (std::abs(sin_a) > 1e-5f) {
            float t1 = (w/2.0f) / sin_a;
            if (t1 > 0.0f && t1 < t_min) { t_min = t1; p_start = {w/2.0f, cos_a * t1}; }
            float t2 = (-w/2.0f) / sin_a;
            if (t2 > 0.0f && t2 < t_min) { t_min = t2; p_start = {-w/2.0f, cos_a * t2}; }
        }
        if (std::abs(cos_a) > 1e-5f) {
            float t1 = (h/2.0f) / cos_a;
            if (t1 > 0.0f && t1 < t_min) { t_min = t1; p_start = {sin_a * t1, h/2.0f}; }
            float t2 = (-h/2.0f) / cos_a;
            if (t2 > 0.0f && t2 < t_min) { t_min = t2; p_start = {sin_a * t2, -h/2.0f}; }
        }
        
        if (std::abs(sin_a) <= 1e-5f) p_start = {0.0f, (cos_a > 0.0f) ? h/2.0f : -h/2.0f};
        if (std::abs(cos_a) <= 1e-5f) p_start = {(sin_a > 0.0f) ? w/2.0f : -w/2.0f, 0.0f};
        
        polygon_points.push_back(p_start);
        
        std::vector<Vec2> corners = {
            {w/2.0f, -h/2.0f}, {w/2.0f, h/2.0f}, {-w/2.0f, h/2.0f}, {-w/2.0f, -h/2.0f}
        };
        
        for (const auto& c : corners) {
            float ca = get_angle(c.x, c.y);
            if (ca > start_angle && ca <= end_angle) {
                polygon_points.push_back(c);
            }
        }
        
        polygon_points.push_back({0.0f, -h/2.0f});
    }
    
    std::vector<UIVertex> triangles;
    float cx = x + w * 0.5f;
    float cy = y + h * 0.5f;
    
    UIVertex centerVertex = {cx, cy, 0.0f, 0.0f, 0.0f, color};
    
    if (polygon_points.size() >= 2) {
        triangles.reserve((polygon_points.size() - 1) * 3);
        for (std::size_t i = 0; i < polygon_points.size() - 1; ++i) {
            UIVertex v1 = {cx + polygon_points[i].x, cy + polygon_points[i].y, 0.0f, 0.0f, 0.0f, color};
            UIVertex v2 = {cx + polygon_points[i+1].x, cy + polygon_points[i+1].y, 0.0f, 0.0f, 0.0f, color};
            
            triangles.push_back(centerVertex);
            triangles.push_back(v1);
            triangles.push_back(v2);
        }
    }
    
    return triangles;
}

} // namespace Client::UI

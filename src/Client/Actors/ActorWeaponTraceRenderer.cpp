#include "ActorWeaponTraceRenderer.h"
#include <algorithm>
#include <cmath>

namespace Client::Actors {

ActorWeaponTraceRenderer::ActorWeaponTraceRenderer(IWeaponTraceBackend* backend)
    : backend_(backend) {
}

std::expected<void, TraceError> ActorWeaponTraceRenderer::StartTrace(TraceId id) {
    if (!backend_) return std::unexpected(TraceError::InvalidBackend);
    
    auto it = std::ranges::find_if(instances_, [id](const TraceInstance& inst) {
        return inst.id == id;
    });

    if (it != instances_.end()) {
        if (it->active) {
            return std::unexpected(TraceError::TraceAlreadyExists);
        }
        it->active = true;
        it->points.clear();
    } else {
        instances_.push_back(TraceInstance{id, {}, true});
    }

    return {};
}

std::expected<void, TraceError> ActorWeaponTraceRenderer::StopTrace(TraceId id) {
    auto it = std::ranges::find_if(instances_, [id](const TraceInstance& inst) {
        return inst.id == id;
    });

    if (it == instances_.end() || !it->active) {
        return std::unexpected(TraceError::TraceNotFound);
    }

    it->active = false;
    return {};
}

std::expected<void, TraceError> ActorWeaponTraceRenderer::AddPoint(TraceId id, const Client::Core::Vec3& p0, const Client::Core::Vec3& p1) {
    auto it = std::ranges::find_if(instances_, [id](const TraceInstance& inst) {
        return inst.id == id && inst.active;
    });

    if (it == instances_.end()) {
        return std::unexpected(TraceError::TraceNotFound);
    }

    it->points.push_back({p0, p1, 0.0f});
    return {};
}

std::expected<void, TraceError> ActorWeaponTraceRenderer::Update(float deltaTime) {
    if (!backend_) return std::unexpected(TraceError::InvalidBackend);

    for (auto& inst : instances_) {
        for (auto& p : inst.points) {
            p.age += deltaTime;
        }

        std::erase_if(inst.points, [](const TracePoint& p) {
            return p.age >= 1.0f; // Points fade out after 1.0s
        });

        if (inst.points.size() >= 2) {
            std::vector<TraceVertex> vertices;
            BuildVertices(inst, vertices);
            backend_->RenderTrace(inst.id, vertices);
        }
    }
    
    std::erase_if(instances_, [](const TraceInstance& inst) {
        return !inst.active && inst.points.empty();
    });

    return {};
}

void ActorWeaponTraceRenderer::BuildVertices(const TraceInstance& inst, std::vector<TraceVertex>& outVertices) {
    if (inst.points.size() < 4) {
        // Not enough points for a good spline, fallback to linear strips
        for (const auto& pt : inst.points) {
            float alpha = std::clamp(1.0f - pt.age, 0.0f, 1.0f);
            outVertices.push_back({pt.base, alpha});
            outVertices.push_back({pt.tip, alpha});
        }
        return;
    }
    
    GenerateCubicSpline(inst.points, outVertices);
}

void ActorWeaponTraceRenderer::GenerateCubicSpline(const std::vector<TracePoint>& points, std::vector<TraceVertex>& outVertices) {
    const int segments = 4; // Subdivisions between points
    
    for (size_t i = 1; i < points.size() - 2; ++i) {
        const auto& p0 = points[i - 1];
        const auto& p1 = points[i];
        const auto& p2 = points[i + 1];
        const auto& p3 = points[i + 2];
        
        for (int s = 0; s < segments; ++s) {
            float t = static_cast<float>(s) / static_cast<float>(segments);
            float t2 = t * t;
            float t3 = t2 * t;
            
            // Catmull-Rom weights
            float w0 = -0.5f * t3 + t2 - 0.5f * t;
            float w1 = 1.5f * t3 - 2.5f * t2 + 1.0f;
            float w2 = -1.5f * t3 + 2.0f * t2 + 0.5f * t;
            float w3 = 0.5f * t3 - 0.5f * t2;
            
            // Base spline
            Client::Core::Vec3 interpBase;
            interpBase.x = w0 * p0.base.x + w1 * p1.base.x + w2 * p2.base.x + w3 * p3.base.x;
            interpBase.y = w0 * p0.base.y + w1 * p1.base.y + w2 * p2.base.y + w3 * p3.base.y;
            interpBase.z = w0 * p0.base.z + w1 * p1.base.z + w2 * p2.base.z + w3 * p3.base.z;
            
            // Tip spline
            Client::Core::Vec3 interpTip;
            interpTip.x = w0 * p0.tip.x + w1 * p1.tip.x + w2 * p2.tip.x + w3 * p3.tip.x;
            interpTip.y = w0 * p0.tip.y + w1 * p1.tip.y + w2 * p2.tip.y + w3 * p3.tip.y;
            interpTip.z = w0 * p0.tip.z + w1 * p1.tip.z + w2 * p2.tip.z + w3 * p3.tip.z;
            
            // Age interpolation for alpha
            float age = p1.age + (p2.age - p1.age) * t;
            float alpha = std::clamp(1.0f - age, 0.0f, 1.0f);
            
            outVertices.push_back({interpBase, alpha});
            outVertices.push_back({interpTip, alpha});
        }
    }
    
    // Add the very last point
    float finalAlpha = std::clamp(1.0f - points[points.size() - 2].age, 0.0f, 1.0f);
    outVertices.push_back({points[points.size() - 2].base, finalAlpha});
    outVertices.push_back({points[points.size() - 2].tip, finalAlpha});
}

} // namespace Client::Actors

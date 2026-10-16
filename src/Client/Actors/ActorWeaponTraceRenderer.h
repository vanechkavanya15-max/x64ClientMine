#pragma once

#include "Client/Core/StrongTypes.h"
#include <expected>
#include <vector>
#include <cstdint>
#include <string_view>
#include <format>
#include "Client/Core/MathSIMD.h"

namespace Client::Actors {

    using TraceId = Client::Core::StrongType<struct TraceIdTag, uint32_t, 0>;

    enum class TraceError : uint8_t {
        TraceAlreadyExists,
        TraceNotFound,
        InvalidBackend
    };

    constexpr std::string_view to_string(TraceError error) noexcept {
        switch (error) {
            case TraceError::TraceAlreadyExists: return "TraceAlreadyExists";
            case TraceError::TraceNotFound: return "TraceNotFound";
            case TraceError::InvalidBackend: return "InvalidBackend";
            default: return "Unknown";
        }
    }

    struct TraceVertex {
        Client::Core::Vec3 position;
        float alpha;
        
        bool operator==(const TraceVertex& other) const {
            return position.x == other.position.x && position.y == other.position.y && position.z == other.position.z && alpha == other.alpha;
        }
    };

    class IWeaponTraceBackend {
    public:
        virtual ~IWeaponTraceBackend() = default;
        virtual void RenderTrace(TraceId id, const std::vector<TraceVertex>& vertices) = 0;
    };

    class ActorWeaponTraceRenderer {
    public:
        explicit ActorWeaponTraceRenderer(IWeaponTraceBackend* backend);

        std::expected<void, TraceError> StartTrace(TraceId id);
        std::expected<void, TraceError> StopTrace(TraceId id);
        std::expected<void, TraceError> AddPoint(TraceId id, const Client::Core::Vec3& p0, const Client::Core::Vec3& p1);
        std::expected<void, TraceError> Update(float deltaTime);

    private:
        struct TracePoint {
            Client::Core::Vec3 base;
            Client::Core::Vec3 tip;
            float age = 0.0f;
        };

        struct TraceInstance {
            TraceId id;
            std::vector<TracePoint> points;
            bool active = true;
        };

        IWeaponTraceBackend* backend_;
        std::vector<TraceInstance> instances_;
        
        void BuildVertices(const TraceInstance& inst, std::vector<TraceVertex>& outVertices);
        void GenerateCubicSpline(const std::vector<TracePoint>& points, std::vector<TraceVertex>& outVertices);
    };

} // namespace Client::Actors

template <>
struct std::formatter<Client::Actors::TraceError> : std::formatter<std::string_view> {
    auto format(Client::Actors::TraceError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Actors::to_string(err), ctx);
    }
};

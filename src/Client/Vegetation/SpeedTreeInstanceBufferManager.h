#pragma once

#include <cstdint>
#include <vector>
#include <span>
#include <memory>
#include <string>

#include "Client/Core/ExpectedResult.h"

namespace Client::Vegetation
{

enum class SpeedTreeInstanceError
{
    None = 0,
    BufferTooSmall,
    UpdateFailed,
    InvalidInstanceData
};

std::string to_string(SpeedTreeInstanceError error);

// Mockable hardware buffer interface for dependency inversion and headless testing
class IHardwareBuffer
{
public:
    virtual ~IHardwareBuffer() = default;

    virtual bool UpdateData(std::span<const uint8_t> data, uint32_t offset = 0) = 0;
    virtual size_t GetSize() const noexcept = 0;
    virtual uint32_t GetStride() const noexcept = 0;
};

// Represents the per-instance data payload for a SpeedTree (e.g. model matrix)
struct SpeedTreeInstanceData
{
    float transform[16];
    float windOffset;
    float scale;
    float padding[2];
};

class SpeedTreeInstanceBufferManager
{
public:
    explicit SpeedTreeInstanceBufferManager(std::unique_ptr<IHardwareBuffer> buffer);
    ~SpeedTreeInstanceBufferManager() = default;

    // No copy
    SpeedTreeInstanceBufferManager(const SpeedTreeInstanceBufferManager&) = delete;
    SpeedTreeInstanceBufferManager& operator=(const SpeedTreeInstanceBufferManager&) = delete;

    // Movable
    SpeedTreeInstanceBufferManager(SpeedTreeInstanceBufferManager&&) noexcept = default;
    SpeedTreeInstanceBufferManager& operator=(SpeedTreeInstanceBufferManager&&) noexcept = default;

    Client::Core::ExpectedResult<void, SpeedTreeInstanceError> AddInstance(const SpeedTreeInstanceData& instance);
    void ClearInstances();
    
    Client::Core::ExpectedResult<void, SpeedTreeInstanceError> FlushToHardware();
    
    size_t GetInstanceCount() const noexcept;
    size_t GetMaxCapacity() const noexcept;

private:
    std::unique_ptr<IHardwareBuffer> m_buffer;
    std::vector<SpeedTreeInstanceData> m_instances;
    size_t m_maxCapacity;
};

} // namespace Client::Vegetation

#include "SpeedTreeInstanceBufferManager.h"
#include <cstring>
#include <stdexcept>

namespace Client::Vegetation
{

std::string to_string(SpeedTreeInstanceError error)
{
    switch (error)
    {
        case SpeedTreeInstanceError::None:
            return "Brak bledu";
        case SpeedTreeInstanceError::BufferTooSmall:
            return "Bufor jest zbyt maly aby pomiescic nowe instancje";
        case SpeedTreeInstanceError::UpdateFailed:
            return "Aktualizacja bufora sprzetowego nie powiodla sie";
        case SpeedTreeInstanceError::InvalidInstanceData:
            return "Nieprawidlowe dane instancji";
        default:
            return "Nieznany blad";
    }
}

SpeedTreeInstanceBufferManager::SpeedTreeInstanceBufferManager(std::unique_ptr<IHardwareBuffer> buffer)
    : m_buffer(std::move(buffer))
{
    if (!m_buffer)
    {
        throw std::invalid_argument("Hardware buffer cannot be null");
    }

    size_t bufferSize = m_buffer->GetSize();
    size_t stride = m_buffer->GetStride();

    if (stride == 0 || stride != sizeof(SpeedTreeInstanceData))
    {
        throw std::invalid_argument("Hardware buffer stride must match SpeedTreeInstanceData size");
    }

    m_maxCapacity = bufferSize / stride;
    m_instances.reserve(m_maxCapacity);
}

Client::Core::ExpectedResult<void, SpeedTreeInstanceError> SpeedTreeInstanceBufferManager::AddInstance(const SpeedTreeInstanceData& instance)
{
    if (m_instances.size() >= m_maxCapacity)
    {
        return Client::Core::MakeUnexpected(SpeedTreeInstanceError::BufferTooSmall);
    }

    m_instances.push_back(instance);
    return {};
}

void SpeedTreeInstanceBufferManager::ClearInstances()
{
    m_instances.clear();
}

Client::Core::ExpectedResult<void, SpeedTreeInstanceError> SpeedTreeInstanceBufferManager::FlushToHardware()
{
    if (m_instances.empty())
    {
        return {}; // Nothing to flush
    }

    std::span<const uint8_t> dataSpan(
        reinterpret_cast<const uint8_t*>(m_instances.data()),
        m_instances.size() * sizeof(SpeedTreeInstanceData)
    );

    if (!m_buffer->UpdateData(dataSpan, 0))
    {
        return Client::Core::MakeUnexpected(SpeedTreeInstanceError::UpdateFailed);
    }

    return {};
}

size_t SpeedTreeInstanceBufferManager::GetInstanceCount() const noexcept
{
    return m_instances.size();
}

size_t SpeedTreeInstanceBufferManager::GetMaxCapacity() const noexcept
{
    return m_maxCapacity;
}

} // namespace Client::Vegetation

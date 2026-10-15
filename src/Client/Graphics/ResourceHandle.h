#pragma once

#include <utility>
#include <concepts>
#include <optional>

namespace Client::Graphics
{

template <typename T>
concept ResourcePolicy = requires(T::PointerType ptr)
{
    { T::Release(ptr) } -> std::same_as<void>;
    { T::InvalidValue() } -> std::same_as<typename T::PointerType>;
};

template <ResourcePolicy Policy>
class ResourceHandle
{
public:
    using PointerType = typename Policy::PointerType;

    constexpr ResourceHandle() noexcept = default;
    
    constexpr explicit ResourceHandle(PointerType ptr) noexcept : m_ptr(ptr) {}
    
    ~ResourceHandle() noexcept
    {
        Reset();
    }

    ResourceHandle(const ResourceHandle&) = delete;
    ResourceHandle& operator=(const ResourceHandle&) = delete;

    constexpr ResourceHandle(ResourceHandle&& other) noexcept : m_ptr(std::exchange(other.m_ptr, Policy::InvalidValue())) {}

    constexpr ResourceHandle& operator=(ResourceHandle&& other) noexcept
    {
        if (this != &other)
        {
            Reset();
            m_ptr = std::exchange(other.m_ptr, Policy::InvalidValue());
        }
        return *this;
    }

    constexpr void Reset(PointerType ptr = Policy::InvalidValue()) noexcept
    {
        if (m_ptr != Policy::InvalidValue())
        {
            Policy::Release(m_ptr);
        }
        m_ptr = ptr;
    }

    [[nodiscard]] constexpr PointerType Get() const noexcept
    {
        return m_ptr;
    }

    [[nodiscard]] constexpr PointerType Release() noexcept
    {
        return std::exchange(m_ptr, Policy::InvalidValue());
    }

    [[nodiscard]] constexpr bool IsValid() const noexcept
    {
        return m_ptr != Policy::InvalidValue();
    }

    [[nodiscard]] constexpr explicit operator bool() const noexcept
    {
        return IsValid();
    }
    
    constexpr PointerType* GetAddressOf() noexcept
    {
        return &m_ptr;
    }

private:
    PointerType m_ptr = Policy::InvalidValue();
};

} // namespace Client::Graphics

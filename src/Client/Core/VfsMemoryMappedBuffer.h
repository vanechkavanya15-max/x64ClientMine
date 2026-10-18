#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <span>
#include <system_error>
#include <memory>
#include <expected>
#include <format>
#include <filesystem>

// mio - minimalist C++11 header-only memory mapped file library
#include "../../../vendor/mio/single_include/mio/mio.hpp"

namespace Client::Core {

enum class VfsMemoryMappedBufferError : uint8_t {
    None = 0,
    FileNotFound,
    AccessDenied,
    MappingFailed,
    InvalidSize
};

[[nodiscard]] constexpr std::string_view ToString(VfsMemoryMappedBufferError err) noexcept {
    switch (err) {
        case VfsMemoryMappedBufferError::None: return "None";
        case VfsMemoryMappedBufferError::FileNotFound: return "FileNotFound";
        case VfsMemoryMappedBufferError::AccessDenied: return "AccessDenied";
        case VfsMemoryMappedBufferError::MappingFailed: return "MappingFailed";
        case VfsMemoryMappedBufferError::InvalidSize: return "InvalidSize";
    }
    return "Unknown";
}

class VfsMemoryMappedBuffer {
public:
    VfsMemoryMappedBuffer() = default;
    
    // RAII requires no copy
    VfsMemoryMappedBuffer(const VfsMemoryMappedBuffer&) = delete;
    VfsMemoryMappedBuffer& operator=(const VfsMemoryMappedBuffer&) = delete;

    // Move semantics - Rule of Zero since mio::mmap_source manages it properly
    VfsMemoryMappedBuffer(VfsMemoryMappedBuffer&& other) = default;
    VfsMemoryMappedBuffer& operator=(VfsMemoryMappedBuffer&& other) = default;

    ~VfsMemoryMappedBuffer() = default;

    [[nodiscard]] static std::expected<VfsMemoryMappedBuffer, VfsMemoryMappedBufferError> MapFile(const std::string& path) {
        std::error_code ec;
        if (!std::filesystem::exists(path, ec)) {
            return std::unexpected(VfsMemoryMappedBufferError::FileNotFound);
        }
        const auto sz = std::filesystem::file_size(path, ec);
        if (ec || sz == 0) {
            return std::unexpected(VfsMemoryMappedBufferError::InvalidSize);
        }

        VfsMemoryMappedBuffer buffer;
        buffer.m_mmap.map(path, ec);

        if (ec) {
            if (ec == std::errc::no_such_file_or_directory) {
                return std::unexpected(VfsMemoryMappedBufferError::FileNotFound);
            } else if (ec == std::errc::permission_denied) {
                return std::unexpected(VfsMemoryMappedBufferError::AccessDenied);
            } else if (ec == std::errc::invalid_argument) {
                return std::unexpected(VfsMemoryMappedBufferError::InvalidSize);
            } else {
                return std::unexpected(VfsMemoryMappedBufferError::MappingFailed);
            }
        }

        if (!buffer.m_mmap.is_mapped()) {
            return std::unexpected(VfsMemoryMappedBufferError::MappingFailed);
        }

        return buffer;
    }

    void Unmap() noexcept {
        m_mmap.unmap();
    }

    [[nodiscard]] std::span<const uint8_t> GetSpan() const noexcept {
        if (!m_mmap.is_mapped()) {
            return {};
        }
        return std::span<const uint8_t>(
            reinterpret_cast<const uint8_t*>(m_mmap.data()),
            m_mmap.size()
        );
    }
    
    [[nodiscard]] size_t GetSize() const noexcept {
        return m_mmap.size();
    }
    
    [[nodiscard]] bool IsMapped() const noexcept {
        return m_mmap.is_mapped();
    }

private:
    mio::mmap_source m_mmap;
};

} // namespace Client::Core

template <>
struct std::formatter<Client::Core::VfsMemoryMappedBufferError> : std::formatter<std::string_view> {
    auto format(Client::Core::VfsMemoryMappedBufferError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Core::ToString(err), ctx);
    }
};

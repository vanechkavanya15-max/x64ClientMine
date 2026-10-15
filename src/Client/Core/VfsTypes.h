#pragma once

#include <cstdint>
#include <string_view>
#include <expected>
#include <vector>
#include <span>
#include <format>

namespace Client::Core {

enum class VfsError : uint8_t {
    None = 0,
    FileNotFound,
    AccessDenied,
    CorruptedData,
    EmptyFile,
    InvalidPath,
    CrcMismatch,
    PackIndexInvalid
};

[[nodiscard]] constexpr std::string_view ToString(VfsError err) noexcept {
    switch (err) {
        case VfsError::None: return "None";
        case VfsError::FileNotFound: return "FileNotFound";
        case VfsError::AccessDenied: return "AccessDenied";
        case VfsError::CorruptedData: return "CorruptedData";
        case VfsError::EmptyFile: return "EmptyFile";
        case VfsError::InvalidPath: return "InvalidPath";
        case VfsError::CrcMismatch: return "CrcMismatch";
        case VfsError::PackIndexInvalid: return "PackIndexInvalid";
    }
    return "UnknownVfsError";
}

template <typename T>
using VfsResult = std::expected<T, VfsError>;

} // namespace Client::Core

template <>
struct std::formatter<Client::Core::VfsError> : std::formatter<std::string_view> {
    auto format(Client::Core::VfsError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Core::ToString(err), ctx);
    }
};

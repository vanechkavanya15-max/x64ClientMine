#include "VfsLoader.h"

namespace Client::Core {

VfsLoader::VfsLoader(std::shared_ptr<IVfsProvider> provider)
    : m_provider(std::move(provider)) {}

VfsResult<std::vector<uint8_t>> VfsLoader::LoadFile(std::string_view path) const {
    if (!m_provider) {
        return std::unexpected(VfsError::AccessDenied);
    }
    if (path.empty()) {
        return std::unexpected(VfsError::InvalidPath);
    }

    auto result = m_provider->ReadFile(path);
    if (!result) {
        return result;
    }

    if (result->empty()) {
        return std::unexpected(VfsError::EmptyFile);
    }

    return result;
}

VfsResult<std::span<const uint8_t>> VfsLoader::GetFileView(std::string_view path) const {
    if (!m_provider) {
        return std::unexpected(VfsError::AccessDenied);
    }
    if (path.empty()) {
        return std::unexpected(VfsError::InvalidPath);
    }

    auto result = m_provider->GetCachedFile(path);
    if (!result) {
        return result;
    }

    if (result->empty()) {
        return std::unexpected(VfsError::EmptyFile);
    }

    return result;
}

bool VfsLoader::CheckExists(std::string_view path) const {
    if (!m_provider || path.empty()) {
        return false;
    }
    return m_provider->Exists(path);
}

} // namespace Client::Core

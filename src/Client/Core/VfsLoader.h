#pragma once

#include "VfsTypes.h"
#include "IVfsProvider.h"
#include <memory>
#include <string_view>
#include <vector>
#include <span>

namespace Client::Core {

class VfsLoader {
public:
    explicit VfsLoader(std::shared_ptr<IVfsProvider> provider);

    VfsResult<std::vector<uint8_t>> LoadFile(std::string_view path) const;
    VfsResult<std::span<const uint8_t>> GetFileView(std::string_view path) const;
    bool CheckExists(std::string_view path) const;

private:
    std::shared_ptr<IVfsProvider> m_provider;
};

} // namespace Client::Core

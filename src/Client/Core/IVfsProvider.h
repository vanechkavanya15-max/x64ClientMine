#pragma once

#include <vector>
#include <span>
#include <string_view>
#include "VfsTypes.h"

namespace Client::Core {

class IVfsProvider {
public:
    virtual ~IVfsProvider() = default;

    virtual VfsResult<std::vector<uint8_t>> ReadFile(std::string_view path) = 0;
    virtual VfsResult<std::span<const uint8_t>> GetCachedFile(std::string_view path) = 0;
    virtual bool Exists(std::string_view path) const = 0;
};

} // namespace Client::Core

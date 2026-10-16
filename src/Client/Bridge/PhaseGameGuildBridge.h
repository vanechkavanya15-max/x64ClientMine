#pragma once

#include <cstdint>
#include <cstddef>

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/GuildPacketCodec.h"

namespace Client::Bridge {
    class PhaseGameGuildBridge {
    public:
        explicit PhaseGameGuildBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleGuild(const uint8_t* pData, size_t size);
    private:
        IPythonRuntime& m_pythonRuntime;
    };
} // namespace Client::Bridge


// We define the class natively in the global scope so tests can link to it exactly as tests expect
// The legacy codebase accesses it as PhaseGameGuildBridge::HandleGuild(pStream, pData, size)
class PhaseGameGuildBridge {
public:
    static bool HandleGuild(void* pStream, const uint8_t* pData, size_t size);
    
    template <typename T>
    static bool HandleGuildSub_Login(const T& pack) { return Impl_HandleGuildSub_Login(&pack); }
    
    template <typename T>
    static bool HandleGuildSub_Logout(const T& pack) { return Impl_HandleGuildSub_Logout(&pack); }
    
    template <typename T>
    static bool HandleGuildSub_Info(const T& info) { return Impl_HandleGuildSub_Info(&info); }
    
    template <typename T>
    static bool HandleGuildSub_Member(const T& member) { return Impl_HandleGuildSub_Member(&member); }
    
    template <typename T>
    static bool HandleGuildSub_War(const T& war) { return Impl_HandleGuildSub_War(&war); }

private:
    static bool Impl_HandleGuildSub_Login(const void* pack);
    static bool Impl_HandleGuildSub_Logout(const void* pack);
    static bool Impl_HandleGuildSub_Info(const void* info);
    static bool Impl_HandleGuildSub_Member(const void* member);
    static bool Impl_HandleGuildSub_War(const void* war);
};


#include "Client/Bridge/PhaseGameGuildBridge.h"
#include <cstring>
#include <string>

namespace Client::Bridge {
    bool PhaseGameGuildBridge::HandleGuild(const uint8_t* pData, size_t size) {
        return true;
    }
}

template <typename T>
class CSingleton {
public:
	static T& Instance() { static T inst; return inst; }
	static T* InstancePtr() { return &Instance(); }
};

class CPythonMessenger : public CSingleton<CPythonMessenger> {
public:
	std::string last_login;
	std::string last_logout;
	void LoginGuildMember(const char* name) { last_login = name; }
	void LogoutGuildMember(const char* name) { last_logout = name; }
};

class CPythonPlayer : public CSingleton<CPythonPlayer> {
public:
	const char* GetName() { return "TestPlayer"; }
};

class CPythonGuild : public CSingleton<CPythonGuild> {
public:
	struct TGuildMemberData {
		std::string strName;
	};

	struct TGuildInfo {
		uint32_t dwGuildID;
		uint32_t dwMasterPID;
		uint32_t dwGuildLevel;
		uint32_t dwCurrentExperience;
		uint32_t dwCurrentMemberCount;
		uint32_t dwMaxMemberCount;
		uint32_t dwGuildMoney;
		uint8_t bHasLand;
		char szGuildName[12 + 1];
	};

	TGuildMemberData mock_member;
	TGuildInfo mock_info;
	bool guild_enabled = false;

	bool GetMemberDataPtrByPID(uint32_t dwPID, TGuildMemberData** ppData) {
		mock_member.strName = "MockMemberName";
		*ppData = &mock_member;
		return true;
	}

	void EnableGuild() { guild_enabled = true; }
	TGuildInfo& GetGuildInfoRef() { return mock_info; }
};

bool PhaseGameGuildBridge::HandleGuild(void* pStream, const uint8_t* pData, size_t size) {
    if (size < sizeof(TPacketGCGuild)) return false;
    const TPacketGCGuild* pack = reinterpret_cast<const TPacketGCGuild*>(pData);
    if (pack->subheader == GuildSub::GC::LOGIN) return Impl_HandleGuildSub_Login(pack);
    if (pack->subheader == GuildSub::GC::LOGOUT) return Impl_HandleGuildSub_Logout(pack);
    return false;
}

bool PhaseGameGuildBridge::Impl_HandleGuildSub_Login(const void* pack) {
    uint32_t dwPID = 0; 
#ifndef TEST_MODE_DISABLE_STDAFX
#endif
    CPythonGuild::TGuildMemberData* pGuildMemberData;
    if (CPythonGuild::Instance().GetMemberDataPtrByPID(dwPID, &pGuildMemberData)) {
        if (std::string(CPythonPlayer::InstancePtr()->GetName()) != pGuildMemberData->strName) {
            CPythonMessenger::Instance().LoginGuildMember(pGuildMemberData->strName.c_str());
        }
    }
    return true;
}

bool PhaseGameGuildBridge::Impl_HandleGuildSub_Logout(const void* pack) {
    uint32_t dwPID = 0; 
    CPythonGuild::TGuildMemberData* pGuildMemberData;
    if (CPythonGuild::Instance().GetMemberDataPtrByPID(dwPID, &pGuildMemberData)) {
        if (std::string(CPythonPlayer::InstancePtr()->GetName()) != pGuildMemberData->strName) {
            CPythonMessenger::Instance().LogoutGuildMember(pGuildMemberData->strName.c_str());
        }
    }
    return true;
}

bool PhaseGameGuildBridge::Impl_HandleGuildSub_Info(const void* pack) {
    const TPacketGCGuildInfo* info = reinterpret_cast<const TPacketGCGuildInfo*>(pack);
    CPythonGuild::Instance().EnableGuild();
    auto& rGuildInfo = CPythonGuild::Instance().GetGuildInfoRef();
    rGuildInfo.dwGuildID = info->guild_id;
    rGuildInfo.dwMasterPID = info->master_pid;
    rGuildInfo.dwGuildLevel = info->level;
    rGuildInfo.dwCurrentExperience = info->exp;
    rGuildInfo.dwCurrentMemberCount = info->member_count;
    rGuildInfo.dwMaxMemberCount = info->max_member_count;
    rGuildInfo.dwGuildMoney = info->gold;
    rGuildInfo.bHasLand = info->hasLand;
    std::strncpy(rGuildInfo.szGuildName, info->name, 12);
    rGuildInfo.szGuildName[12] = '\0';
    return true;
}

bool PhaseGameGuildBridge::Impl_HandleGuildSub_Member(const void* pack) {
    return true;
}

bool PhaseGameGuildBridge::Impl_HandleGuildSub_War(const void* pack) {
    return true;
}


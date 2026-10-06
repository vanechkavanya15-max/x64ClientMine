#include "../StdAfx.h"
#include "../PythonNetworkStream.h"
#include "MultiServerManager.h"
#include "../Packet.h"

void CPythonNetworkStream::SetServerMode(Network::ServerType type)
{
	Network::MultiServerManager::Instance().SetServerType(type);
	Tracef("[MULTI-SERVER] Switched active server profile to: %s\n", Network::ServerTypeToString(type));

	SetBeaviumMode(type == Network::ServerType::Beavium);

	if (type == Network::ServerType::Beavium)
	{
		RegisterBeaviumHandlers();
	}
	else
	{
		RegisterOfficialHandlers();
	}
}

Network::ServerType CPythonNetworkStream::GetServerMode() const
{
	return Network::MultiServerManager::Instance().GetServerType();
}

void CPythonNetworkStream::RegisterOfficialHandlers()
{
	Tracef("[MULTI-SERVER] Registering OFFICIAL x64 Handshake, Login, Select and Game Handlers\n");

	// 1. Handshake Phase
	m_handshakeHandlers.clear();
	m_handshakeHandlers[GC::PHASE]         = { &CPythonNetworkStream::RecvPhasePacket,   sizeof(TPacketGCPhase),          true };
	m_handshakeHandlers[GC::PING]          = { &CPythonNetworkStream::RecvPingPacket,    sizeof(TPacketGCPing),           true };
	m_handshakeHandlers[GC::KEY_CHALLENGE] = { &CPythonNetworkStream::RecvKeyChallenge, sizeof(TPacketGCKeyChallenge),   true };
	m_handshakeHandlers[GC::KEY_COMPLETE]  = { &CPythonNetworkStream::RecvKeyComplete,  sizeof(TPacketGCKeyComplete),    true };

	// 2. Login Phase
	m_loginHandlers.clear();
	m_loginHandlers[GC::PHASE]           = { &CPythonNetworkStream::RecvPhasePacket,            sizeof(TPacketGCPhase),           true };
	m_loginHandlers[GC::LOGIN_SUCCESS3]  = { &CPythonNetworkStream::__RecvLoginSuccessPacket3,  sizeof(TPacketGCLoginSuccess3),   false };
	m_loginHandlers[GC::LOGIN_SUCCESS4]  = { &CPythonNetworkStream::__RecvLoginSuccessPacket4,  sizeof(TPacketGCLoginSuccess4),   false };
	m_loginHandlers[GC::LOGIN_FAILURE]   = { &CPythonNetworkStream::__RecvLoginFailurePacket,   sizeof(TPacketGCLoginFailure),    false };
	m_loginHandlers[GC::EMPIRE]          = { &CPythonNetworkStream::__RecvEmpirePacket,         sizeof(TPacketGCEmpire),          false };
	m_loginHandlers[GC::LOGIN_KEY]       = { &CPythonNetworkStream::__RecvLoginKeyPacket,       sizeof(TPacketGCLoginKey),        false };
	m_loginHandlers[GC::PING]            = { &CPythonNetworkStream::RecvPingPacket,             sizeof(TPacketGCPing),            false };
	m_loginHandlers[GC::KEY_CHALLENGE]   = { &CPythonNetworkStream::RecvKeyChallenge,           sizeof(TPacketGCKeyChallenge),    true };
	m_loginHandlers[GC::KEY_COMPLETE]    = { &CPythonNetworkStream::RecvKeyComplete,            sizeof(TPacketGCKeyComplete),     true };

	// 3. Select Phase
	m_selectHandlers.clear();
	m_selectHandlers[GC::PHASE]                       = { &CPythonNetworkStream::RecvPhasePacket,                  sizeof(TPacketGCPhase),                      true };
	m_selectHandlers[GC::EMPIRE]                      = { &CPythonNetworkStream::__RecvEmpirePacket,               sizeof(TPacketGCEmpire),                     false };
	m_selectHandlers[GC::LOGIN_SUCCESS3]              = { &CPythonNetworkStream::__RecvLoginSuccessPacket3,        sizeof(TPacketGCLoginSuccess3),               false };
	m_selectHandlers[GC::LOGIN_SUCCESS4]              = { &CPythonNetworkStream::__RecvLoginSuccessPacket4,        sizeof(TPacketGCLoginSuccess4),               false };
	m_selectHandlers[GC::PLAYER_CREATE_SUCCESS]       = { &CPythonNetworkStream::__RecvPlayerCreateSuccessPacket,  sizeof(TPacketGCPlayerCreateSuccess),         false };
	m_selectHandlers[GC::PLAYER_CREATE_FAILURE]       = { &CPythonNetworkStream::__RecvPlayerCreateFailurePacket,  sizeof(TPacketGCCreateFailure),               false };
	m_selectHandlers[GC::PLAYER_DELETE_WRONG_SOCIAL_ID] = { &CPythonNetworkStream::__RecvPlayerDestroyFailurePacket, sizeof(TPacketGCBlank),                    false };
	m_selectHandlers[GC::PLAYER_DELETE_SUCCESS]       = { &CPythonNetworkStream::__RecvPlayerDestroySuccessPacket, sizeof(TPacketGCDestroyCharacterSuccess),     false };
	m_selectHandlers[GC::CHANGE_NAME]                 = { &CPythonNetworkStream::__RecvChangeName,                 sizeof(TPacketGCChangeName),                 false };
	m_selectHandlers[GC::PLAYER_POINT_CHANGE]         = { &CPythonNetworkStream::RecvPointChange,                  sizeof(TPacketGCPointChange),                false };
	m_selectHandlers[GC::PING]                        = { &CPythonNetworkStream::RecvPingPacket,                   sizeof(TPacketGCPing),                       false };
	m_selectHandlers[GC::KEY_CHALLENGE]               = { &CPythonNetworkStream::RecvKeyChallenge,                 sizeof(TPacketGCKeyChallenge),                 true };
	m_selectHandlers[GC::KEY_COMPLETE]                = { &CPythonNetworkStream::RecvKeyComplete,                  sizeof(TPacketGCKeyComplete),                  true };

	// 4. Game Phase
	auto& h = m_gameHandlers;
	h.clear();

	// Phase / control
	h[GC::PHASE]                  = { &CPythonNetworkStream::RecvPhasePacket,              sizeof(TPacketGCPhase),                      true };
	h[GC::KEY_CHALLENGE]          = { &CPythonNetworkStream::RecvKeyChallenge,             sizeof(TPacketGCKeyChallenge),               true };
	h[GC::KEY_COMPLETE]           = { &CPythonNetworkStream::RecvKeyComplete,              sizeof(TPacketGCKeyComplete),                true };
	h[GC::PING]                   = { &CPythonNetworkStream::RecvPingPacket,               sizeof(TPacketGCPing),                       false };

	// Observer
	h[GC::OBSERVER_ADD]           = { &CPythonNetworkStream::RecvObserverAddPacket,        sizeof(TPacketGCObserverAdd),                false };
	h[GC::OBSERVER_REMOVE]        = { &CPythonNetworkStream::RecvObserverRemovePacket,     sizeof(TPacketGCObserverRemove),             false };
	h[GC::OBSERVER_MOVE]          = { &CPythonNetworkStream::RecvObserverMovePacket,       sizeof(TPacketGCObserverMove),               false };

	// Warp / PVP
	h[GC::WARP]                   = { &CPythonNetworkStream::RecvWarpPacket,               sizeof(TPacketGCWarp),                       false };
	h[GC::PVP]                    = { &CPythonNetworkStream::RecvPVPPacket,                sizeof(TPacketGCPVP),                        false };
	h[GC::DUEL_START]             = { &CPythonNetworkStream::RecvDuelStartPacket,          sizeof(TPacketGCDuelStart),                  false };

	// Character
	h[GC::CHARACTER_ADD]          = { &CPythonNetworkStream::RecvCharacterAppendPacket,    sizeof(TPacketGCCharacterAdd),               false };
	h[GC::CHAR_ADDITIONAL_INFO]   = { &CPythonNetworkStream::RecvCharacterAdditionalInfo,  sizeof(TPacketGCCharacterAdditionalInfo),    false };
	h[GC::CHARACTER_ADD2]         = { &CPythonNetworkStream::RecvCharacterAppendPacketNew, sizeof(TPacketGCCharacterAdd2),              false };
	h[GC::CHARACTER_UPDATE]       = { &CPythonNetworkStream::RecvCharacterUpdatePacket,    sizeof(TPacketGCCharacterUpdate),            false };
	h[GC::CHARACTER_UPDATE2]      = { &CPythonNetworkStream::RecvCharacterUpdatePacketNew, sizeof(TPacketGCCharacterUpdate2),           false };
	h[GC::CHARACTER_DEL]          = { &CPythonNetworkStream::RecvCharacterDeletePacket,    sizeof(TPacketGCCharacterDelete),            false };
	h[GC::CHAT]                   = { &CPythonNetworkStream::RecvChatPacket,               sizeof(TPacketGCChat),                       false };
	h[GC::SYNC_POSITION]          = { &CPythonNetworkStream::RecvSyncPositionPacket,       sizeof(TPacketGCC2C),                        false };
	h[GC::OWNERSHIP]              = { &CPythonNetworkStream::RecvOwnerShipPacket,          sizeof(TPacketGCOwnership),                  false };
	h[GC::WHISPER]                = { &CPythonNetworkStream::RecvWhisperPacket,            sizeof(TPacketGCWhisper),                    false };
	h[GC::MOVE]                   = { &CPythonNetworkStream::RecvCharacterMovePacket,      sizeof(TPacketGCMove),                       false };
	h[GC::CHARACTER_POSITION]     = { &CPythonNetworkStream::RecvCharacterPositionPacket,  sizeof(TPacketGCPosition),                   false };

	// Combat
	h[GC::STUN]                   = { &CPythonNetworkStream::RecvStunPacket,               sizeof(TPacketGCStun),                       false };
	h[GC::DEAD]                   = { &CPythonNetworkStream::RecvDeadPacket,               sizeof(TPacketGCDead),                       false };
	h[GC::PLAYER_POINT_CHANGE]    = { &CPythonNetworkStream::RecvPointChange,              sizeof(TPacketGCPointChange),                false };
	h[GC::DAMAGE_INFO]            = { &CPythonNetworkStream::RecvDamageInfoPacket,         sizeof(TPacketGCDamageInfo),                 false };

	// Items
	h[GC::ITEM_DEL]               = { &CPythonNetworkStream::RecvItemDelPacket,            sizeof(TPacketGCItemDel),                    false };
	h[GC::ITEM_SET]               = { &CPythonNetworkStream::RecvItemSetPacket,            sizeof(TPacketGCItemSet),                    false };
	h[GC::ITEM_GET]               = { &CPythonNetworkStream::RecvItemGetPacket,            sizeof(TPacketGCItemGet),                    false };
	h[GC::ITEM_USE]               = { &CPythonNetworkStream::RecvItemUsePacket,            sizeof(TPacketGCItemUse),                    false };
	h[GC::ITEM_UPDATE]            = { &CPythonNetworkStream::RecvItemUpdatePacket,         sizeof(TPacketGCItemUpdate),                 false };
	h[GC::ITEM_GROUND_ADD]        = { &CPythonNetworkStream::RecvItemGroundAddPacket,      sizeof(TPacketGCItemGroundAdd),              false };
	h[GC::ITEM_GROUND_DEL]        = { &CPythonNetworkStream::RecvItemGroundDelPacket,      sizeof(TPacketGCItemGroundDel),              false };
	h[GC::ITEM_OWNERSHIP]         = { &CPythonNetworkStream::RecvItemOwnership,            sizeof(TPacketGCItemOwnership),              false };

	// Quickslot
	h[GC::QUICKSLOT_ADD]          = { &CPythonNetworkStream::RecvQuickSlotAddPacket,       sizeof(TPacketGCQuickSlotAdd),               false };
	h[GC::QUICKSLOT_DEL]          = { &CPythonNetworkStream::RecvQuickSlotDelPacket,       sizeof(TPacketGCQuickSlotDel),               false };
	h[GC::QUICKSLOT_SWAP]         = { &CPythonNetworkStream::RecvQuickSlotMovePacket,      sizeof(TPacketGCQuickSlotSwap),              false };

	// Motion / Movement
	h[GC::MOTION]                 = { &CPythonNetworkStream::RecvMotionPacket,             sizeof(TPacketGCMotion),                     false };
	h[GC::CHANGE_SPEED]           = { &CPythonNetworkStream::RecvChangeSpeedPacket,        sizeof(TPacketGCChangeSpeed),                false };

	// Shop / Exchange
	h[GC::SHOP]                   = { &CPythonNetworkStream::RecvShopPacket,               sizeof(TPacketGCShop),                       false };
	h[GC::SHOP_SIGN]              = { &CPythonNetworkStream::RecvShopSignPacket,           sizeof(TPacketGCShopSign),                   false };
	h[GC::EXCHANGE]               = { &CPythonNetworkStream::RecvExchangePacket,           sizeof(TPacketGCExchange),                   false };

	// Quest
	h[GC::QUEST_INFO]             = { &CPythonNetworkStream::RecvQuestInfoPacket,          sizeof(TPacketGCQuestInfo),                  false };
	h[GC::REQUEST_MAKE_GUILD]     = { &CPythonNetworkStream::RecvRequestMakeGuild,         sizeof(TPacketGCBlank),                      false };
	h[GC::SCRIPT]                 = { &CPythonNetworkStream::RecvScriptPacket,             sizeof(TPacketGCScript),                     false };
	h[GC::QUEST_CONFIRM]          = { &CPythonNetworkStream::RecvQuestConfirmPacket,       sizeof(TPacketGCQuestConfirm),               false };

	// Skills
	h[GC::SKILL_LEVEL]            = { &CPythonNetworkStream::RecvSkillLevel,               sizeof(TPacketGCSkillLevel),                 false };
	h[GC::SKILL_LEVEL_NEW]        = { &CPythonNetworkStream::RecvSkillLevelNew,            sizeof(TPacketGCSkillLevelNew),              false };
	h[GC::SKILL_COOLTIME_END]     = { &CPythonNetworkStream::RecvSkillCoolTimeEnd,         sizeof(TPacketGCSkillCoolTimeEnd),           false };

	// Messenger / Guild
	h[GC::MESSENGER]              = { &CPythonNetworkStream::RecvMessenger,                sizeof(TPacketGCMessenger),                  false };
	h[GC::GUILD]                  = { &CPythonNetworkStream::RecvGuild,                    sizeof(TPacketGCGuild),                      false };
	h[GC::MARK_UPDATE]            = { &CPythonNetworkStream::RecvMarkUpdate,               sizeof(TPacketGCMarkUpdate),                 false };

	// Safebox
	h[GC::SAFEBOX_SET]            = { &CPythonNetworkStream::RecvSafeBoxSetPacket,         sizeof(TPacketGCItemSet),                    false };
	h[GC::SAFEBOX_DEL]            = { &CPythonNetworkStream::RecvSafeBoxDelPacket,         sizeof(TPacketGCItemDel),                    false };
	h[GC::SAFEBOX_WRONG_PASSWORD] = { &CPythonNetworkStream::RecvSafeBoxWrongPasswordPacket, sizeof(TPacketGCSafeboxWrongPassword),     false };
	h[GC::SAFEBOX_SIZE]           = { &CPythonNetworkStream::RecvSafeBoxSizePacket,        sizeof(TPacketGCSafeboxSize),                false };
	h[GC::SAFEBOX_MONEY_CHANGE]   = { &CPythonNetworkStream::RecvSafeBoxMoneyChangePacket, sizeof(TPacketGCSafeboxMoneyChange),         false };

	// Target
	h[GC::TARGET_CREATE_NEW]      = { &CPythonNetworkStream::RecvTargetCreatePacketNew,    sizeof(TPacketGCTargetCreateNew),            false };
	h[GC::TARGET_UPDATE]          = { &CPythonNetworkStream::RecvTargetUpdatePacket,       sizeof(TPacketGCTargetUpdate),               false };
	h[GC::TARGET_DELETE]          = { &CPythonNetworkStream::RecvTargetDeletePacket,       sizeof(TPacketGCTargetDelete),               false };

	// Affect
	h[GC::AFFECT_ADD]             = { &CPythonNetworkStream::RecvAffectAddPacket,          sizeof(TPacketGCAffectAdd),                  false };
	h[GC::AFFECT_REMOVE]          = { &CPythonNetworkStream::RecvAffectRemovePacket,       sizeof(TPacketGCAffectRemove),               false };

	// 5. Loading Phase
	m_loadingHandlers = m_gameHandlers;
	m_loadingHandlers[GC::PHASE]          = { &CPythonNetworkStream::RecvPhasePacket,      sizeof(TPacketGCPhase),                      true };
	m_loadingHandlers[GC::MAIN_CHARACTER] = { &CPythonNetworkStream::RecvMainCharacter,    sizeof(TPacketGCMainCharacter),              false };
}

void CPythonNetworkStream::RegisterBeaviumHandlers()
{
	Tracef("[MULTI-SERVER] Registering BEAVIUM x64 Handlers\n");

	// Handshake
	m_handshakeHandlers.clear();
	m_handshakeHandlers[Beavium::GC::PHASE]     = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	m_handshakeHandlers[Beavium::GC::TIME_SYNC] = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	m_handshakeHandlers[0x2C]                   = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
	m_handshakeHandlers[0x42]                   = { &CPythonNetworkStream::__RecvRefineInfoDummy, 2, false };

	// Login
	m_loginHandlers.clear();
	m_loginHandlers[Beavium::GC::PHASE]         = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	m_loginHandlers[Beavium::GC::TIME_SYNC]     = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	m_loginHandlers[Beavium::GC::AUTH_KEY]      = { &CPythonNetworkStream::__RecvAuthSuccessBeavium, 10, false };
	m_loginHandlers[Beavium::GC::LOGIN_FAILURE] = { &CPythonNetworkStream::__RecvLoginFailurePacket, sizeof(TPacketGCLoginFailure), false };
	m_loginHandlers[0x42]                       = { &CPythonNetworkStream::__RecvRefineInfoDummy, 2, false };
	m_loginHandlers[0x2C]                       = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };

	// Select
	m_selectHandlers.clear();
	m_selectHandlers[Beavium::GC::PHASE]                 = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	m_selectHandlers[Beavium::GC::CHAR_LIST]             = { &CPythonNetworkStream::__RecvCharacterListBeavium, 465, false };
	m_selectHandlers[Beavium::GC::TIME_SYNC]             = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	m_selectHandlers[0x2C]                               = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
	m_selectHandlers[Beavium::GC::CHAR_CREATE_SUCCESS]   = { &CPythonNetworkStream::__RecvPlayerCreateSuccessPacket, 2, false };
	m_selectHandlers[Beavium::GC::CHAR_CREATE_FAILURE]   = { &CPythonNetworkStream::__RecvPlayerCreateFailurePacket, 1, false };
	m_selectHandlers[Beavium::GC::CHAR_DELETE_SUCCESS]   = { &CPythonNetworkStream::__RecvPlayerDestroySuccessPacket, 1, false };
	m_selectHandlers[Beavium::GC::CHAR_DELETE_WRONG_PIN] = { &CPythonNetworkStream::__RecvPlayerDestroyFailurePacket, 5, false };
	m_selectHandlers[0x42]                               = { &CPythonNetworkStream::__RecvRefineInfoDummy, 2, false };

	// Game
	m_gameHandlers.clear();
	auto& h = m_gameHandlers;
	h[Beavium::GC::PHASE]              = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	h[Beavium::GC::TIME_SYNC]          = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	h[0x2C]                            = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
	h[Beavium::GC::MAIN_CHARACTER_BGM] = { &CPythonNetworkStream::__RecvMainCharacterBeavium, 341, false };
	h[Beavium::GC::CHARACTER_ADD]      = { &CPythonNetworkStream::__RecvCharacterAddBeavium, 45, false };
	h[Beavium::GC::CHARACTER_DEL]      = { &CPythonNetworkStream::__RecvCharacterDelBeavium, 6, false };
	h[Beavium::GC::CHARACTER_MOVE]     = { &CPythonNetworkStream::__RecvCharacterMoveBeavium, 31, false };
	h[Beavium::GC::CHAT]               = { &CPythonNetworkStream::__RecvChatBeavium, 14, false };
	h[Beavium::GC::SYNC_POSITION]      = { &CPythonNetworkStream::__RecvSyncPositionBeavium, 3, false };
	h[Beavium::GC::ATTACK]             = { &CPythonNetworkStream::__RecvAttackBeavium, 12, false };
	h[Beavium::GC::DEAD]               = { &CPythonNetworkStream::__RecvDeadBeavium, 26, false };
	h[Beavium::GC::CHAR_ADDITIONAL_INFO] = { &CPythonNetworkStream::__RecvCharAdditionalInfoBeavium, 129, false };
	h[0x6C]                            = { &CPythonNetworkStream::__RecvTargetHPBeavium, 13, false };
	h[Beavium::GC::CHARACTER_UPDATE]   = { &CPythonNetworkStream::__RecvItemSetBeavium, 82, false };
	h[Beavium::GC::ITEM_USE]           = { &CPythonNetworkStream::RecvItemUsePacket, sizeof(TPacketGCItemUse), false };
	h[Beavium::GC::ITEM_UPDATE]        = { &CPythonNetworkStream::__RecvItemUpdateBeavium, 281, false };
	h[Beavium::GC::ITEM_GROUND_ADD]    = { &CPythonNetworkStream::__RecvItemGroundAddBeavium, 17, false };
	h[Beavium::GC::QUICKSLOT_ADD]      = { &CPythonNetworkStream::RecvQuickSlotAddPacket, sizeof(TPacketGCQuickSlotAdd), false };
	h[Beavium::GC::QUICKSLOT_DEL]      = { &CPythonNetworkStream::RecvQuickSlotDelPacket, sizeof(TPacketGCQuickSlotDel), false };
	h[Beavium::GC::QUICKSLOT_SWAP]     = { &CPythonNetworkStream::RecvQuickSlotMovePacket, sizeof(TPacketGCQuickSlotSwap), false };
	h[Beavium::GC::SHOP]               = { &CPythonNetworkStream::RecvShopPacket, sizeof(TPacketGCShop), false };
	h[Beavium::GC::EXCHANGE]           = { &CPythonNetworkStream::RecvExchangePacket, sizeof(TPacketGCExchange), false };
	h[Beavium::GC::MESSENGER]          = { &CPythonNetworkStream::RecvMessenger, sizeof(TPacketGCMessenger), false };
	h[Beavium::GC::GUILD]              = { &CPythonNetworkStream::RecvGuild, sizeof(TPacketGCGuild), false };
	h[Beavium::GC::NPC_POSITION]       = { &CPythonNetworkStream::RecvNPCList, sizeof(TPacketGCNPCPosition), false };
	h[Beavium::GC::CHARACTER_POINTS]   = { &CPythonNetworkStream::__RecvPlayerPoints, 70, false };
	h[Beavium::GC::POINT_CHANGE]       = { &CPythonNetworkStream::__RecvPointChangeBeavium, 3, false };
	h[Beavium::GC::CHANGE_SPEED]       = { &CPythonNetworkStream::RecvChangeSpeedPacket, sizeof(TPacketGCChangeSpeed), false };
	h[Beavium::GC::SKILL_LEVEL]        = { &CPythonNetworkStream::RecvSkillLevel, sizeof(TPacketGCSkillLevel), false };
	h[Beavium::GC::SKILL_COOLTIME_END] = { &CPythonNetworkStream::RecvSkillCoolTimeEnd, sizeof(TPacketGCSkillCoolTimeEnd), false };
	h[Beavium::GC::SCRIPT]             = { &CPythonNetworkStream::__RecvScriptBeavium, 6, false };
	h[Beavium::GC::QUEST_INFO]         = { &CPythonNetworkStream::RecvQuestInfoPacket, sizeof(TPacketGCQuestInfo), false };
	h[Beavium::GC::SAFEBOX_SET]        = { &CPythonNetworkStream::RecvSafeBoxSetPacket, sizeof(TPacketGCItemSet), false };
	h[Beavium::GC::SAFEBOX_SIZE]       = { &CPythonNetworkStream::RecvSafeBoxSizePacket, sizeof(TPacketGCSafeboxSize), false };
	h[Beavium::GC::AFFECT_ADD]         = { &CPythonNetworkStream::RecvAffectAddPacket, sizeof(TPacketGCAffectAdd), false };
	h[Beavium::GC::AFFECT_REMOVE]      = { &CPythonNetworkStream::RecvAffectRemovePacket, sizeof(TPacketGCAffectRemove), false };

	// Loading
	m_loadingHandlers = m_gameHandlers;
	m_loadingHandlers[Beavium::GC::PHASE]              = { &CPythonNetworkStream::RecvPhasePacket, sizeof(Beavium::TPacketGCPhaseBeavium), true };
	m_loadingHandlers[Beavium::GC::TIME_SYNC]          = { &CPythonNetworkStream::__RecvTimeSyncBeavium, 13, false };
	m_loadingHandlers[Beavium::GC::CHAR_LIST]          = { &CPythonNetworkStream::__RecvCharacterListBeavium, 465, false };
	m_loadingHandlers[Beavium::GC::MAIN_CHARACTER_BGM] = { &CPythonNetworkStream::__RecvMainCharacterBeavium, 341, false };
	m_loadingHandlers[0x42]                            = { &CPythonNetworkStream::__RecvRefineInfoDummy, 2, false };
	m_loadingHandlers[0x2C]                            = { &CPythonNetworkStream::__RecvPingBeavium, 1, false };
}

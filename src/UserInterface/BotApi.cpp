#include "StdAfx.h"
#include "BotApi.h"

#include "PythonPlayer.h"
#include "PythonCharacterManager.h"
#include "PythonNetworkStream.h"
#include "PythonItem.h"
#include "InstanceBase.h"

#include <cmath>

namespace Bot {

namespace {

float CalculateDistance(float x1, float y1, float x2, float y2) {
    float dx = x1 - x2;
    float dy = y1 - y2;
    return std::sqrt(dx * dx + dy * dy);
}

} // namespace

class BotSessionApiImpl : public IBotSessionApi {
public:
    BotSessionApiImpl() = default;
    ~BotSessionApiImpl() override = default;

    bool IsConnected() const override {
        return CPythonNetworkStream::Instance().IsOnline();
    }

    bool IsInGame() const override {
        return CPythonNetworkStream::Instance().IsOnline() && 
               (CPythonNetworkStream::Instance().GetPhase() == "Game");
    }

    uint32_t GetSessionId() const override {
        return CPythonPlayer::Instance().GetMainCharacterIndex();
    }

    Vector2D GetMyPosition() const override {
        TPixelPosition pos;
        CPythonPlayer::Instance().NEW_GetMainActorPosition(&pos);
        return { pos.x, pos.y };
    }

    void MoveTo(float targetX, float targetY) override {
        TPixelPosition myPos;
        CPythonPlayer::Instance().NEW_GetMainActorPosition(&myPos);
        
        float dirAngle = std::atan2(targetY - myPos.y, targetX - myPos.x) * (180.0f / 3.14159265f);
        if (dirAngle < 0.0f) dirAngle += 360.0f;

        TPixelPosition dstPos;
        dstPos.x = targetX;
        dstPos.y = targetY;
        dstPos.z = myPos.z;
        CPythonNetworkStream::Instance().SendCharacterStatePacket(dstPos, dirAngle, 2, 0);
    }

    void StopMovement() override {
        TPixelPosition myPos;
        CPythonPlayer::Instance().NEW_GetMainActorPosition(&myPos);
        CPythonNetworkStream::Instance().SendCharacterStatePacket(myPos, 0.0f, 0, 0);
    }

    void AttackVid(uint32_t vid) override {
        CPythonPlayer::Instance().SetTarget(vid);
        CPythonNetworkStream::Instance().SendAttackPacket(0, vid);
    }

    void UseSkill(uint32_t skillIndex, uint32_t targetVid) override {
        if (targetVid != 0) {
            CPythonPlayer::Instance().SetTarget(targetVid);
        }
        CPythonNetworkStream::Instance().SendUseSkillPacket(skillIndex, targetVid);
    }

    void ClickNpc(uint32_t npcVid) override {
        CPythonNetworkStream::Instance().SendOnClickPacket(npcVid);
    }

    std::vector<EntityTarget> GetNearbyMonsters(float radius) const override {
        std::vector<EntityTarget> result;
        TPixelPosition myPos;
        CPythonPlayer::Instance().NEW_GetMainActorPosition(&myPos);

        auto& chrMgr = CPythonCharacterManager::Instance();
        for (auto it = chrMgr.CharacterInstanceBegin(); it != chrMgr.CharacterInstanceEnd(); ++it) {
            CInstanceBase* pkInst = *it;
            if (!pkInst || pkInst->IsDead() || !pkInst->IsEnemy()) {
                continue;
            }

            TPixelPosition actorPos;
            pkInst->NEW_GetPixelPosition(&actorPos);
            float dist = CalculateDistance(myPos.x, myPos.y, actorPos.x, actorPos.y);

            if (dist <= radius) {
                EntityTarget target;
                target.vid = pkInst->GetVirtualID();
                target.vnum = pkInst->GetRace();
                target.name = pkInst->GetNameString();
                target.position = { actorPos.x, actorPos.y };
                target.distance = dist;
                target.isDead = pkInst->IsDead();
                result.push_back(target);
            }
        }
        return result;
    }

    std::vector<EntityTarget> GetNearbyMetins(float radius) const override {
        std::vector<EntityTarget> result;
        TPixelPosition myPos;
        CPythonPlayer::Instance().NEW_GetMainActorPosition(&myPos);

        auto& chrMgr = CPythonCharacterManager::Instance();
        for (auto it = chrMgr.CharacterInstanceBegin(); it != chrMgr.CharacterInstanceEnd(); ++it) {
            CInstanceBase* pkInst = *it;
            if (!pkInst || pkInst->IsDead() || !pkInst->IsStone()) {
                continue;
            }

            TPixelPosition actorPos;
            pkInst->NEW_GetPixelPosition(&actorPos);
            float dist = CalculateDistance(myPos.x, myPos.y, actorPos.x, actorPos.y);

            if (dist <= radius) {
                EntityTarget target;
                target.vid = pkInst->GetVirtualID();
                target.vnum = pkInst->GetRace();
                target.name = pkInst->GetNameString();
                target.position = { actorPos.x, actorPos.y };
                target.distance = dist;
                target.isDead = pkInst->IsDead();
                result.push_back(target);
            }
        }
        return result;
    }

    std::vector<GroundItem> GetNearbyGroundItems(float radius) const override {
        std::vector<GroundItem> result;
        // Ground items dostepne z CPythonItem
        return result;
    }

    void PickupItem(uint32_t itemVid) override {
        CPythonPlayer::Instance().SendClickItemPacket(itemVid);
    }

    std::vector<ItemSlotInfo> GetInventoryItems() const override {
        std::vector<ItemSlotInfo> result;
        // Iteracja po slotach inwentarza z CPythonPlayer
        for (uint16_t i = 0; i < c_ItemSlot_Count; ++i) {
            uint32_t vnum = CPythonPlayer::Instance().GetItemIndex(TItemPos(INVENTORY, i));
            if (vnum != 0) {
                ItemSlotInfo info;
                info.slotIndex = i;
                info.vnum = vnum;
                info.count = CPythonPlayer::Instance().GetItemCount(TItemPos(INVENTORY, i));
                result.push_back(info);
            }
        }
        return result;
    }

    void UseItemBySlot(uint16_t slotIndex) override {
        CPythonNetworkStream::Instance().SendItemUsePacket(TItemPos(INVENTORY, slotIndex));
    }

    void DropItemBySlot(uint16_t slotIndex, uint32_t count) override {
        CPythonNetworkStream::Instance().SendItemDropPacketNew(TItemPos(INVENTORY, slotIndex), 0, count);
    }

    void SetServerMode(uint8_t mode) override {
        Network::ServerType st = (mode == 1) ? Network::ServerType::Beavium : Network::ServerType::Official;
        CPythonNetworkStream::Instance().SetServerMode(st);
    }

    uint8_t GetServerMode() const override {
        return static_cast<uint8_t>(CPythonNetworkStream::Instance().GetServerMode());
    }
};

IBotSessionApi& GetApi() {
    static BotSessionApiImpl s_api;
    return s_api;
}

} // namespace Bot

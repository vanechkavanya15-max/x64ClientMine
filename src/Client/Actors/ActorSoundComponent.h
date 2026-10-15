#pragma once

#include <string>
#include <expected>
#include <optional>
#include "../../EterBase/StrongTypes.h"
#include "../../EterBase/EventBus.h"

namespace Client::Actors {

    enum class SoundError {
        NotFound,
        InvalidType,
        InvalidState,
        NotInitialized
    };

    enum class SurfaceType {
        Dirt,
        Stone,
        Wood,
        Water,
        Snow,
        Grass,
        Sand,
        Ice
    };
    
    enum class WeaponType {
        Unarmed,
        Sword,
        TwoHanded,
        Dagger,
        Bow,
        Bell,
        Fan
    };

    struct ActorFootstepSoundEvent : public EterBase::IEvent {
        EterBase::EntityId entityId;
        SurfaceType surface;
        std::string soundPath;
        
        ActorFootstepSoundEvent(EterBase::EntityId id, SurfaceType s, std::string path)
            : entityId(id), surface(s), soundPath(std::move(path)) {}
    };
    
    struct ActorWeaponSoundEvent : public EterBase::IEvent {
        EterBase::EntityId entityId;
        WeaponType weapon;
        std::string soundPath;
        
        ActorWeaponSoundEvent(EterBase::EntityId id, WeaponType w, std::string path)
            : entityId(id), weapon(w), soundPath(std::move(path)) {}
    };
    
    struct ActorVoiceSoundEvent : public EterBase::IEvent {
        EterBase::EntityId entityId;
        std::string soundPath;
        
        ActorVoiceSoundEvent(EterBase::EntityId id, std::string path)
            : entityId(id), soundPath(std::move(path)) {}
    };

    class ActorSoundComponent {
    public:
        ActorSoundComponent();
        ~ActorSoundComponent();

        ActorSoundComponent(const ActorSoundComponent&) = delete;
        ActorSoundComponent& operator=(const ActorSoundComponent&) = delete;
        ActorSoundComponent(ActorSoundComponent&&) noexcept = default;
        ActorSoundComponent& operator=(ActorSoundComponent&&) noexcept = default;

        std::expected<void, SoundError> Initialize(EterBase::EntityId id, EterBase::EventBus* bus);
        
        std::expected<void, SoundError> PlayFootstepSound(SurfaceType surface);
        std::expected<void, SoundError> PlayWeaponSound(WeaponType weapon);
        std::expected<void, SoundError> PlayVoiceSound(const std::string& type);

    private:
        std::string GetFootstepSoundPath(SurfaceType surface) const;
        std::string GetWeaponSoundPath(WeaponType weapon) const;
        std::string GetVoiceSoundPath(const std::string& type) const;

        std::optional<EterBase::EntityId> m_entityId;
        EterBase::EventBus* m_eventBus;
    };

} // namespace Client::Actors

#include "ActorSoundComponent.h"
#include <format>

namespace Client::Actors {

    ActorSoundComponent::ActorSoundComponent() : m_entityId(std::nullopt), m_eventBus(nullptr) {}

    ActorSoundComponent::~ActorSoundComponent() = default;

    std::expected<void, SoundError> ActorSoundComponent::Initialize(EterBase::EntityId id, EterBase::EventBus* bus) {
        if (!bus) {
            return std::unexpected(SoundError::NotInitialized);
        }
        m_entityId = id;
        m_eventBus = bus;
        return {};
    }
    
    std::string ActorSoundComponent::GetFootstepSoundPath(SurfaceType surface) const {
        switch (surface) {
            case SurfaceType::Dirt: return "sound/pc/footstep/dirt.wav";
            case SurfaceType::Stone: return "sound/pc/footstep/stone.wav";
            case SurfaceType::Wood: return "sound/pc/footstep/wood.wav";
            case SurfaceType::Water: return "sound/pc/footstep/water.wav";
            case SurfaceType::Snow: return "sound/pc/footstep/snow.wav";
            case SurfaceType::Grass: return "sound/pc/footstep/grass.wav";
            case SurfaceType::Sand: return "sound/pc/footstep/sand.wav";
            case SurfaceType::Ice: return "sound/pc/footstep/ice.wav";
            default: return "sound/pc/footstep/dirt.wav";
        }
    }
    
    std::string ActorSoundComponent::GetWeaponSoundPath(WeaponType weapon) const {
        switch (weapon) {
            case WeaponType::Unarmed: return "sound/pc/warrior/unarmed.wav";
            case WeaponType::Sword: return "sound/pc/warrior/sword.wav";
            case WeaponType::TwoHanded: return "sound/pc/warrior/twohanded.wav";
            case WeaponType::Dagger: return "sound/pc/assassin/dagger.wav";
            case WeaponType::Bow: return "sound/pc/assassin/bow.wav";
            case WeaponType::Bell: return "sound/pc/shaman/bell.wav";
            case WeaponType::Fan: return "sound/pc/shaman/fan.wav";
            default: return "sound/pc/warrior/unarmed.wav";
        }
    }

    std::string ActorSoundComponent::GetVoiceSoundPath(const std::string& type) const {
        return std::format("sound/pc/voice/{}.wav", type);
    }

    std::expected<void, SoundError> ActorSoundComponent::PlayFootstepSound(SurfaceType surface) {
        if (!m_entityId || !m_eventBus) {
            return std::unexpected(SoundError::NotInitialized);
        }
        
        ActorFootstepSoundEvent event{
            *m_entityId,
            surface,
            GetFootstepSoundPath(surface)
        };
        m_eventBus->Publish(event);
        return {};
    }
    
    std::expected<void, SoundError> ActorSoundComponent::PlayWeaponSound(WeaponType weapon) {
        if (!m_entityId || !m_eventBus) {
            return std::unexpected(SoundError::NotInitialized);
        }
        
        ActorWeaponSoundEvent event{
            *m_entityId,
            weapon,
            GetWeaponSoundPath(weapon)
        };
        m_eventBus->Publish(event);
        return {};
    }
    
    std::expected<void, SoundError> ActorSoundComponent::PlayVoiceSound(const std::string& type) {
        if (!m_entityId || !m_eventBus) {
            return std::unexpected(SoundError::NotInitialized);
        }
        
        ActorVoiceSoundEvent event{
            *m_entityId,
            GetVoiceSoundPath(type)
        };
        m_eventBus->Publish(event);
        return {};
    }

} // namespace Client::Actors

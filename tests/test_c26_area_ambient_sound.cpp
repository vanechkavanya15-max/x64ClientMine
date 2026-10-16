#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#ifndef TEST_MODE_DISABLE_STDAFX
#define TEST_MODE_DISABLE_STDAFX
#endif

#include "doctest.h"
#include "Client/World/AreaAmbientSoundManager.h"
#include "Client/Core/StrongTypes.h"

using namespace Client::World;
using namespace Client::Core;

TEST_CASE("AreaAmbientSoundManager - Registration") {
    AreaAmbientSoundManager manager;
    
    SUBCASE("Successful registration") {
        auto result = manager.RegisterSound(AmbientSoundId(1), "sound1.wav", MapCoords{0,0,0}, 100.0f, 1.0f);
        CHECK(result.has_value() == true);
        
        auto record = manager.GetSoundRecord(AmbientSoundId(1));
        CHECK(record.has_value() == true);
        CHECK(record->filePath == "sound1.wav");
    }
    
    SUBCASE("Invalid ID") {
        auto result = manager.RegisterSound(AmbientSoundId(0), "sound1.wav", MapCoords{0,0,0}, 100.0f, 1.0f);
        CHECK(result.has_value() == false);
        CHECK(result.error() == AmbientSoundError::InvalidId);
    }
    
    SUBCASE("Invalid Volume") {
        auto result = manager.RegisterSound(AmbientSoundId(1), "sound1.wav", MapCoords{0,0,0}, 100.0f, 1.5f);
        CHECK(result.has_value() == false);
        CHECK(result.error() == AmbientSoundError::InvalidVolume);
        
        auto result2 = manager.RegisterSound(AmbientSoundId(1), "sound1.wav", MapCoords{0,0,0}, 100.0f, -0.5f);
        CHECK(result2.has_value() == false);
        CHECK(result2.error() == AmbientSoundError::InvalidVolume);
    }
    
    SUBCASE("Invalid Radius") {
        auto result = manager.RegisterSound(AmbientSoundId(1), "sound1.wav", MapCoords{0,0,0}, 0.0f, 1.0f);
        CHECK(result.has_value() == false);
        CHECK(result.error() == AmbientSoundError::InvalidRadius);
    }
    
    SUBCASE("Duplicate Registration") {
        manager.RegisterSound(AmbientSoundId(1), "sound1.wav", MapCoords{0,0,0}, 100.0f, 1.0f);
        auto result = manager.RegisterSound(AmbientSoundId(1), "sound2.wav", MapCoords{1,1,1}, 50.0f, 0.5f);
        CHECK(result.has_value() == false);
        CHECK(result.error() == AmbientSoundError::AlreadyExists);
    }
}

TEST_CASE("AreaAmbientSoundManager - Unregistration") {
    AreaAmbientSoundManager manager;
    manager.RegisterSound(AmbientSoundId(1), "sound1.wav", MapCoords{0,0,0}, 100.0f, 1.0f);
    
    SUBCASE("Successful unregistration") {
        auto result = manager.UnregisterSound(AmbientSoundId(1));
        CHECK(result.has_value() == true);
        
        auto record = manager.GetSoundRecord(AmbientSoundId(1));
        CHECK(record.has_value() == false);
    }
    
    SUBCASE("NotFound unregistration") {
        auto result = manager.UnregisterSound(AmbientSoundId(999));
        CHECK(result.has_value() == false);
        CHECK(result.error() == AmbientSoundError::NotFound);
    }
}

TEST_CASE("AreaAmbientSoundManager - Attenuation and Range") {
    AreaAmbientSoundManager manager;
    manager.RegisterSound(AmbientSoundId(1), "sound1.wav", MapCoords{0,0,0}, 100.0f, 1.0f);
    manager.RegisterSound(AmbientSoundId(2), "sound2.wav", MapCoords{50,0,0}, 50.0f, 0.8f);
    
    SUBCASE("Volume calculation at center") {
        auto vol = manager.CalculateAttenuatedVolume(AmbientSoundId(1), MapCoords{0,0,0});
        CHECK(vol.has_value() == true);
        CHECK(*vol == doctest::Approx(1.0f));
    }
    
    SUBCASE("Volume calculation at halfway") {
        auto vol = manager.CalculateAttenuatedVolume(AmbientSoundId(1), MapCoords{50,0,0});
        CHECK(vol.has_value() == true);
        CHECK(*vol == doctest::Approx(0.5f));
    }
    
    SUBCASE("Volume calculation out of range") {
        auto vol = manager.CalculateAttenuatedVolume(AmbientSoundId(1), MapCoords{100,0,0});
        CHECK(vol.has_value() == true);
        CHECK(*vol == doctest::Approx(0.0f));
        
        auto vol2 = manager.CalculateAttenuatedVolume(AmbientSoundId(1), MapCoords{150,0,0});
        CHECK(vol2.has_value() == true);
        CHECK(*vol2 == doctest::Approx(0.0f));
    }
    
    SUBCASE("Active sounds in range") {
        auto sounds_at_origin = manager.GetActiveSoundsInRange(MapCoords{0,0,0});
        CHECK(sounds_at_origin.size() == 1);
        CHECK(sounds_at_origin[0] == AmbientSoundId(1));
        
        auto sounds_at_40 = manager.GetActiveSoundsInRange(MapCoords{40,0,0});
        CHECK(sounds_at_40.size() == 2);
        
        auto sounds_at_200 = manager.GetActiveSoundsInRange(MapCoords{200,0,0});
        CHECK(sounds_at_200.size() == 0);
    }
}

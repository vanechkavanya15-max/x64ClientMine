#pragma once
#include "EterBase/Singleton.h"
#include "Type.h"
#include "MaSoundInstance.h"

#include <string>
#include <vector>

struct SoundFile
{
	std::string name;
	std::vector<std::byte> buffer;
};

class SoundEngine : public CSingleton<SoundEngine>
{
public:
	enum ESoundConfig
	{
		SOUND_INSTANCE_3D_MAX_NUM = 32,
	};

public:
	SoundEngine() = default;
	~SoundEngine() = default;

	bool Initialize() { return true; }

	void SetSoundVolume(float volume) {}

	bool PlaySound2D(const std::string& name) { return true; }

	MaSoundInstance* PlaySound3D(const std::string& name, float fx, float fy, float fz) { return nullptr; }

	MaSoundInstance* PlayAmbienceSound3D(float fx, float fy, float fz, const std::string& name, int loopCount = 1) { return nullptr; }

	void StopAllSound3D() {}

	void UpdateSoundInstance(float fx, float fy, float fz, uint32_t dwcurFrame, const NSound::TSoundInstanceVector* c_pSoundInstanceVector, bool checkFrequency = false) {}

	bool FadeInMusic(const std::string& path, float targetVolume = 1.0f, float fadeInDurationSecondsFromMin = 1.5f) { return true; }

	void FadeOutMusic(const std::string& name, float targetVolume = 0.0f, float fadeOutDurationSecondsFromMax = 1.5f) {}

	void FadeOutAllMusic() {}

	void SetMusicVolume(float volume) {}

	float GetMusicVolume() const { return 0.0f; }

	void SaveVolume(bool isMinimized) {}

	void RestoreVolume() {}

	void SetMasterVolume(float volume) {}

	void SetListenerPosition(float x, float y, float z) {}

	void SetListenerOrientation(float forwardX, float forwardY, float forwardZ,
								float upX, float upY, float upZ) {}

	void Update() {}
};

#pragma once
#include <string>
#include <vector>

inline constexpr float CS_CLIENT_FPS = 61.0f;

class MaSoundInstance
{
public:
	void Destroy() {}
	bool IsInitialized() const { return false; }
	bool IsPlaying() const { return false; }
	bool Play() { return true; }
	bool Resume() { return true; }
	bool Stop() { return true; }
	void Loop() {}
	float GetVolume() const { return 0.0f; }
	void SetVolume(float) {}
	void SetPitch(float) {}
	void SetPosition(float, float, float) {}
	const std::string& GetIdentity() const { static std::string s_empty; return s_empty; }
	void Config3D(bool, float = 100.0f, float = 4000.0f) {}
	void Fade(float, float) {}
	void StopFading() {}
	bool IsFading() const { return false; }
	void Update() {}
};

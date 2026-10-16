#pragma once

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include <vector>
#include <cstdint>

class CEffectParticleBatcher
{
public:
	static CEffectParticleBatcher& Instance();

	void BeginFrame() noexcept;
	void AddParticle(uint32_t dwEffectID, uintptr_t dwTextureID, uint32_t dwBlendMode, const TParticleInstanceData& data);
	void FlushToRenderQueue();
	void Clear() noexcept;

private:
	CEffectParticleBatcher();
	~CEffectParticleBatcher() = default;
	CEffectParticleBatcher(const CEffectParticleBatcher&) = delete;
	CEffectParticleBatcher& operator=(const CEffectParticleBatcher&) = delete;

	struct BatchGroup
	{
		uint32_t dwBlendMode{ 0 };
		uintptr_t dwTextureID{ 0 };
		uint32_t dwEffectID{ 0 };
		std::vector<TParticleInstanceData> particles;
	};

	std::vector<BatchGroup> m_groups;
};

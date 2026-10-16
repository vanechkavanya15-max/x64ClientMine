#pragma once

#include "EterLib/RenderPacketTypes.h"
#include "EterLib/RenderQueueManager.h"
#include <vector>
#include <map>
#include <cstdint>

class CEffectParticleBatcher
{
public:
	static CEffectParticleBatcher& Instance();

	void BeginFrame();
	void AddParticle(uint32_t dwEffectID, uint32_t dwTextureID, uint32_t dwBlendMode, const TParticleInstanceData& data);
	void FlushToRenderQueue();
	void Clear();

private:
	CEffectParticleBatcher();
	~CEffectParticleBatcher() = default;
	CEffectParticleBatcher(const CEffectParticleBatcher&) = delete;
	CEffectParticleBatcher& operator=(const CEffectParticleBatcher&) = delete;

	struct BatchKey
	{
		uint32_t dwBlendMode;
		uint32_t dwTextureID;
		uint32_t dwEffectID;

		constexpr bool operator<(const BatchKey& other) const noexcept
		{
			if (dwBlendMode != other.dwBlendMode)
				return dwBlendMode < other.dwBlendMode;
			if (dwTextureID != other.dwTextureID)
				return dwTextureID < other.dwTextureID;
			return dwEffectID < other.dwEffectID;
		}
	};

	std::map<BatchKey, std::vector<TParticleInstanceData>> m_batchMap;
};

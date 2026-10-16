#include "StdAfx.h"
#include "EffectParticleBatcher.h"
#include <cstring>

CEffectParticleBatcher::CEffectParticleBatcher()
{
}

CEffectParticleBatcher& CEffectParticleBatcher::Instance()
{
	static CEffectParticleBatcher s_instance;
	return s_instance;
}

void CEffectParticleBatcher::BeginFrame()
{
	Clear();
}

void CEffectParticleBatcher::Clear()
{
	m_batchMap.clear();
}

void CEffectParticleBatcher::AddParticle(uint32_t dwEffectID, uint32_t dwTextureID, uint32_t dwBlendMode, const TParticleInstanceData& data)
{
	BatchKey key{ dwBlendMode, dwTextureID, dwEffectID };
	m_batchMap[key].push_back(data);
}

void CEffectParticleBatcher::FlushToRenderQueue()
{
	if (m_batchMap.empty())
		return;

	CRenderQueueManager& rqm = CRenderQueueManager::Instance();

	for (auto& [key, particles] : m_batchMap)
	{
		if (particles.empty())
			continue;

		const size_t count = particles.size();
		TParticleInstanceData* pContinuousBuffer = rqm.AllocateParticleData(count);
		if (!pContinuousBuffer)
			continue;

		std::memcpy(pContinuousBuffer, particles.data(), count * sizeof(TParticleInstanceData));

		TEffectInstancePacket packet{};
		packet.dwEffectID = key.dwEffectID;
		packet.dwTextureID = key.dwTextureID;
		packet.dwBlendMode = key.dwBlendMode;
		packet.dwParticleCount = static_cast<uint32_t>(count);
		packet.pParticles = pContinuousBuffer;

		rqm.SubmitEffect(packet);
	}

	m_batchMap.clear();
}

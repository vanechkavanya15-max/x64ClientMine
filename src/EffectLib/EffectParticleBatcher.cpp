#include "StdAfx.h"
#include "EffectParticleBatcher.h"
#include <cstring>

CEffectParticleBatcher::CEffectParticleBatcher()
{
	m_groups.reserve(64);
}

CEffectParticleBatcher& CEffectParticleBatcher::Instance()
{
	static CEffectParticleBatcher s_instance;
	return s_instance;
}

void CEffectParticleBatcher::BeginFrame() noexcept
{
	Clear();
}

void CEffectParticleBatcher::Clear() noexcept
{
	for (auto& group : m_groups)
	{
		group.particles.clear();
	}
}

void CEffectParticleBatcher::AddParticle(uint32_t dwEffectID, uintptr_t dwTextureID, uint32_t dwBlendMode, const TParticleInstanceData& data)
{
	for (auto& group : m_groups)
	{
		if (group.dwBlendMode == dwBlendMode &&
			group.dwTextureID == dwTextureID &&
			group.dwEffectID == dwEffectID)
		{
			group.particles.push_back(data);
			return;
		}
	}

	for (auto& group : m_groups)
	{
		if (group.particles.empty())
		{
			group.dwBlendMode = dwBlendMode;
			group.dwTextureID = dwTextureID;
			group.dwEffectID = dwEffectID;
			group.particles.push_back(data);
			return;
		}
	}

	BatchGroup newGroup{};
	newGroup.dwBlendMode = dwBlendMode;
	newGroup.dwTextureID = dwTextureID;
	newGroup.dwEffectID = dwEffectID;
	newGroup.particles.reserve(256);
	newGroup.particles.push_back(data);
	m_groups.push_back(std::move(newGroup));
}

void CEffectParticleBatcher::FlushToRenderQueue()
{
	CRenderQueueManager& rqm = CRenderQueueManager::Instance();

	for (auto& group : m_groups)
	{
		if (group.particles.empty())
			continue;

		const size_t count = group.particles.size();
		TParticleInstanceData* pContinuousBuffer = rqm.AllocateParticleData(count);
		if (pContinuousBuffer)
		{
			std::memcpy(pContinuousBuffer, group.particles.data(), count * sizeof(TParticleInstanceData));

			TEffectInstancePacket packet{};
			packet.dwEffectID = group.dwEffectID;
			packet.dwTextureID = group.dwTextureID;
			packet.dwBlendMode = group.dwBlendMode;
			packet.dwParticleCount = static_cast<uint32_t>(count);
			packet.pParticles = pContinuousBuffer;

			rqm.SubmitEffect(packet);
		}

		// Czyscimy tylko rozmiar, zachowujac zaalokowana pojemnosc wektora (zero alokacji w kolejnych klatkach)
		group.particles.clear();
	}
}

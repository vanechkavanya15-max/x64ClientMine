#include "StdAfx.h"
#include "Motion.h"
#include "EterModelLib/GltfTypes.h"

CGrannyMotion::CGrannyMotion()
{
	Initialize();
}

CGrannyMotion::~CGrannyMotion()
{
	Destroy();
}

bool CGrannyMotion::IsEmpty()
{
	return (m_pGltfMotion == NULL && m_pgrnAni == NULL);
}

void CGrannyMotion::Destroy()
{
	Initialize();
}

void CGrannyMotion::Initialize()
{
	m_pgrnAni = NULL;
	m_pGltfMotion = NULL;
}

bool CGrannyMotion::BindGrannyAnimation(granny_animation * pgrnAni)
{
	assert(IsEmpty());
	m_pgrnAni = pgrnAni;
	return true;
}

bool CGrannyMotion::BindGltfAnimation(const GltfMotionData * pGltfMotion)
{
	assert(IsEmpty());

	m_pGltfMotion = pGltfMotion;
	return true;
}

granny_animation* CGrannyMotion::GetGrannyAnimationPointer() const
{
	return m_pgrnAni;
}

const GltfMotionData* CGrannyMotion::GetGltfAnimationPointer() const
{
	return m_pGltfMotion;
}

const char * CGrannyMotion::GetName() const
{
	if (m_pGltfMotion)
		return m_pGltfMotion->name.c_str();

	if (m_pgrnAni)
		return m_pgrnAni->Name;

	return "";
}

float CGrannyMotion::GetDuration() const
{
	if (m_pGltfMotion)
		return m_pGltfMotion->duration;

	if (m_pgrnAni)
		return m_pgrnAni->Duration;

	return 0.0f;
}

void CGrannyMotion::GetTextTrack(const char * c_szTextTrackName, int * pCount, float * pArray) const
{
	if (m_pGltfMotion)
	{
		for (size_t i = 0; i < m_pGltfMotion->events.size(); ++i)
		{
			const auto& ev = m_pGltfMotion->events[i];
			if (!_stricmp(c_szTextTrackName, ev.type.c_str()))
			{
				pArray[(*pCount)++] = ev.time;
			}
		}
	}
	else if (m_pgrnAni && m_pgrnAni->TrackGroups)
	{
		for (int tg = 0; tg < m_pgrnAni->TrackGroupCount; ++tg)
		{
			granny_track_group* pGroup = m_pgrnAni->TrackGroups[tg];
			if (!pGroup)
				continue;

			for (int i = 0; i < pGroup->TextTrackCount; ++i)
			{
				granny_text_track & rTextTrack = pGroup->TextTracks[i];
				if (!_stricmp(c_szTextTrackName, rTextTrack.Name))
				{
					for (int j = 0; j < rTextTrack.EntryCount; ++j)
					{
						pArray[(*pCount)++] = rTextTrack.Entries[j].TimeStamp;
					}
				}
			}
		}
	}
}



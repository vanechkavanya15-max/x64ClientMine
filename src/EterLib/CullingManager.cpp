#include "StdAfx.h"
#include "CullingManager.h"
#include "GrpObjectInstance.h"
#include "RenderConventions.h"

//#define COUNT_SHOWING_SPHERE

#ifdef COUNT_SHOWING_SPHERE
int showingcount = 0;
#endif

void CCullingManager::RayTraceCallback(const Vector3d &p1,          // source pos of ray
							  const Vector3d &/*dir*/,          // dest pos of ray
							  float /*distance*/,
							  const Vector3d &sect,
							  SpherePack *sphere)
{
	float actualDist = p1.Distance(sect);
	if (m_RayFarDistance<=0.0f || m_RayFarDistance>=actualDist)
	{
#ifdef SPHERELIB_STRICT
		if (sphere->IS_SPHERE)
			puts("CCullingManager::RayTraceCallback");
#endif		
		m_list.push_back(static_cast<CGraphicObjectInstance *>(sphere->GetUserData()));
	}
}


void CCullingManager::VisibilityCallback(const Frustum &/*f*/,SpherePack *sphere,ViewState state)
{
#ifdef SPHERELIB_STRICT
		if (sphere->IS_SPHERE)
			puts("CCullingManager::VisibilityCallback");
#endif

	auto * pInstance = static_cast<CGraphicObjectInstance*>(sphere->GetUserData());
	if (state == VS_OUTSIDE)
	{
#ifdef COUNT_SHOWING_SPHERE
		if (pInstance->isShow())
		{
			Tracef("SH : %p  ",sphere->GetUserData());
			showingcount--;
			Tracef("show size : %5d\n",showingcount);
		}

#endif
		pInstance->Hide();
	}
	else
	{
#ifdef COUNT_SHOWING_SPHERE
		if (!pInstance->isShow())
		{
			Tracef("HS : %p  ",sphere->GetUserData());
			showingcount++;
			Tracef("show size : %5d\n",showingcount);
		}
#endif
		pInstance->Show();
	}
}

void CCullingManager::RangeTestCallback(const Vector3d &/*p*/,float /*distance*/,SpherePack *sphere,ViewState state)
{
#ifdef SPHERELIB_STRICT
		if (sphere->IS_SPHERE)
			puts("CCullingManager::RangeTestCallback");
#endif
	if (state!=VS_OUTSIDE)
	{
		m_list.push_back(static_cast<CGraphicObjectInstance *>(sphere->GetUserData()));
	}
}

void CCullingManager::Reset()
{
	m_Factory->Reset();
}

void CCullingManager::Update()
{
	m_Factory->Process();
}

void CCullingManager::Process()
{
	UpdateViewMatrix();
	UpdateProjMatrix();
	BuildViewFrustum();
	m_Factory->FrustumTest(GetFrustum(), this);
}

CCullingManager::CullingHandle CCullingManager::Register(CGraphicObjectInstance * obj)
{
	assert(obj);
#ifdef COUNT_SHOWING_SPHERE
	Tracef("CR : %p  ",obj);
	showingcount++;
	Tracef("show size : %5d\n",showingcount);
#endif
	Vector3d center;
	float radius;
	obj->GetBoundingSphere(center,radius);
	return m_Factory->AddSphere_(center,radius,obj, false);
}

void CCullingManager::Unregister(CullingHandle h)
{
#ifdef COUNT_SHOWING_SPHERE
	if (static_cast<CGraphicObjectInstance*>(h->GetUserData())->isShow())
	{
		Tracef("DE : %p  ",h->GetUserData());
		showingcount--;
		Tracef("show size : %5d\n",showingcount);
	}
#endif
	m_Factory->Remove(h);
}

CCullingManager::CCullingManager()
	: m_RayFarDistance(kDefaultCullingRayFarDistance)
{
	m_Factory = std::make_unique<SpherePackFactory>(
		kDefaultCullingMaxSpheres,	// maximum count
		kDefaultCullingRootRadius,	// root radius
		kDefaultCullingRadius,		// leaf radius
		kDefaultCullingExtraRadius	// extra radius
		);
}

CCullingManager::~CCullingManager() = default;

void CCullingManager::FindRange(const Vector3d &p, float radius)
{
	m_list.clear();
	// reserve to avoid reallocations
	m_list.reserve(256);
	m_Factory->RangeTest(p, radius, this);
}

void CCullingManager::FindRay(const Vector3d &p1, const Vector3d &dir)
{
	m_RayFarDistance = kDefaultCullingRayFarDistance;
	m_list.clear();
	m_list.reserve(64);
	m_Factory->RayTrace(p1,dir,this);
}

void CCullingManager::FindRayDistance(const Vector3d &p1, const Vector3d &dir, float distance)
{
	m_RayFarDistance = distance;
	m_list.clear();
	m_list.reserve(64);
	m_Factory->RayTrace(p1, dir * distance, this);
}

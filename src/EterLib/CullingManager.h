#pragma once

#include "GrpScreen.h"
#include "RenderConventions.h"

#include "Eterbase/Singleton.h"
#include "SphereLib/spherepack.h"
#include <memory>
#include <vector>

class CGraphicObjectInstance;
template <class T>
struct RangeTester : public SpherePackCallback
{
	T * f;
	float dist;
	RangeTester(T * fn, float distance=-1)
		: f(fn), dist(distance)
	{}

	virtual ~RangeTester() = default;


	void RayTraceCallback(const Vector3d &p1,          // source pos of ray
		const Vector3d &dir,          // dest pos of ray
		float distance,
		const Vector3d &sect,
		SpherePack *sphere) override
	{
#ifdef SPHERELIB_STRICT
		if (sphere->IS_SPHERE)
			puts("RangeTester::RayTraceCallback");
#endif	
		float actualDist = p1.Distance(sect);
		if (dist<=0.0f || dist>=actualDist)
			(*f)(static_cast<CGraphicObjectInstance *>(sphere->GetUserData()));
	};
	
	
	void VisibilityCallback(const Frustum &f,SpherePack *sphere,ViewState state) override {};

	void RangeTestCallback(const Vector3d &p,float distance,SpherePack *sphere,ViewState state) override
	{
#ifdef SPHERELIB_STRICT
		if (sphere->IS_SPHERE)
			puts("RangeTester::RangeTestCallback");
#endif	
		if (state!=VS_OUTSIDE)
			(*f)(static_cast<CGraphicObjectInstance *>(sphere->GetUserData()));
	}

	void PointTest2dCallback(const Vector3d &p, SpherePack *sphere,ViewState state) override
	{
#ifdef SPHERELIB_STRICT
		if (sphere->IS_SPHERE)
			puts("RangeTester::PointTest2dCallback");	
#endif
		if (state!=VS_OUTSIDE)
		{
#ifdef SPHERELIB_STRICT
			puts("FIND!!");
#endif
			(*f)(static_cast<CGraphicObjectInstance *>(sphere->GetUserData()));
		}
	}
};

class CCullingManager : public CSingleton<CCullingManager>, public SpherePackCallback, private CScreen
{
public:
	static constexpr int   kDefaultMaxSpheres = kDefaultCullingMaxSpheres;
	static constexpr float kDefaultRootRadius = kDefaultCullingRootRadius;
	static constexpr float kDefaultLeafRadius = kDefaultCullingRadius;
	static constexpr float kDefaultExtraRadius = kDefaultCullingExtraRadius;
	static constexpr float kDefaultRayFarDistance = kDefaultCullingRayFarDistance;

	typedef SpherePack * CullingHandle;
	typedef std::vector<CGraphicObjectInstance *> TRangeList;

	CCullingManager();
	~CCullingManager() override;

	CCullingManager(const CCullingManager&) = delete;
	CCullingManager& operator=(const CCullingManager&) = delete;

	void RayTraceCallback(const Vector3d &p1,          // source pos of ray
		const Vector3d &dir,          // dest pos of ray
		float distance,
		const Vector3d &sect,
		SpherePack *sphere) override;
	
	
	void VisibilityCallback(const Frustum &f,SpherePack *sphere,ViewState state) override;
	
	void RangeTestCallback(const Vector3d &p,float distance,SpherePack *sphere,ViewState state) override;	

	void Reset();
	void Update();
	void Process();
	
	void FindRange(const Vector3d &p, float radius);
	void FindRay(const Vector3d &p1, const Vector3d &dir);
	void FindRayDistance(const Vector3d &p1, const Vector3d &dir, float distance);

	void RangeTest(const Vector3d& p, float radius, SpherePackCallback* callback)
	{
		m_Factory->RangeTest(p, radius, callback);
	}

	void PointTest2d(const Vector3d& p, SpherePackCallback* callback)
	{
		m_Factory->PointTest2d(p, callback);
	}

	template <class T>
	void ForInRange2d(const Vector3d& p, T* pFunc)
	{
		RangeTester<T> r(pFunc);
		m_Factory->PointTest2d(p, &r);
	}

	template <class T>
	void ForInRange(const Vector3d &p, float radius, T* pFunc)
	{
		RangeTester<T> r(pFunc);
		m_Factory->RangeTest(p, radius, &r/*this*/);	
	}

	template <class T>
	void ForInRay(const Vector3d &p1, const Vector3d &dir, T* pFunc)
	{
		RangeTester<T> r(pFunc);
		m_Factory->RayTrace(p1, dir, &r/*this*/);		
	}

	template <class T>
	void ForInRayDistance(const Vector3d &p, const Vector3d &dir, float distance, T* pFunc)
	{
		RangeTester<T> r(pFunc, distance);
		m_Factory->RayTrace(p, dir, &r/*this*/);		
	}

	CullingHandle Register(CGraphicObjectInstance * ob);
	void Unregister(CullingHandle h);

	TRangeList::iterator begin() { return m_list.begin(); }
	TRangeList::iterator end() { return m_list.end(); }

protected:
	TRangeList m_list;

	float m_RayFarDistance;

	std::unique_ptr<SpherePackFactory> m_Factory;
};

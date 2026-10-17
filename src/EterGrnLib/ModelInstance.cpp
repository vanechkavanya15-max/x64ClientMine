#include "StdAfx.h"
#include "ModelInstance.h"
#include "Model.h"
#include "EterLib/ResourceManager.h"


CGrannyModel* CGrannyModelInstance::GetModel()
{
	return m_pModel;
}

void CGrannyModelInstance::SetMaterialImagePointer(const char* c_szImageName, CGraphicImage* pImage)
{
	m_kMtrlPal.SetMaterialImagePointer(c_szImageName, pImage);
}

void CGrannyModelInstance::SetMaterialData(const char* c_szImageName, const SMaterialData& c_rkMaterialData)
{
	m_kMtrlPal.SetMaterialData(c_szImageName, c_rkMaterialData);

	material_data_ = c_rkMaterialData;
}

void CGrannyModelInstance::SetSpecularInfo(const char* c_szMtrlName, BOOL bEnable, float fPower)
{
	m_kMtrlPal.SetSpecularInfo(c_szMtrlName, bEnable, fPower);
}

void CGrannyModelInstance::SetLocalTime(float fLocalTime)
{
	m_fLocalTime = fLocalTime;
}

int CGrannyModelInstance::ResetLocalTime()
{
	m_fLocalTime = 0.0f;
	return 0;
}

float CGrannyModelInstance::GetLocalTime()
{
	return m_fLocalTime;
}

void CGrannyModelInstance::SetParentModelInstance(const CGrannyModelInstance* c_pParentModelInstance, const char * c_szBoneName)
{
	int iBoneIndex;
	if (!c_pParentModelInstance->GetBoneIndexByName(c_szBoneName, &iBoneIndex))
		return;

	SetParentModelInstance(c_pParentModelInstance, iBoneIndex);
}

void CGrannyModelInstance::SetParentModelInstance(const CGrannyModelInstance* c_pParentModelInstance, int iBone)
{
	mc_pParentInstance = c_pParentModelInstance;
	m_iParentBoneIndex = iBone;
}

const D3DXMATRIX * CGrannyModelInstance::GetCompositeBoneMatrices() const
{
	granny_world_pose* pWorldPose = __GetWorldPosePtr();
	if (!pWorldPose)
		return NULL;

	return reinterpret_cast<const D3DXMATRIX*>(GrannyGetWorldPoseComposite4x4Array(pWorldPose));
}

int CGrannyModelInstance::GetBoneCount() const
{
	if (!m_pgrnModelInstance)
		return 0;

	granny_skeleton* pSkeleton = GrannyGetSourceSkeleton(m_pgrnModelInstance);
	return pSkeleton ? pSkeleton->BoneCount : 0;
}


void CGrannyModelInstance::LinkSkeleton(CGrannyModelInstance** ppkSkeletonInst)
{
	if (ppkSkeletonInst == m_ppkSkeletonInst)
		return;

	m_ppkSkeletonInst = ppkSkeletonInst;
	if (ppkSkeletonInst && *ppkSkeletonInst && *ppkSkeletonInst != this)
	{
		if (m_pgrnWorldPoseReal)
		{
			GrannyFreeWorldPose(m_pgrnWorldPoseReal);
			m_pgrnWorldPoseReal = NULL;
		}
		__DestroyMeshBindingVector();
		__CreateMeshBindingVector(*ppkSkeletonInst);
	}
}

void CGrannyModelInstance::UnlinkSkeleton()
{
	m_ppkSkeletonInst = NULL;
	if (!m_pgrnWorldPoseReal && m_pgrnModelInstance)
	{
		granny_skeleton* pgrnSkeleton = GrannyGetSourceSkeleton(m_pgrnModelInstance);
		if (pgrnSkeleton)
			m_pgrnWorldPoseReal = GrannyNewWorldPose(pgrnSkeleton->BoneCount);
		__DestroyMeshBindingVector();
		__CreateMeshBindingVector(NULL);
	}
}

bool CGrannyModelInstance::IsLinkedSkeleton() const
{
	return m_ppkSkeletonInst != NULL && *m_ppkSkeletonInst != NULL && *m_ppkSkeletonInst != this;
}

CGrannyModelInstance* CGrannyModelInstance::GetSkeletonInstance() const
{
	if (m_ppkSkeletonInst && *m_ppkSkeletonInst != this)
		return *m_ppkSkeletonInst;
	return NULL;
}

const CGrannyModelInstance* CGrannyModelInstance::GetRootSkeletonInstance() const
{
	const CGrannyModelInstance* pCurrent = this;
	int depth = 0;
	while (pCurrent && pCurrent->m_ppkSkeletonInst && *pCurrent->m_ppkSkeletonInst && 
	       *pCurrent->m_ppkSkeletonInst != pCurrent && depth < 8)
	{
		pCurrent = *pCurrent->m_ppkSkeletonInst;
		++depth;
	}
	return pCurrent;
}

void CGrannyModelInstance::CheckSkeletonLink()
{
	if (!m_ppkSkeletonInst || !*m_ppkSkeletonInst || *m_ppkSkeletonInst == this)
		return;

	if (m_pgrnWorldPoseReal)
	{
		GrannyFreeWorldPose(m_pgrnWorldPoseReal);
		m_pgrnWorldPoseReal = NULL;
		__DestroyMeshBindingVector();
		__CreateMeshBindingVector(*m_ppkSkeletonInst);
	}
}

const D3DXMATRIX * CGrannyModelInstance::GetActiveMeshMatrices() const
{
	uint8_t frontIndex = m_activeTransformBuffer.load(std::memory_order_acquire);
	if (frontIndex >= TRANSFORM_BUFFER_COUNT)
		frontIndex = 0;
	return m_meshMatrices[frontIndex];
}

bool CGrannyModelInstance::IsEmpty()
{
	if (m_pModel)
	{
		if (m_pModel->GetMeshCount() <= 0)
			return true;
		// NO_MESH_BUG_FIX
		if (!m_meshMatrices[0] && !m_meshMatrices[1])
			return true;
		// END_OF_NO_MESH_BUG_FIX
		return false;
	}

	return true;
}

bool CGrannyModelInstance::CreateDeviceObjects()
{	
	__CreateDynamicVertexBuffer();

	return true;
}

void CGrannyModelInstance::DestroyDeviceObjects()
{
	__DestroyDynamicVertexBuffer();
}

void CGrannyModelInstance::__Initialize()
{
	if (m_pModel)
	{
		m_pModel->Release();
	}

	m_pModel = NULL;
	mc_pParentInstance = NULL;
	m_iParentBoneIndex = 0;
	m_pCustomParentBoneMatrix = NULL;

	m_pgrnModelInstance = NULL;	

	// WORK
	m_pgrnWorldPoseReal = NULL;	
	// END_OF_WORK

	// TEST
	// m_pgrnWorldPose = NULL;
	m_ppkSkeletonInst = NULL;
	// END_OF_TEST

	// Zachowujemy bufory m_meshMatrices, jesli byly zaalokowane (re-use bez alokacji na stercie)
	m_meshMatrixCount = 0;
	m_activeTransformBuffer.store(0, std::memory_order_relaxed);
	m_localPose.Reset();

	m_pgrnCtrl = NULL;
	m_pgrnAni = NULL;

	// MR-12: Fix specular isolation issue
	material_data_ = {};
	m_dwOldUpdateFrame = 0;
	// MR-12: -- END OF -- Fix specular isolation issue
}

CGrannyModelInstance::CGrannyModelInstance()
{
	m_pModel = NULL;
	for (int b = 0; b < TRANSFORM_BUFFER_COUNT; ++b)
		m_meshMatrices[b] = NULL;
	m_meshMatrixCapacity = 0;
	m_meshMatrixCount = 0;
	ReserveMeshMatrices(DEFAULT_MESH_MATRIX_CAPACITY);
	__Initialize();
}

CGrannyModelInstance::~CGrannyModelInstance()
{
	Clear();
	__DestroyMeshMatrices(true);
}

CDynamicPool<CGrannyModelInstance> CGrannyModelInstance::ms_kPool;

CGrannyModelInstance* CGrannyModelInstance::New()
{
	return ms_kPool.Alloc();
}

void CGrannyModelInstance::Delete(CGrannyModelInstance* pkInst)
{
	pkInst->Clear();
	ms_kPool.Free(pkInst);
}

void CGrannyModelInstance::DestroySystem()
{
	ms_kPool.Destroy();
}

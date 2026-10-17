#include "StdAfx.h"
#include "LODController.h"
#include "EterModelLib/GltfModel.h"
#include "EterModelLib/GltfModelInstance.h"
#include "Eterlib/ResourceManager.h"
#include "Eterlib/GrpImage.h"
#include "Eterlib/StateManager.h"
#include "EterLib/RenderOwnership.h"
#include "EterLib/Render/ActorSkinningPipeline.h"
#include "Eterbase/Filename.h"
#include <algorithm>
#include <cctype>

static float LODHEIGHT_ACTOR		=	500.0f;
static float LODDISTANCE_ACTOR		=	5000.0f;
static float LODDISTANCE_BUILDING	=	25000.0f;

static const float c_fNearLodScale = 3.0f;
static const float c_fFarLodScale = 25.0f;
static const float LOD_APPLY_MAX = 2000.0f;
static const float LOD_APPLY_MIN = 500.0f;

bool ms_isMinLODModeEnable=false;

static CGraphicVertexBuffer gs_emptyVB;
static LPDIRECT3DVERTEXDECLARATION9 s_pGltfVertexDecl = NULL;

void CGrannyLODController::ReleaseGltfVertexDeclaration()
{
	if (s_pGltfVertexDecl)
	{
		s_pGltfVertexDecl->Release();
		s_pGltfVertexDecl = NULL;
	}
	EterLib::Render::ActorSkinningPipeline::Instance().Shutdown();
}

static CGraphicVertexBuffer* __AllocDeformVertexBuffer(unsigned deformableVertexCount)
{
	if (deformableVertexCount == 0)
		return &gs_emptyVB;

	unsigned capacity = (((deformableVertexCount-1) / 500) + 1) * 500;
	CGraphicVertexBuffer* pkNewVB = new CGraphicVertexBuffer;

	if (!pkNewVB->Create(
		capacity,
		D3DFVF_XYZ|D3DFVF_NORMAL|D3DFVF_TEX1,
		D3DUSAGE_DYNAMIC,
		D3DPOOL_DEFAULT))
	{
		TraceError("NEW_ERROR: capacity %d (deformableVertexCount %d)", capacity, deformableVertexCount);
	}

	return pkNewVB;
}

void __FreeDeformVertexBuffer(CGraphicVertexBuffer* pkDelVB)
{
	if (pkDelVB)
	{
		if (pkDelVB == &gs_emptyVB)
			return;

		pkDelVB->Destroy();
		delete pkDelVB;
	}
}

void CGrannyLODController::SetMinLODMode(bool isEnable)
{
	ms_isMinLODModeEnable=isEnable;
}

void CGrannyLODController::SetMaterialImagePointer(const char* c_szImageName, CGraphicImage* pImage)
{
	std::deque<CGrannyModelInstance *>::iterator i;
	for (i=m_que_pkModelInst.begin(); i!=m_que_pkModelInst.end(); ++i)
	{
		CGrannyModelInstance* pkModelInst=(*i);
		pkModelInst->SetMaterialImagePointer(c_szImageName, pImage);
	}

	if (m_pGltfModelInstance && pImage && c_szImageName)
	{
		m_gltfRenderer.SetMaterialImagePointer(m_pGltfModelInstance, c_szImageName, pImage);
	}
}

void CGrannyLODController::SetMaterialData(const char* c_szImageName, const SMaterialData& c_rkMaterialData)
{
	std::deque<CGrannyModelInstance *>::iterator i;
	for (i=m_que_pkModelInst.begin(); i!=m_que_pkModelInst.end(); ++i)
	{
		CGrannyModelInstance* pkModelInst=(*i);
		pkModelInst->SetMaterialData(c_szImageName, c_rkMaterialData);
	}

	if (c_rkMaterialData.pImage)
	{
		SetMaterialImagePointer(c_szImageName, c_rkMaterialData.pImage);
	}
}

void CGrannyLODController::SetSpecularInfo(const char* c_szMtrlName, BOOL bEnable, float fPower)
{
	std::deque<CGrannyModelInstance *>::iterator i;
	for (i=m_que_pkModelInst.begin(); i!=m_que_pkModelInst.end(); ++i)
	{
		CGrannyModelInstance* pkModelInst=(*i);
		pkModelInst->SetSpecularInfo(c_szMtrlName, bEnable, fPower);
	}
}

static std::map<uint32_t, CGrannyLODController*> s_activeLODControllers;
static CGrannyLODController* s_pCurrentActiveController = nullptr;

void CGrannyLODController::RegisterLODController(uint32_t modelId, CGrannyLODController* pController)
{
	if (modelId != 0 && pController)
	{
		s_activeLODControllers[modelId] = pController;
		pController->m_dwModelID = modelId;
	}
}

void CGrannyLODController::UnregisterLODController(uint32_t modelId)
{
	if (modelId != 0)
	{
		s_activeLODControllers.erase(modelId);
	}
}

CGrannyLODController* CGrannyLODController::GetLODController(uint32_t modelId)
{
	auto it = s_activeLODControllers.find(modelId);
	if (it != s_activeLODControllers.end())
		return it->second;
	return nullptr;
}

CGrannyLODController* CGrannyLODController::GetCurrentActiveController()
{
	return s_pCurrentActiveController;
}

void ExecuteModelDrawItemD3D9(const TModelDrawItem& item)
{
	CGrannyLODController* pController = CGrannyLODController::GetLODController(item.modelId);
	if (!pController)
		pController = CGrannyLODController::GetCurrentActiveController();

	if (pController)
	{
		pController->ExecuteModelDrawItemD3D9(item);
	}
}

CGrannyLODController::CGrannyLODController() : 
	m_pCurrentModelInstance(NULL),
	m_bLODLevel(0),
	m_dwSampleFrameCounter(0),
	m_pAttachedParentModel(NULL),
	m_fLODDistance(0.0f),
	m_dwLODAniFPS(CGrannyModelInstance::ANIFPS_MAX),
	m_pkSharedDeformableVertexBuffer(NULL),
	m_dwModelID(0),
	m_pGltfModelInstance(NULL)
	/////////////////////////////////////////////////////
{
	D3DXMatrixIdentity(&m_matWorld);
}

CGrannyLODController::~CGrannyLODController()
{
	__FreeDeformVertexBuffer(m_pkSharedDeformableVertexBuffer);

	Clear();

	if (m_pGltfModelInstance)
	{
		delete m_pGltfModelInstance;
		m_pGltfModelInstance = NULL;
	}

	if (m_dwModelID != 0)
	{
		UnregisterLODController(m_dwModelID);
	}
	if (s_pCurrentActiveController == this)
	{
		s_pCurrentActiveController = nullptr;
	}
}

void CGrannyLODController::Clear()
{
	ReleaseGltfStaticGpuBuffers();
	m_gltfRenderer.Clear();
	m_dwSampleFrameCounter = 0;

	if (m_pGltfModelInstance)
	{
		m_pGltfModelInstance->SetParentModelInstance(NULL, -1);
		m_pGltfModelInstance->SetParentBoneMatrix(NULL);
		m_pGltfModelInstance->LinkSkeleton(NULL);
		delete m_pGltfModelInstance;
		m_pGltfModelInstance = NULL;
	}

	if (m_pAttachedParentModel)
	{
		m_pAttachedParentModel->DetachModelInstance(this);
	}

	// Bezpieczne odpiecie wszystkich dzieci przed usunieciem instancji modeli nadrzednych
	for (size_t c = 0; c < m_AttachedModelDataVector.size(); ++c)
	{
		CGrannyLODController * pkChild = m_AttachedModelDataVector[c].pkLODController;
		if (pkChild)
		{
			pkChild->m_pAttachedParentModel = NULL;
			if (pkChild->m_pGltfModelInstance)
			{
				pkChild->m_pGltfModelInstance->SetParentModelInstance(NULL, -1);
				pkChild->m_pGltfModelInstance->SetParentBoneMatrix(NULL);
				pkChild->m_pGltfModelInstance->LinkSkeleton(NULL);
			}
			for (size_t lod = 0; lod < pkChild->m_que_pkModelInst.size(); ++lod)
			{
				CGrannyModelInstance * pChildInst = pkChild->m_que_pkModelInst[lod];
				if (pChildInst)
				{
					pChildInst->SetParentModelInstance(NULL, 0);
					pChildInst->SetCustomParentBoneMatrix(NULL);
					pChildInst->UnlinkSkeleton();
				}
			}
		}
	}
	m_AttachedModelDataVector.clear();

	m_pCurrentModelInstance = NULL;
	m_pAttachedParentModel = NULL;

	std::for_each(m_que_pkModelInst.begin(), m_que_pkModelInst.end(), CGrannyModelInstance::Delete);
	m_que_pkModelInst.clear();
}

void CGrannyLODController::AddModel(CGraphicThing * pThing, int iSrcModel, CGrannyLODController * pSkelLODController)
{
	if (!pThing)
		return;

	if (pThing->IsGltf())
	{
		if (m_pGltfModelInstance)
		{
			// Model glTF LOD0 juz istnieje w kontrolerze - nie zastepujemy go nizszymi LODami
			return;
		}

		m_pGltfModelInstance = new EterModelLib::CGltfModelInstance(pThing->GetGltfModelPointer());

		if (pSkelLODController != NULL && pSkelLODController->isGltfModelInstance())
		{
			m_pGltfModelInstance->SetLinkedModelInstance(pSkelLODController->GetGltfModelInstance());
		}

		InitGltfMaterials();
		SetLODLimits(0.0f, LODDISTANCE_ACTOR);
		return;
	}

	if (pSkelLODController && pSkelLODController->m_que_pkModelInst.empty())
	{
		assert(!"EMPTY SKELETON(CANNON LINK)");
		return;
	}

	assert(pThing->GetReferenceCount()>=1);

	pThing->AddReference();	
	
	if (pThing->GetModelCount() <= iSrcModel)
	{
		pThing->Release();
		return;
	}
	CGrannyModel * pModel = pThing->GetModelPointer(iSrcModel);
	if (!pModel)
	{
		pThing->Release();
		return;
	}
	
	CGrannyModelInstance * pModelInstance = CGrannyModelInstance::New();
	
	__ReserveSharedDeformableVertexBuffer(pModel->GetDeformVertexCount());

	if (pSkelLODController)
	{
		pModelInstance->SetLinkedModelPointer(pModel, m_pkSharedDeformableVertexBuffer, &pSkelLODController->m_pCurrentModelInstance);
	}
	else
	{		
		pModelInstance->SetLinkedModelPointer(pModel, m_pkSharedDeformableVertexBuffer, NULL);
	}

	// END_OF_WORK
	
	if (!m_pCurrentModelInstance)
	{
		m_pCurrentModelInstance = pModelInstance;
		pModelInstance->DeformNoSkin(&ms_matIdentity);

		D3DXVECTOR3 vtMin, vtMax;
		pModelInstance->GetBoundBox(&vtMin, &vtMax);

		float fSize = 0.0f;
		fSize = fMAX(fSize, fabs(vtMin.x - vtMax.x));
		fSize = fMAX(fSize, fabs(vtMin.y - vtMax.y));
		fSize = fMAX(fSize, fabs(vtMin.z - vtMax.z));		

		if (fSize<LODHEIGHT_ACTOR)			
			SetLODLimits(0.0f, LODDISTANCE_ACTOR);
		else
			// 
			SetLODLimits(0.0f, LODDISTANCE_BUILDING);
	}
	else
	{
		// FIXME : CModelInstance::m_pgrnWorldPose를 Update에서 사용하는데,
		//         Deform을 하지 않으면 NULL 입니다. 구조가 조금 바뀌어야 할지도.. - [levites]
 		pModelInstance->DeformNoSkin(&ms_matIdentity);
	}	

	pThing->Release();

	m_que_pkModelInst.push_front(pModelInstance);	
}



void CGrannyLODController::__ReserveSharedDeformableVertexBuffer(DWORD deformableVertexCount)
{
	if (m_pkSharedDeformableVertexBuffer && 
		m_pkSharedDeformableVertexBuffer->GetVertexCount() >= deformableVertexCount)
		return;	

	__FreeDeformVertexBuffer(m_pkSharedDeformableVertexBuffer);

	m_pkSharedDeformableVertexBuffer = __AllocDeformVertexBuffer(deformableVertexCount);
}

void CGrannyLODController::AttachModelInstance(CGrannyLODController * pSrcLODController, const char * c_szBoneName)
{
	if (!pSrcLODController)
		return;

	// 1. Doczepianie instancji Granny
	CGrannyModelInstance * pSrcInstance = pSrcLODController->GetModelInstance();
	CGrannyModelInstance * pDestInstance = GetModelInstance();
	if (pSrcInstance && pDestInstance)
	{
		pSrcInstance->SetParentModelInstance(pDestInstance, c_szBoneName);
	}

	// 2. Obsluga glTF (np. bron przymocowana do dloni Bip001 R Hand)
	if (pSrcLODController->m_pGltfModelInstance)
	{
		if (m_pGltfModelInstance)
		{
			pSrcLODController->m_pGltfModelInstance->SetParentModelInstance(m_pGltfModelInstance, c_szBoneName);
		}
		else if (pDestInstance)
		{
			int iBoneIndex = -1;
			if (pDestInstance->GetBoneIndexByName(c_szBoneName, &iBoneIndex))
			{
				const float * pBoneMat = pDestInstance->GetBoneMatrixPointer(iBoneIndex);
				if (pBoneMat)
					pSrcLODController->m_pGltfModelInstance->SetParentBoneMatrix((const EterModelLib::Matrix4x4*)pBoneMat);
			}
		}
	}

	// Link Parent Data
	pSrcLODController->m_pAttachedParentModel = this;

	// Link Child Data
	std::vector<TAttachingModelData>::iterator itor = m_AttachedModelDataVector.begin();
	for (; m_AttachedModelDataVector.end() != itor;)
	{
		TAttachingModelData & rData = *itor;
		if (pSrcLODController == rData.pkLODController)
		{
			itor = m_AttachedModelDataVector.erase(itor);
		}
		else
		{
			++itor;
		}
	}

	TAttachingModelData AttachingModelData;
	AttachingModelData.pkLODController = pSrcLODController;
	AttachingModelData.strBoneName = c_szBoneName;
	m_AttachedModelDataVector.push_back(AttachingModelData);
}

void CGrannyLODController::DetachModelInstance(CGrannyLODController * pSrcLODController)
{
	if (!pSrcLODController)
		return;

	if (pSrcLODController->m_pGltfModelInstance)
	{
		pSrcLODController->m_pGltfModelInstance->SetParentModelInstance(NULL, -1);
		pSrcLODController->m_pGltfModelInstance->SetParentBoneMatrix(NULL);
		pSrcLODController->m_pGltfModelInstance->LinkSkeleton(NULL);
	}

	// Bezpieczne odpiecie wskaznika instancji rodzica we wszystkich poziomach LOD instancji dziecka
	for (size_t lod = 0; lod < pSrcLODController->m_que_pkModelInst.size(); ++lod)
	{
		CGrannyModelInstance * pInst = pSrcLODController->m_que_pkModelInst[lod];
		if (pInst)
		{
			pInst->SetParentModelInstance(NULL, 0);
			pInst->SetCustomParentBoneMatrix(NULL);
			pInst->UnlinkSkeleton();
		}
	}

	CGrannyModelInstance * pSrcInstance = pSrcLODController->GetModelInstance();
	if (pSrcInstance)
	{
		pSrcInstance->SetParentModelInstance(NULL, 0);
		pSrcInstance->SetCustomParentBoneMatrix(NULL);
		pSrcInstance->UnlinkSkeleton();
	}

	// Unlink Child Data
	std::vector<TAttachingModelData>::iterator itor = m_AttachedModelDataVector.begin();
	for (; m_AttachedModelDataVector.end() != itor;)
	{
		TAttachingModelData & rData = *itor;
		if (pSrcLODController == rData.pkLODController)
		{
			itor = m_AttachedModelDataVector.erase(itor);
		}
		else
		{
			++itor;
		}
	}

	// Unlink Parent Data
	pSrcLODController->m_pAttachedParentModel = NULL;
}

void CGrannyLODController::SetLODLimits(float /*fNearLOD*/, float fFarLOD)
{
	m_fLODDistance = fFarLOD;
}

void CGrannyLODController::SetLODLevel(BYTE bLodLevel)
{
	assert(m_que_pkModelInst.size() > 0);
	
	if (m_que_pkModelInst.size() > 0)
		m_bLODLevel	= (BYTE) MIN(m_que_pkModelInst.size() - 1, bLodLevel);
}

void CGrannyLODController::CreateDeviceObjects()
{
	std::for_each(m_que_pkModelInst.begin(),
				  m_que_pkModelInst.end(),
				  CGrannyModelInstance::FCreateDeviceObjects());
}

void CGrannyLODController::DestroyDeviceObjects()
{
	ReleaseGltfStaticGpuBuffers();

	if (s_pGltfVertexDecl)
	{
		s_pGltfVertexDecl->Release();
		s_pGltfVertexDecl = NULL;
	}

	std::for_each(m_que_pkModelInst.begin(),
				  m_que_pkModelInst.end(),
				  CGrannyModelInstance::FDestroyDeviceObjects());
}

void CGrannyLODController::RenderWithOneTexture()
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::Model))
	{
		if (m_pGltfModelInstance)
		{
			RenderGltfModel(EModelPass::Opaque);
			return;
		}

		if (!m_pCurrentModelInstance)
			return;

		m_pCurrentModelInstance->RenderWithOneTexture();
	}
}

void CGrannyLODController::BlendRenderWithOneTexture()
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::Model))
	{
		if (m_pGltfModelInstance)
		{
			if (HaveBlendThing())
				RenderGltfModel(EModelPass::Blend);
			return;
		}

		if (!m_pCurrentModelInstance)
			return;

		m_pCurrentModelInstance->BlendRenderWithOneTexture();
	}
}

void CGrannyLODController::RenderWithTwoTexture()
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::Model))
	{
		if (m_pGltfModelInstance)
		{
			RenderGltfModel(EModelPass::Opaque);
			return;
		}

		if (!m_pCurrentModelInstance)
			return;

		m_pCurrentModelInstance->RenderWithTwoTexture();
	}
}

void CGrannyLODController::BlendRenderWithTwoTexture()
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::Model))
	{
		if (m_pGltfModelInstance)
		{
			if (HaveBlendThing())
				RenderGltfModel(EModelPass::Blend);
			return;
		}

		if (!m_pCurrentModelInstance)
			return;

		m_pCurrentModelInstance->BlendRenderWithTwoTexture();
	}
}

bool CGrannyLODController::HaveBlendThing()
{
	if (m_pGltfModelInstance)
		return m_gltfRenderer.HaveBlendThing(m_pGltfModelInstance);

	return (0 != GetModelInstance()) ? GetModelInstance()->HaveBlendThing() : false;
}

void CGrannyLODController::Update(float fElapsedTime, float fDistanceFromCenter, float fDistanceFromCamera)
{
	UpdateLODLevel(fDistanceFromCenter, fDistanceFromCamera);
	UpdateTime(fElapsedTime);
}

void CGrannyLODController::UpdateLODLevel(float fDistanceFromCenter, float fDistanceFromCamera)
{
	if (m_pGltfModelInstance)
		return;

	if (m_que_pkModelInst.size()<=1)
		return;
	
	assert(m_pCurrentModelInstance != NULL);

	
	if (fDistanceFromCenter > LOD_APPLY_MIN) // 중심 LOD 예외 취소
	{	
		// 카메라부터 멀어질 수록 fLODRate가 작아진다
		// 3개 LOD가 있을때.. 가장 먼게 0, 가까울 수록 숫자가 커진다

		// 100fps 50fps 33fps 25fps 20fps
		// 10ms 20ms 30ms 40ms 50ms
		float fLODFactor = fMINMAX(0.0f, (m_fLODDistance-fDistanceFromCamera), m_fLODDistance);
		
		if (m_fLODDistance>0.0f)
			m_dwLODAniFPS = (DWORD) ((CGrannyModelInstance::ANIFPS_MAX - CGrannyModelInstance::ANIFPS_MIN) * fLODFactor / m_fLODDistance + static_cast<float>(CGrannyModelInstance::ANIFPS_MIN));
		else
			m_dwLODAniFPS = CGrannyModelInstance::ANIFPS_MIN;
		
		assert(m_dwLODAniFPS > 0);
		m_dwLODAniFPS /= 10;
		m_dwLODAniFPS *= 10;

		float fLODStep = m_fLODDistance / m_que_pkModelInst.size();
		BYTE bLODLevel = BYTE(fLODFactor / fLODStep);

		if (m_fLODDistance <= 5000.0f)
		{
			if (fDistanceFromCamera < 500.0f)
			{
				bLODLevel = 0;
			}
			else if (fDistanceFromCamera < 1500.0f)
			{
				bLODLevel = 1;
			}
			else if (fDistanceFromCamera < 2500.0f)
			{
				bLODLevel = 2;
			}
			else
			{
				bLODLevel = 3;
			}

			bLODLevel = (BYTE) (m_que_pkModelInst.size() - std::min((size_t)bLODLevel, m_que_pkModelInst.size()) - 1);
		}
		
		if (ms_isMinLODModeEnable)
			bLODLevel=0;

		SetLODLevel(bLODLevel);

		if (m_pCurrentModelInstance != m_que_pkModelInst[m_bLODLevel])
		{
			SetCurrentModelInstance(m_que_pkModelInst[m_bLODLevel]);
		}
	}
	else
	{
		m_dwLODAniFPS=CGrannyModelInstance::ANIFPS_MAX;
		
		if (!m_que_pkModelInst.empty())
		{
			if (m_pCurrentModelInstance != m_que_pkModelInst.back())
			{
				SetCurrentModelInstance(m_que_pkModelInst.back());
			}
		}
	}
}

void CGrannyLODController::UpdateTime(float fElapsedTime)
{
	if (m_pGltfModelInstance)
	{
		m_pGltfModelInstance->Update(fElapsedTime);
		// Zero CPU Vertex Deformation: skeleton & skinning matrices updated in Update()
		RefreshAttachedModelInstance();
	}

	if (!m_pCurrentModelInstance)
		return;

	m_pCurrentModelInstance->Update(m_dwLODAniFPS);

	//DWORD t3=timeGetTime();
	m_pCurrentModelInstance->UpdateLocalTime(fElapsedTime);

	//DWORD t4=timeGetTime();

#ifdef __PERFORMANCE_CHECKER__
	{
		static FILE* fp=fopen("perf_lod_update.txt", "w");

		if (t4-t1>3)
		{
			fprintf(fp, "LOD.Total %d (Time %f)\n", t4-t1, timeGetTime()/1000.0f);
			fprintf(fp, "LOD.SMI %d\n", t2-t1);
			fprintf(fp, "LOD.UP %d\n", t3-t2);
			fprintf(fp, "LOD.UL %d\n", t4-t3);
			fprintf(fp, "-------------------------------- \n");
			fflush(fp);
		}			
		fflush(fp);
	}
#endif
}

void CGrannyLODController::SetCurrentModelInstance(CGrannyModelInstance * pgrnModelInstance)
{
	// Copy Motion
	pgrnModelInstance->CopyMotion(m_pCurrentModelInstance, true);
	m_pCurrentModelInstance = pgrnModelInstance;

	// Change children attaching link
	RefreshAttachedModelInstance();

	// Change parent attaching link
	if (m_pAttachedParentModel)
	{
		m_pAttachedParentModel->RefreshAttachedModelInstance();
	}
}

void CGrannyLODController::RefreshAttachedModelInstance()
{
	for (DWORD i = 0; i < m_AttachedModelDataVector.size(); ++i)
	{
		TAttachingModelData & rModelData = m_AttachedModelDataVector[i];
		CGrannyLODController * pkChildController = rModelData.pkLODController;
		if (!pkChildController)
			continue;

		// 1. Dziecko to model glTF (np. bron glTF)
		if (pkChildController->m_pGltfModelInstance)
		{
			if (m_pGltfModelInstance)
			{
				pkChildController->m_pGltfModelInstance->SetParentModelInstance(m_pGltfModelInstance, rModelData.strBoneName.c_str());
			}
			else if (m_pCurrentModelInstance)
			{
				int iBoneIndex = -1;
				if (m_pCurrentModelInstance->GetBoneIndexByName(rModelData.strBoneName.c_str(), &iBoneIndex))
				{
					const float * pBoneMat = m_pCurrentModelInstance->GetBoneMatrixPointer(iBoneIndex);
					if (pBoneMat)
						pkChildController->m_pGltfModelInstance->SetParentBoneMatrix((const EterModelLib::Matrix4x4*)pBoneMat);
				}
			}
		}

		// 2. Dziecko to model Granny
		CGrannyModelInstance * pSrcInstance = pkChildController->GetModelInstance();
		if (pSrcInstance)
		{
			if (m_pGltfModelInstance)
			{
				const auto * pBoneMat = m_pGltfModelInstance->GetBoneMatrixPointer(rModelData.strBoneName.c_str());
				if (pBoneMat)
					pSrcInstance->SetCustomParentBoneMatrix(&pBoneMat->m[0][0]);
			}
			else if (m_pCurrentModelInstance)
			{
				pSrcInstance->SetParentModelInstance(m_pCurrentModelInstance, rModelData.strBoneName.c_str());
			}
		}
	}
}


void CGrannyLODController::DeformAll(const D3DXMATRIX * c_pWorldMatrix)
{
	if (c_pWorldMatrix)
		m_matWorld = *c_pWorldMatrix;

	if (m_pGltfModelInstance)
		m_pGltfModelInstance->Update(0.0f);

	std::deque<CGrannyModelInstance *>::iterator i;
	for (i=m_que_pkModelInst.begin(); i!=m_que_pkModelInst.end(); ++i)
	{
		CGrannyModelInstance* pkModelInst=(*i);
		pkModelInst->Deform(c_pWorldMatrix);		
	}	
}

void CGrannyLODController::DeformNoSkin(const D3DXMATRIX * c_pWorldMatrix)
{
	if (c_pWorldMatrix)
		m_matWorld = *c_pWorldMatrix;

	if (m_pGltfModelInstance)
		m_pGltfModelInstance->Update(0.0f);

	if (m_pCurrentModelInstance)
		m_pCurrentModelInstance->DeformNoSkin(c_pWorldMatrix);
}

void CGrannyLODController::Deform(const D3DXMATRIX * c_pWorldMatrix)
{
	if (c_pWorldMatrix)
		m_matWorld = *c_pWorldMatrix;

	if (m_pGltfModelInstance)
		m_pGltfModelInstance->Update(0.0f);

	if (m_pCurrentModelInstance)
		m_pCurrentModelInstance->Deform(c_pWorldMatrix);
}

void CGrannyLODController::RenderToShadowMap()
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::Model))
	{
		if (m_pGltfModelInstance)
		{
			RenderGltfModel(EModelPass::ShadowDepth);
			return;
		}

		if (m_pCurrentModelInstance)
			m_pCurrentModelInstance->RenderWithoutTexture();
	}
}

void CGrannyLODController::RenderShadow()
{
	if (RenderOwnership::LegacyDraws(ERenderCategory::Model))
	{
		if (m_pGltfModelInstance)
		{
			RenderGltfModel(EModelPass::Opaque);
			return;
		}

		if (m_pCurrentModelInstance)
			m_pCurrentModelInstance->RenderWithOneTexture();
	}
}

void CGrannyLODController::ReloadTexture()
{
	if (m_pCurrentModelInstance)
		m_pCurrentModelInstance->ReloadTexture();
}

void CGrannyLODController::GetBoundBox(D3DXVECTOR3 * vtMin, D3DXVECTOR3 * vtMax)
{
	if (m_pGltfModelInstance && m_pGltfModelInstance->GetModel())
	{
		const std::vector<GltfVertex>& vertices = m_pGltfModelInstance->GetModel()->GetModelData().vertices;
		if (!vertices.empty())
		{
			vtMin->x = vtMax->x = vertices[0].position.x;
			vtMin->y = vtMax->y = vertices[0].position.y;
			vtMin->z = vtMax->z = vertices[0].position.z;
			for (size_t i = 1; i < vertices.size(); ++i)
			{
				const auto& pos = vertices[i].position;
				if (pos.x < vtMin->x) vtMin->x = pos.x;
				if (pos.x > vtMax->x) vtMax->x = pos.x;
				if (pos.y < vtMin->y) vtMin->y = pos.y;
				if (pos.y > vtMax->y) vtMax->y = pos.y;
				if (pos.z < vtMin->z) vtMin->z = pos.z;
				if (pos.z > vtMax->z) vtMax->z = pos.z;
			}
			return;
		}
	}

	if (m_pCurrentModelInstance)
		m_pCurrentModelInstance->GetBoundBox(vtMin, vtMax);
}

bool CGrannyLODController::Intersect(const D3DXMATRIX * c_pMatrix, float * u, float * v, float * t)
{
	if (m_pGltfModelInstance)
	{
		D3DXVECTOR3 vtMin, vtMax;
		GetBoundBox(&vtMin, &vtMax);
		return IntersectCube(c_pMatrix,
			vtMin.x, vtMin.y, vtMin.z,
			vtMax.x, vtMax.y, vtMax.z,
			ms_vtPickRayOrig,
			ms_vtPickRayDir,
			u, v, t);
	}

	if (!m_pCurrentModelInstance)
		return false;
	return m_pCurrentModelInstance->Intersect(c_pMatrix, u, v, t);
}

void CGrannyLODController::SetLocalTime(float fLocalTime)
{
	if (m_pGltfModelInstance)
		m_pGltfModelInstance->SetCurrentTime(fLocalTime);

	if (m_pCurrentModelInstance)
		m_pCurrentModelInstance->SetLocalTime(fLocalTime);
}

void CGrannyLODController::ResetLocalTime()
{
	if (m_pGltfModelInstance)
		m_pGltfModelInstance->SetCurrentTime(0.0f);

	if (!m_pCurrentModelInstance)
		return;

	m_pCurrentModelInstance->ResetLocalTime();
}

void CGrannyLODController::SetMotionPointer(const CGrannyMotion * c_pMotion, float fBlendTime, int iLoopCount, float speedRatio)
{
	if (m_pGltfModelInstance && c_pMotion)
	{
		const GltfMotionData* pGltfMotion = c_pMotion->GetGltfAnimationPointer();
		if (pGltfMotion != NULL)
		{
			m_pGltfModelInstance->SetPlaySpeed(speedRatio);
			m_pGltfModelInstance->PlayMotion(pGltfMotion, iLoopCount == 0);
		}
		else
		{
			m_pGltfModelInstance->SetPlaySpeed(speedRatio);
			m_pGltfModelInstance->PlayMotion(c_pMotion->GetName(), iLoopCount == 0);
		}
	}

	if (!m_pCurrentModelInstance)
		return;

	m_pCurrentModelInstance->SetMotionPointer(c_pMotion, fBlendTime, iLoopCount, speedRatio);
}

void CGrannyLODController::ChangeMotionPointer(const CGrannyMotion * c_pMotion, int iLoopCount, float speedRatio)
{
	if (m_pGltfModelInstance && c_pMotion)
	{
		const GltfMotionData* pGltfMotion = c_pMotion->GetGltfAnimationPointer();
		if (pGltfMotion != NULL)
		{
			m_pGltfModelInstance->SetPlaySpeed(speedRatio);
			m_pGltfModelInstance->PlayMotion(pGltfMotion, iLoopCount == 0);
		}
		else
		{
			m_pGltfModelInstance->SetPlaySpeed(speedRatio);
			m_pGltfModelInstance->PlayMotion(c_pMotion->GetName(), iLoopCount == 0);
		}
	}

	if (!m_pCurrentModelInstance)
		return;

	m_pCurrentModelInstance->ChangeMotionPointer(c_pMotion, iLoopCount, speedRatio);
}

void CGrannyLODController::SetMotionAtEnd()
{
	if (m_pGltfModelInstance)
		m_pGltfModelInstance->SetCurrentTime(m_pGltfModelInstance->GetDuration());

	if (m_pCurrentModelInstance)
		m_pCurrentModelInstance->SetMotionAtEnd();
}

BOOL CGrannyLODController::isModelInstance()
{
	if (m_pGltfModelInstance != NULL)
		return TRUE;

	if (!m_pCurrentModelInstance)
		return FALSE;

	return TRUE;
}

CGrannyModelInstance * CGrannyLODController::GetModelInstance()
{
	return m_pCurrentModelInstance;
}

bool CGrannyLODController::isGltfModelInstance() const
{
	return (m_pGltfModelInstance != NULL);
}

EterModelLib::CGltfModelInstance * CGrannyLODController::GetGltfModelInstance()
{
	return m_pGltfModelInstance;
}

const float * CGrannyLODController::GetBoneMatrixPointer(int iBone) const
{
	if (m_pGltfModelInstance)
	{
		const auto* pMat = m_pGltfModelInstance->GetBoneMatrixPointer(iBone);
		if (pMat)
		{
			return &pMat->m[0][0];
		}
		return NULL;
	}

	if (m_pCurrentModelInstance)
		return m_pCurrentModelInstance->GetBoneMatrixPointer(iBone);

	return NULL;
}

const float * CGrannyLODController::GetBoneMatrixPointer(const char* c_szBoneName) const
{
	if (m_pGltfModelInstance)
	{
		const auto* pMat = m_pGltfModelInstance->GetBoneMatrixPointer(c_szBoneName);
		if (pMat)
		{
			return &pMat->m[0][0];
		}
		return NULL;
	}

	if (m_pCurrentModelInstance)
	{
		int boneIdx = -1;
		if (m_pCurrentModelInstance->GetBoneIndexByName(c_szBoneName, &boneIdx))
			return m_pCurrentModelInstance->GetBoneMatrixPointer(boneIdx);
	}

	return NULL;
}

bool CGrannyLODController::GetBoneIndexByName(const char* c_szBoneName, int* pBoneIndex) const
{
	if (m_pGltfModelInstance && m_pGltfModelInstance->GetModel())
	{
		int boneIdx = m_pGltfModelInstance->GetModel()->FindBoneIndex(c_szBoneName);
		if (boneIdx >= 0)
		{
			if (pBoneIndex)
				*pBoneIndex = boneIdx;
			return true;
		}
		return false;
	}

	if (m_pCurrentModelInstance)
		return m_pCurrentModelInstance->GetBoneIndexByName(c_szBoneName, pBoneIndex);

	return false;
}

const std::vector<EterModelLib::GltfVertex>& CGrannyLODController::GetDeformedVertices() const
{
	static const std::vector<EterModelLib::GltfVertex> s_empty;
	if (m_pGltfModelInstance)
		return m_pGltfModelInstance->GetDeformedVertices();
	return s_empty;
}

DWORD CGrannyLODController::GetDeformableVertexCount() const
{
	if (m_pGltfModelInstance && m_pGltfModelInstance->GetModel())
		return (DWORD)m_pGltfModelInstance->GetModel()->GetVertexCount();
	if (m_pCurrentModelInstance)
		return m_pCurrentModelInstance->GetDeformableVertexCount();
	return 0;
}

DWORD CGrannyLODController::GetVertexCount() const
{
	if (m_pGltfModelInstance && m_pGltfModelInstance->GetModel())
		return (DWORD)m_pGltfModelInstance->GetModel()->GetVertexCount();
	if (m_pCurrentModelInstance)
		return m_pCurrentModelInstance->GetVertexCount();
	return 0;
}

LPDIRECT3DVERTEXBUFFER9 CGrannyLODController::GetGltfStaticVertexBuffer() const
{
	return m_gltfRenderer.GetStaticVertexBuffer(m_pGltfModelInstance);
}

LPDIRECT3DINDEXBUFFER9 CGrannyLODController::GetGltfStaticIndexBuffer() const
{
	return m_gltfRenderer.GetStaticIndexBuffer(m_pGltfModelInstance);
}

void CGrannyLODController::InitGltfMaterials()
{
	m_gltfRenderer.InitMaterials(m_pGltfModelInstance);
}

void CGrannyLODController::ReleaseGltfStaticGpuBuffers()
{
	// Zarzadzane bezposrednio w CGltfModel
}

bool CGrannyLODController::EnsureGltfStaticGpuBuffers()
{
	return m_gltfRenderer.EnsureStaticGpuBuffers(ms_lpd3dDevice, m_pGltfModelInstance);
}

void CGrannyLODController::RenderGltfModel(EModelPass pass)
{
	s_pCurrentActiveController = this;
	m_gltfRenderer.Render(ms_lpd3dDevice, m_pGltfModelInstance, m_matWorld, m_dwModelID, pass);
}

void CGrannyLODController::ExecuteModelDrawItemD3D9(const TModelDrawItem& item)
{
	m_gltfRenderer.ExecuteDrawItemD3D9(ms_lpd3dDevice, m_pGltfModelInstance, item);
}

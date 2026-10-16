#include "StdAfx.h"
#include "Eterbase/Debug.h"
#include "ModelInstance.h"
#include "Model.h"


void CGrannyModelInstance::Update(DWORD dwAniFPS)
{		
	if (!dwAniFPS)
		return;

	const DWORD c_dwCurUpdateFrame = (DWORD) (GetLocalTime() * static_cast<float>(ANIFPS_MAX));
	const DWORD ANIFPS_STEP = ANIFPS_MAX/dwAniFPS;
	if (c_dwCurUpdateFrame>ANIFPS_STEP && c_dwCurUpdateFrame/ANIFPS_STEP==m_dwOldUpdateFrame/ANIFPS_STEP)
		return;

	m_dwOldUpdateFrame=c_dwCurUpdateFrame;

	GrannyFreeCompletedModelControls(m_pgrnModelInstance); //Black screen fix

	//DWORD t1=timeGetTime();
	GrannySetModelClock(m_pgrnModelInstance, GetLocalTime());	
	//DWORD t2=timeGetTime();

#ifdef __PERFORMANCE_CHECKER__
	{
		static FILE* fp=fopen("perf_grn_setmodelclock.txt", "w");

		if (t2-t1>3)
		{
			fprintf(fp, "%f:%x:- GrannySetModelClock(time=%f) = %dms\n", timeGetTime()/1000.0f, this, GetLocalTime(), t2-t1);
			fflush(fp);
		}			
	}
#endif	

}

void CGrannyModelInstance::UpdateLocalTime(float fElapsedTime)
{
	m_fSecondsElapsed = fElapsedTime;
	m_fLocalTime += fElapsedTime;
}

void CGrannyModelInstance::UpdateTransform(D3DXMATRIX * pMatrix, float fSecondsElapsed)
{
	if (!m_pgrnModelInstance)
	{
		TraceError("CGrannyModelIstance::UpdateTransform - m_pgrnModelInstance = NULL");
		return;
	}
	GrannyUpdateModelMatrix(m_pgrnModelInstance, fSecondsElapsed, (const float *) pMatrix, (float *) pMatrix, false);
	//Tracef("%f %f %f",pMatrix->_41,pMatrix->_42,pMatrix->_43);
	
}

void CGrannyModelInstance::DeformPNTVerticesDirect3D()
{
	// Kosztowna deformacja CPU GrannyDeformVertices wylaczona.
	// Wierzcholki modeli postaci pozostaja statyczne w buforze GPU ze stalymi wagami (bone_weights[4]) i indeksami kosci (bone_indices[4]).
	return;
}

void CGrannyModelInstance::Deform(const D3DXMATRIX * c_pWorldMatrix)
{
	if (IsEmpty())
		return;

	// 1. Obliczenia transformacji macierzy na CPU w pamieci RAM (bezpieczne dla watkow roboczych)
	DeformNoSkin(c_pWorldMatrix);

	// 2. Kosztowna deformacja CPU GrannyDeformVertices zostala wylaczona
}

void CGrannyModelInstance::UpdateSkeleton(const D3DXMATRIX * c_pWorldMatrix, float /*fLocalTime*/, bool bSkipBoneSampling)
{	
	CheckSkeletonLink();

	if (!bSkipBoneSampling)
		UpdateWorldPose();
	UpdateWorldMatrices(c_pWorldMatrix);
}

void CGrannyModelInstance::UpdateWorldPose()
{
	// WEP	= m_iParentBoneIndex != 0 -> UpdateWorldPose(O)
	// LOD	= UpdateWorldPose(O)
	// Hair	= UpdateWorldPose(X)

	CheckSkeletonLink();

	if (m_ppkSkeletonInst)
		if (*m_ppkSkeletonInst!=this)
			return;

	if (!m_pgrnModelInstance)
		return;

	granny_skeleton * pgrnSkeleton = GrannyGetSourceSkeleton(m_pgrnModelInstance);
	if (!pgrnSkeleton)
		return;

	granny_world_pose* pWorldPose = __GetWorldPosePtr();
	if (!pWorldPose)
		return;

	// Izolowany bufor local_pose per-instancja (brak wspoldzielonego s_SharedLocalPose)
	granny_local_pose * pgrnLocalPose = m_localPose.Get(pgrnSkeleton->BoneCount);	
	if (!pgrnLocalPose)
		return;

	const float * pAttachBoneMatrix = m_pCustomParentBoneMatrix ? m_pCustomParentBoneMatrix : ((mc_pParentInstance) ? mc_pParentInstance->GetBoneMatrixPointer(m_iParentBoneIndex) : NULL);

	GrannySampleModelAnimationsAccelerated(m_pgrnModelInstance, pgrnSkeleton->BoneCount, pAttachBoneMatrix, pgrnLocalPose, pWorldPose);
	/*
	GrannySampleModelAnimations(m_pgrnModelInstance, 0, pgrnSkeleton->BoneCount, pgrnLocalPose);
	GrannyBuildWorldPose(pgrnSkeleton, 0, pgrnSkeleton->BoneCount, pgrnLocalPose, pAttachBoneMatrix, m_pgrnWorldPose);
	*/
	GrannyFreeCompletedModelControls(m_pgrnModelInstance);	
}

void CGrannyModelInstance::UpdateWorldMatrices(const D3DXMATRIX* c_pWorldMatrix)
{
	// NO_MESH_BUG_FIX
	if (!m_meshMatrices[0] || !m_meshMatrices[1])
		return;
	// END_OF_NO_MESH_BUG_FIX
	
	if (!m_pModel || m_pModel->GetMeshCount() <= 0)
		return;
	
	assert(ms_lpd3dMatStack != NULL);
	
	int meshCount = m_pModel->GetMeshCount();
	
	granny_world_pose* pWorldPose = __GetWorldPosePtr();
	if (!pWorldPose)
		return;

	granny_matrix_4x4 * pgrnMatCompositeBuffer = GrannyGetWorldPoseComposite4x4Array(pWorldPose);
	if (!pgrnMatCompositeBuffer)
		return;

	D3DXMATRIX * boneMatrices = (D3DXMATRIX *) pgrnMatCompositeBuffer;

	// CPU Double-Buffering: Watki robocze zapisuja transformacje do izolowanego bufora RAM (back-buffer)
	uint8_t currentFront = m_activeTransformBuffer.load(std::memory_order_relaxed);
	uint8_t backIndex = 1 - currentFront;
	D3DXMATRIX* pWorkMatrices = m_meshMatrices[backIndex];
	if (!pWorkMatrices)
		return;

	for (int i = 0; i < meshCount; ++i)
	{
		D3DXMATRIX & rWorldMatrix = pWorkMatrices[i];

		const CGrannyMesh * pMesh = m_pModel->GetMeshPointer(i);
		if (!pMesh)
			continue;

		// WORK
		int * boneIndices = __GetMeshBoneIndices(i);
		if (!boneIndices)
			continue;
		// END_OF_WORK

		if (pMesh->CanDeformPNTVertices())
		{
			rWorldMatrix = *c_pWorldMatrix;			
		}
		else
		{
			int iBone = *boneIndices;
			D3DXMatrixMultiply(&rWorldMatrix, &boneMatrices[iBone], c_pWorldMatrix);
		}
	}

	// Atomowa zamiana bufora (Flip) - nowy bufor staje sie front-bufferem
	m_activeTransformBuffer.store(backIndex, std::memory_order_release);

#ifdef _TEST
	TEST_matWorld = *c_pWorldMatrix;
#endif
}

void CGrannyModelInstance::DeformPNTVertices(void * /*pvDest*/)
{
	// Kosztowna deformacja CPU GrannyDeformVertices wylaczona.
	// Wierzcholki modeli postaci pozostaja statyczne w buforze GPU ze stalymi wagami i indeksami kosci.
	return;
}

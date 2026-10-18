import re

with open('src/EterLib/GrpScreen.cpp', 'r') as f:
    content = f.read()

# Add <chrono> include
content = re.sub(r'#include <utf8\.h>', '#include <utf8.h>\n#include <chrono>', content)

# Replace s_llFrameStartTime and s_llPerformanceFreq
content = re.sub(
    r'static float s_fLastFrameTimeMs = 0\.0f;\nstatic float s_fLastFPS = 0\.0f;\nstatic LARGE_INTEGER s_llFrameStartTime = \{ 0 \};\nstatic LARGE_INTEGER s_llPerformanceFreq = \{ 0 \};',
    'static float s_fLastFrameTimeMs = 0.0f;\nstatic float s_fLastFPS = 0.0f;\nstatic std::chrono::high_resolution_clock::time_point s_llFrameStartTime;',
    content)

# Replace Begin() method
begin_replacement = """bool CScreen::Begin()
{
	assert(ms_lpd3dDevice != NULL);
	ResetFaceCount();
	STATEMANAGER.ResetDrawCallCounter();
	STATEMANAGER.ResetPrimitiveCounter();

	s_llFrameStartTime = std::chrono::high_resolution_clock::now();

	if (!STATEMANAGER.BeginScene())
	{
		Tracenf("BeginScene FAILED\\n");
		return false;
	}

	return true;
}"""
content = re.sub(r'bool CScreen::Begin\(\)\s*\{.*?return true;\s*\}', begin_replacement, content, flags=re.DOTALL)

# Replace End() method
end_replacement = """void CScreen::End()
{
	STATEMANAGER.EndScene();

	auto llFrameEndTime = std::chrono::high_resolution_clock::now();
	std::chrono::duration<double> dDeltaSec = llFrameEndTime - s_llFrameStartTime;

	s_fLastFrameTimeMs = static_cast<float>(dDeltaSec.count() * 1000.0);
	s_fLastFPS = (dDeltaSec.count() > 0.000001) ? static_cast<float>(1.0 / dDeltaSec.count()) : 0.0f;
}"""
content = re.sub(r'void CScreen::End\(\)\s*\{.*?\s*\}', end_replacement, content, flags=re.DOTALL)

# Replace RenderCircle2d
render_circle_2d_replacement = """void CScreen::RenderCircle2d(float fx, float fy, float fz, float fRadius, int iStep)
{
	if (iStep < 3)
		return;

	float delta = 2 * D3DX_PI / float(iStep);
	float theta = 0.0f;

	D3DXVECTOR3 firstPoint(fx + fRadius, fy, fz);
	D3DXVECTOR3 prevPoint = firstPoint;

	for (int count = 1; count < iStep; count++)
	{
		theta += delta;
		D3DXVECTOR3 curPoint(fx + fRadius * cosf(theta), fy + fRadius * sinf(theta), fz);
		RenderLine3d(prevPoint.x, prevPoint.y, prevPoint.z, curPoint.x, curPoint.y, curPoint.z);
		prevPoint = curPoint;
	}
	RenderLine3d(prevPoint.x, prevPoint.y, prevPoint.z, firstPoint.x, firstPoint.y, firstPoint.z);
}"""
content = re.sub(r'void CScreen::RenderCircle2d\(float fx, float fy, float fz, float fRadius, int iStep\)\s*\{.*?\}', render_circle_2d_replacement, content, flags=re.DOTALL)

# Replace RenderCircle3d
render_circle_3d_replacement = """void CScreen::RenderCircle3d(float fx, float fy, float fz, float fRadius, int iStep)
{
	if (iStep < 3)
		return;

	float delta = 2 * D3DX_PI / float(iStep);
	float theta = 0.0f;

	const D3DXMATRIX & c_rmatInvView = CCameraManager::Instance().GetCurrentCamera()->GetBillboardMatrix();

	D3DXVECTOR3 firstPoint(fRadius, 0.0f, 0.0f);
	D3DXVec3TransformCoord(&firstPoint, &firstPoint, &c_rmatInvView);
	D3DXVECTOR3 prevPoint = firstPoint;

	for (int count = 1; count < iStep; count++)
	{
		theta += delta;
		D3DXVECTOR3 curPoint(fRadius * cosf(theta), fRadius * sinf(theta), 0.0f);
		D3DXVec3TransformCoord(&curPoint, &curPoint, &c_rmatInvView);
		RenderLine3d(fx + prevPoint.x, fy + prevPoint.y, fz + prevPoint.z,
					 fx + curPoint.x, fy + curPoint.y, fz + curPoint.z);
		prevPoint = curPoint;
	}
	RenderLine3d(fx + prevPoint.x, fy + prevPoint.y, fz + prevPoint.z,
				 fx + firstPoint.x, fy + firstPoint.y, fz + firstPoint.z);
}"""
content = re.sub(r'void CScreen::RenderCircle3d\(float fx, float fy, float fz, float fRadius, int iStep\)\s*\{.*?\}', render_circle_3d_replacement, content, flags=re.DOTALL)

with open('src/EterLib/GrpScreen.cpp', 'w') as f:
    f.write(content)

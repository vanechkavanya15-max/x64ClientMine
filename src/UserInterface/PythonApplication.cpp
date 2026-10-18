#include "StdAfx.h"
#include "eterBase/Error.h"
#include "eterlib/Camera.h"
#include "eterlib/AttributeInstance.h"
#include "EterLib/FrameTimer.h"
#include "gamelib/AreaTerrain.h"
#include "EterGrnLib/Material.h"

#include "resource.h"
#include "PythonApplication.h"
#include "PythonCharacterManager.h"
#include "EterLib/StateManager.h"
#include "EterLib/StateManagerGuards.h"
#include "EterLib/RenderQueueManager.h"
#include "EterLib/Render/DynamicQuadBatcher.h"
#include "EterLib/CFrameSnapshot.h"
#include "EterLib/D3D9RenderBackend.h"
#include "EterLib/WgpuRenderBackend.h"
#include "EterLib/WgpuModule.h"
#include "EterLib/RenderOwnership.h"
#include "Client/Graphics/Debug/DebugRenderer.h"
#include "TestHarness/TestHarnessServer.h"
#include "TestHarness/DeterministicTickController.h"
#include "TestHarness/MockWorldDriver.h"
#include "Client/Core/GameFramePipeline.h"

#include "ProcessScanner.h"
#include "FastBootOverlay.h"
#include <stb_image_write.h>

#include <utf8.h>
#include <thread>
#include <chrono>
#include <mmsystem.h>
#include <wtsapi32.h>
#include <immintrin.h>
#pragma comment(lib, "wtsapi32.lib")

std::atomic<bool> CPythonApplication::ms_bSessionLocked{false};
std::atomic<bool> CPythonApplication::ms_bInModalSizeMove{false};

namespace
{
	constexpr DWORD MAX_MESSAGES_PER_FRAME  = 64;
	constexpr auto  MAX_MESSAGE_PUMP_BUDGET = std::chrono::microseconds(1500);

	class FramePacingRegulator
	{
	public:
		static FramePacingRegulator& Instance() noexcept
		{
			static FramePacingRegulator s_instance;
			return s_instance;
		}

		FramePacingRegulator()
		{
			m_hTimer = ::CreateWaitableTimerExW(
				nullptr,
				nullptr,
				CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
				TIMER_ALL_ACCESS
			);
			if (!m_hTimer)
			{
				m_hTimer = ::CreateWaitableTimerW(nullptr, TRUE, nullptr);
			}
			m_nextFrameTarget = std::chrono::steady_clock::now();
		}

		~FramePacingRegulator()
		{
			if (m_hTimer)
			{
				::CloseHandle(m_hTimer);
				m_hTimer = nullptr;
			}
		}

		void Reset() noexcept
		{
			m_nextFrameTarget = std::chrono::steady_clock::now();
		}

		bool WaitForNextFrame(std::chrono::nanoseconds targetFrameDuration, bool allowEarlyWakeOnInput) noexcept
		{
			using clock = std::chrono::steady_clock;
			const auto now = clock::now();

			m_nextFrameTarget += targetFrameDuration;
			if (m_nextFrameTarget < now)
			{
				m_nextFrameTarget = now;
				return false;
			}

			bool wokeOnInput = false;

			while (true)
			{
				const auto current = clock::now();
				const auto remaining = m_nextFrameTarget - current;
				if (remaining <= std::chrono::nanoseconds::zero())
					break;

				const auto sleepThreshold = m_schedulerOvershoot + std::chrono::microseconds(350);
				if (m_hTimer && remaining > sleepThreshold)
				{
					const auto sleepDuration = remaining - m_schedulerOvershoot;
					LARGE_INTEGER liDueTime{};
					liDueTime.QuadPart = -static_cast<LONGLONG>(
						std::chrono::duration_cast<std::chrono::nanoseconds>(sleepDuration).count() / 100
					);

					if (liDueTime.QuadPart < 0 && ::SetWaitableTimer(m_hTimer, &liDueTime, 0, nullptr, nullptr, FALSE))
					{
						const auto sleepStart = clock::now();
						DWORD dwWakeMask = allowEarlyWakeOnInput ? QS_ALLINPUT : 0;
						DWORD dwFlags    = allowEarlyWakeOnInput ? MWMO_INPUTAVAILABLE : 0;

						DWORD waitRes = ::MsgWaitForMultipleObjectsEx(
							1, &m_hTimer, INFINITE, dwWakeMask, dwFlags
						);

						const auto actualSlept = clock::now() - sleepStart;
						if (waitRes == WAIT_OBJECT_0 + 1)
						{
							wokeOnInput = true;
							break;
						}

						if (actualSlept > sleepDuration)
						{
							const auto overshoot = actualSlept - sleepDuration;
							const auto maxOvershoot = std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::milliseconds(4));
							const auto clampedOvershoot = (overshoot < maxOvershoot) ? overshoot : maxOvershoot;
							m_schedulerOvershoot = std::chrono::nanoseconds(
								(m_schedulerOvershoot.count() * 7 + clampedOvershoot.count()) / 8
							);
						}
						else
						{
							m_schedulerOvershoot = std::chrono::nanoseconds(
								(m_schedulerOvershoot.count() * 31) / 32
							);
						}
						continue;
					}
				}

				_mm_pause();
			}

			return wokeOnInput;
		}

		void RegulateFrame(uint32_t targetFps, bool allowEarlyWakeOnInput = true) noexcept
		{
			if (targetFps == 0)
				targetFps = 60;
			const auto targetDuration = std::chrono::nanoseconds(1'000'000'000ULL / targetFps);
			WaitForNextFrame(targetDuration, allowEarlyWakeOnInput);
		}

	private:
		HANDLE m_hTimer = nullptr;
		std::chrono::steady_clock::time_point m_nextFrameTarget{};
		std::chrono::nanoseconds m_schedulerOvershoot = std::chrono::microseconds(500);
	};

	using HighPrecisionFramePacer = FramePacingRegulator;
	static FramePacingRegulator& s_framePacer = FramePacingRegulator::Instance();
}

void CPythonApplication::ResetFramePacer()
{
	FramePacingRegulator::Instance().Reset();
}

static bool s_bWindowVisibleState = true;

void SetWindowVisible(bool visible)
{
	s_bWindowVisibleState = visible;
	if (CPythonApplication::InstancePtr())
	{
		CPythonApplication::Instance().SetWindowVisible(visible);
	}
}

void CPythonApplication::SetWindowVisible(bool visible)
{
	s_bWindowVisibleState = visible;
	if (m_hWnd)
	{
		ShowWindow(m_hWnd, visible ? SW_RESTORE : SW_HIDE);
		m_isMinimizedWnd = !visible;
	}
	SetRenderThrottling(!visible);
}

float MIN_FOG = 2400.0f;
double g_specularSpd=0.007f;

CPythonApplication * CPythonApplication::ms_pInstance;
uint32_t CPythonApplication::ms_dwTurboTicks = 0;

void CPythonApplication::SetTurboTicks(uint32_t ticks)
{
	ms_dwTurboTicks = ticks;
	if (ms_pInstance)
	{
		ms_pInstance->m_dwTurboTicks = ticks;
	}
}

uint32_t CPythonApplication::GetTurboTicks()
{
	return ms_dwTurboTicks;
}

bool CPythonApplication::IsTurboTicksMode()
{
	return ms_dwTurboTicks > 0;
}

float c_fDefaultCameraRotateSpeed = 2.25f; // Zwiekszona o 50% czulosc obrotu kamery (bylo 1.5f)
float c_fDefaultCameraPitchSpeed = 2.25f; // Zwiekszona o 50% czulosc pochylenia kamery (bylo 1.5f)
float c_fDefaultCameraZoomSpeed = 0.05f;

CPythonApplication::CPythonApplication() :
m_bCursorVisible(TRUE),
m_bLiarCursorOn(false),
m_iCursorMode(CURSOR_MODE_HARDWARE),
m_isWindowed(false),
m_isFrameSkipDisable(false),
m_poMouseHandler(NULL),
m_dwUpdateFPS(0),
m_dwRenderFPS(0),
m_fAveRenderTime(0.0f),
m_dwFaceCount(0),
m_fGlobalTime(0.0f),
m_fGlobalElapsedTime(0.0f),
m_dwLButtonDownTime(0),
m_dwLastIdleTime(0),
m_IsMovingMainWindow(false),
m_dwTurboTicks(0)
{
#ifndef _DEBUG
	SetEterExceptionHandler();
#endif

	m_dwTurboTicks = ms_dwTurboTicks;
	CTimer::Instance().UseCustomTime();
	m_dwWidth = 800;
	m_dwHeight = 600;

	ms_pInstance = this;
	m_isWindowFullScreenEnable = FALSE;

	m_v3CenterPosition = D3DXVECTOR3(0.0f, 0.0f, 0.0f);
	m_dwStartLocalTime = ELTimer_GetMSec();
	m_tServerTime = 0;
	m_tLocalStartTime = 0;

	m_iPort = 0;
	m_iFPS = 60;

	m_isActivateWnd = false;
	m_isMinimizedWnd = !s_bWindowVisibleState;
	if (!s_bWindowVisibleState)
	{
		m_bRenderThrottling.store(true);
	}

	m_fRotationSpeed = 0.0f;
	m_fPitchSpeed = 0.0f;
	m_fZoomSpeed = 0.0f;

	m_fFaceSpd=0.0f;

	m_dwFaceAccCount=0;
	m_dwFaceAccTime=0;

	m_dwFaceSpdSum=0;
	m_dwFaceSpdCount=0;

	m_FlyingManager.SetMapManagerPtr(&m_pyBackground);

	m_iCursorNum = CURSOR_SHAPE_NORMAL;
	m_iContinuousCursorNum = CURSOR_SHAPE_NORMAL;

	m_isSpecialCameraMode = FALSE;
	m_fCameraRotateSpeed = c_fDefaultCameraRotateSpeed;
	m_fCameraPitchSpeed = c_fDefaultCameraPitchSpeed;
	m_fCameraZoomSpeed = c_fDefaultCameraZoomSpeed;

	m_iCameraMode = CAMERA_MODE_NORMAL;
	m_fBlendCameraStartTime = 0.0f;
	m_fBlendCameraBlendTime = 0.0f;

	m_iForceSightRange = -1;

	CCameraManager::Instance().AddCamera(EVENT_CAMERA_NUMBER);

	m_InitialMouseMovingPoint = {};
}

CPythonApplication::~CPythonApplication()
{
	UserInterface::FastBoot::FastBootOverlay::Instance().Shutdown();
	ms_pInstance = nullptr;
}

void CPythonApplication::GetMousePosition(POINT* ppt)
{
	CMSApplication::GetMousePosition(ppt);
}

void CPythonApplication::SetMinFog(float fMinFog)
{
	MIN_FOG = fMinFog;
}

void CPythonApplication::SetFrameSkip(bool isEnable)
{
	if (isEnable)
		m_isFrameSkipDisable=false;
	else
		m_isFrameSkipDisable=true;
}

void CPythonApplication::NotifyHack(const char* c_szFormat, ...)
{
	char szBuf[1024];

	va_list args;
	va_start(args, c_szFormat);	
	_vsnprintf(szBuf, sizeof(szBuf), c_szFormat, args);
	va_end(args);
	m_pyNetworkStream.NotifyHack(szBuf);
}

void CPythonApplication::GetInfo(UINT eInfo, std::string* pstInfo)
{
	switch (eInfo)
	{
	case INFO_ACTOR:
		m_kChrMgr.GetInfo(pstInfo);
		break;
	case INFO_EFFECT:
		m_kEftMgr.GetInfo(pstInfo);			
		break;
	case INFO_ITEM:
		m_pyItem.GetInfo(pstInfo);
		break;
	case INFO_TEXTTAIL:
		m_pyTextTail.GetInfo(pstInfo);
		break;
	}
}

void CPythonApplication::Abort()
{
	TraceError("============================================================================================================");
	TraceError("Abort!!!!\n\n");

	PostQuitMessage(0);
}

void CPythonApplication::Exit()
{
	TraceError("CPythonApplication::Exit called!");
	PostQuitMessage(0);
}

namespace
{
	class ScopedPythonGilRelease
	{
	public:
		ScopedPythonGilRelease() : m_threadState(nullptr)
		{
			if (Py_IsInitialized())
			{
				m_threadState = PyEval_SaveThread();
			}
		}

		~ScopedPythonGilRelease()
		{
			if (m_threadState)
			{
				PyEval_RestoreThread(m_threadState);
			}
		}

		ScopedPythonGilRelease(const ScopedPythonGilRelease&) = delete;
		ScopedPythonGilRelease& operator=(const ScopedPythonGilRelease&) = delete;

	private:
		PyThreadState* m_threadState;
	};
}

void CPythonApplication::ExtractSnapshot()
{
	CCullingManager::Instance().Update();
	if (!m_pyBackground.IsMapReady())
		return;

	float fAspect = m_kWndMgr.GetAspect();
	float fFarClip = m_pyBackground.GetFarClip();

	m_pyGraphic.SetPerspective(30.0f, fAspect, 100.0, fFarClip);

	static uint32_t s_dwFrameIdx = 0;
	++s_dwFrameIdx;
	CCamera* pCam = CCameraManager::Instance().GetCurrentCamera();
	CFrameSnapshot::Capture(pCam, 30.0f, fAspect, 100.0f, fFarClip, m_fGlobalTime, s_dwFrameIdx);

	CCullingManager::Instance().Process();

	const TFrameSnapshot& snapshot = CFrameSnapshot::Current();
	TRenderFrameUniforms uniforms = CRenderQueueManager::Instance().GetFrameUniforms();
	uniforms.matView = snapshot.matView;
	uniforms.matProj = snapshot.matProj;
	uniforms.matViewProj = snapshot.matViewProj;
	uniforms.vCameraPos = snapshot.v3CameraPos;
	uniforms.fTime = snapshot.fTime;
	uniforms.dwFrameIndex = snapshot.dwFrameIndex;
	CRenderQueueManager::Instance().SetFrameUniforms(uniforms);
}

void CPythonApplication::RenderWorld3D()
{
	if (!m_pyBackground.IsMapReady())
		return;

	long lx, ly;
	m_kWndMgr.GetMousePosition(lx, ly);
	m_pyGraphic.SetCursorPosition(lx, ly);

	// Release Python GIL during heavy 3D deformation, culling, and draw queue emission
	ScopedPythonGilRelease gilRelease;

	m_kChrMgr.Deform();
	m_pyBackground.RenderCharacterShadowToTexture();

	m_pyGraphic.SetGameRenderState();
	m_pyGraphic.PushState();

	m_pyBackground.BeginEnvironment();
	m_pyBackground.Render();

	m_pyBackground.SetCharacterDirLight();
	m_kChrMgr.Render();

	m_pyBackground.SetBackgroundDirLight();
	m_pyBackground.RenderWater();
	m_pyBackground.RenderSnow();
	m_pyBackground.RenderEffect();

	m_pyBackground.EndEnvironment();

	m_kEftMgr.Render();
	m_pyItem.Render();
	m_FlyingManager.Render();

	m_pyBackground.BeginEnvironment();
	m_pyBackground.RenderPCBlocker();
	m_pyBackground.EndEnvironment();

	// Transmit diagnostic debug lines to render queue
	Client::Graphics::Debug::DebugRenderer::Instance().FlushToRenderQueue();

	// Strangler Fig: Render queued packets through active backend (D3D9 / WGPU)
	CRenderQueueManager::Instance().Flush(CRenderQueueManager::Instance().GetActiveBackend());

	// Clear debug line buffer after render
	Client::Graphics::Debug::DebugRenderer::Instance().Clear();

	// Isolate state
	m_pyGraphic.PopState();
}

void CPythonApplication::RenderGame()
{
	ExtractSnapshot();
	RenderWorld3D();
}

void CPythonApplication::RenderUI2D()
{
	m_pyGraphic.SetInterfaceRenderState();
	OnUIRender();
	OnMouseRender();
	FlushFrameUI();
	UserInterface::FastBoot::FastBootOverlay::Instance().Render();
}

void CPythonApplication::LogicTick(float fixedDelta)
{
	CTimer& rkTimer = CTimer::Instance();
	rkTimer.Advance();

	m_fGlobalTime = rkTimer.GetCurrentSecond();
	m_fGlobalElapsedTime = rkTimer.GetElapsedSecond();

	// Phase 3: NETWORK_PUMP - TCP stream packet parsing
	{
		Client::Core::ScopedPhaseTimer netTimer(Client::Core::FramePhase::NetworkPump);
		m_pyNetworkStream.Process();
		m_kGuildMarkUploader.Process();
		m_kGuildMarkDownloader.Process();
		m_kAccountConnector.Process();
		UserInterface::FastBoot::FastBootOverlay::Instance().Update();
	}

	// Phase 3b: FIXED_UPDATE - Apply network event queue in fixed timestep
	m_pyNetworkStream.FlushGamePacketsFixedUpdate(8);

	// Phase 4: WORLD_SIMULATION - Native C++ world simulation
	if (m_pyBackground.IsMapReady())
	{
		UpdateGame();
	}

	// Phase 5: UI_SYNCHRONIZATION - Update UI, camera and mouse
	{
		Client::Core::ScopedPhaseTimer uiTimer(Client::Core::FramePhase::UiSynchronization);
		CResourceManager::Instance().Update();
		OnCameraUpdate();
		OnMouseUpdate();
		OnUIUpdate();
	}
}


void CPythonApplication::UpdateGame()
{
	Client::Core::ScopedPhaseTimer simTimer(Client::Core::FramePhase::WorldSimulation);

	POINT ptMouse;
	GetMousePosition(&ptMouse);

	CGraphicTextInstance::Hyperlink_UpdateMousePos(ptMouse.x, ptMouse.y);


	// Workaround for SetTransform crash during Alt+Tab
	//if (m_isActivateWnd)
	{
		CScreen s;
		float fAspect = UI::CWindowManager::Instance().GetAspect();
		float fFarClip = CPythonBackground::Instance().GetFarClip();

		s.SetPerspective(30.0f,fAspect, 100.0f, fFarClip);
		s.BuildViewFrustum();
	}

	TPixelPosition kPPosMainActor;
	m_pyPlayer.NEW_GetMainActorPosition(&kPPosMainActor);

	m_pyBackground.Update(kPPosMainActor.x, kPPosMainActor.y, kPPosMainActor.z);

	m_GameEventManager.SetCenterPosition(kPPosMainActor.x, kPPosMainActor.y, kPPosMainActor.z);
	m_GameEventManager.Update();

	m_kChrMgr.Update();

	m_kEftMgr.Update();
	m_kEftMgr.UpdateSound();
	
	m_FlyingManager.Update();
	m_pyItem.Update(ptMouse);
	m_pyPlayer.Update();

	// Re-acquire main actor position after Update so character sound plays at current position
	m_pyPlayer.NEW_GetMainActorPosition(&kPPosMainActor);
	SetCenterPosition(kPPosMainActor.x, kPPosMainActor.y, kPPosMainActor.z);
}

bool CPythonApplication::Process()
{
	// Faza 1: IPC_INGEST - Obsluga zadan od agentow AI (Named Pipe) i MockWorld
	{
		Client::Core::ScopedPhaseTimer ipcTimer(Client::Core::FramePhase::IpcIngest);
		UserInterface::TestHarness::TestHarnessServer::Instance().ProcessMainThreadQueue();

		if (UserInterface::TestHarness::MockWorldDriver::Instance().IsMockWorldEnabled() &&
			!UserInterface::TestHarness::MockWorldDriver::Instance().HasEntered())
		{
			UserInterface::TestHarness::MockWorldDriver::Instance().EnterMockWorld();
		}
	}

	ELTimer_SetFrameMSec();

	// 	m_Profiler.Clear();
	DWORD dwStart = ELTimer_GetMSec();

	///////////////////////////////////////////////////////////////////////////////////////////////////
	static DWORD	s_dwUpdateFrameCount = 0;
	static DWORD	s_dwRenderFrameCount = 0;
	static DWORD	s_dwFaceCount = 0;
	static UINT		s_uiLoad = 0;
	static DWORD	s_dwCheckTime = ELTimer_GetMSec();

	if (ELTimer_GetMSec() - s_dwCheckTime > 1000) [[unlikely]] {
		m_dwUpdateFPS		= s_dwUpdateFrameCount;
		m_dwRenderFPS		= s_dwRenderFrameCount;
		m_dwLoad			= s_uiLoad;

		m_dwFaceCount		= s_dwFaceCount / std::max(1ul, s_dwRenderFrameCount);

		s_dwCheckTime		= ELTimer_GetMSec();

		s_uiLoad = s_dwFaceCount = s_dwUpdateFrameCount = s_dwRenderFrameCount = 0;
	}

	// Update Time
	static BOOL s_bFrameSkip = false;

	// Faza 2: INPUT_UPDATE - Peryferia OS i wirtualne kontrolery (odswiezane przy kazdej klatce)
	if (!m_isMinimizedWnd && !ms_bSessionLocked.load(std::memory_order_acquire))
	{
		Client::Core::ScopedPhaseTimer inputTimer(Client::Core::FramePhase::InputUpdate);
		UpdateKeyboard();

		POINT Point;
		if (GetCursorPos(&Point)) [[likely]] {
			ScreenToClient(m_hWnd, &Point);
			OnMouseMove(Point.x, Point.y);		
		}

		__UpdateCamera();
	}

	// Taktowanie logiki gry sztywnym krokiem czasowym (60Hz) lub kontrolerem deterministycznym
	DWORD updatestart = ELTimer_GetMSec();
	auto updateLogicLambda = [&](float fixedDelta) {
		LogicTick(fixedDelta);
		++s_dwUpdateFrameCount;
	};

	if (UserInterface::TestHarness::DeterministicTickController::Instance().IsTestMode() &&
		UserInterface::TestHarness::DeterministicTickController::Instance().IsFrozen())
	{
		uint32_t stepCount = UserInterface::TestHarness::DeterministicTickController::Instance().ConsumePendingSteps();
		float stepDelta = UserInterface::TestHarness::DeterministicTickController::Instance().GetStepDeltaTime();
		for (uint32_t i = 0; i < stepCount; ++i)
		{
			updateLogicLambda(stepDelta);
			UserInterface::TestHarness::DeterministicTickController::Instance().IncrementTickCount();
			UserInterface::TestHarness::DeterministicTickController::Instance().NotifyTickCompleted(stepDelta);
		}
	}
	else
	{
		if (UserInterface::TestHarness::DeterministicTickController::Instance().IsTestMode())
		{
			(void)UserInterface::TestHarness::DeterministicTickController::Instance().ConsumePendingSteps();
		}
		FrameTimer::Instance().Tick(updateLogicLambda);
	}

	m_dwCurUpdateTime = ELTimer_GetMSec() - updatestart;

	// Faza 6: TELEMETRY_RECORD - Zapis telemetrii wykonawczej dla agentow AI
	{
		Client::Core::GameFramePipeline::Instance().RecordFrameComplete(
			s_dwUpdateFrameCount,
			UserInterface::TestHarness::DeterministicTickController::Instance().GetCurrentTick(),
			m_fGlobalElapsedTime
		);
		Client::Core::GameFramePipeline::Instance().SetEngineModeFlags(
			UserInterface::TestHarness::DeterministicTickController::Instance().IsTestMode(),
			UserInterface::TestHarness::DeterministicTickController::Instance().IsFrozen(),
			g_bHeadlessMode
		);
	}

	// Stary mechanizm dlawiacy s_bFrameSkip wylaczony - taktowanie klatek logiki prowadzi FrameTimer (144Hz+)
	s_bFrameSkip = false;
	bool canRender = false;

	if (!s_bFrameSkip)
	{
		//		static double pos=0.0f;
		//		CGrannyMaterial::TranslateSpecularMatrix(fabs(sin(pos)*0.005), fabs(cos(pos)*0.005), 0.0f);
		//		pos+=0.01f;

		CGrannyMaterial::TranslateSpecularMatrix(g_specularSpd, g_specularSpd, 0.0f);

		DWORD dwRenderStartTime = ELTimer_GetMSec();		

		canRender = !g_bHeadlessMode && !m_isMinimizedWnd && !m_bRenderThrottling.load(std::memory_order_acquire) && !ms_bSessionLocked.load(std::memory_order_acquire);

		if (canRender) [[likely]] {
			if (!RenderOwnership::IsWgpuConfigured() && m_pyGraphic.IsLostDevice()) [[unlikely]] {
				const int devState = CheckDeviceState();
				if (devState == DEVICE_STATE_OK)
				{
					UserInterface::FastBoot::FastBootOverlay::Instance().OnLostDevice();
					CD3D9RenderBackend::Instance().OnLostDevice();
					CPythonBackground& rkBG = CPythonBackground::Instance();
					rkBG.ReleaseCharacterShadowTexture();

					if (m_pyGraphic.RestoreDevice()) {
						rkBG.CreateCharacterShadowTexture();
						CD3D9RenderBackend::Instance().OnResetDevice();
						UserInterface::FastBoot::FastBootOverlay::Instance().OnResetDevice();
					}
					else
					{
						canRender = false;
					}
				}
				else
				{
					canRender = false;
				}
			}
		}

		if (canRender) [[likely]]
		{
			ExtractSnapshot();

			if (m_pyGraphic.Begin()) [[likely]] {
				EterLib::Render::DynamicQuadBatcher::Instance().BeginFrame();
				CRenderQueueManager::Instance().BeginFrame();

				m_pyGraphic.ClearDepthBuffer();
				m_pyGraphic.SetClearColor(0.0f, 0.0f, 0.0f);
				m_pyGraphic.Clear();

				// Faza 7A: Natywne renderowanie swiata 3D (ze zwolnionym GIL)
				if (m_pyBackground.IsMapReady())
				{
					RenderWorld3D();
				}

				// Faza 7B: Nakladka interfejsu 2D
				RenderUI2D();

				CRenderQueueManager::Instance().EndFrame();
				m_pyGraphic.End();

				// W trybie WGPU Direct-to-Screen prezentacja odbywa sie bezposrednio w m2_wgpu_render_frame (surface.present())
				if (!RenderOwnership::IsWgpuConfigured())
				{
					ScopedPythonGilRelease gilReleasePresent;
					m_pyGraphic.Show();
				}
				//DWORD t2 = ELTimer_GetMSec();

				DWORD dwRenderEndTime = ELTimer_GetMSec();

				static DWORD s_dwRenderCheckTime = dwRenderEndTime;
				static DWORD s_dwRenderRangeTime = 0;
				static DWORD s_dwRenderRangeFrame = 0;

				m_dwCurRenderTime = dwRenderEndTime - dwRenderStartTime;			
				s_dwRenderRangeTime += m_dwCurRenderTime;				
				++s_dwRenderRangeFrame;			

				if (dwRenderEndTime-s_dwRenderCheckTime>1000) [[unlikely]] {
					m_fAveRenderTime=float(double(s_dwRenderRangeTime)/double(s_dwRenderRangeFrame));

					s_dwRenderCheckTime=ELTimer_GetMSec();
					s_dwRenderRangeTime=0;
					s_dwRenderRangeFrame=0;
				}										

				DWORD dwCurFaceCount=m_pyGraphic.GetFaceCount();
				m_pyGraphic.ResetFaceCount();
				s_dwFaceCount += dwCurFaceCount;

				if (dwCurFaceCount > 5000)
				{
					m_dwFaceAccCount += dwCurFaceCount;
					m_dwFaceAccTime += m_dwCurRenderTime;

					m_fFaceSpd=(m_dwFaceAccCount/m_dwFaceAccTime);

					// Automatic view distance adjustment
					if (-1 == m_iForceSightRange)
					{
						static float s_fAveRenderTime = 16.0f;
						float fRatio=0.3f;
						s_fAveRenderTime=(s_fAveRenderTime*(100.0f-fRatio)+std::max(16.0f, (float)m_dwCurRenderTime)*fRatio)/100.0f;


						float fFar=25600.0f;
						float fNear=MIN_FOG;
						double dbAvePow=double(1000.0f/s_fAveRenderTime);
						double dbMaxPow=60.0;
						float fDistance=std::max((float)(fNear+(fFar-fNear)*(dbAvePow)/dbMaxPow), fNear);
						m_pyBackground.SetViewDistanceSet(0, fDistance);
					}
					// Forced view distance setting
					else
					{
						m_pyBackground.SetViewDistanceSet(0, float(m_iForceSightRange));
					}
				}
				else
				{
					// When polygon count is under 10000, set maximum view distance
					m_pyBackground.SetViewDistanceSet(0, 25600.0f);
				}

				++s_dwRenderFrameCount;
			}
		}
	}

	// Pomiar czystego czasu pracy CPU przed wejsciem w faze oczekiwania Frame Pacera
	s_uiLoad += ELTimer_GetMSec() - dwStart;

	// Faza 8: UJEDNOLICONY FRAME PACING (zastepuje rozproszone sleep_for(15), sleep_for(1) i Sleep(10))
	if (UserInterface::TestHarness::DeterministicTickController::Instance().IsTestMode())
	{
		UserInterface::TestHarness::DeterministicTickController::Instance().ThrottleFrameLoop(g_bHeadlessMode, 60);
	}
	else if (!ms_bInModalSizeMove.load(std::memory_order_acquire))
	{
		std::chrono::nanoseconds targetFrameTime;
		if (!canRender && m_pyGraphic.IsLostDevice())
		{
			// T178: Lost Device throttle (50ms) during Win+L lock or screensaver
			targetFrameTime = std::chrono::nanoseconds(50'000'000);
		}
		else if (g_bHeadlessMode || m_isMinimizedWnd || !canRender)
		{
			// W trybie headless / zminimalizowanym utrzymujemy stabilny puls 60 Hz (16.666 ms)
			targetFrameTime = std::chrono::nanoseconds(16'666'667);
		}
		else
		{
			const uint32_t targetFps = (m_iFPS > 60) ? m_iFPS : 144;
			targetFrameTime = std::chrono::nanoseconds(1'000'000'000ULL / targetFps);
		}

		s_framePacer.WaitForNextFrame(targetFrameTime, !g_bHeadlessMode);
	}

	return true;
}

bool CPythonApplication::ProcessFrame()
{
	return Process();
}


void CPythonApplication::UpdateClientRect()
{
	RECT rcApp;
	GetClientRect(&rcApp);
	OnSizeChange(rcApp.right - rcApp.left, rcApp.bottom - rcApp.top);
}

void CPythonApplication::SetMouseHandler(PyObject* poMouseHandler)
{	
	m_poMouseHandler = poMouseHandler;
}

int CPythonApplication::CheckDeviceState()
{
	CGraphicDevice::EDeviceState e_deviceState = m_grpDevice.GetDeviceState();

	switch (e_deviceState)
	{
		// µð¹ÙÀÌ½º°¡ ¾øÀ¸¸é ÇÁ·Î±×·¥ÀÌ Á¾·á µÇ¾î¾ß ÇÑ´Ù.
	case CGraphicDevice::DEVICESTATE_NULL:
		return DEVICE_STATE_FALSE;

		// DEVICESTATE_BROKENÀÏ ¶§´Â ´ÙÀ½ ·çÇÁ¿¡¼­ º¹±¸ µÉ ¼ö ÀÖµµ·Ï ¸®ÅÏ ÇÑ´Ù.
		// ±×³É ÁøÇàÇÒ °æ¿ì DrawPrimitive °°Àº °ÍÀ» ÇÏ¸é ÇÁ·Î±×·¥ÀÌ ÅÍÁø´Ù.
	case CGraphicDevice::DEVICESTATE_BROKEN:
		return DEVICE_STATE_SKIP;

	case CGraphicDevice::DEVICESTATE_NEEDS_RESET:
		if (!m_grpDevice.Reset())
			return DEVICE_STATE_SKIP;

		break;
	}

	return DEVICE_STATE_OK;
}

bool CPythonApplication::CreateDevice(int width, int height, int Windowed, int bit, int frequency)
{
	int iRet;

	m_grpDevice.InitBackBufferCount(2);
	iRet = m_grpDevice.Create(GetWindowHandle(), width, height, Windowed ? true : false, bit, frequency);

	switch (iRet)
	{
		case CGraphicDevice::CREATE_OK:
			return true;

		case CGraphicDevice::CREATE_REFRESHRATE:
			return true;

		case CGraphicDevice::CREATE_ENUM:
		case CGraphicDevice::CREATE_DETECT:
			SET_EXCEPTION(CREATE_NO_APPROPRIATE_DEVICE);
			TraceError("CreateDevice: Enum & Detect failed");
			return false;

		case CGraphicDevice::CREATE_NO_DIRECTX:
			SET_EXCEPTION(CREATE_NO_DIRECTX);
			TraceError("CreateDevice: DirectX 8.1 or greater required to run game");
			return false;

		case CGraphicDevice::CREATE_DEVICE:
			SET_EXCEPTION(CREATE_DEVICE);
			TraceError("CreateDevice: GraphicDevice create failed");
			return false;

		case CGraphicDevice::CREATE_FORMAT:
			SET_EXCEPTION(CREATE_FORMAT);
			TraceError("CreateDevice: Change the screen format");
			return false;

		case CGraphicDevice::CREATE_GET_DEVICE_CAPS:
			PyErr_SetString(PyExc_RuntimeError, "GetDevCaps failed");
			TraceError("CreateDevice: GetDevCaps failed");
			return false;

		case CGraphicDevice::CREATE_GET_DEVICE_CAPS2:
			PyErr_SetString(PyExc_RuntimeError, "GetDevCaps2 failed");
			TraceError("CreateDevice: GetDevCaps2 failed");
			return false;

		default:
			if (iRet & CGraphicDevice::CREATE_OK)
			{
				if (iRet & CGraphicDevice::CREATE_NO_TNL)
				{
					CGrannyLODController::SetMinLODMode(true);
				}
				return true;
			}

			SET_EXCEPTION(UNKNOWN_ERROR);
			TraceError("CreateDevice: Unknown Error!");
			return false;
	}
}

void CPythonApplication::SetUserMovingMainWindow(bool flag)
{
	if (flag && !GetCursorPos(&m_InitialMouseMovingPoint))
		return;

	m_IsMovingMainWindow = flag;
}

bool CPythonApplication::IsUserMovingMainWindow() const
{
	return m_IsMovingMainWindow;
}

void CPythonApplication::UpdateMainWindowPosition()
{
	POINT finalPoint{};
	if (GetCursorPos(&finalPoint))
	{
		LONG xDiff = finalPoint.x - m_InitialMouseMovingPoint.x;
		LONG yDiff = finalPoint.y - m_InitialMouseMovingPoint.y;

		RECT r{};
		GetWindowRect(&r);

		SetPosition(r.left + xDiff, r.top + yDiff);
		m_InitialMouseMovingPoint = finalPoint;
	}
}

void CPythonApplication::Loop()
{	
	if (m_dwTurboTicks > 0)
	{
		TraceError("CPythonApplication::Loop: Starting deterministic turbo-ticks simulation (%u ticks)...", m_dwTurboTicks);

		for (uint32_t currentTick = 0; currentTick < m_dwTurboTicks; ++currentTick)
		{
			MSG msg{};
			uint32_t pumpedCount = 0;
			constexpr uint32_t MAX_TURBO_MESSAGES = 64;
			while (pumpedCount < MAX_TURBO_MESSAGES && ::PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE))
			{
				if (msg.message == WM_QUIT)
				{
					TraceError("CPythonApplication::Loop: WM_QUIT received at tick %u/%u", currentTick, m_dwTurboTicks);
					return;
				}
				::TranslateMessage(&msg);
				::DispatchMessage(&msg);
				++pumpedCount;
			}

			// Filar 4: TestHarnessEngine - Przetworzenie kolejki watku glownego z serwera Named Pipe
			UserInterface::TestHarness::TestHarnessServer::Instance().ProcessMainThreadQueue();

			// Filar 4: MockWorldDriver - wejscie w mock world jesli flaga --mock-world aktywna
			if (UserInterface::TestHarness::MockWorldDriver::Instance().IsMockWorldEnabled() &&
				!UserInterface::TestHarness::MockWorldDriver::Instance().HasEntered())
			{
				UserInterface::TestHarness::MockWorldDriver::Instance().EnterMockWorld();
			}

			ELTimer_SetFrameMSec();

			// Deterministyczny krok czasowy logiki gry (16.666ms na tick)
			CTimer& rkTimer = CTimer::Instance();
			rkTimer.AdvanceFixed(1.0 / 60.0);

			m_fGlobalTime = rkTimer.GetCurrentSecond();
			m_fGlobalElapsedTime = rkTimer.GetElapsedSecond();

			// Network I/O
			m_pyNetworkStream.Process();
			m_kGuildMarkUploader.Process();
			m_kGuildMarkDownloader.Process();
			m_kAccountConnector.Process();
			UserInterface::FastBoot::FastBootOverlay::Instance().Update();

			// Update Game Playing
			CResourceManager::Instance().Update();
			OnCameraUpdate();
			OnMouseUpdate();
			OnUIUpdate();

			UserInterface::TestHarness::DeterministicTickController::Instance().IncrementTickCount();
		}

		TraceError("CPythonApplication::Loop: Deterministic turbo-ticks simulation finished successfully (%u ticks). Exiting cleanly.", m_dwTurboTicks);
		return;
	}

	s_framePacer.Reset();

	while (1)
	{	
		const auto pumpStart = std::chrono::steady_clock::now();
		uint32_t pumpedMessages = 0;
		bool hasMouseMove = false;
		MSG lastMouseMoveMsg{};
		MSG msg{};

		while (pumpedMessages < MAX_MESSAGES_PER_FRAME &&
		       (std::chrono::steady_clock::now() - pumpStart) < MAX_MESSAGE_PUMP_BUDGET &&
		       ::PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE))
		{
			if (msg.message == WM_QUIT)
			{
				TraceError("CPythonApplication::Loop: WM_QUIT received");
				return;
			}

			if (msg.message == WM_MOUSEMOVE)
			{
				lastMouseMoveMsg = msg;
				hasMouseMove = true;
				++pumpedMessages;
				continue;
			}

			::TranslateMessage(&msg);
			::DispatchMessage(&msg);
			++pumpedMessages;
		}

		if (hasMouseMove)
		{
			::TranslateMessage(&lastMouseMoveMsg);
			::DispatchMessage(&lastMouseMoveMsg);
		}

		if (IsUserMovingMainWindow())
			UpdateMainWindowPosition();

		if (!ProcessFrame())
		{
			TraceError("CPythonApplication::Loop: ProcessFrame() returned false");
			break;
		}

		m_dwLastIdleTime = ELTimer_GetMSec();
	}
}

bool LoadLocaleData(const char* localePath)
{
	CPythonNonPlayer&	rkNPCMgr	= CPythonNonPlayer::Instance();
	CItemManager&		rkItemMgr	= CItemManager::Instance();	
	CPythonSkill&		rkSkillMgr	= CPythonSkill::Instance();
	CPythonNetworkStream& rkNetStream = CPythonNetworkStream::Instance();

	char szItemList[256];
	char szItemProto[256];
	char szItemDesc[256];
	char szMobProto[256];
	char szSkillDescFileName[256];
	char szSkillTableFileName[256];
	char szInsultList[256];

	snprintf (szItemList,	sizeof (szItemList),	"%s/item_list.txt", GetLocalePathCommon());
	snprintf (szItemProto,	sizeof (szItemProto),	"%s/item_proto",	localePath);
	snprintf (szItemDesc,	sizeof (szItemDesc),	"%s/itemdesc.txt",	localePath);
	snprintf (szMobProto,	sizeof (szMobProto),	"%s/mob_proto",		localePath);
	snprintf (szSkillDescFileName, sizeof (szSkillDescFileName),	"%s/SkillDesc.txt", localePath);
	snprintf (szSkillTableFileName, sizeof (szSkillTableFileName),	"%s/SkillTable.txt", GetLocalePathCommon());
	snprintf (szInsultList,	sizeof (szInsultList),	"%s/insult.txt", localePath);

	rkNPCMgr.Destroy();
	rkItemMgr.Destroy();
	rkSkillMgr.Destroy();

	if (!rkItemMgr.LoadItemList(szItemList))
	{
		TraceError("LoadLocaleData - LoadItemList(%s) Error", szItemList);
	}

	if (!rkItemMgr.LoadItemTable(szItemProto))
	{
		TraceError("LoadLocaleData - LoadItemProto(%s) Error", szItemProto);
	}

	if (!rkItemMgr.LoadItemDesc(szItemDesc))
	{
		Tracenf("LoadLocaleData - LoadItemDesc(%s) Error", szItemDesc);
	}

	if (!rkNPCMgr.LoadNonPlayerData(szMobProto))
	{
		TraceError("LoadLocaleData - LoadMobProto(%s) Error", szMobProto);
	}

	if (!rkSkillMgr.RegisterSkillDesc(szSkillDescFileName))
	{
		TraceError("LoadLocaleData - RegisterSkillDesc(%s) Error", szSkillDescFileName);
	}

	if (!rkSkillMgr.RegisterSkillTable(szSkillTableFileName))
	{
		TraceError("LoadLocaleData - RegisterSkillTable(%s) Error", szSkillTableFileName);
	}

	if (!rkNetStream.LoadInsultList(szInsultList))
	{
		Tracenf("CPythonApplication - CPythonNetworkStream::LoadInsultList(%s)", szInsultList);
	}

	return true;
}

unsigned __GetWindowMode(bool windowed)
{
	if (windowed)
		return WS_OVERLAPPED | WS_CAPTION |   WS_SYSMENU | WS_MINIMIZEBOX;

	return WS_POPUP;
}

bool CPythonApplication::Create(PyObject * poSelf, const char * c_szName, int width, int height, int Windowed)
{
	// Initialize Game Thread Pool first - required by other systems
	CGameThreadPool* pThreadPool = CGameThreadPool::InstancePtr();
	if (pThreadPool)
	{
		pThreadPool->Initialize();
	}

	Windowed = CPythonSystem::Instance().IsWindowed() ? 1 : 0;

	bool bAnotherWindow = false;

	std::wstring wWindowName = Utf8ToWide(c_szName ? c_szName : "");

	if (FindWindowW(nullptr, wWindowName.c_str()))
		bAnotherWindow = true;

	m_dwWidth = width;
	m_dwHeight = height;

	// Window
	UINT WindowMode = __GetWindowMode(Windowed ? true : false);

	if (!CMSWindow::Create(c_szName, 4, 0, WindowMode, ::LoadIcon( GetInstance(), MAKEINTRESOURCE( IDI_METIN2 ) ), IDC_CURSOR_NORMAL))
	{
		TraceError("CMSWindow::Create failed");
		SET_EXCEPTION(CREATE_WINDOW);
		return false;
	}

	if (m_pySystem.IsUseDefaultIME())
	{
		CPythonIME::Instance().UseDefaultIME();
	}

#if defined(ENABLE_DISCORD_RPC)
	m_pyNetworkStream.Discord_Start();
#endif

	if (!m_pySystem.IsWindowed())
	{
		m_isWindowed = false;
		m_isWindowFullScreenEnable = TRUE;
		__SetFullScreenWindow(GetWindowHandle(), width, height, m_pySystem.GetBPP());

		Windowed = true;
	}
	else
	{
		AdjustSize(m_pySystem.GetWidth(), m_pySystem.GetHeight());

		if (Windowed)
		{
			m_isWindowed = true;

			if (bAnotherWindow)
			{
				RECT rc;

				GetClientRect(&rc);

				int windowWidth = rc.right - rc.left;
				int windowHeight = (rc.bottom - rc.top);

				CMSApplication::SetPosition(GetScreenWidth() - windowWidth, GetScreenHeight() - 60 - windowHeight);
			}
			SetPosition(-8, 0); //Fix
		}
		else
		{
			m_isWindowed = false;
			SetPosition(0, 0);
		}
	}

		///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// Cursor
		if (!CreateCursors())
		{
			TraceError("CMSWindow::Cursors Create Error");
			SET_EXCEPTION("CREATE_CURSOR");
			return false;
		}

		if (!m_pySystem.IsNoSoundCard() && !g_bHeadlessMode)
		{
			// Sound
			if (!m_SoundEngine.Initialize())
			{
				TraceError("Failed to initialize sound manager!");
				return false; // Is this important enough to stop the client?
			}
		}

		extern bool GRAPHICS_CAPS_SOFTWARE_TILING;

		if (!m_pySystem.IsAutoTiling())
			GRAPHICS_CAPS_SOFTWARE_TILING = m_pySystem.IsSoftwareTiling();

		// Device
		if (!CreateDevice(m_pySystem.GetWidth(), m_pySystem.GetHeight(), Windowed, m_pySystem.GetBPP(), m_pySystem.GetFrequency()))
			return false;

		CD3D9RenderBackend::Instance().Initialize(GetWindowHandle(), m_pySystem.GetWidth(), m_pySystem.GetHeight());
		CRenderQueueManager::Instance().SetActiveBackend(&CD3D9RenderBackend::Instance());
		if (RenderOwnership::IsWgpuConfigured())
		{
			if (CWgpuRenderBackend::Instance().Initialize(GetWindowHandle(), m_pySystem.GetWidth(), m_pySystem.GetHeight()))
			{
				CRenderQueueManager::Instance().SetActiveBackend(&CWgpuRenderBackend::Instance());
			}
		}

		if (!g_bHeadlessMode)
		{
			UserInterface::FastBoot::FastBootOverlay::Instance().Initialize(GetWindowHandle(), STATEMANAGER.GetDevice());

			extern bool g_bFastBootAutoConnect;
			if (g_bFastBootAutoConnect && !UserInterface::FastBoot::FastBootOverlay::Instance().IsConnecting())
			{
				UserInterface::FastBoot::FastBootOverlay::Instance().ConnectCurrentProfile();
			}
		}

		if (m_pySystem.IsAutoTiling())
		{
			if (m_grpDevice.IsFastTNL())
			{
				m_pyBackground.ReserveSoftwareTilingEnable(false);
			}
			else
			{
				m_pyBackground.ReserveSoftwareTilingEnable(true);
			}
		}
		else
		{
			m_pyBackground.ReserveSoftwareTilingEnable(m_pySystem.IsSoftwareTiling());
		}

		if (!g_bHeadlessMode)
		{
			if (s_bWindowVisibleState && !m_bRenderThrottling.load())
			{
				SetVisibleMode(true);

				if (m_isWindowFullScreenEnable)
				{
					SetWindowPos(GetWindowHandle(), HWND_TOP, 0, 0, width, height, SWP_SHOWWINDOW);
				}
			}
			else
			{
				SetVisibleMode(false);
				SetRenderThrottling(true);
			}
		}

		if (!InitializeKeyboard(GetWindowHandle()))
			return false;

		m_pySystem.GetDisplaySettings();

		// Mouse
		if (m_pySystem.IsSoftwareCursor())
			SetCursorMode(CURSOR_MODE_SOFTWARE);
		else
			SetCursorMode(CURSOR_MODE_HARDWARE);

		// Network
		if (!m_netDevice.Create())
		{
			TraceError("NetDevice::Create failed");
			SET_EXCEPTION("CREATE_NETWORK");
			return false;
		}

		if (!m_grpDevice.IsFastTNL())
			CGrannyLODController::SetMinLODMode(true);

		m_RaceManager.Create();
		m_pyItem.Create();

		// Other Modules
		DefaultFont_Startup();

		CPythonIME::Instance().Create(GetWindowHandle());
		CPythonIME::Instance().SetText("", 0);
		CPythonTextTail::Instance().Initialize();

		// Light Manager
		m_LightManager.Initialize();

		CGraphicImageInstance::CreateSystem(32);

		// ¹é¾÷
		STICKYKEYS sStickKeys;
		memset(&sStickKeys, 0, sizeof(sStickKeys));
		sStickKeys.cbSize = sizeof(sStickKeys);
		SystemParametersInfo( SPI_GETSTICKYKEYS, sizeof(sStickKeys), &sStickKeys, 0 );
		m_dwStickyKeysFlag = sStickKeys.dwFlags;

		// ¼³Á¤
		sStickKeys.dwFlags &= ~(SKF_AVAILABLE|SKF_HOTKEYACTIVE);
		SystemParametersInfo( SPI_SETSTICKYKEYS, sizeof(sStickKeys), &sStickKeys, 0 );

		// SphereMap
		CGrannyMaterial::CreateSphereMap(0, "d:/ymir work/special/spheremap.jpg");
		CGrannyMaterial::CreateSphereMap(1, "d:/ymir work/special/spheremap01.jpg");
		return true;
}

void CPythonApplication::SetGlobalCenterPosition(int32_t x, int32_t y)
{
	CPythonBackground& rkBG=CPythonBackground::Instance();
	rkBG.GlobalPositionToLocalPosition(x, y);

	float z = CPythonBackground::Instance().GetHeight(x, y);

	CPythonApplication::Instance().SetCenterPosition(x, y, z);
}

void CPythonApplication::SetCenterPosition(float fx, float fy, float fz)
{
	m_v3CenterPosition.x = +fx;
	m_v3CenterPosition.y = -fy;
	m_v3CenterPosition.z = +fz;
}

void CPythonApplication::GetCenterPosition(TPixelPosition * pPixelPosition)
{
	pPixelPosition->x = +m_v3CenterPosition.x;
	pPixelPosition->y = -m_v3CenterPosition.y;
	pPixelPosition->z = +m_v3CenterPosition.z;
}


void CPythonApplication::SetServerTime(time_t tTime)
{
	m_dwStartLocalTime	= ELTimer_GetMSec();
	m_tServerTime		= tTime;
	m_tLocalStartTime	= time(0);
}

time_t CPythonApplication::GetServerTime()
{
	return (ELTimer_GetMSec() - m_dwStartLocalTime) + m_tServerTime;
}

// 2005.03.28 - MALL ¾ÆÀÌÅÛ¿¡ µé¾îÀÖ´Â ½Ã°£ÀÇ ´ÜÀ§°¡ ¼­¹ö¿¡¼­ time(0) À¸·Î ¸¸µé¾îÁö´Â
//              °ªÀÌ±â ¶§¹®¿¡ ´ÜÀ§¸¦ ¸ÂÃß±â À§ÇØ ½Ã°£ °ü·Ã Ã³¸®¸¦ º°µµ·Î Ãß°¡
time_t CPythonApplication::GetServerTimeStamp()
{
	return (time(0) - m_tLocalStartTime) + m_tServerTime;
}

float CPythonApplication::GetGlobalTime()
{
	return m_fGlobalTime;
}

float CPythonApplication::GetGlobalElapsedTime()
{
	return m_fGlobalElapsedTime;
}

void CPythonApplication::SetFPS(int iFPS)
{
	m_iFPS = iFPS;
}

int CPythonApplication::GetWidth()
{
	return m_dwWidth;
}

int CPythonApplication::GetHeight()
{
	return m_dwHeight;
}

void CPythonApplication::SetConnectData(const char * c_szIP, int iPort)
{
	m_strIP = c_szIP;
	m_iPort = iPort;
}

void CPythonApplication::GetConnectData(std::string & rstIP, int & riPort)
{
	rstIP	= m_strIP;
	riPort	= m_iPort;
}

void CPythonApplication::EnableSpecialCameraMode()
{
	m_isSpecialCameraMode = TRUE;
}

void CPythonApplication::SetCameraSpeed(int iPercentage)
{
	m_fCameraRotateSpeed = c_fDefaultCameraRotateSpeed * float(iPercentage) / 100.0f;
	m_fCameraPitchSpeed = c_fDefaultCameraPitchSpeed * float(iPercentage) / 100.0f;
	m_fCameraZoomSpeed = c_fDefaultCameraZoomSpeed * float(iPercentage) / 100.0f;
}

void CPythonApplication::SetForceSightRange(int iRange)
{
	m_iForceSightRange = iRange;
}

void CPythonApplication::Clear()
{
	m_pySystem.Clear();
}

void CPythonApplication::Destroy()
{
	// SphereMap
	CGrannyMaterial::DestroySphereMap();

	m_kWndMgr.Destroy();

	CPythonSystem::Instance().SaveConfig();

	DestroyCollisionInstanceSystem();

	m_pySystem.SaveInterfaceStatus();

	m_pyEventManager.Destroy();	
	m_FlyingManager.Destroy();

	m_pyMiniMap.Destroy();

	m_pyTextTail.Destroy();
	m_pyChat.Destroy();	
	m_kChrMgr.Destroy();
	m_RaceManager.Destroy();

	m_pyItem.Destroy();
	m_kItemMgr.Destroy();

	m_pyBackground.Destroy();

	m_kEftMgr.Destroy();
	m_LightManager.Destroy();

	// Game Thread Pool
	CGameThreadPool::Instance().Destroy();

	// DEFAULT_FONT
	DefaultFont_Cleanup();
	// END_OF_DEFAULT_FONT

	m_pyGraphic.Destroy();
	
#if defined(ENABLE_DISCORD_RPC)
	m_pyNetworkStream.Discord_Close();
#endif	
	
	//m_pyNetworkDatagram.Destroy();	

	m_pyRes.Destroy();

	m_kGuildMarkDownloader.Disconnect();

	CGrannyModelInstance::DestroySystem();
	CGraphicImageInstance::DestroySystem();

	CWgpuRenderBackend::Instance().Shutdown();
	CD3D9RenderBackend::Instance().Shutdown();
	CWgpuModule::Instance().FreeLibrary();

	m_grpDevice.Destroy();

	//CSpeedTreeForestDirectX::Instance().Clear();

	CAttributeInstance::DestroySystem();
	CTextFileLoader::DestroySystem();
	DestroyCursors();

	CMSApplication::Destroy();

	STICKYKEYS sStickKeys;
	memset(&sStickKeys, 0, sizeof(sStickKeys));
	sStickKeys.cbSize = sizeof(sStickKeys);
	sStickKeys.dwFlags = m_dwStickyKeysFlag;
	SystemParametersInfo( SPI_SETSTICKYKEYS, sizeof(sStickKeys), &sStickKeys, 0 );
}

bool CPythonApplication::SaveScreenShot(const char* c_szFileName)
{
	LPDIRECT3DDEVICE9 lpDevice = STATEMANAGER.GetDevice();
	if (!lpDevice || !c_szFileName || !c_szFileName[0])
		return false;

	LPDIRECT3DSURFACE9 pBackBuffer = nullptr;
	if (FAILED(lpDevice->GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO, &pBackBuffer)) || !pBackBuffer)
		return false;

	D3DSURFACE_DESC desc;
	pBackBuffer->GetDesc(&desc);

	LPDIRECT3DSURFACE9 pOffscreenSurface = nullptr;
	if (FAILED(lpDevice->CreateOffscreenPlainSurface(desc.Width, desc.Height, desc.Format, D3DPOOL_SYSTEMMEM, &pOffscreenSurface, nullptr)) || !pOffscreenSurface)
	{
		pBackBuffer->Release();
		return false;
	}

	if (FAILED(lpDevice->GetRenderTargetData(pBackBuffer, pOffscreenSurface)))
	{
		pOffscreenSurface->Release();
		pBackBuffer->Release();
		return false;
	}

	D3DLOCKED_RECT lockedRect;
	if (FAILED(pOffscreenSurface->LockRect(&lockedRect, nullptr, D3DLOCK_READONLY)))
	{
		pOffscreenSurface->Release();
		pBackBuffer->Release();
		return false;
	}

	// Convert D3DFMT_X8R8G8B8 or A8R8G8B8 to RGBA
	std::vector<uint8_t> rgba(desc.Width * desc.Height * 4);
	const uint8_t* pSrc = static_cast<const uint8_t*>(lockedRect.pBits);
	for (UINT y = 0; y < desc.Height; ++y)
	{
		const uint32_t* pSrcRow = reinterpret_cast<const uint32_t*>(pSrc + y * lockedRect.Pitch);
		uint8_t* pDstRow = rgba.data() + y * desc.Width * 4;
		for (UINT x = 0; x < desc.Width; ++x)
		{
			uint32_t pixel = pSrcRow[x];
			pDstRow[x * 4 + 0] = static_cast<uint8_t>((pixel >> 16) & 0xFF); // R
			pDstRow[x * 4 + 1] = static_cast<uint8_t>((pixel >> 8) & 0xFF);  // G
			pDstRow[x * 4 + 2] = static_cast<uint8_t>(pixel & 0xFF);         // B
			pDstRow[x * 4 + 3] = 0xFF;                                       // A
		}
	}

	pOffscreenSurface->UnlockRect();
	pOffscreenSurface->Release();
	pBackBuffer->Release();

	return stbi_write_png(c_szFileName, desc.Width, desc.Height, 4, rgba.data(), desc.Width * 4) != 0;
}

void CPythonApplication::FlushFrameUI()
{
	CDynamicQuadBatcher::Instance().FlushToQueue();
	CRenderQueueManager::Instance().FlushUI();
}


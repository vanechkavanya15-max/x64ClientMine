#include "StdAfx.h"
#include "PythonWindow.h"
#include "PythonSlotWindow.h"
#include "PythonGridSlotWindow.h"
#include "PythonInternedStrings.h"
#include "Client/UI/WindowManagerButtonApiBridge.h"
#include "Client/UI/WindowManagerLayoutApiBridge.h"
#include "Client/UI/WindowManagerSlotApiBridge.h"

namespace {
	static std::unordered_map<const UI::CWindow*, uint32_t> s_winToId;
	static std::unordered_map<uint32_t, UI::CWindow*> s_idToWin;
	static uint32_t s_nextId = 1;

	inline uint32_t WindowToId(const UI::CWindow* pWin)
	{
		if (!pWin)
			return 0;
		auto it = s_winToId.find(pWin);
		if (it != s_winToId.end())
			return it->second;
		uint32_t id = s_nextId++;
		s_winToId[pWin] = id;
		s_idToWin[id] = const_cast<UI::CWindow*>(pWin);
		return id;
	}

	inline UI::CWindow* IdToWindow(uint32_t id)
	{
		if (id == 0)
			return nullptr;
		auto it = s_idToWin.find(id);
		if (it != s_idToWin.end())
			return it->second;
		return nullptr;
	}

	inline void UnregisterWindowId(const UI::CWindow* pWin)
	{
		if (!pWin)
			return;
		auto it = s_winToId.find(pWin);
		if (it != s_winToId.end())
		{
			s_idToWin.erase(it->second);
			s_winToId.erase(it);
		}
	}

	inline void EnsureLayoutWindow(uint32_t winId)
	{
		if (winId == 0)
			return;
		auto& layoutBridge = Client::UI::WindowManagerLayoutApiBridge::Instance();
		if (!layoutBridge.HasWindow(winId))
			layoutBridge.CreateWindow(winId);
	}
}

bool PyTuple_GetWindow(PyObject* poArgs, int pos, UI::CWindow ** ppRetWindow)
{
	if (!UI::CWindowManager::InstancePtr())
	{
		if (ppRetWindow)
			*ppRetWindow = nullptr;
		return false;
	}

	if (!PyTuple_GetPointer(poArgs, pos, ppRetWindow))
		return false;

	return *ppRetWindow != nullptr;
}

bool PyFastCall_GetWindow(PyObject * poArg, UI::CWindow ** ppRetWindow)
{
	if (!UI::CWindowManager::InstancePtr())
	{
		if (ppRetWindow)
			*ppRetWindow = nullptr;
		return false;
	}

	if (!poArg || !PyLong_Check(poArg))
		return false;

	unsigned long long ullVal = PyLong_AsUnsignedLongLong(poArg);
	if (ullVal == (unsigned long long)-1)
	{
		if (PyErr_Occurred())
			PyErr_Clear();
		return false;
	}

	*ppRetWindow = reinterpret_cast<UI::CWindow*>(ullVal);
	return *ppRetWindow != nullptr;
}

bool PyFastCall_GetWindow(PyObject * const * poArgs, Py_ssize_t nargs, Py_ssize_t pos, UI::CWindow ** ppRetWindow)
{
	if (pos < 0 || pos >= nargs)
		return false;

	return PyFastCall_GetWindow(poArgs[pos], ppRetWindow);
}

bool PyTuple_GetWindowOrId(PyObject* poArgs, int pos, UI::CWindow ** ppRetWindow, uint32_t * pRetId)
{
	if (ppRetWindow)
		*ppRetWindow = nullptr;
	if (pRetId)
		*pRetId = 0;

	if (!poArgs)
		return false;

	if (PyTuple_GetWindow(poArgs, pos, ppRetWindow) && *ppRetWindow)
	{
		if (pRetId)
			*pRetId = WindowToId(*ppRetWindow);
		return true;
	}

	unsigned int uId = 0;
	if (PyTuple_GetUnsignedInteger(poArgs, pos, &uId) && uId > 0)
	{
		if (pRetId)
			*pRetId = uId;
		if (ppRetWindow)
			*ppRetWindow = IdToWindow(uId);
		return true;
	}

	int iId = 0;
	if (PyTuple_GetInteger(poArgs, pos, &iId) && iId > 0)
	{
		if (pRetId)
			*pRetId = static_cast<uint32_t>(iId);
		if (ppRetWindow)
			*ppRetWindow = IdToWindow(static_cast<uint32_t>(iId));
		return true;
	}

	return false;
}

PyObject * wndMgrGetAspect(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("f", 0.0f);

	return Py_BuildValue("f", UI::CWindowManager::Instance().GetAspect());
}

PyObject * wndMgrOnceIgnoreMouseLeftButtonUpEvent(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindowManager::Instance().OnceIgnoreMouseLeftButtonUpEvent();
	return Py_BuildNone();
}

PyObject * wndMgrSetMouseHandler(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	PyObject * poHandler;
	if (!PyTuple_GetObject(poArgs, 0, &poHandler))
		return Py_BuildException();

	UI::CWindowManager::Instance().SetMouseHandler(poHandler);
	return Py_BuildNone();
}

/////////////////////////////////////////////////////////////////////////////////////////////////
///// Register /////
// Window
PyObject * wndMgrRegister(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindowManager& kWndMgr=UI::CWindowManager::Instance();
	UI::CWindow * pWindow = kWndMgr.RegisterWindow(po, szLayer);
	if (!pWindow)
		return Py_BuildException();
	
	return Py_BuildValue("K", pWindow);
}

// SlotWindow
PyObject * wndMgrRegisterSlotWindow(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterSlotWindow(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// GridSlotWindow
PyObject * wndMgrRegisterGridSlotWindow(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterGridSlotWindow(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// TextLine
PyObject * wndMgrRegisterTextLine(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterTextLine(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// MarkBox
PyObject * wndMgrRegisterMarkBox(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterMarkBox(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// ImageBox
PyObject * wndMgrRegisterImageBox(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterImageBox(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// ExpandedImageBox
PyObject * wndMgrRegisterExpandedImageBox(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterExpandedImageBox(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// AniImageBox
PyObject * wndMgrRegisterAniImageBox(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterAniImageBox(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// RegisterButton
PyObject * wndMgrRegisterButton(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterButton(po, szLayer);
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerButtonApiBridge::Instance().RegisterButton(
			Client::UI::ButtonId(winId), Client::UI::ButtonType::Push);
	}
	return Py_BuildValue("K", pWindow);
}

// RadioButton
PyObject * wndMgrRegisterRadioButton(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterRadioButton(po, szLayer);
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerButtonApiBridge::Instance().RegisterButton(
			Client::UI::ButtonId(winId), Client::UI::ButtonType::Radio);
	}
	return Py_BuildValue("K", pWindow);
}

// ToggleButton
PyObject * wndMgrRegisterToggleButton(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterToggleButton(po, szLayer);
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerButtonApiBridge::Instance().RegisterButton(
			Client::UI::ButtonId(winId), Client::UI::ButtonType::Toggle);
	}
	return Py_BuildValue("K", pWindow);
}

// DragButton
PyObject * wndMgrRegisterDragButton(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterDragButton(po, szLayer);
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerButtonApiBridge::Instance().RegisterButton(
			Client::UI::ButtonId(winId), Client::UI::ButtonType::Push);
	}
	return Py_BuildValue("K", pWindow);
}

// Box
PyObject * wndMgrRegisterBox(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterBox(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// Bar
PyObject * wndMgrRegisterBar(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterBar(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// Line
PyObject * wndMgrRegisterLine(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterLine(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// Slot
PyObject * wndMgrRegisterBar3D(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterBar3D(po, szLayer);
	return Py_BuildValue("K", pWindow);
}

// NumberLine
PyObject * wndMgrRegisterNumberLine(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	PyObject * po;
	if (!PyTuple_GetObject(poArgs, 0, &po))
		return Py_BuildException();
	char * szLayer;
	if (!PyTuple_GetString(poArgs, 1, &szLayer))
		return Py_BuildException();

	UI::CWindow * pWindow = UI::CWindowManager::Instance().RegisterNumberLine(po, szLayer);
	return Py_BuildValue("K", pWindow);
}
///// Register /////
/////////////////////////////////////////////////////////////////////////////////////////////////

PyObject * wndMgrDestroy(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		Client::UI::WindowManagerButtonApiBridge::Instance().UnregisterButton(Client::UI::ButtonId(winId));
		Client::UI::WindowManagerLayoutApiBridge::Instance().DestroyWindow(winId);
		UnregisterWindowId(pWin);
	}

	UI::CWindowManager::Instance().DestroyWindow(pWin);
	return Py_BuildNone();
}

PyObject * wndMgrIsFocus(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("K", 0);

	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("K", pWindow == UI::CWindowManager::Instance().GetActivateWindow());
}

PyObject * wndMgrSetFocus(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	UI::CWindowManager::Instance().ActivateWindow(pWin);
	return Py_BuildNone();
}

PyObject * wndMgrKillFocus(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (PyTuple_GetWindow(poArgs, 0, &pWin))
	{
		if (pWin == UI::CWindowManager::Instance().GetActivateWindow())
		{
			UI::CWindowManager::Instance().DeactivateWindow();
		}
	}
	else
	{
		UI::CWindowManager::Instance().DeactivateWindow();
	}

	return Py_BuildNone();
}

PyObject * wndMgrLock(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	UI::CWindowManager::Instance().LockWindow(pWin);
	return Py_BuildNone();
}

PyObject * wndMgrUnlock(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindowManager::Instance().UnlockWindow();
	return Py_BuildNone();
}

PyObject * wndMgrSetName(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	char * szName;
	if (!PyTuple_GetString(poArgs, 1, &szName))
		return Py_BuildException();

	pWin->SetName(szName);
	return Py_BuildNone();
}

PyObject * wndMgrSetTop(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	UI::CWindowManager::Instance().SetTop(pWin);
	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerLayoutApiBridge::Instance().BringToFront(winId);
	}
	return Py_BuildNone();
}

PyObject * wndMgrShow(PyObject * poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (!UI::CWindowManager::InstancePtr())
		Py_RETURN_NONE;

	if (nargs != 1)
		return Py_BadArgument();

	UI::CWindow * pWin;
	if (!PyFastCall_GetWindow(poArgs[0], &pWin))
		return Py_BuildException();

	pWin->Show();
	Py_RETURN_NONE;
}

PyObject * wndMgrHide(PyObject * poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (!UI::CWindowManager::InstancePtr())
		Py_RETURN_NONE;

	if (nargs != 1)
		return Py_BadArgument();

	UI::CWindow * pWin;
	if (!PyFastCall_GetWindow(poArgs[0], &pWin))
		return Py_BuildException();

	pWin->Hide();
	Py_RETURN_NONE;
}

PyObject * wndMgrIsShow(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	return Py_BuildValue("i", pWin->IsShow());
}

PyObject * wndMgrIsRTL(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	return Py_BuildValue("i", pWin->IsFlag(UI::CWindow::FLAG_RTL));
}

PyObject * wndMgrSetScreenSize(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	int width;
	if (!PyTuple_GetInteger(poArgs, 0, &width))
		return Py_BuildException();
	int height;
	if (!PyTuple_GetInteger(poArgs, 1, &height))
		return Py_BuildException();

	UI::CWindowManager::Instance().SetScreenSize(width, height);
	return Py_BuildNone();
}

PyObject * wndMgrSetParent(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	UI::CWindow * pParentWin;
	if (!PyTuple_GetWindow(poArgs, 1, &pParentWin))
		return Py_BuildException();

	UI::CWindowManager::Instance().SetParent(pWin, pParentWin);

	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		if (pParentWin)
		{
			uint32_t parentId = WindowToId(pParentWin);
			EnsureLayoutWindow(parentId);
			Client::UI::WindowManagerLayoutApiBridge::Instance().AttachWindow(winId, parentId);
		}
		else
		{
			Client::UI::WindowManagerLayoutApiBridge::Instance().DetachWindow(winId);
		}
	}
	return Py_BuildNone();
}

PyObject * wndMgrSetPickAlways(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	UI::CWindowManager::Instance().SetPickAlways(pWin);
	return Py_BuildNone();
}

PyObject * wndMgrSetWndSize(PyObject * poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (!UI::CWindowManager::InstancePtr())
		Py_RETURN_NONE;

	if (nargs != 3)
		return Py_BadArgument();

	UI::CWindow * pWin;
	if (!PyFastCall_GetWindow(poArgs[0], &pWin))
		return Py_BuildException();

	if (!PyLong_Check(poArgs[1]) || !PyLong_Check(poArgs[2]))
		return Py_BadArgument();

	int width = PyLong_AsLong(poArgs[1]);
	int height = PyLong_AsLong(poArgs[2]);

	pWin->SetSize(width, height);
	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerLayoutApiBridge::Instance().SetWindowSize(winId, width, height);
	}
	Py_RETURN_NONE;
}

PyObject * wndMgrSetWndPosition(PyObject * poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (!UI::CWindowManager::InstancePtr())
		Py_RETURN_NONE;

	if (nargs != 3)
		return Py_BadArgument();

	UI::CWindow * pWin;
	if (!PyFastCall_GetWindow(poArgs[0], &pWin))
		return Py_BuildException();

	if (!PyLong_Check(poArgs[1]) || !PyLong_Check(poArgs[2]))
		return Py_BadArgument();

	int x = PyLong_AsLong(poArgs[1]);
	int y = PyLong_AsLong(poArgs[2]);

	pWin->SetPosition(x, y);
	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerLayoutApiBridge::Instance().SetWindowPosition(winId, x, y);
	}
	Py_RETURN_NONE;
}

PyObject * wndMgrGetName(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("s", "");

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	return Py_BuildValue("s", pWin->GetName());
}

PyObject * wndMgrGetWndWidth(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	return Py_BuildValue("i", pWin->GetWidth());
}

PyObject * wndMgrGetWndHeight(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	return Py_BuildValue("i", pWin->GetHeight());
}

PyObject * wndMgrGetWndLocalPosition(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("ii", 0, 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	long lx, ly;
	pWin->GetPosition(&lx, &ly);

	return Py_BuildValue("ii", lx, ly);
}

PyObject * wndMgrGetWndGlobalPosition(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("ii", 0, 0);

	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		EnsureLayoutWindow(winId);
		long screenWidth = UI::CWindowManager::Instance().GetScreenWidth();
		long screenHeight = UI::CWindowManager::Instance().GetScreenHeight();
		auto globalRectRes = Client::UI::WindowManagerLayoutApiBridge::Instance().CalculateGlobalRect(winId, screenWidth, screenHeight);
		if (globalRectRes.has_value())
		{
			return Py_BuildValue("ii", globalRectRes->left, globalRectRes->top);
		}
	}

	RECT & rRect = pWindow->GetRect();
	return Py_BuildValue("ii", rRect.left, rRect.top);
}

PyObject * wndMgrGetWindowRect(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("iiii", 0, 0, 0, 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		long screenWidth = UI::CWindowManager::Instance().GetScreenWidth();
		long screenHeight = UI::CWindowManager::Instance().GetScreenHeight();
		auto globalRectRes = Client::UI::WindowManagerLayoutApiBridge::Instance().CalculateGlobalRect(winId, screenWidth, screenHeight);
		if (globalRectRes.has_value())
		{
			return Py_BuildValue("iiii", globalRectRes->left, globalRectRes->top, globalRectRes->Width(), globalRectRes->Height());
		}
	}

	RECT & rRect = pWin->GetRect();
	return Py_BuildValue("iiii", rRect.left, rRect.top, rRect.right - rRect.left, rRect.bottom - rRect.top);
}

PyObject * wndMgrSetWindowHorizontalAlign(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iAlign;
	if (!PyTuple_GetInteger(poArgs, 1, &iAlign))
		return Py_BuildException();

	pWin->SetHorizontalAlign(iAlign);
	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		Client::UI::HorizontalAlign hAlign = Client::UI::HorizontalAlign::Left;
		if (iAlign == UI::CWindow::HORIZONTAL_ALIGN_CENTER)
			hAlign = Client::UI::HorizontalAlign::Center;
		else if (iAlign == UI::CWindow::HORIZONTAL_ALIGN_RIGHT)
			hAlign = Client::UI::HorizontalAlign::Right;
		Client::UI::WindowManagerLayoutApiBridge::Instance().SetHorizontalAlign(winId, hAlign);
	}

	return Py_BuildNone();
}

PyObject * wndMgrSetWindowVerticalAlign(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iAlign;
	if (!PyTuple_GetInteger(poArgs, 1, &iAlign))
		return Py_BuildException();

	pWin->SetVerticalAlign(iAlign);
	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		Client::UI::VerticalAlign vAlign = Client::UI::VerticalAlign::Top;
		if (iAlign == UI::CWindow::VERTICAL_ALIGN_CENTER)
			vAlign = Client::UI::VerticalAlign::Center;
		else if (iAlign == UI::CWindow::VERTICAL_ALIGN_BOTTOM)
			vAlign = Client::UI::VerticalAlign::Bottom;
		Client::UI::WindowManagerLayoutApiBridge::Instance().SetVerticalAlign(winId, vAlign);
	}

	return Py_BuildNone();
}

PyObject * wndMgrIsIn(PyObject * poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (!UI::CWindowManager::InstancePtr())
		return PyBool_FromLong(0);

	if (nargs != 1)
		return Py_BadArgument();

	UI::CWindow * pWin;
	if (!PyFastCall_GetWindow(poArgs[0], &pWin))
		return Py_BuildException();

	return PyBool_FromLong(pWin == UI::CWindowManager::Instance().GetPointWindow());
}

PyObject * wndMgrGetMouseLocalPosition(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("ii", 0, 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	long lx, ly;
	pWin->GetMouseLocalPosition(lx, ly);
	return Py_BuildValue("ii", lx, ly);
}

PyObject * wndMgrGetHyperlink(PyObject * poSelf, PyObject * poArgs)
{
	char retBuf[1024];
	int retLen = CGraphicTextInstance::Hyperlink_GetText(retBuf, sizeof(retBuf)-1);
	retBuf[retLen] = '\0';

	return Py_BuildValue("s#", retBuf, retLen);
}

PyObject * wndMgrGetScreenWidth(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	return Py_BuildValue("i", UI::CWindowManager::Instance().GetScreenWidth());
}

PyObject * wndMgrGetScreenHeight(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	return Py_BuildValue("i", UI::CWindowManager::Instance().GetScreenHeight());
}

PyObject * wndMgrGetMousePosition(PyObject * poSelf, PyObject * const * poArgs, Py_ssize_t nargs)
{
	if (!UI::CWindowManager::InstancePtr())
	{
		PyObject * poTuple = PyTuple_New(2);
		if (!poTuple)
			return NULL;
		PyTuple_SET_ITEM(poTuple, 0, PyLong_FromLong(0));
		PyTuple_SET_ITEM(poTuple, 1, PyLong_FromLong(0));
		return poTuple;
	}

	if (nargs != 0)
		return Py_BadArgument();

	long lx, ly;
	UI::CWindowManager::Instance().GetMousePosition(lx, ly);
	PyObject * poTuple = PyTuple_New(2);
	if (!poTuple)
		return NULL;
	PyTuple_SET_ITEM(poTuple, 0, PyLong_FromLong(lx));
	PyTuple_SET_ITEM(poTuple, 1, PyLong_FromLong(ly));
	return poTuple;
}

PyObject * wndMgrIsDragging(PyObject * poSelf, PyObject * poArgs)
{
	if (Client::UI::WindowManagerSlotApiBridge::Instance().IsDragging())
		return Py_BuildValue("i", 1);

	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	return Py_BuildValue("i", UI::CWindowManager::Instance().IsDragging());
}

PyObject * wndMgrIsPickedWindow(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	UI::CWindow * pPickedWin = UI::CWindowManager::Instance().GetPointWindow();
	return Py_BuildValue("i", pWin == pPickedWin ? 1 : 0);
}

PyObject * wndMgrGetChildCount(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildValue("i", 0);

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	return Py_BuildValue("i", pWin->GetChildCount());
}

PyObject * wndMgrAddFlag(PyObject * poSelf, PyObject * poArgs)
{
	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	char * pszFlag;
	if (!PyTuple_GetString(poArgs, 1, &pszFlag))
		return Py_BuildException();

	if (pszFlag && *pszFlag)
	{
		if (!stricmp(pszFlag, "movable"))
			pWin->AddFlag(UI::CWindow::FLAG_MOVABLE);
		else if (!stricmp(pszFlag, "limit"))
			pWin->AddFlag(UI::CWindow::FLAG_LIMIT);
		else if (!stricmp(pszFlag, "dragable"))
			pWin->AddFlag(UI::CWindow::FLAG_DRAGABLE);
		else if (!stricmp(pszFlag, "attach"))
			pWin->AddFlag(UI::CWindow::FLAG_ATTACH);
		else if (!stricmp(pszFlag, "restrict_x"))
			pWin->AddFlag(UI::CWindow::FLAG_RESTRICT_X);
		else if (!stricmp(pszFlag, "restrict_y"))
			pWin->AddFlag(UI::CWindow::FLAG_RESTRICT_Y);
		else if (!stricmp(pszFlag, "float"))
			pWin->AddFlag(UI::CWindow::FLAG_FLOAT);
		else if (!stricmp(pszFlag, "not_pick"))
			pWin->AddFlag(UI::CWindow::FLAG_NOT_PICK);
		else if (!stricmp(pszFlag, "ignore_size"))
			pWin->AddFlag(UI::CWindow::FLAG_IGNORE_SIZE);
		else if (!stricmp(pszFlag, "rtl"))
			pWin->AddFlag(UI::CWindow::FLAG_RTL);
		else if (!stricmp(pszFlag, "ltr"))
			pWin->RemoveFlag(UI::CWindow::FLAG_RTL);
		else
			TraceError("Unknown window flag %s", pszFlag);
	}

	return Py_BuildNone();
}

PyObject * wndMgrSetLimitBias(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int l;
	if (!PyTuple_GetInteger(poArgs, 1, &l))
		return Py_BuildException();
	int r;
	if (!PyTuple_GetInteger(poArgs, 2, &r))
		return Py_BuildException();
	int t;
	if (!PyTuple_GetInteger(poArgs, 3, &t))
		return Py_BuildException();
	int b;
	if (!PyTuple_GetInteger(poArgs, 4, &b))
		return Py_BuildException();

	pWin->SetLimitBias(l, r, t, b);
	return Py_BuildNone();
}

PyObject * wndMgrUpdateRect(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	pWin->UpdateRect();
	if (pWin)
	{
		uint32_t winId = WindowToId(pWin);
		EnsureLayoutWindow(winId);
		long screenWidth = UI::CWindowManager::InstancePtr() ? UI::CWindowManager::Instance().GetScreenWidth() : 800;
		long screenHeight = UI::CWindowManager::InstancePtr() ? UI::CWindowManager::Instance().GetScreenHeight() : 600;
		Client::UI::WindowManagerLayoutApiBridge::Instance().CalculateGlobalRect(winId, screenWidth, screenHeight);
	}
	return Py_BuildNone();
}

PyObject * wndMgrAppendSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iIndex))
		return Py_BuildException();

	int ixPosition;
	if (!PyTuple_GetInteger(poArgs, 2, &ixPosition))
		return Py_BuildException();

	int iyPosition;
	if (!PyTuple_GetInteger(poArgs, 3, &iyPosition))
		return Py_BuildException();

	int ixCellSize;
	if (!PyTuple_GetInteger(poArgs, 4, &ixCellSize))
		return Py_BuildException();

	int iyCellSize;
	if (!PyTuple_GetInteger(poArgs, 5, &iyCellSize))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->AppendSlot(iIndex, ixPosition, iyPosition, ixCellSize, iyCellSize);

	return Py_BuildNone();
}

PyObject * wndMgrArrangeSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iStartIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iStartIndex))
		return Py_BuildException();

	int ixCellCount;
	if (!PyTuple_GetInteger(poArgs, 2, &ixCellCount))
		return Py_BuildException();

	int iyCellCount;
	if (!PyTuple_GetInteger(poArgs, 3, &iyCellCount))
		return Py_BuildException();

	int ixCellSize;
	if (!PyTuple_GetInteger(poArgs, 4, &ixCellSize))
		return Py_BuildException();

	int iyCellSize;
	if (!PyTuple_GetInteger(poArgs, 5, &iyCellSize))
		return Py_BuildException();

	int ixBlank;
	if (!PyTuple_GetInteger(poArgs, 6, &ixBlank))
		return Py_BuildException();

	int iyBlank;
	if (!PyTuple_GetInteger(poArgs, 7, &iyBlank))
		return Py_BuildException();

	if (!pWin->IsType(UI::CGridSlotWindow::Type()))
	{
		TraceError("wndMgr.ArrangeSlot : not a grid window");
		return Py_BuildException();
	}

	UI::CGridSlotWindow * pGridSlotWin = (UI::CGridSlotWindow *)pWin;
	pGridSlotWin->ArrangeGridSlot(iStartIndex, ixCellCount, iyCellCount, ixCellSize, iyCellSize, ixBlank, iyBlank);

	return Py_BuildNone();
}

PyObject * wndMgrSetSlotBaseImage(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	float fr;
	if (!PyTuple_GetFloat(poArgs, 2, &fr))
		return Py_BuildException();
	float fg;
	if (!PyTuple_GetFloat(poArgs, 3, &fg))
		return Py_BuildException();
	float fb;
	if (!PyTuple_GetFloat(poArgs, 4, &fb))
		return Py_BuildException();
	float fa;
	if (!PyTuple_GetFloat(poArgs, 5, &fa))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
	{
		TraceError("wndMgr.ArrangeSlot : not a slot window");
		return Py_BuildException();
	}

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetSlotBaseImage(szFileName, fr, fg, fb, fa);

	return Py_BuildNone();
}

PyObject * wndMgrSetCoverButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	char * szUpImageName;
	if (!PyTuple_GetString(poArgs, 2, &szUpImageName))
		return Py_BuildException();
	char * szOverImageName;
	if (!PyTuple_GetString(poArgs, 3, &szOverImageName))
		return Py_BuildException();
	char * szDownImageName;
	if (!PyTuple_GetString(poArgs, 4, &szDownImageName))
		return Py_BuildException();
	char * szDisableImageName;
	if (!PyTuple_GetString(poArgs, 5, &szDisableImageName))
		return Py_BuildException();

	int iLeftButtonEnable;
	if (!PyTuple_GetInteger(poArgs, 6, &iLeftButtonEnable))
		return Py_BuildException();
	int iRightButtonEnable;
	if (!PyTuple_GetInteger(poArgs, 7, &iRightButtonEnable))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->SetCoverButton(iSlotIndex, szUpImageName, szOverImageName, szDownImageName, szDisableImageName, iLeftButtonEnable, iRightButtonEnable);

	return Py_BuildNone();
}

PyObject * wndMgrEnableCoverButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->EnableCoverButton(iSlotIndex);

	return Py_BuildNone();
}

PyObject * wndMgrDisableCoverButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->DisableCoverButton(iSlotIndex);

	return Py_BuildNone();
}

PyObject * wndMgrIsDisableCoverButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	return Py_BuildValue("i", pSlotWin->IsDisableCoverButton(iSlotIndex));
}

PyObject * wndMgrSetAlwaysRenderCoverButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotIndex;
	bool bAlwaysRender = false;

	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!PyTuple_GetBoolean(poArgs, 2, &bAlwaysRender))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->SetAlwaysRenderCoverButton(iSlotIndex, bAlwaysRender);

	return Py_BuildNone();
}

PyObject * wndMgrAppendSlotButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	char * szUpImageName;
	if (!PyTuple_GetString(poArgs, 1, &szUpImageName))
		return Py_BuildException();
	char * szOverImageName;
	if (!PyTuple_GetString(poArgs, 2, &szOverImageName))
		return Py_BuildException();
	char * szDownImageName;
	if (!PyTuple_GetString(poArgs, 3, &szDownImageName))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->AppendSlotButton(szUpImageName, szOverImageName, szDownImageName);

	return Py_BuildNone();
}

PyObject * wndMgrAppendRequirementSignImage(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	char * szImageName;
	if (!PyTuple_GetString(poArgs, 1, &szImageName))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->AppendRequirementSignImage(szImageName);

	return Py_BuildNone();
}

PyObject * wndMgrShowSlotButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotNumber;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotNumber))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->ShowSlotButton(iSlotNumber);

	return Py_BuildNone();
}

PyObject * wndMgrHideAllSlotButton(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->HideAllSlotButton();

	return Py_BuildNone();
}

PyObject * wndMgrShowRequirementSign(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotNumber;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotNumber))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->ShowRequirementSign(iSlotNumber);

	return Py_BuildNone();
}

PyObject * wndMgrHideRequirementSign(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iSlotNumber;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotNumber))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWindow;
	pSlotWin->HideRequirementSign(iSlotNumber);

	return Py_BuildNone();
}

PyObject * wndMgrRefreshSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->RefreshSlot();

	return Py_BuildNone();
}

PyObject * wndMgrSetUseMode(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 1, &iFlag))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetUseMode(iFlag);

	return Py_BuildNone();
}

PyObject * wndMgrSetUsableItem(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 1, &iFlag))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetUsableItem(iFlag);

	return Py_BuildNone();
}

PyObject * wndMgrClearSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().ClearSlot(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex))
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->ClearSlot(iSlotIndex);

	return Py_BuildNone();
}

PyObject * wndMgrClearAllSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().ClearAllSlots();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->ClearAllSlot();

	return Py_BuildNone();
}

PyObject * wndMgrHasSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;

	return Py_BuildValue("i", pSlotWin->HasSlot(iSlotIndex));
}

PyObject * wndMgrSetSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	int iItemIndex;
	if (!PyTuple_GetInteger(poArgs, 2, &iItemIndex))
		return Py_BuildException();

	int iWidth;
	if (!PyTuple_GetInteger(poArgs, 3, &iWidth))
		return Py_BuildException();

	int iHeight;
	if (!PyTuple_GetInteger(poArgs, 4, &iHeight))
		return Py_BuildException();

	unsigned long long iImageHandle;
	if (!PyTuple_GetUnsignedLongLong(poArgs, 5, &iImageHandle))
		return Py_BuildException();

	D3DXCOLOR diffuseColor;
	PyObject* pTuple;
	if (!PyTuple_GetObject(poArgs, 6, &pTuple))
	{
		diffuseColor = D3DXCOLOR(1.0, 1.0, 1.0, 1.0);
		//return Py_BuildException();
	}
	else
	// get diffuse color from pTuple
	{
		if (PyTuple_Size(pTuple) != 4)
			return Py_BuildException();
		if (!PyTuple_GetFloat(pTuple, 0, &diffuseColor.r))
			return Py_BuildException();
		if (!PyTuple_GetFloat(pTuple, 1, &diffuseColor.g))
			return Py_BuildException();
		if (!PyTuple_GetFloat(pTuple, 2, &diffuseColor.b))
			return Py_BuildException();
		if (!PyTuple_GetFloat(pTuple, 3, &diffuseColor.a))
			return Py_BuildException();
	}

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().SetSlot(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex)),
		Client::Core::ItemVnum(static_cast<uint32_t>(iItemIndex)),
		1
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetSlot(iSlotIndex, iItemIndex, iWidth, iHeight, (CGraphicImage *)iImageHandle, diffuseColor);

	return Py_BuildNone();
}

PyObject * wndMgrSetSlotCount(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	int iCount;
	if (!PyTuple_GetInteger(poArgs, 2, &iCount))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().SetSlotCount(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex)),
		static_cast<uint32_t>(iCount)
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetSlotCount(iSlotIndex, iCount);

	return Py_BuildNone();
}

PyObject * wndMgrSetSlotCountNew(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	int iGrade;
	if (!PyTuple_GetInteger(poArgs, 2, &iGrade))
		return Py_BuildException();

	int iCount;
	if (!PyTuple_GetInteger(poArgs, 3, &iCount))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().SetSlotCount(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex)),
		static_cast<uint32_t>(iCount)
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetSlotCountNew(iSlotIndex, iGrade, iCount);

	return Py_BuildNone();
}

PyObject * wndMgrSetSlotCoolTime(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	float fCoolTime;
	if (!PyTuple_GetFloat(poArgs, 2, &fCoolTime))
		return Py_BuildException();

	float fElapsedTime = 0.0f;
	PyTuple_GetFloat(poArgs, 3, &fElapsedTime);

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().SetSlotCoolTime(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex)),
		0,
		fCoolTime,
		fElapsedTime
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetSlotCoolTime(iSlotIndex, fCoolTime, fElapsedTime);

	return Py_BuildNone();
}

PyObject* wndMgrStoreSlotCoolTime(PyObject* poSelf, PyObject* poArgs)
{
	UI::CWindow* pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iKey;
	if (!PyTuple_GetInteger(poArgs, 1, &iKey))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 2, &iSlotIndex))
		return Py_BuildException();

	float fCoolTime;
	if (!PyTuple_GetFloat(poArgs, 3, &fCoolTime))
		return Py_BuildException();

	float fElapsedTime = 0.0f;
	PyTuple_GetFloat(poArgs, 4, &fElapsedTime);

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().StoreSlotCoolTime(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex)),
		0,
		std::to_string(iKey)
	);

	UI::CSlotWindow* pSlotWin = (UI::CSlotWindow*)pWin;
	pSlotWin->StoreSlotCoolTime(iKey, iSlotIndex, fCoolTime, fElapsedTime);

	return Py_BuildNone();
}

PyObject* wndMgrRestoreSlotCoolTime(PyObject* poSelf, PyObject* poArgs)
{
	UI::CWindow* pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iKey;
	if (!PyTuple_GetInteger(poArgs, 1, &iKey))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().RestoreSlotCoolTime(
		Client::Core::SlotIndex(0),
		0,
		std::to_string(iKey)
	);

	UI::CSlotWindow* pSlotWin = (UI::CSlotWindow*)pWin;
	pSlotWin->RestoreSlotCoolTime(iKey);

	return Py_BuildNone();
}

PyObject* wndMgrTransferSlotCoolTime(PyObject* poSelf, PyObject* poArgs)
{
	UI::CWindow* pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iIndex1;
	if (!PyTuple_GetInteger(poArgs, 1, &iIndex1))
		return Py_BuildException();

	int iIndex2;
	if (!PyTuple_GetInteger(poArgs, 2, &iIndex2))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().TransferSlotCoolTime(
		Client::Core::SlotIndex(static_cast<uint16_t>(iIndex1)),
		Client::Core::SlotIndex(static_cast<uint16_t>(iIndex2)),
		0
	);

	UI::CSlotWindow* pSlotWin = (UI::CSlotWindow*)pWin;
	pSlotWin->TransferSlotCoolTime(iIndex1, iIndex2);

	return Py_BuildNone();
}

PyObject * wndMgrSetToggleSlot(PyObject * poSelf, PyObject * poArgs)
{
	assert(!"wndMgrSetToggleSlot");
	return Py_BuildNone();
}

PyObject * wndMgrActivateSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->ActivateSlot(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndMgrDeactivateSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->DeactivateSlot(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndMgrEnableSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().EnableSlot(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex)),
		true
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->EnableSlot(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndMgrDisableSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().EnableSlot(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex)),
		false
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->DisableSlot(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndMgrShowSlotBaseImage(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->ShowSlotBaseImage(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndMgrHideSlotBaseImage(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->HideSlotBaseImage(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndMgrSetSlotType(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iType;
	if (!PyTuple_GetInteger(poArgs, 1, &iType))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetSlotType(iType);

	return Py_BuildNone();
}

PyObject * wndMgrSetSlotStyle(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iStyle;
	if (!PyTuple_GetInteger(poArgs, 1, &iStyle))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SetSlotStyle(iStyle);

	return Py_BuildNone();
}

PyObject * wndMgrSelectSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	int iIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->SelectSlot(iIndex);

	return Py_BuildNone();
}

PyObject * wndMgrClearSelected(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->ClearSelected();

	return Py_BuildNone();
}

PyObject * wndMgrGetSelectedSlotCount(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	return Py_BuildValue("i", pSlotWin->GetSelectedSlotCount());
}

PyObject * wndMgrGetSelectedSlotNumber(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iSlotNumber;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotNumber))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	return Py_BuildValue("i", pSlotWin->GetSelectedSlotNumber(iSlotNumber));
}

PyObject * wndMgrIsSelectedSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iSlotNumber;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotNumber))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	return Py_BuildValue("i", pSlotWin->isSelectedSlot(iSlotNumber));
}

PyObject * wndMgrGetSlotCount(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	return Py_BuildValue("i", pSlotWin->GetSlotCount());
}

PyObject * wndMgrLockSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().LockSlot(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex))
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->LockSlot(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndMgrUnlockSlot(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWin;
	if (!PyTuple_GetWindow(poArgs, 0, &pWin))
		return Py_BuildException();
	int iSlotIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iSlotIndex))
		return Py_BuildException();

	if (!pWin->IsType(UI::CSlotWindow::Type()))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().UnlockSlot(
		Client::Core::SlotIndex(static_cast<uint16_t>(iSlotIndex))
	);

	UI::CSlotWindow * pSlotWin = (UI::CSlotWindow *)pWin;
	pSlotWin->UnlockSlot(iSlotIndex);
	return Py_BuildNone();
}

PyObject * wndBarSetColor(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iColor;
	if (!PyTuple_GetInteger(poArgs, 1, &iColor))
		return Py_BuildException();

	if (pWindow->IsType(UI::CBar3D::Type()))
	{
		int iLeftColor = iColor;

		int iRightColor;
		if (!PyTuple_GetInteger(poArgs, 2, &iRightColor))
			return Py_BuildException();
		int iCenterColor;
		if (!PyTuple_GetInteger(poArgs, 3, &iCenterColor))
			return Py_BuildException();

		((UI::CBar3D *)pWindow)->SetColor(iLeftColor, iRightColor, iCenterColor);
	}
	else
	{
		((UI::CWindow *)pWindow)->SetColor(iColor);
	}
	return Py_BuildNone();
}

PyObject * wndMgrAttachIcon(PyObject * poSelf, PyObject * poArgs)
{
	int iType;
	if (!PyTuple_GetInteger(poArgs, 0, &iType))
		return Py_BuildException();
	int iIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &iIndex))
		return Py_BuildException();
	int iSlotNumber;
	if (!PyTuple_GetInteger(poArgs, 2, &iSlotNumber))
		return Py_BuildException();
	int iWidth;
	if (!PyTuple_GetInteger(poArgs, 3, &iWidth))
		return Py_BuildException();
	int iHeight;
	if (!PyTuple_GetInteger(poArgs, 4, &iHeight))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().AttachIcon(
		static_cast<uint32_t>(iType),
		static_cast<uint32_t>(iIndex),
		static_cast<uint32_t>(iSlotNumber),
		static_cast<uint8_t>(iWidth),
		static_cast<uint8_t>(iHeight)
	);

	if (UI::CWindowManager::InstancePtr())
	{
		UI::CWindowManager::Instance().AttachIcon(iType, iIndex, iSlotNumber, iWidth, iHeight);
	}
	return Py_BuildNone();
}

PyObject * wndMgrDeattachIcon(PyObject * poSelf, PyObject * poArgs)
{
	Client::UI::WindowManagerSlotApiBridge::Instance().DeattachIcon();

	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindowManager::Instance().DeattachIcon();
	return Py_BuildNone();
}

PyObject * wndMgrSetAttachingFlag(PyObject * poSelf, PyObject * poArgs)
{
	BOOL bFlag;
	if (!PyTuple_GetInteger(poArgs, 0, &bFlag))
		return Py_BuildException();

	Client::UI::WindowManagerSlotApiBridge::Instance().SetAttachingFlag(bFlag != 0);

	if (!UI::CWindowManager::InstancePtr())
		return Py_BuildNone();

	UI::CWindowManager::Instance().SetAttachingFlag(bFlag);
	return Py_BuildNone();
}

// Text
PyObject * wndTextSetMax(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iMax;
	if (!PyTuple_GetInteger(poArgs, 1, &iMax))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetMax(iMax);
	return Py_BuildNone();
}
PyObject * wndTextSetHorizontalAlign(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iType;
	if (!PyTuple_GetInteger(poArgs, 1, &iType))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetHorizontalAlign(iType);
	return Py_BuildNone();
}
PyObject * wndTextSetVerticalAlign(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iType;
	if (!PyTuple_GetInteger(poArgs, 1, &iType))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetVerticalAlign(iType);
	return Py_BuildNone();
}
PyObject * wndTextSetSecret(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 1, &iFlag))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetSecret(iFlag);
	return Py_BuildNone();
}
PyObject * wndTextSetOutline(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 1, &iFlag))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetOutline(iFlag);
	return Py_BuildNone();
}
PyObject * wndTextSetFeather(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 1, &iFlag))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetFeather(iFlag);
	return Py_BuildNone();
}
PyObject * wndTextSetMultiLine(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 1, &iFlag))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetMultiLine(iFlag);
	return Py_BuildNone();
}
PyObject * wndTextSetFontName(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFontName;
	if (!PyTuple_GetString(poArgs, 1, &szFontName))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetFontName(szFontName);
	return Py_BuildNone();
}
PyObject * wndMgrSetDefaultFontName(PyObject * poSelf, PyObject * poArgs)
{
	char * szFontName;
	if (!PyTuple_GetString(poArgs, 0, &szFontName))
		return Py_BuildException();

	UI::CWindowManager::Instance().SetDefaultFontName(szFontName);
	return Py_BuildNone();
}
PyObject * wndMgrDumpTree(PyObject * poSelf, PyObject * poArgs)
{
	std::string json = UI::CWindowManager::Instance().DumpTreeJson();
	return Py_BuildValue("s", json.c_str());
}
PyObject * wndTextSetFontColor(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	if (2 == PyTuple_Size(poArgs))
	{
		int iColor;
		if (!PyTuple_GetInteger(poArgs, 1, &iColor))
			return Py_BuildException();
		((UI::CTextLine*)pWindow)->SetFontColor(iColor);
	}
	else if (4 == PyTuple_Size(poArgs))
	{
		float fr;
		if (!PyTuple_GetFloat(poArgs, 1, &fr))
			return Py_BuildException();
		float fg;
		if (!PyTuple_GetFloat(poArgs, 2, &fg))
			return Py_BuildException();
		float fb;
		if (!PyTuple_GetFloat(poArgs, 3, &fb))
			return Py_BuildException();
		float fa = 1.0f;

		BYTE argb[4] =
		{
			(BYTE) (255.0f * fb),
			(BYTE) (255.0f * fg),
			(BYTE) (255.0f * fr),
			(BYTE) (255.0f * fa)
		};
		((UI::CTextLine*)pWindow)->SetFontColor(*((DWORD *) argb));
	}
	else
	{
		return Py_BuildException();
	}

	return Py_BuildNone();
}
PyObject * wndTextSetLimitWidth(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fWidth;
	if (!PyTuple_GetFloat(poArgs, 1, &fWidth))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetLimitWidth(fWidth);
	return Py_BuildNone();
}
PyObject * wndTextSetBaseDirection(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iDir;
	if (!PyTuple_GetInteger(poArgs, 1, &iDir))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetBaseDirection(iDir);
	return Py_BuildNone();
}
PyObject * wndTextSetText(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szText;
	if (!PyTuple_GetString(poArgs, 1, &szText))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetText(szText);
	return Py_BuildNone();
}
PyObject * wndTextGetText(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("s", ((UI::CTextLine*)pWindow)->GetText());
}

PyObject * wndTextGetTextSize(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int nWidth;
	int nHeight;

	UI::CTextLine* pkTextLine=(UI::CTextLine*)pWindow;
	pkTextLine->GetTextSize(&nWidth, &nHeight);

	return Py_BuildValue("(ii)", nWidth, nHeight);
}


PyObject * wndTextShowCursor(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->ShowCursor();
	return Py_BuildNone();
}
PyObject * wndTextHideCursor(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->HideCursor();
	return Py_BuildNone();
}
PyObject * wndTextGetCursorPosition(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("i", ((UI::CTextLine*)pWindow)->GetCursorPosition());
}

PyObject * wndTextSetSelection(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;

	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int iStart;

	if (!PyTuple_GetInteger(poArgs, 1, &iStart))
		return Py_BuildException();

	int iEnd;

	if (!PyTuple_GetInteger(poArgs, 2, &iEnd))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->SetSelection(iStart, iEnd);

	return Py_BuildNone();
}

PyObject * wndTextClearSelection(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;

	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CTextLine*)pWindow)->ClearSelection();

	return Py_BuildNone();
}

PyObject * wndNumberSetNumber(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szNumber;
	if (!PyTuple_GetString(poArgs, 1, &szNumber))
		return Py_BuildException();

	((UI::CNumberLine*)pWindow)->SetNumber(szNumber);

	return Py_BuildNone();
}

PyObject * wndNumberSetNumberHorizontalAlignCenter(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CNumberLine*)pWindow)->SetHorizontalAlign(UI::CWindow::HORIZONTAL_ALIGN_CENTER);

	return Py_BuildNone();
}

PyObject * wndNumberSetNumberHorizontalAlignRight(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CNumberLine*)pWindow)->SetHorizontalAlign(UI::CWindow::HORIZONTAL_ALIGN_RIGHT);

	return Py_BuildNone();
}

PyObject * wndNumberSetPath(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szPath;
	if (!PyTuple_GetString(poArgs, 1, &szPath))
		return Py_BuildException();

	((UI::CNumberLine*)pWindow)->SetPath(szPath);

	return Py_BuildNone();
}

PyObject * wndMarkBox_SetImageFilename(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	((UI::CMarkBox*)pWindow)->LoadImage(szFileName);
	return Py_BuildNone();
}

PyObject * wndMarkBox_SetImage(PyObject * poSelf, PyObject * poArgs)
{
	// 아무것도 하지 않음
	return Py_BuildNone();
}

PyObject * wndMarkBox_Load(PyObject * poSelf, PyObject * poArgs)
{
	// 아무것도 하지 않음
	return Py_BuildNone();
}

PyObject * wndMarkBox_SetIndex(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	int nIndex;
	if (!PyTuple_GetInteger(poArgs, 1, &nIndex))
		return Py_BuildException();

	((UI::CMarkBox*)pWindow)->SetIndex(nIndex);
	return Py_BuildNone();
}

PyObject * wndMarkBox_SetScale(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	float fScale;
	if (!PyTuple_GetFloat(poArgs, 1, &fScale))
		return Py_BuildException();

	((UI::CMarkBox*)pWindow)->SetScale(fScale);
	return Py_BuildNone();
}


PyObject * wndMarkBox_SetDiffuseColor(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fr;
	if (!PyTuple_GetFloat(poArgs, 1, &fr))
		return Py_BuildException();
	float fg;
	if (!PyTuple_GetFloat(poArgs, 2, &fg))
		return Py_BuildException();
	float fb;
	if (!PyTuple_GetFloat(poArgs, 3, &fb))
		return Py_BuildException();
	float fa;
	if (!PyTuple_GetFloat(poArgs, 4, &fa))
		return Py_BuildException();

	((UI::CMarkBox*)pWindow)->SetDiffuseColor(fr, fg, fb, fa);

	return Py_BuildNone();
}


PyObject * wndImageLoadImage(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	if (!((UI::CImageBox*)pWindow)->LoadImage(szFileName))
		return Py_BuildException("Failed to load image (filename: %s)", szFileName);

	return Py_BuildNone();
}

PyObject* wndImageLoadImageFromFile(PyObject* poSelf, PyObject* poArgs)
{
	UI::CWindow* pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char* szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	if (!((UI::CImageBox*)pWindow)->LoadImageFromFile(szFileName))
		return Py_BuildException("Failed to load image from file (filename: %s)", szFileName);

	return Py_BuildNone();
}

PyObject * wndImageSetDiffuseColor(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fr;
	if (!PyTuple_GetFloat(poArgs, 1, &fr))
		return Py_BuildException();
	float fg;
	if (!PyTuple_GetFloat(poArgs, 2, &fg))
		return Py_BuildException();
	float fb;
	if (!PyTuple_GetFloat(poArgs, 3, &fb))
		return Py_BuildException();
	float fa;
	if (!PyTuple_GetFloat(poArgs, 4, &fa))
		return Py_BuildException();

	((UI::CImageBox*)pWindow)->SetDiffuseColor(fr, fg, fb, fa);

	return Py_BuildNone();
}

PyObject * wndImageGetWidth(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("i", ((UI::CImageBox*)pWindow)->GetWidth());
}

PyObject * wndImageGetHeight(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("i", ((UI::CImageBox*)pWindow)->GetHeight());
}

PyObject * wndImageSetScale(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fx;
	if (!PyTuple_GetFloat(poArgs, 1, &fx))
		return Py_BuildException();
	float fy;
	if (!PyTuple_GetFloat(poArgs, 2, &fy))
		return Py_BuildException();

	((UI::CExpandedImageBox*)pWindow)->SetScale(fx, fy);

	return Py_BuildNone();
}
PyObject * wndImageSetOrigin(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fx;
	if (!PyTuple_GetFloat(poArgs, 1, &fx))
		return Py_BuildException();
	float fy;
	if (!PyTuple_GetFloat(poArgs, 2, &fy))
		return Py_BuildException();

	((UI::CExpandedImageBox*)pWindow)->SetOrigin(fx, fy);

	return Py_BuildNone();
}
PyObject * wndImageSetRotation(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fRotation;
	if (!PyTuple_GetFloat(poArgs, 1, &fRotation))
		return Py_BuildException();

	((UI::CExpandedImageBox*)pWindow)->SetRotation(fRotation);

	return Py_BuildNone();
}
PyObject * wndImageSetRenderingRect(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fLeft;
	if (!PyTuple_GetFloat(poArgs, 1, &fLeft))
		return Py_BuildException();
	float fTop;
	if (!PyTuple_GetFloat(poArgs, 2, &fTop))
		return Py_BuildException();
	float fRight;
	if (!PyTuple_GetFloat(poArgs, 3, &fRight))
		return Py_BuildException();
	float fBottom;
	if (!PyTuple_GetFloat(poArgs, 4, &fBottom))
		return Py_BuildException();

	if (pWindow->IsType(UI::CExpandedImageBox::Type()))
		((UI::CExpandedImageBox*)pWindow)->SetRenderingRect(fLeft, fTop, fRight, fBottom);
	else if (pWindow->IsType(UI::CAniImageBox::Type()))
		((UI::CAniImageBox*)pWindow)->SetRenderingRect(fLeft, fTop, fRight, fBottom);

	return Py_BuildNone();
}
PyObject * wndImageSetRenderingMode(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int iMode;
	if (!PyTuple_GetInteger(poArgs, 1, &iMode))
		return Py_BuildException();

	if (pWindow->IsType(UI::CExpandedImageBox::Type()))
		((UI::CExpandedImageBox*)pWindow)->SetRenderingMode(iMode);

	return Py_BuildNone();
}
PyObject * wndImageSetDelay(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	float fDelay;
	if (!PyTuple_GetFloat(poArgs, 1, &fDelay))
		return Py_BuildException();

	((UI::CAniImageBox*)pWindow)->SetDelay(fDelay);

	return Py_BuildNone();
}
PyObject * wndImageAppendImage(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	((UI::CAniImageBox*)pWindow)->AppendImage(szFileName);

	return Py_BuildNone();
}

PyObject * wndButtonSetUpVisual(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	((UI::CButton*)pWindow)->SetUpVisual(szFileName);

	return Py_BuildNone();
}
PyObject * wndButtonSetOverVisual(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	((UI::CButton*)pWindow)->SetOverVisual(szFileName);

	return Py_BuildNone();
}
PyObject * wndButtonSetDownVisual(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	((UI::CButton*)pWindow)->SetDownVisual(szFileName);

	return Py_BuildNone();
}
PyObject * wndButtonSetDisableVisual(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	char * szFileName;
	if (!PyTuple_GetString(poArgs, 1, &szFileName))
		return Py_BuildException();

	((UI::CButton*)pWindow)->SetDisableVisual(szFileName);

	return Py_BuildNone();
}
PyObject * wndButtonGetUpVisualFileName(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("s", ((UI::CButton*)pWindow)->GetUpVisualFileName());
}
PyObject * wndButtonGetOverVisualFileName(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("s", ((UI::CButton*)pWindow)->GetOverVisualFileName());
}
PyObject * wndButtonGetDownVisualFileName(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("s", ((UI::CButton*)pWindow)->GetDownVisualFileName());
}
PyObject * wndButtonFlash(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CButton*)pWindow)->Flash();
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		Client::UI::WindowManagerButtonApiBridge::Instance().SetButtonFlash(Client::UI::ButtonId(winId), true);
	}

	return Py_BuildNone();
}
PyObject * wndButtonEnable(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CButton*)pWindow)->Enable();
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		Client::UI::WindowManagerButtonApiBridge::Instance().SetButtonEnable(Client::UI::ButtonId(winId), true);
	}

	return Py_BuildNone();
}
PyObject * wndButtonDisable(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CButton*)pWindow)->Disable();
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		Client::UI::WindowManagerButtonApiBridge::Instance().SetButtonEnable(Client::UI::ButtonId(winId), false);
	}

	return Py_BuildNone();
}
PyObject * wndButtonDown(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CButton*)pWindow)->Down();
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		Client::UI::WindowManagerButtonApiBridge::Instance().SetButtonDown(Client::UI::ButtonId(winId), true);
	}

	return Py_BuildNone();
}
PyObject * wndButtonSetUp(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	((UI::CButton*)pWindow)->SetUp();
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		Client::UI::WindowManagerButtonApiBridge::Instance().SetButtonDown(Client::UI::ButtonId(winId), false);
	}

	return Py_BuildNone();
}
PyObject * wndButtonSetRestrictMovementArea(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();
	int ix;
	if (!PyTuple_GetInteger(poArgs, 1, &ix))
		return Py_BuildException();
	int iy;
	if (!PyTuple_GetInteger(poArgs, 2, &iy))
		return Py_BuildException();
	int iwidth;
	if (!PyTuple_GetInteger(poArgs, 3, &iwidth))
		return Py_BuildException();
	int iheight;
	if (!PyTuple_GetInteger(poArgs, 4, &iheight))
		return Py_BuildException();

	((UI::CDragButton*)pWindow)->SetRestrictMovementArea(ix, iy, iwidth, iheight);

	return Py_BuildNone();
}

PyObject * wndButtonIsDown(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		auto res = Client::UI::WindowManagerButtonApiBridge::Instance().IsButtonDown(Client::UI::ButtonId(winId));
		if (res.has_value())
			return Py_BuildValue("i", res.value() ? 1 : 0);
	}

	return Py_BuildValue("i", ((UI::CButton*)pWindow)->IsPressed());
}

PyObject * wndButtonSetState(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow = nullptr;
	uint32_t uButtonId = 0;
	int iState = 0;

	if (!PyTuple_GetWindowOrId(poArgs, 0, &pWindow, &uButtonId))
		return Py_BuildException();

	if (!PyTuple_GetInteger(poArgs, 1, &iState))
		return Py_BuildException();

	Client::UI::ButtonVisualState visualState = Client::UI::ButtonVisualState::Normal;
	switch (iState)
	{
		case 0: visualState = Client::UI::ButtonVisualState::Normal; break;
		case 1: visualState = Client::UI::ButtonVisualState::Over; break;
		case 2: visualState = Client::UI::ButtonVisualState::Down; break;
		case 3: visualState = Client::UI::ButtonVisualState::Disable; break;
		default: break;
	}

	Client::UI::WindowManagerButtonApiBridge::Instance().SetButtonState(
		Client::UI::ButtonId(uButtonId), visualState);

	if (pWindow)
	{
		UI::CButton* pButton = static_cast<UI::CButton*>(pWindow);
		switch (visualState)
		{
			case Client::UI::ButtonVisualState::Normal:
				pButton->SetUp();
				break;
			case Client::UI::ButtonVisualState::Over:
				pButton->Over();
				break;
			case Client::UI::ButtonVisualState::Down:
				pButton->Down();
				break;
			case Client::UI::ButtonVisualState::Disable:
				pButton->Disable();
				break;
		}
	}

	return Py_BuildNone();
}

PyObject * wndButtonHover(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow = nullptr;
	uint32_t uButtonId = 0;
	int isHover = 0;

	if (!PyTuple_GetWindowOrId(poArgs, 0, &pWindow, &uButtonId))
		return Py_BuildException();

	if (!PyTuple_GetInteger(poArgs, 1, &isHover))
		return Py_BuildException();

	if (isHover)
	{
		Client::UI::WindowManagerButtonApiBridge::Instance().SimulateMouseEnter(
			Client::UI::ButtonId(uButtonId));
		if (pWindow)
			static_cast<UI::CButton*>(pWindow)->Over();
	}
	else
	{
		Client::UI::WindowManagerButtonApiBridge::Instance().SimulateMouseLeave(
			Client::UI::ButtonId(uButtonId));
		if (pWindow)
			static_cast<UI::CButton*>(pWindow)->SetUp();
	}

	return Py_BuildNone();
}

PyObject * wndButtonSimulateMouseEnter(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow = nullptr;
	uint32_t uButtonId = 0;
	if (!PyTuple_GetWindowOrId(poArgs, 0, &pWindow, &uButtonId))
		return Py_BuildException();

	Client::UI::WindowManagerButtonApiBridge::Instance().SimulateMouseEnter(
		Client::UI::ButtonId(uButtonId));
	if (pWindow)
		static_cast<UI::CButton*>(pWindow)->Over();

	return Py_BuildNone();
}

PyObject * wndButtonSimulateMouseLeave(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow = nullptr;
	uint32_t uButtonId = 0;
	if (!PyTuple_GetWindowOrId(poArgs, 0, &pWindow, &uButtonId))
		return Py_BuildException();

	Client::UI::WindowManagerButtonApiBridge::Instance().SimulateMouseLeave(
		Client::UI::ButtonId(uButtonId));
	if (pWindow)
		static_cast<UI::CButton*>(pWindow)->SetUp();

	return Py_BuildNone();
}

PyObject * wndButtonSimulateMousePress(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow = nullptr;
	uint32_t uButtonId = 0;
	if (!PyTuple_GetWindowOrId(poArgs, 0, &pWindow, &uButtonId))
		return Py_BuildException();

	Client::UI::WindowManagerButtonApiBridge::Instance().SimulateMousePress(
		Client::UI::ButtonId(uButtonId));
	if (pWindow)
		static_cast<UI::CButton*>(pWindow)->Down();

	return Py_BuildNone();
}

PyObject * wndButtonSimulateMouseRelease(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow = nullptr;
	uint32_t uButtonId = 0;
	if (!PyTuple_GetWindowOrId(poArgs, 0, &pWindow, &uButtonId))
		return Py_BuildException();

	Client::UI::WindowManagerButtonApiBridge::Instance().SimulateMouseRelease(
		Client::UI::ButtonId(uButtonId));
	if (pWindow)
		static_cast<UI::CButton*>(pWindow)->SetUp();

	return Py_BuildNone();
}

PyObject * wndButtonGetVisualState(PyObject * poSelf, PyObject * poArgs)
{
	UI::CWindow * pWindow = nullptr;
	uint32_t uButtonId = 0;
	if (!PyTuple_GetWindowOrId(poArgs, 0, &pWindow, &uButtonId))
		return Py_BuildException();

	auto res = Client::UI::WindowManagerButtonApiBridge::Instance().GetButtonVisualState(
		Client::UI::ButtonId(uButtonId));
	if (res.has_value())
		return Py_BuildValue("i", static_cast<int>(res.value()));

	return Py_BuildValue("i", 0);
}

extern BOOL g_bOutlineBoxEnable;
extern BOOL g_bShowOverInWindowName;

PyObject * wndMgrSetOutlineFlag(PyObject * poSelf, PyObject * poArgs)
{
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 0, &iFlag))
		return Py_BuildException();

	g_bOutlineBoxEnable = iFlag;

	return Py_BuildNone();
}

PyObject * wndMgrShowOverInWindowName(PyObject * poSelf, PyObject * poArgs)
{
	int iFlag;
	if (!PyTuple_GetInteger(poArgs, 0, &iFlag))
		return Py_BuildException();

	g_bShowOverInWindowName = iFlag;

	return Py_BuildNone();
}

PyObject* wndMgrEnableScissorRect(PyObject* poSelf, PyObject* poArgs)
{
	UI::CWindow* pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	pWindow->EnableScissorRect();
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerLayoutApiBridge::Instance().EnableScissorRect(winId, true);
	}
	Py_RETURN_NONE;
}

PyObject* wndMgrDisableScissorRect(PyObject* poSelf, PyObject* poArgs)
{
	UI::CWindow* pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	pWindow->DisableScissorRect();
	if (pWindow)
	{
		uint32_t winId = WindowToId(pWindow);
		EnsureLayoutWindow(winId);
		Client::UI::WindowManagerLayoutApiBridge::Instance().EnableScissorRect(winId, false);
	}
	Py_RETURN_NONE;
}

PyObject* wndMgrIsScissorRectEnabled(PyObject* poSelf, PyObject* poArgs)
{
	UI::CWindow* pWindow;
	if (!PyTuple_GetWindow(poArgs, 0, &pWindow))
		return Py_BuildException();

	return Py_BuildValue("b", pWindow->IsScissorRectEnabled());
}

void initwndMgr()
{
	UI::PythonInternedStrings::Initialize();

	static PyMethodDef s_methods[] =
	{
		// WindowManager
		{ "SetMouseHandler",			wndMgrSetMouseHandler,				METH_VARARGS },
		{ "SetScreenSize",				wndMgrSetScreenSize,				METH_VARARGS },
		{ "GetScreenWidth",				wndMgrGetScreenWidth,				METH_VARARGS },
		{ "GetScreenHeight",			wndMgrGetScreenHeight,				METH_VARARGS },
		{ "AttachIcon",					wndMgrAttachIcon,					METH_VARARGS },
		{ "DeattachIcon",				wndMgrDeattachIcon,					METH_VARARGS },
		{ "SetAttachingFlag",			wndMgrSetAttachingFlag,				METH_VARARGS },
		{ "GetAspect",					wndMgrGetAspect,					METH_VARARGS },
		{ "GetHyperlink",				wndMgrGetHyperlink,					METH_VARARGS },
		{ "OnceIgnoreMouseLeftButtonUpEvent",	wndMgrOnceIgnoreMouseLeftButtonUpEvent,		METH_VARARGS },

		// Window
		{ "Register",					wndMgrRegister,						METH_VARARGS },
		{ "RegisterSlotWindow",			wndMgrRegisterSlotWindow,			METH_VARARGS },
		{ "RegisterGridSlotWindow",		wndMgrRegisterGridSlotWindow,		METH_VARARGS },
		{ "RegisterTextLine",			wndMgrRegisterTextLine,				METH_VARARGS },
		{ "RegisterMarkBox",			wndMgrRegisterMarkBox,				METH_VARARGS },
		{ "RegisterImageBox",			wndMgrRegisterImageBox,				METH_VARARGS },
		{ "RegisterExpandedImageBox",	wndMgrRegisterExpandedImageBox,		METH_VARARGS },
		{ "RegisterAniImageBox",		wndMgrRegisterAniImageBox,			METH_VARARGS },
		{ "RegisterButton",				wndMgrRegisterButton,				METH_VARARGS },
		{ "RegisterRadioButton",		wndMgrRegisterRadioButton,			METH_VARARGS },
		{ "RegisterToggleButton",		wndMgrRegisterToggleButton,			METH_VARARGS },
		{ "RegisterDragButton",			wndMgrRegisterDragButton,			METH_VARARGS },
		{ "RegisterBox",				wndMgrRegisterBox,					METH_VARARGS },
		{ "RegisterBar",				wndMgrRegisterBar,					METH_VARARGS },
		{ "RegisterLine",				wndMgrRegisterLine,					METH_VARARGS },
		{ "RegisterBar3D",				wndMgrRegisterBar3D,				METH_VARARGS },
		{ "RegisterNumberLine",			wndMgrRegisterNumberLine,			METH_VARARGS },
		{ "Destroy",					wndMgrDestroy,						METH_VARARGS },
		{ "AddFlag",					wndMgrAddFlag,						METH_VARARGS },
		{ "IsRTL",						wndMgrIsRTL,						METH_VARARGS },
		{ "SetDefaultFontName",			wndMgrSetDefaultFontName,			METH_VARARGS },
		{ "DumpTree",					wndMgrDumpTree,						METH_VARARGS },

		// Base Window
		{ "SetName",					wndMgrSetName,						METH_VARARGS },
		{ "GetName",					wndMgrGetName,						METH_VARARGS },

		{ "SetTop",						wndMgrSetTop,						METH_VARARGS },
		{ "Show",						reinterpret_cast<PyCFunction>(wndMgrShow),							METH_FASTCALL },
		{ "Hide",						reinterpret_cast<PyCFunction>(wndMgrHide),							METH_FASTCALL },
		{ "IsShow",						wndMgrIsShow,						METH_VARARGS },
		{ "SetParent",					wndMgrSetParent,					METH_VARARGS },
		{ "SetPickAlways",				wndMgrSetPickAlways,				METH_VARARGS },

		{ "IsFocus",					wndMgrIsFocus,						METH_VARARGS },
		{ "SetFocus",					wndMgrSetFocus,						METH_VARARGS },
		{ "KillFocus",					wndMgrKillFocus,					METH_VARARGS },
		{ "Lock",						wndMgrLock,							METH_VARARGS },
		{ "Unlock",						wndMgrUnlock,						METH_VARARGS },

		{ "SetWindowSize",				reinterpret_cast<PyCFunction>(wndMgrSetWndSize),					METH_FASTCALL },
		{ "SetWindowPosition",			reinterpret_cast<PyCFunction>(wndMgrSetWndPosition),				METH_FASTCALL },
		{ "GetWindowWidth",				wndMgrGetWndWidth,					METH_VARARGS },
		{ "GetWindowHeight",			wndMgrGetWndHeight,					METH_VARARGS },
		{ "GetWindowLocalPosition",		wndMgrGetWndLocalPosition,			METH_VARARGS },
		{ "GetWindowGlobalPosition",	wndMgrGetWndGlobalPosition,			METH_VARARGS },
		{ "GetWindowRect",				wndMgrGetWindowRect,				METH_VARARGS },
		{ "SetWindowHorizontalAlign",	wndMgrSetWindowHorizontalAlign,		METH_VARARGS },
		{ "SetWindowVerticalAlign",		wndMgrSetWindowVerticalAlign,		METH_VARARGS },

		{ "GetChildCount",				wndMgrGetChildCount,				METH_VARARGS },

		{ "IsPickedWindow",				wndMgrIsPickedWindow,				METH_VARARGS },
		{ "IsIn",						reinterpret_cast<PyCFunction>(wndMgrIsIn),							METH_FASTCALL },
		{ "GetMouseLocalPosition",		wndMgrGetMouseLocalPosition,		METH_VARARGS },
		{ "GetMousePosition",			reinterpret_cast<PyCFunction>(wndMgrGetMousePosition),				METH_FASTCALL },
		{ "IsDragging",					wndMgrIsDragging,					METH_VARARGS },

		{ "SetLimitBias",				wndMgrSetLimitBias,					METH_VARARGS },

		{ "UpdateRect",					wndMgrUpdateRect,					METH_VARARGS },

		// Slot Window
		{ "AppendSlot",					wndMgrAppendSlot,					METH_VARARGS },
		{ "ArrangeSlot",				wndMgrArrangeSlot,					METH_VARARGS },
		{ "ClearSlot",					wndMgrClearSlot,					METH_VARARGS },
		{ "ClearAllSlot",				wndMgrClearAllSlot,					METH_VARARGS },
		{ "HasSlot",					wndMgrHasSlot,						METH_VARARGS },
		{ "SetSlot",					wndMgrSetSlot,						METH_VARARGS },
		{ "SetSlotCount",				wndMgrSetSlotCount,					METH_VARARGS },
		{ "SetSlotCountNew",			wndMgrSetSlotCountNew,				METH_VARARGS },
		{ "SetSlotCoolTime",			wndMgrSetSlotCoolTime,				METH_VARARGS },
		{ "StoreSlotCoolTime",			wndMgrStoreSlotCoolTime,			METH_VARARGS },
		{ "RestoreSlotCoolTime",		wndMgrRestoreSlotCoolTime,			METH_VARARGS },
		{ "TransferSlotCoolTime",		wndMgrTransferSlotCoolTime,			METH_VARARGS },
		{ "SetToggleSlot",				wndMgrSetToggleSlot,				METH_VARARGS },
		{ "ActivateSlot",				wndMgrActivateSlot,					METH_VARARGS },
		{ "DeactivateSlot",				wndMgrDeactivateSlot,				METH_VARARGS },
		{ "EnableSlot",					wndMgrEnableSlot,					METH_VARARGS },
		{ "DisableSlot",				wndMgrDisableSlot,					METH_VARARGS },
		{ "ShowSlotBaseImage",			wndMgrShowSlotBaseImage,			METH_VARARGS },
		{ "HideSlotBaseImage",			wndMgrHideSlotBaseImage,			METH_VARARGS },
		{ "SetSlotType",				wndMgrSetSlotType,					METH_VARARGS },
		{ "SetSlotStyle",				wndMgrSetSlotStyle,					METH_VARARGS },
		{ "SetSlotBaseImage",			wndMgrSetSlotBaseImage,				METH_VARARGS },

		{ "SetCoverButton",				wndMgrSetCoverButton,				METH_VARARGS },
		{ "EnableCoverButton",			wndMgrEnableCoverButton,			METH_VARARGS },
		{ "DisableCoverButton",			wndMgrDisableCoverButton,			METH_VARARGS },
		{ "IsDisableCoverButton",		wndMgrIsDisableCoverButton,			METH_VARARGS },
		{ "SetAlwaysRenderCoverButton",	wndMgrSetAlwaysRenderCoverButton,	METH_VARARGS },
		
		{ "AppendSlotButton",			wndMgrAppendSlotButton,				METH_VARARGS },
		{ "AppendRequirementSignImage",	wndMgrAppendRequirementSignImage,	METH_VARARGS },
		{ "ShowSlotButton",				wndMgrShowSlotButton,				METH_VARARGS },
		{ "HideAllSlotButton",			wndMgrHideAllSlotButton,			METH_VARARGS },
		{ "ShowRequirementSign",		wndMgrShowRequirementSign,			METH_VARARGS },
		{ "HideRequirementSign",		wndMgrHideRequirementSign,			METH_VARARGS },
		{ "RefreshSlot",				wndMgrRefreshSlot,					METH_VARARGS },
		{ "SetUseMode",					wndMgrSetUseMode,					METH_VARARGS },
		{ "SetUsableItem",				wndMgrSetUsableItem,				METH_VARARGS },

		{ "SelectSlot",					wndMgrSelectSlot,					METH_VARARGS },
		{ "ClearSelected",				wndMgrClearSelected,				METH_VARARGS },
		{ "GetSelectedSlotCount",		wndMgrGetSelectedSlotCount,			METH_VARARGS },
		{ "GetSelectedSlotNumber",		wndMgrGetSelectedSlotNumber,		METH_VARARGS },
		{ "IsSelectedSlot",				wndMgrIsSelectedSlot,				METH_VARARGS },
		{ "GetSlotCount",				wndMgrGetSlotCount,					METH_VARARGS },
		{ "LockSlot",					wndMgrLockSlot,						METH_VARARGS },
		{ "UnlockSlot",					wndMgrUnlockSlot,					METH_VARARGS },

		// Bar
		{ "SetColor",					wndBarSetColor,						METH_VARARGS },

		// TextLine
		{ "SetMax",						wndTextSetMax,						METH_VARARGS },
		{ "SetHorizontalAlign",			wndTextSetHorizontalAlign,			METH_VARARGS },
		{ "SetVerticalAlign",			wndTextSetVerticalAlign,			METH_VARARGS },
		{ "SetSecret",					wndTextSetSecret,					METH_VARARGS },
		{ "SetOutline",					wndTextSetOutline,					METH_VARARGS },
		{ "SetFeather",					wndTextSetFeather,					METH_VARARGS },
		{ "SetMultiLine",				wndTextSetMultiLine,				METH_VARARGS },
		{ "SetText",					wndTextSetText,						METH_VARARGS },
		{ "SetFontName",				wndTextSetFontName,					METH_VARARGS },
		{ "SetFontColor",				wndTextSetFontColor,				METH_VARARGS },
		{ "SetLimitWidth",				wndTextSetLimitWidth,				METH_VARARGS },
		{ "SetBaseDirection",			wndTextSetBaseDirection,			METH_VARARGS },
		{ "GetText",					wndTextGetText,						METH_VARARGS },
		{ "GetTextSize",				wndTextGetTextSize,					METH_VARARGS },
		{ "ShowCursor",					wndTextShowCursor,					METH_VARARGS },
		{ "HideCursor",					wndTextHideCursor,					METH_VARARGS },
		// MR-15: Expose text highlighting to Python
		{ "SetSelection",				wndTextSetSelection,				METH_VARARGS },
		{ "ClearSelection",				wndTextClearSelection,				METH_VARARGS },
		// MR-15: -- END OF -- Expose text highlighting to Python
		{ "GetCursorPosition",			wndTextGetCursorPosition,			METH_VARARGS },
		// NumberLine
		{ "SetNumber",						wndNumberSetNumber,							METH_VARARGS },
		{ "SetNumberHorizontalAlignCenter",	wndNumberSetNumberHorizontalAlignCenter,	METH_VARARGS },
		{ "SetNumberHorizontalAlignRight",	wndNumberSetNumberHorizontalAlignRight,		METH_VARARGS },
		{ "SetPath",						wndNumberSetPath,							METH_VARARGS },

		// ImageBox
		{ "MarkBox_SetImage",			wndMarkBox_SetImage,				METH_VARARGS },
		{ "MarkBox_SetImageFilename",	wndMarkBox_SetImageFilename,		METH_VARARGS },
		{ "MarkBox_Load",				wndMarkBox_Load,					METH_VARARGS },
		{ "MarkBox_SetIndex",			wndMarkBox_SetIndex,				METH_VARARGS },
		{ "MarkBox_SetScale",			wndMarkBox_SetScale,				METH_VARARGS },
		{ "MarkBox_SetDiffuseColor",	wndMarkBox_SetDiffuseColor,			METH_VARARGS },

		// ImageBox
		{ "LoadImage",					wndImageLoadImage,					METH_VARARGS },
		{ "LoadImageFromFile",			wndImageLoadImageFromFile,			METH_VARARGS },
		{ "SetDiffuseColor",			wndImageSetDiffuseColor,			METH_VARARGS },
		{ "GetWidth",					wndImageGetWidth,					METH_VARARGS },
		{ "GetHeight",					wndImageGetHeight,					METH_VARARGS },
		// ExpandedImageBox
		{ "SetScale",					wndImageSetScale,					METH_VARARGS },
		{ "SetOrigin",					wndImageSetOrigin,					METH_VARARGS },
		{ "SetRotation",				wndImageSetRotation,				METH_VARARGS },
		{ "SetRenderingRect",			wndImageSetRenderingRect,			METH_VARARGS },
		{ "SetRenderingMode",			wndImageSetRenderingMode,			METH_VARARGS },
		// AniImageBox
		{ "SetDelay",					wndImageSetDelay,					METH_VARARGS },
		{ "AppendImage",				wndImageAppendImage,				METH_VARARGS },

		// Button
		{ "SetUpVisual",				wndButtonSetUpVisual,				METH_VARARGS },
		{ "SetOverVisual",				wndButtonSetOverVisual,				METH_VARARGS },
		{ "SetDownVisual",				wndButtonSetDownVisual,				METH_VARARGS },
		{ "SetDisableVisual",			wndButtonSetDisableVisual,			METH_VARARGS },
		{ "GetUpVisualFileName",		wndButtonGetUpVisualFileName,		METH_VARARGS },
		{ "GetOverVisualFileName",		wndButtonGetOverVisualFileName,		METH_VARARGS },
		{ "GetDownVisualFileName",		wndButtonGetDownVisualFileName,		METH_VARARGS },
		{ "Flash",						wndButtonFlash,						METH_VARARGS },
		{ "Enable",						wndButtonEnable,					METH_VARARGS },
		{ "Disable",					wndButtonDisable,					METH_VARARGS },
		{ "Down",						wndButtonDown,						METH_VARARGS },
		{ "SetUp",						wndButtonSetUp,						METH_VARARGS },
		{ "IsDown",						wndButtonIsDown,					METH_VARARGS },

		// Button State & Simulation Bridges
		{ "ButtonSetState",				wndButtonSetState,					METH_VARARGS },
		{ "SetButtonState",				wndButtonSetState,					METH_VARARGS },
		{ "ButtonHover",				wndButtonHover,						METH_VARARGS },
		{ "ButtonOver",					wndButtonHover,						METH_VARARGS },
		{ "SimulateMouseEnter",			wndButtonSimulateMouseEnter,		METH_VARARGS },
		{ "SimulateMouseLeave",			wndButtonSimulateMouseLeave,		METH_VARARGS },
		{ "SimulateMousePress",			wndButtonSimulateMousePress,		METH_VARARGS },
		{ "SimulateMouseRelease",		wndButtonSimulateMouseRelease,		METH_VARARGS },
		{ "GetButtonVisualState",		wndButtonGetVisualState,			METH_VARARGS },

		// DragButton
		{ "SetRestrictMovementArea",	wndButtonSetRestrictMovementArea,	METH_VARARGS },

		// For Debug
		{ "SetOutlineFlag",				wndMgrSetOutlineFlag,				METH_VARARGS },
		{ "ShowOverInWindowName",		wndMgrShowOverInWindowName,			METH_VARARGS },

		{ "EnableScissorRect",			wndMgrEnableScissorRect,			METH_VARARGS },
		{ "DisableScissorRect",			wndMgrDisableScissorRect,			METH_VARARGS },
		{ "IsScissorRectEnabled",		wndMgrIsScissorRectEnabled,			METH_VARARGS },

		{ NULL,							NULL,								NULL },
	};

	PyObject * poModule = Py_InitModule("wndMgr", s_methods);

// 하나의 딕셔너리에 너무 많은 Function이 포함 되는 것 같아 이런식으로 딕셔너리를 나누는 것을 고려 중 - [levites]
//	PyObject * poMgrModule = Py_InitModule("wndMgr", s_methods);
//	PyObject * poTextModule = Py_InitModule("wndText", s_methods);
//	PyObject * poSlotModule = Py_InitModule("wndSlot", s_methods);

	PyModule_AddIntConstant(poModule, "BUTTON_STATE_NORMAL",			0);
	PyModule_AddIntConstant(poModule, "BUTTON_STATE_OVER",				1);
	PyModule_AddIntConstant(poModule, "BUTTON_STATE_DOWN",				2);
	PyModule_AddIntConstant(poModule, "BUTTON_STATE_DISABLE",			3);

	PyModule_AddIntConstant(poModule, "SLOT_STYLE_NONE",				UI::SLOT_STYLE_NONE);
	PyModule_AddIntConstant(poModule, "SLOT_STYLE_PICK_UP",				UI::SLOT_STYLE_PICK_UP);
	PyModule_AddIntConstant(poModule, "SLOT_STYLE_SELECT",				UI::SLOT_STYLE_SELECT);

	PyModule_AddIntConstant(poModule, "TEXT_HORIZONTAL_ALIGN_LEFT",		CGraphicTextInstance::HORIZONTAL_ALIGN_LEFT);
	PyModule_AddIntConstant(poModule, "TEXT_HORIZONTAL_ALIGN_RIGHT",	CGraphicTextInstance::HORIZONTAL_ALIGN_RIGHT);
	PyModule_AddIntConstant(poModule, "TEXT_HORIZONTAL_ALIGN_CENTER",	CGraphicTextInstance::HORIZONTAL_ALIGN_CENTER);
	PyModule_AddIntConstant(poModule, "TEXT_VERTICAL_ALIGN_TOP",		CGraphicTextInstance::VERTICAL_ALIGN_TOP);
	PyModule_AddIntConstant(poModule, "TEXT_VERTICAL_ALIGN_BOTTOM",		CGraphicTextInstance::VERTICAL_ALIGN_BOTTOM);
	PyModule_AddIntConstant(poModule, "TEXT_VERTICAL_ALIGN_CENTER",		CGraphicTextInstance::VERTICAL_ALIGN_CENTER);

	PyModule_AddIntConstant(poModule, "TEXT_BASEDIR_AUTO", 0);
	PyModule_AddIntConstant(poModule, "TEXT_BASEDIR_LTR",  1);
	PyModule_AddIntConstant(poModule, "TEXT_BASEDIR_RTL",  2);

	PyModule_AddIntConstant(poModule, "HORIZONTAL_ALIGN_LEFT",			UI::CWindow::HORIZONTAL_ALIGN_LEFT);
	PyModule_AddIntConstant(poModule, "HORIZONTAL_ALIGN_CENTER",		UI::CWindow::HORIZONTAL_ALIGN_CENTER);
	PyModule_AddIntConstant(poModule, "HORIZONTAL_ALIGN_RIGHT",			UI::CWindow::HORIZONTAL_ALIGN_RIGHT);
	PyModule_AddIntConstant(poModule, "VERTICAL_ALIGN_TOP",				UI::CWindow::VERTICAL_ALIGN_TOP);
	PyModule_AddIntConstant(poModule, "VERTICAL_ALIGN_CENTER",			UI::CWindow::VERTICAL_ALIGN_CENTER);
	PyModule_AddIntConstant(poModule, "VERTICAL_ALIGN_BOTTOM",			UI::CWindow::VERTICAL_ALIGN_BOTTOM);

	PyModule_AddIntConstant(poModule, "RENDERING_MODE_MODULATE",		CGraphicExpandedImageInstance::RENDERING_MODE_MODULATE);
}

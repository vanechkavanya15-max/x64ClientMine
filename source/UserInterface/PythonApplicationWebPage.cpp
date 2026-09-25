#include "StdAfx.h"
#include "PythonApplication.h"
#include <shellapi.h>

bool CPythonApplication::IsWebPageMode()
{
	return false;
}

void CPythonApplication::ShowWebPage(const char* c_szURL, const RECT& c_rcWebPage)
{
	ShellExecuteA(NULL, "open", c_szURL, NULL, NULL, SW_SHOWNORMAL);
}

void CPythonApplication::MoveWebPage(const RECT& c_rcWebPage)
{
}

void CPythonApplication::HideWebPage()
{
}

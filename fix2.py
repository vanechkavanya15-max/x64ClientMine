import sys

def process(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Step 1
    content = content.replace('''	if (windowed &&
		strnicmp(d3dAdapterId.Driver, "3dfx", 4) == 0 &&
		D3DFMT_X8R8G8B8 == d3dDisplayMode.Format)
	{
		return CREATE_FORMAT;
	}''', '''	if (windowed &&
		strnicmp(d3dAdapterId.Driver, "3dfx", 4) == 0 &&
		D3DFMT_X8R8G8B8 == d3dDisplayMode.Format)
	{
		safe_release(ms_lpd3d);
		return CREATE_FORMAT;
	}''')

    content = content.replace('''	if (!CreateD3DDeviceWithFallback(hWnd, iHres, iVres, Windowed, iReflashRate, d3dDisplayMode, iRet))
		return CREATE_DEVICE;

	CheckTextureFormats(Windowed, d3dDisplayMode);

	if (FAILED((ms_hLastResult = ms_lpd3dDevice->GetDeviceCaps(&ms_d3dCaps))))
	{
		Tracenf("IDirect3DDevice.GetDeviceCaps - ERROR %d", ms_hLastResult);
		return CREATE_GET_DEVICE_CAPS2;
	}

	if (!Windowed)
		SetWindowPos(hWnd, HWND_TOPMOST, 0, 0, iHres, iVres, SWP_SHOWWINDOW);

	ms_lpd3dDevice->GetViewport(&ms_Viewport);

	if (!InitDeviceResources())
		return CREATE_DEVICE;''', '''	if (!CreateD3DDeviceWithFallback(hWnd, iHres, iVres, Windowed, iReflashRate, d3dDisplayMode, iRet))
	{
		Destroy();
		return CREATE_DEVICE;
	}

	CheckTextureFormats(Windowed, d3dDisplayMode);

	if (FAILED((ms_hLastResult = ms_lpd3dDevice->GetDeviceCaps(&ms_d3dCaps))))
	{
		Tracenf("IDirect3DDevice.GetDeviceCaps - ERROR %d", ms_hLastResult);
		Destroy();
		return CREATE_GET_DEVICE_CAPS2;
	}

	if (!Windowed)
		SetWindowPos(hWnd, HWND_TOPMOST, 0, 0, iHres, iVres, SWP_SHOWWINDOW);

	ms_lpd3dDevice->GetViewport(&ms_Viewport);

	if (!InitDeviceResources())
	{
		Destroy();
		return CREATE_DEVICE;
	}''')

    # Step 2
    content = content.replace('''	D3DVERTEXELEMENT9 pShaderDecl[] = {
		{ 0, 0, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0 },
		{ 1, 0, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0 },
		D3DDECL_END()
	};''', '''	D3DVERTEXELEMENT9 pShaderDecl[] = {
		{ 0, 0, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0 },
		{ 0, 12, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0 },
		D3DDECL_END()
	};''')

    content = content.replace('''	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		wchar_t szError[1024];
		swprintf(szError, L"Failed to create CreatePTStreamVertexShader");
		MessageBoxW(NULL, szError, L"Vertex Shader Error", MB_ICONSTOP);
	}''', '''	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		MessageBoxW(NULL, L"Failed to create CreatePTStreamVertexShader", L"Vertex Shader Error", MB_ICONSTOP);
	}''')

    content = content.replace('''	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		wchar_t szError[1024];
		swprintf(szError, L"Failed to create CreatePNTStreamVertexShader");
		MessageBoxW(NULL, szError, L"Vertex Shader Error", MB_ICONSTOP);
	}''', '''	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		MessageBoxW(NULL, L"Failed to create CreatePNTStreamVertexShader", L"Vertex Shader Error", MB_ICONSTOP);
	}''')

    content = content.replace('''	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		wchar_t szError[1024];
		swprintf(szError, L"Failed to create CreatePNT2StreamVertexShader");
		MessageBoxW(NULL, szError, L"Vertex Shader Error", MB_ICONSTOP);
	}''', '''	if (ms_lpd3dDevice->CreateVertexDeclaration(pShaderDecl, &dwShader) != D3D_OK)
	{
		MessageBoxW(NULL, L"Failed to create CreatePNT2StreamVertexShader", L"Vertex Shader Error", MB_ICONSTOP);
	}''')

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)

process("src/EterLib/GrpDevice.cpp")

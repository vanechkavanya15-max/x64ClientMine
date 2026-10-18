import sys

def process(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Step 3
    content = content.replace('''void CGraphicDevice::CheckTextureFormats(bool windowed, const D3DDISPLAYMODE& d3dDisplayMode)
{
	const D3DFORMAT baseFormatForTextureCheck = windowed ? d3dDisplayMode.Format : ms_d3dPresentParameter.BackBufferFormat;
	if (ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT1) == D3DERR_NOTAVAILABLE)
	{
		ms_bSupportDXT = false;
	}

	if (ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT3) == D3DERR_NOTAVAILABLE)
	{
		ms_bSupportDXT = false;
	}

	if (ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT5) == D3DERR_NOTAVAILABLE)
	{
		ms_bSupportDXT = false;
	}
}''', '''void CGraphicDevice::CheckTextureFormats(bool windowed, const D3DDISPLAYMODE& d3dDisplayMode)
{
	ms_bSupportDXT = true;
	const D3DFORMAT baseFormatForTextureCheck = windowed ? d3dDisplayMode.Format : ms_d3dPresentParameter.BackBufferFormat;
	if (FAILED(ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT1)))
	{
		ms_bSupportDXT = false;
	}

	if (FAILED(ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT3)))
	{
		ms_bSupportDXT = false;
	}

	if (FAILED(ms_lpd3d->CheckDeviceFormat(
		D3DADAPTER_DEFAULT,
		D3DDEVTYPE_HAL,
		baseFormatForTextureCheck,
		0,
		D3DRTYPE_TEXTURE,
		D3DFMT_DXT5)))
	{
		ms_bSupportDXT = false;
	}
}''')

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)

process("src/EterLib/GrpDevice.cpp")

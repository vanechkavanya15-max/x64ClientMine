import sys

def process(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Step 4: ResizeBackBuffer
    content = content.replace('''				rkD3DPP.BackBufferWidth = oldWidth;
				rkD3DPP.BackBufferHeight = oldHeight;
				return false;
			}

			// Recreate D3DPOOL_DEFAULT resources after successful Reset''', '''				rkD3DPP.BackBufferWidth = oldWidth;
				rkD3DPP.BackBufferHeight = oldHeight;
				if (hr != D3DERR_DEVICELOST && hr != D3DERR_DEVICENOTRESET)
				{
					__RecreateDefaultResources();
				}
				return false;
			}

			// Recreate D3DPOOL_DEFAULT resources after successful Reset''')

    # Step 4: ResetWithParams
    content = content.replace('''		else
		{
			TraceError("CGraphicDevice::ResetWithParams: Reset failed (hr=0x%08X)", hr);
		}
		return false;
	}

	ms_d3dPresentParameter = d3dpp;''', '''		else
		{
			TraceError("CGraphicDevice::ResetWithParams: Reset failed (hr=0x%08X)", hr);
			__RecreateDefaultResources();
		}
		return false;
	}

	ms_d3dPresentParameter = d3dpp;''')

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)

process("src/EterLib/GrpDevice.cpp")

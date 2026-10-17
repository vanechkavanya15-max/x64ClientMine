#pragma once

#include <d3d9.h>

namespace EterLib::Render
{
    // RAII Guard dla stanu maszyny Direct3D 9 (IDirect3DStateBlock9)
    // Zapewnia deterministyczny Save (Capture) i Restore (Apply) stanu maszyny Direct3D.
    class D3D9StateBlockScope
    {
    public:
        explicit D3D9StateBlockScope(IDirect3DDevice9* pDevice)
            : m_pStateBlock(nullptr)
        {
            if (pDevice && SUCCEEDED(pDevice->CreateStateBlock(D3DSBT_ALL, &m_pStateBlock)))
            {
                if (FAILED(m_pStateBlock->Capture()))
                {
                    m_pStateBlock->Release();
                    m_pStateBlock = nullptr;
                }
            }
        }

        ~D3D9StateBlockScope()
        {
            Restore();
        }

        D3D9StateBlockScope(const D3D9StateBlockScope&) = delete;
        D3D9StateBlockScope& operator=(const D3D9StateBlockScope&) = delete;

        D3D9StateBlockScope(D3D9StateBlockScope&& other) noexcept
            : m_pStateBlock(other.m_pStateBlock)
        {
            other.m_pStateBlock = nullptr;
        }

        D3D9StateBlockScope& operator=(D3D9StateBlockScope&& other) noexcept
        {
            if (this != &other)
            {
                Restore();
                m_pStateBlock = other.m_pStateBlock;
                other.m_pStateBlock = nullptr;
            }
            return *this;
        }

        [[nodiscard]] bool HasCaptured() const noexcept
        {
            return m_pStateBlock != nullptr;
        }

        void Restore() noexcept
        {
            if (m_pStateBlock)
            {
                m_pStateBlock->Apply();
                m_pStateBlock->Release();
                m_pStateBlock = nullptr;
            }
        }

    private:
        IDirect3DStateBlock9* m_pStateBlock;
    };
}

#pragma once

#include <d3d9.h>
#include <d3dx9.h>
#include <vector>
#include <utility>
#include <tuple>
#include <initializer_list>

class CStateManager;

/**
 * @brief Straznik RAII dla deklaracji wierzcholkow Direct3D 9 (IDirect3DVertexDeclaration9).
 * Zapobiega wyciekom deklaracji wierzcholkow do faz renderowania opartych na FVF.
 */
class ScopedD3DVertexDeclGuard
{
public:
    explicit ScopedD3DVertexDeclGuard(LPDIRECT3DVERTEXDECLARATION9 pNewDecl = nullptr, bool bRestoreOnExit = true);
    ~ScopedD3DVertexDeclGuard() noexcept;

    void Set(LPDIRECT3DVERTEXDECLARATION9 pNewDecl);
    void Clear();
    void Dismiss() noexcept;

    LPDIRECT3DVERTEXDECLARATION9 GetSavedDecl() const noexcept { return m_pSavedDecl; }
    DWORD GetSavedFVF() const noexcept { return m_dwSavedFVF; }
    void SetRestoreOnExit(bool bRestore) noexcept { m_bRestoreOnExit = bRestore; }
    void SetExitTarget(LPDIRECT3DVERTEXDECLARATION9 pExitDecl) noexcept { m_pExitDecl = pExitDecl; }
    void SetExitFVF(DWORD dwExitFVF) noexcept { m_dwExitFVF = dwExitFVF; }

    ScopedD3DVertexDeclGuard(const ScopedD3DVertexDeclGuard&) = delete;
    ScopedD3DVertexDeclGuard& operator=(const ScopedD3DVertexDeclGuard&) = delete;
    ScopedD3DVertexDeclGuard(ScopedD3DVertexDeclGuard&& other) noexcept;
    ScopedD3DVertexDeclGuard& operator=(ScopedD3DVertexDeclGuard&& other) noexcept;

private:
    LPDIRECT3DVERTEXDECLARATION9 m_pSavedDecl = nullptr;
    DWORD m_dwSavedFVF = 0;
    LPDIRECT3DVERTEXDECLARATION9 m_pExitDecl = nullptr;
    DWORD m_dwExitFVF = 0;
    bool m_bRestoreOnExit = true;
    bool m_bActive = true;
};

/**
 * @brief Glowny straznik RAII dla potoku Fixed-Function Pipeline (FFP).
 * Gwarantuje wyczyszczenie Vertex Declaration, Vertex Shader i Pixel Shader do NULL
 * oraz poprawne ustawienie FVF bez mozliwosci wycieku stanu miedzy fazami renderowania.
 */
class ScopedD3DFFPGuard
{
public:
    explicit ScopedD3DFFPGuard(DWORD dwTargetFVF = 0, bool bRestoreOnExit = true);
    ~ScopedD3DFFPGuard() noexcept;

    void SetFVF(DWORD dwTargetFVF);
    void EnsureCleanState();
    void Dismiss() noexcept;

    ScopedD3DFFPGuard(const ScopedD3DFFPGuard&) = delete;
    ScopedD3DFFPGuard& operator=(const ScopedD3DFFPGuard&) = delete;
    ScopedD3DFFPGuard(ScopedD3DFFPGuard&& other) noexcept;
    ScopedD3DFFPGuard& operator=(ScopedD3DFFPGuard&& other) noexcept;

private:
    LPDIRECT3DVERTEXDECLARATION9 m_pSavedDecl = nullptr;
    LPDIRECT3DVERTEXSHADER9 m_pSavedVS = nullptr;
    LPDIRECT3DPIXELSHADER9 m_pSavedPS = nullptr;
    DWORD m_dwSavedFVF = 0;
    bool m_bRestoreOnExit = true;
    bool m_bActive = true;
};

/**
 * @brief Straznik RAII dla potoku renderowania diagnostycznego (DebugRenderer).
 * Zapisuje biezace RenderStates (ZENABLE, ZWRITEENABLE, LIGHTING, ALPHABLENDENABLE, CULLMODE),
 * TextureStageStates (COLOROP, COLORARG1 dla stage 0 i 1) oraz FVF.
 * Wylacza oswietlenie (D3DRS_LIGHTING = FALSE), wylacza tekstury (STATEMANAGER.SetTexture(0, NULL), STATEMANAGER.SetTexture(1, NULL)),
 * ustawia FVF na D3DFVF_XYZ | D3DFVF_DIFFUSE.
 * Destruktor przywraca dokladnie wszystkie poprzednie stany, zapobiegajac zatruciu potoku terenu i interfejsu.
 */
class ScopedDebugRenderGuard
{
public:
    explicit ScopedDebugRenderGuard(DWORD dwTargetFVF = (D3DFVF_XYZ | D3DFVF_DIFFUSE), bool bDisableLighting = true, bool bRestoreOnExit = true);
    ~ScopedDebugRenderGuard() noexcept;

    void Dismiss() noexcept;

    ScopedDebugRenderGuard(const ScopedDebugRenderGuard&) = delete;
    ScopedDebugRenderGuard& operator=(const ScopedDebugRenderGuard&) = delete;
    ScopedDebugRenderGuard(ScopedDebugRenderGuard&& other) noexcept;
    ScopedDebugRenderGuard& operator=(ScopedDebugRenderGuard&& other) noexcept;

private:
    DWORD m_dwSavedZEnable = 0;
    DWORD m_dwSavedZWriteEnable = 0;
    DWORD m_dwSavedLighting = 0;
    DWORD m_dwSavedAlphaBlendEnable = 0;
    DWORD m_dwSavedCullMode = 0;

    DWORD m_dwSavedStage0ColorOp = 0;
    DWORD m_dwSavedStage0ColorArg1 = 0;
    DWORD m_dwSavedStage1ColorOp = 0;
    DWORD m_dwSavedStage1ColorArg1 = 0;

    LPDIRECT3DBASETEXTURE9 m_pSavedTexture0 = nullptr;
    LPDIRECT3DBASETEXTURE9 m_pSavedTexture1 = nullptr;

    DWORD m_dwSavedFVF = 0;
    LPDIRECT3DVERTEXDECLARATION9 m_pSavedDecl = nullptr;
    LPDIRECT3DVERTEXSHADER9 m_pSavedVS = nullptr;
    LPDIRECT3DPIXELSHADER9 m_pSavedPS = nullptr;

    D3DXMATRIX m_savedWorldMatrix;

    bool m_bRestoreOnExit = true;
    bool m_bActive = true;
};

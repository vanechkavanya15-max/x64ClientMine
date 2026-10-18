#pragma once
#include "StdAfx.h"
#include "PythonPlayer.h"
#include "PythonCharacterManager.h"
#include "PythonBackground.h"
#include "PythonItem.h"
#include "PythonTextTail.h"
#include "PythonMiniMap.h"
#include "NetworkActorManager.h"
#include "EffectLib/EffectManager.h"
#include "GameLib/FlyingObjectManager.h"

namespace UserInterface::Phase
{
    // Bezpieczny wrapper RAII zapobiegajacy wyciekom referencji przy Py_BuildValue
    struct ScopedPyObject
    {
        PyObject* ptr{nullptr};
        explicit ScopedPyObject(PyObject* p) noexcept : ptr(p) {}
        ~ScopedPyObject() noexcept { Py_XDECREF(ptr); }
        ScopedPyObject(const ScopedPyObject&) = delete;
        ScopedPyObject& operator=(const ScopedPyObject&) = delete;
        operator PyObject*() const noexcept { return ptr; }
    };

    // Atomowe czyszczenie wszystkich menedzerow swiata przed zaladowaniem nowej mapy / fazy
    inline void PurgeWorldResourcesAtomically(CNetworkActorManager* pNetActorMgr)
    {
        // 1. Zatrzymanie i wyczyszczenie gracza oraz celow
        CPythonPlayer::Instance().Clear();

        // 2. Usuniecie wszystkich aktorow sceny i wyczyszczenie menedzera sieciowego aktorow
        CPythonCharacterManager::Instance().DeleteAllInstances();
        if (pNetActorMgr)
        {
            pNetActorMgr->Destroy();
        }

        // 3. Wyczyszczenie przedmiotow na ziemi, napisow (TextTail), efektow i pociskow
        CPythonItem::Instance().Destroy();
        CPythonTextTail::Instance().Clear();
        CFlyingManager::Instance().DeleteAllInstances();
        CEffectManager::Instance().DeleteAllInstances();

        // 4. Zwolnienie obszarow gildii i minimapy
        CPythonMiniMap::Instance().ClearGuildArea();
        CPythonBackground::Instance().ClearGuildArea();
    }
}

#pragma once

#include "StdAfx.h"

// ============================================================================
// Domain: Motion, Rotation & Combat Animation (src/UserInterface/PythonCharacterModule_Motion.cpp)
// ============================================================================
PyObject * chrSetRotation(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetRotationAll(PyObject* poSelf, PyObject* poArgs);
PyObject * chrBlendRotation(PyObject* poSelf, PyObject* poArgs);
PyObject * chrGetRotation(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetPixelPosition(PyObject* poSelf, PyObject* poArgs);
PyObject * chrMoveToDestPosition(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetMotionMode(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs);
PyObject * chrSetMotion(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs);
PyObject * chrPushOnceMotion(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetLoopMotion(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs);
PyObject * chrBlendLoopMotion(PyObject* poSelf, PyObject* poArgs);
PyObject * chrPushLoopMotion(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetDirection(PyObject* poSelf, PyObject* poArgs);
PyObject * chrtestSetComboType(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetComboType(PyObject* poSelf, PyObject* poArgs);
PyObject * chrFaintTest(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetMoveSpeed(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetAttackSpeed(PyObject* poSelf, PyObject* poArgs);

// ============================================================================
// Domain: Instance Management & Visual Appearance (src/UserInterface/PythonCharacterModule_Visual.cpp)
// ============================================================================
PyObject * chrCreateInstance(PyObject* poSelf, PyObject* poArgs);
PyObject * chrDeleteInstance(PyObject* poSelf, PyObject* poArgs);
PyObject * chrDeleteInstanceByFade(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSelectInstance(PyObject* poSelf, PyObject* poArgs);
PyObject * chrHasInstance(PyObject* poSelf, PyObject* poArgs);
PyObject * chrIsEnemy(PyObject* poSelf, PyObject* poArgs);
PyObject * chrIsNPC(PyObject* poSelf, PyObject* poArgs);
PyObject * chrIsGameMaster(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetArmor(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs);
PyObject * chrSetWeapon(PyObject* poSelf, PyObject * const * poArgs, Py_ssize_t nargs);
PyObject * chrSetHair(PyObject* poSelf, PyObject* poArgs);
PyObject * chrChangeHair(PyObject* poSelf, PyObject* poArgs);
PyObject * chrSetAcce(PyObject* poSelf, PyObject* poArgs);
PyObject * chrChangeShape(PyObject* poSelf, PyObject* poArgs);
PyObject * chrShowEffect(PyObject* poSelf, PyObject* poArgs);
PyObject * chrClearEffect(PyObject* poSelf, PyObject* poArgs);
PyObject * chrAttachEffectByName(PyObject* poSelf, PyObject* poArgs);
PyObject * chrAttachEffectByID(PyObject* poSelf, PyObject* poArgs);

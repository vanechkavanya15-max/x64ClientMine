#pragma once

#include <Python.h>

// Trade (Exchange)
PyObject* netSendExchangeStartPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendExchangeItemAddPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendExchangeItemDelPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendExchangeElkAddPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendExchangeAcceptPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendExchangeExitPacket(PyObject* poSelf, PyObject* poArgs);

// Shop
PyObject* netSendShopEndPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendShopBuyPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendShopSellPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendShopSellPacketNew(PyObject* poSelf, PyObject* poArgs);

// Safebox & Mall
PyObject* netSendSafeboxSaveMoneyPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendSafeboxWithdrawMoneyPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendSafeboxCheckinPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendSafeboxCheckoutPacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendSafeboxItemMovePacket(PyObject* poSelf, PyObject* poArgs);
PyObject* netSendMallCheckoutPacket(PyObject* poSelf, PyObject* poArgs);

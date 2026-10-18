#include "stdafx.h"
#include "PythonExchange.h"
#include "ExchangeErrors.h"
#include <cstring>

void CPythonExchange::InvalidateAcceptState()
{
	m_self.accept = 0;
	m_victim.accept = 0;
}

void CPythonExchange::SetSelfName(const char *name)
{
	if (!name) return;
	strncpy(m_self.name, name, CHARACTER_NAME_MAX_LEN);
	m_self.name[CHARACTER_NAME_MAX_LEN] = '\0';
}

void CPythonExchange::SetTargetName(const char *name)
{
	if (!name) return;
	strncpy(m_victim.name, name, CHARACTER_NAME_MAX_LEN);
	m_victim.name[CHARACTER_NAME_MAX_LEN] = '\0';
}

const char * CPythonExchange::GetNameFromSelf() const
{
	return m_self.name;
}

const char * CPythonExchange::GetNameFromTarget() const
{
	return m_victim.name;
}

void CPythonExchange::SetElkToTarget(uint64_t elk)
{	
	InvalidateAcceptState();
	m_victim.elk = elk;
}

void CPythonExchange::SetElkToSelf(uint64_t elk)
{
	InvalidateAcceptState();
	m_self.elk = elk;
}

uint64_t CPythonExchange::GetElkFromTarget() const
{
	return m_victim.elk;
}

uint64_t CPythonExchange::GetElkFromSelf() const
{
	return m_self.elk;
}

void CPythonExchange::SetItemToTarget(DWORD pos, DWORD vnum, BYTE count)
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_victim.item_vnum[pos] = vnum;
	m_victim.item_count[pos] = count;
	memset(m_victim.item_metin[pos], 0, sizeof(m_victim.item_metin[pos]));
	memset(m_victim.item_attr[pos], 0, sizeof(m_victim.item_attr[pos]));
}

void CPythonExchange::SetItemToSelf(DWORD pos, DWORD vnum, BYTE count)
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_self.item_vnum[pos] = vnum;
	m_self.item_count[pos] = count;
	memset(m_self.item_metin[pos], 0, sizeof(m_self.item_metin[pos]));
	memset(m_self.item_attr[pos], 0, sizeof(m_self.item_attr[pos]));
}

void CPythonExchange::SetItemMetinSocketToTarget(int pos, int imetinpos, DWORD vnum)
{
	if (pos < 0 || pos >= EXCHANGE_ITEM_MAX_NUM || imetinpos < 0 || imetinpos >= ITEM_SOCKET_SLOT_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_victim.item_metin[pos][imetinpos] = vnum;
}

void CPythonExchange::SetItemMetinSocketToSelf(int pos, int imetinpos, DWORD vnum)
{
	if (pos < 0 || pos >= EXCHANGE_ITEM_MAX_NUM || imetinpos < 0 || imetinpos >= ITEM_SOCKET_SLOT_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_self.item_metin[pos][imetinpos] = vnum;
}

void CPythonExchange::SetItemAttributeToTarget(int pos, int iattrpos, BYTE byType, short sValue)
{
	if (pos < 0 || pos >= EXCHANGE_ITEM_MAX_NUM || iattrpos < 0 || iattrpos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_victim.item_attr[pos][iattrpos].bType = byType;
	m_victim.item_attr[pos][iattrpos].sValue = sValue;
}

void CPythonExchange::SetItemAttributeToSelf(int pos, int iattrpos, BYTE byType, short sValue)
{
	if (pos < 0 || pos >= EXCHANGE_ITEM_MAX_NUM || iattrpos < 0 || iattrpos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_self.item_attr[pos][iattrpos].bType = byType;
	m_self.item_attr[pos][iattrpos].sValue = sValue;
}

void CPythonExchange::DelItemOfTarget(BYTE pos)
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_victim.item_vnum[pos] = 0;
	m_victim.item_count[pos] = 0;
	memset(m_victim.item_metin[pos], 0, sizeof(m_victim.item_metin[pos]));
	memset(m_victim.item_attr[pos], 0, sizeof(m_victim.item_attr[pos]));
}

void CPythonExchange::DelItemOfSelf(BYTE pos)
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return;

	InvalidateAcceptState();
	m_self.item_vnum[pos] = 0;
	m_self.item_count[pos] = 0;
	memset(m_self.item_metin[pos], 0, sizeof(m_self.item_metin[pos]));
	memset(m_self.item_attr[pos], 0, sizeof(m_self.item_attr[pos]));
}

DWORD CPythonExchange::GetItemVnumFromTarget(BYTE pos) const
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return 0;

	return m_victim.item_vnum[pos];
}

DWORD CPythonExchange::GetItemVnumFromSelf(BYTE pos) const
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return 0;

	return m_self.item_vnum[pos];
}

BYTE CPythonExchange::GetItemCountFromTarget(BYTE pos) const
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return 0;

	return m_victim.item_count[pos];
}

BYTE CPythonExchange::GetItemCountFromSelf(BYTE pos) const
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM)
		return 0;

	return m_self.item_count[pos];
}

DWORD CPythonExchange::GetItemMetinSocketFromTarget(BYTE pos, int iMetinSocketPos) const
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM || iMetinSocketPos < 0 || iMetinSocketPos >= ITEM_SOCKET_SLOT_MAX_NUM)
		return 0;

	return m_victim.item_metin[pos][iMetinSocketPos];
}

DWORD CPythonExchange::GetItemMetinSocketFromSelf(BYTE pos, int iMetinSocketPos) const
{
	if (pos >= EXCHANGE_ITEM_MAX_NUM || iMetinSocketPos < 0 || iMetinSocketPos >= ITEM_SOCKET_SLOT_MAX_NUM)
		return 0;

	return m_self.item_metin[pos][iMetinSocketPos];
}

void CPythonExchange::GetItemAttributeFromTarget(BYTE pos, int iAttrPos, BYTE * pbyType, short * psValue) const
{
	if (!pbyType || !psValue || pos >= EXCHANGE_ITEM_MAX_NUM || iAttrPos < 0 || iAttrPos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM)
		return;

	*pbyType = m_victim.item_attr[pos][iAttrPos].bType;
	*psValue = m_victim.item_attr[pos][iAttrPos].sValue;
}

void CPythonExchange::GetItemAttributeFromSelf(BYTE pos, int iAttrPos, BYTE * pbyType, short * psValue) const
{
	if (!pbyType || !psValue || pos >= EXCHANGE_ITEM_MAX_NUM || iAttrPos < 0 || iAttrPos >= ITEM_ATTRIBUTE_SLOT_MAX_NUM)
		return;

	*pbyType = m_self.item_attr[pos][iAttrPos].bType;
	*psValue = m_self.item_attr[pos][iAttrPos].sValue;
}

void CPythonExchange::SetAcceptToTarget(BYTE Accept)
{
	m_victim.accept = Accept ? true : false;
}

void CPythonExchange::SetAcceptToSelf(BYTE Accept)
{
	m_self.accept = Accept ? true : false;
}

bool CPythonExchange::GetAcceptFromTarget() const
{
	return m_victim.accept ? true : false;
}

bool CPythonExchange::GetAcceptFromSelf() const
{
	return m_self.accept ? true : false;
}

bool CPythonExchange::GetElkMode() const
{
	return m_elk_mode;
}

void CPythonExchange::SetElkMode(bool value)
{
	m_elk_mode = value;
}

void CPythonExchange::Start()
{
	m_isTrading = true;
}

void CPythonExchange::End()
{
	m_isTrading = false;
	Clear();
}

bool CPythonExchange::isTrading()
{
	return m_isTrading;
}

void CPythonExchange::Clear()
{
	memset(&m_self, 0, sizeof(m_self));
	memset(&m_victim, 0, sizeof(m_victim));
}

CPythonExchange::CPythonExchange()
{
	Clear();
	m_isTrading = false;
	m_elk_mode = false;
}

CPythonExchange::~CPythonExchange()
{
}

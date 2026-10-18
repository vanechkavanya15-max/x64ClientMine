#pragma once

#include "Packet.h"

/*
 *	교환 창 관련
 */
class CPythonExchange : public CSingleton<CPythonExchange>
{
	public:
		enum
		{
			EXCHANGE_ITEM_MAX_NUM = 12,
		};

		typedef struct trade
		{
			char					name[CHARACTER_NAME_MAX_LEN + 1];

			DWORD					item_vnum[EXCHANGE_ITEM_MAX_NUM];
			BYTE					item_count[EXCHANGE_ITEM_MAX_NUM];
			DWORD					item_metin[EXCHANGE_ITEM_MAX_NUM][ITEM_SOCKET_SLOT_MAX_NUM];
			TPlayerItemAttribute	item_attr[EXCHANGE_ITEM_MAX_NUM][ITEM_ATTRIBUTE_SLOT_MAX_NUM];

			BYTE					accept;
			uint64_t				elk;
		} TExchangeData;

	public:
		CPythonExchange();
		virtual ~CPythonExchange();

		void			Clear();

		void			Start();
		void			End();
		bool			isTrading();

		// Interface

		void			SetSelfName(const char *name);
		void			SetTargetName(const char *name);

		const char *	GetNameFromSelf() const;
		const char *	GetNameFromTarget() const;

		void			SetElkToTarget(uint64_t elk);
		void			SetElkToSelf(uint64_t elk);

		uint64_t		GetElkFromTarget() const;
		uint64_t		GetElkFromSelf() const;

		void			SetItemToTarget(DWORD pos, DWORD vnum, BYTE count);
		void			SetItemToSelf(DWORD pos, DWORD vnum, BYTE count);

		void			SetItemMetinSocketToTarget(int pos, int imetinpos, DWORD vnum);
		void			SetItemMetinSocketToSelf(int pos, int imetinpos, DWORD vnum);

		void			SetItemAttributeToTarget(int pos, int iattrpos, BYTE byType, short sValue);
		void			SetItemAttributeToSelf(int pos, int iattrpos, BYTE byType, short sValue);

		void			DelItemOfTarget(BYTE pos);
		void			DelItemOfSelf(BYTE pos);

		DWORD			GetItemVnumFromTarget(BYTE pos) const;
		DWORD			GetItemVnumFromSelf(BYTE pos) const;

		BYTE			GetItemCountFromTarget(BYTE pos) const;
		BYTE			GetItemCountFromSelf(BYTE pos) const;

		DWORD			GetItemMetinSocketFromTarget(BYTE pos, int iMetinSocketPos) const;
		DWORD			GetItemMetinSocketFromSelf(BYTE pos, int iMetinSocketPos) const;

		void			GetItemAttributeFromTarget(BYTE pos, int iAttrPos, BYTE * pbyType, short * psValue) const;
		void			GetItemAttributeFromSelf(BYTE pos, int iAttrPos, BYTE * pbyType, short * psValue) const;

		void			SetAcceptToTarget(BYTE Accept);
		void			SetAcceptToSelf(BYTE Accept);

		bool			GetAcceptFromTarget() const;
		bool			GetAcceptFromSelf() const;

		bool			GetElkMode() const;
		void			SetElkMode(bool value);

	protected:
		void			InvalidateAcceptState();

	protected:
		bool				m_isTrading;

		bool				m_elk_mode;   // 엘크를 클릭해서 교환했을때를 위한 변종임.
		TExchangeData		m_self;
		TExchangeData		m_victim;
};

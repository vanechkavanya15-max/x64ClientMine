#pragma once

#include "EterLib/RenderPacketTypes.h"
#include <cstdint>

class CDungeonModelInstance;

/**
 * @brief Ekstraktor geometrii i tekstur wnetrz lochow (CDungeonModelInstance).
 * Pobiera stan siatek, wierzcholkow TPNT2Vertex, tekstur i map swiatla
 * oraz emituje pakiety TDungeonBlockPacket do CRenderQueueManager.
 * Zgodny z Master Blueprint Faza 4 (SRP, zero alokacji na klatke).
 */
class CDungeonBlockRenderExtractor
{
public:
	CDungeonBlockRenderExtractor() = delete;
	~CDungeonBlockRenderExtractor() = delete;

	/**
	 * @brief Ekstrahuje pakiety ze stanu CDungeonModelInstance i emituje do CRenderQueueManager.
	 * @param pInstance Wskaznik na instancje modelu lochu.
	 * @return true jesli wyemitowano co najmniej jeden pakiet, false w przeciwnym razie.
	 */
	static bool ExtractAndSubmit(CDungeonModelInstance* pInstance);
};

using DungeonBlockRenderExtractor = CDungeonBlockRenderExtractor;

namespace GameLib::Render
{
	using ::CDungeonBlockRenderExtractor;
	using ::DungeonBlockRenderExtractor;
}

#pragma once

#include "EterLib/RenderPacketTypes.h"
#include <cstdint>

class CDungeonModelInstance;
class CGrannyLODController;

/**
 * @brief Ekstraktor geometrii i tekstur wnetrz lochow (CDungeonModelInstance oraz glTF CGrannyLODController).
 * Pobiera stan siatek, wierzcholkow TPNT2Vertex / glTF, tekstur i map swiatla
 * oraz emituje pakiety TDungeonBlockPacket do CRenderQueueManager.
 * Zgodny z Master Blueprint Faza 4 (SRP, zero alokacji na klatke).
 */
class CDungeonBlockRenderExtractor
{
public:
	CDungeonBlockRenderExtractor() = delete;
	~CDungeonBlockRenderExtractor() = delete;

	/**
	 * @brief Sprawdza, czy ekstraktor kolejki lochow jest aktywny dla biezacej klatki (Single-Owner Pipeline DEC-3).
	 * Zwraca true, jesli kolejka renderowania CRenderQueueManager zarzadza geometria lochu,
	 * blokujac bezposrednie wywolania LegacyDraws w potoku D3D9.
	 */
	static bool IsActive();

	/**
	 * @brief Reczne przelaczanie aktywnosci ekstraktora lochow.
	 */
	static void SetActive(bool bActive);

	/**
	 * @brief Ekstrahuje pakiety ze stanu CDungeonModelInstance i emituje do CRenderQueueManager.
	 * @param pInstance Wskaznik na instancje modelu lochu.
	 * @return true jesli wyemitowano co najmniej jeden pakiet, false w przeciwnym razie.
	 */
	static bool ExtractAndSubmit(CDungeonModelInstance* pInstance);

	/**
	 * @brief Ekstrahuje pakiety ze stanu CGrannyLODController (model glTF lochu) i emituje do CRenderQueueManager.
	 * @param pController Wskaznik na kontroler LOD lochu glTF.
	 * @param pmatWorld Wskaznik na macierz swiata.
	 * @return true jesli wyemitowano co najmniej jeden pakiet, false w przeciwnym razie.
	 */
	static bool ExtractAndSubmit(CGrannyLODController* pController, const D3DXMATRIX* pmatWorld = nullptr);

private:
	static bool s_bActive;
};

using DungeonBlockRenderExtractor = CDungeonBlockRenderExtractor;

namespace GameLib::Render
{
	using ::CDungeonBlockRenderExtractor;
	using ::DungeonBlockRenderExtractor;
}

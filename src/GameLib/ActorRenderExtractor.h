#pragma once

#include "EterLib/RenderPacketTypes.h"
#include <cstdint>

class CActorInstance;

/**
 * @brief Ekstraktor danych renderowania aktora (CActorInstance).
 * Odpowiada za przeksztalcenie stanu aktora do czystej struktury C-ABI POD TActorRenderPacket.
 * Realizuje zasade Single Responsibility Principle (SRP) i Master Blueprint Faza 3.
 */
class ActorRenderExtractor
{
public:
	ActorRenderExtractor() = delete;
	~ActorRenderExtractor() = delete;

	/**
	 * @brief Ekstrahuje pelny stan aktora: identyfikator, macierz swiata, ID siatki i tekstury,
	 * palete macierzy kosci szkieletu oraz macierze socketow dla broni, helmu, fryzury i aury.
	 * @param actor Instancja zrodlowa aktora.
	 * @param outPacket Docelowy pakiet TActorRenderPacket do wyslania do kolejki renderowania.
	 * @return true jesli ekstrakcja powiodla sie, false w przeciwnym razie.
	 */
	static bool Extract(CActorInstance& actor, TActorRenderPacket& outPacket);

	/**
	 * @brief Ekstrahuje macierze socketow dla doczepianych elementow (bron prawa/lewa dlon, helm, fryzura, aury).
	 * @param actor Instancja zrodlowa aktora.
	 * @param outPacket Docelowy pakiet TActorRenderPacket z uzupelnionymi macierzami socketow.
	 * @return true jesli ekstrakcja powiodla sie.
	 */
	static bool ExtractSockets(CActorInstance& actor, TActorRenderPacket& outPacket);

	/**
	 * @brief Ekstrahuje palete macierzy kosci szkieletu (do 64 kosci dla hardware skinningu GPU).
	 * @param actor Instancja zrodlowa aktora.
	 * @param outPacket Docelowy pakiet TActorRenderPacket z uzupelniona paleta macierzy kosci.
	 * @return true jesli ekstrakcja powiodla sie.
	 */
	static bool ExtractBonePalette(CActorInstance& actor, TActorRenderPacket& outPacket);
};

namespace GameLib::Render
{
	using ::ActorRenderExtractor;
}

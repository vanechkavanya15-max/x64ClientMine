#pragma once

#include <cstdint>
#include <string>
#include <memory>
#include <unordered_map>
#include <string_view>
#include <optional>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::Skills {

/**
 * @brief Enumy bledow operacji generowania i cacheowania ikon umiejetnosci.
 */
enum class SkillIconError : uint8_t {
    None = 0,
    InvalidVnum,
    InvalidGrade,
    GeneratorError,
    CacheMiss,
    OverrideFailed,
    InvalidArgument
};

/**
 * @brief Poziomy zaawansowania umiejetnosci w systemie.
 */
enum class SkillGrade : uint8_t { 
    Normal = 0, 
    Master = 1, 
    GrandMaster = 2, 
    PerfectMaster = 3 
};

/**
 * @brief Interfejs generatora sciezek do ikon umiejetnosci.
 * Zapewnia enkapsulacje algorytmu budowania sciezki.
 */
class ISkillPathGenerator {
public:
    virtual ~ISkillPathGenerator() = default;

    /**
     * @brief Generuje sciezke dla zadanego indeksu umiejetnosci i poziomu.
     * @param skillIndex Mocny typ indeksu umiejetnosci.
     * @param grade Poziom umiejetnosci.
     * @return Result zawierajacy wygenerowana sciezke lub blad.
     */
    [[nodiscard]] virtual Client::Core::Result<std::string, SkillIconError> GeneratePath(
        Client::Core::SkillIndex skillIndex, SkillGrade grade) const = 0;
};

/**
 * @brief Interfejs pamieci podrecznej dla wygenerowanych sciezek ikon.
 */
class ISkillIconCache {
public:
    virtual ~ISkillIconCache() = default;

    /**
     * @brief Pobiera zbuforowana sciezke.
     * @param skillIndex Mocny typ indeksu.
     * @param grade Poziom umiejetnosci.
     * @return Opcjonalna sciezka jezeli istnieje w cache.
     */
    [[nodiscard]] virtual std::optional<std::string> GetCachedPath(
        Client::Core::SkillIndex skillIndex, SkillGrade grade) const = 0;

    /**
     * @brief Zapisuje wygenerowana sciezke w buforze.
     * @param skillIndex Mocny typ indeksu.
     * @param grade Poziom umiejetnosci.
     * @param path Wygenerowana sciezka zasobu.
     */
    virtual void CachePath(Client::Core::SkillIndex skillIndex, SkillGrade grade, std::string_view path) = 0;
    
    /**
     * @brief Czysci caly bufor.
     */
    virtual void Clear() = 0;
};

/**
 * @brief Klasa przechowujaca klucz dla operacji na mapach (cache, overrides).
 */
struct SkillIconKey {
    Client::Core::SkillIndex skillIndex;
    SkillGrade grade;

    constexpr bool operator==(const SkillIconKey& other) const noexcept {
        return skillIndex == other.skillIndex && grade == other.grade;
    }
};

} // namespace Client::Skills

template <>
struct std::hash<Client::Skills::SkillIconKey> {
    std::size_t operator()(const Client::Skills::SkillIconKey& key) const noexcept {
        auto h1 = std::hash<uint32_t>{}(key.skillIndex.value());
        auto h2 = std::hash<uint8_t>{}(static_cast<uint8_t>(key.grade));
        return h1 ^ (h2 << 1);
    }
};

namespace Client::Skills {

/**
 * @brief Domyslna implementacja generatora sciezek.
 */
class DefaultSkillPathGenerator final : public ISkillPathGenerator {
public:
    DefaultSkillPathGenerator() = default;
    ~DefaultSkillPathGenerator() override = default;

    [[nodiscard]] Client::Core::Result<std::string, SkillIconError> GeneratePath(
        Client::Core::SkillIndex skillIndex, SkillGrade grade) const override;
        
private:
    [[nodiscard]] Client::Core::Result<std::string, SkillIconError> ValidateIndex(Client::Core::SkillIndex index) const;
    [[nodiscard]] Client::Core::Result<std::string_view, SkillIconError> GetGradeSuffix(SkillGrade grade) const;
    [[nodiscard]] Client::Core::Result<std::string, SkillIconError> BuildPath(uint32_t rawVnum, std::string_view suffix) const;
};

/**
 * @brief Domyslna implementacja bufora ikon.
 */
class DefaultSkillIconCache final : public ISkillIconCache {
public:
    DefaultSkillIconCache() = default;
    ~DefaultSkillIconCache() override = default;

    [[nodiscard]] std::optional<std::string> GetCachedPath(
        Client::Core::SkillIndex skillIndex, SkillGrade grade) const override;

    void CachePath(Client::Core::SkillIndex skillIndex, SkillGrade grade, std::string_view path) override;
    
    void Clear() override;

private:
    std::unordered_map<SkillIconKey, std::string> cacheMap_;
};

/**
 * @brief Glowny koordynator zarzadzajacy ikonami umiejetnosci.
 * Odpowiada za orkiestracje generatora, bufora oraz nadpisan.
 */
class SkillGradeIconManager final {
public:
    /**
     * @brief Konstruktor wstrzykujacy zaleznosci.
     * @param generator Wskaznik na generator sciezek (nie moze byc nullptr).
     * @param cache Wskaznik na pamiec podreczna (nie moze byc nullptr).
     */
    SkillGradeIconManager(
        std::unique_ptr<ISkillPathGenerator> generator,
        std::unique_ptr<ISkillIconCache> cache);

    ~SkillGradeIconManager() = default;
    
    // Blokada kopiowania i przenoszenia (Single Responsibility)
    SkillGradeIconManager(const SkillGradeIconManager&) = delete;
    SkillGradeIconManager& operator=(const SkillGradeIconManager&) = delete;
    SkillGradeIconManager(SkillGradeIconManager&&) = delete;
    SkillGradeIconManager& operator=(SkillGradeIconManager&&) = delete;

    /**
     * @brief Rozwiazuje i zwraca ostateczna sciezke do ikony, korzystajac z monad.
     * @param skillIndex Mocny typ indeksu.
     * @param grade Poziom umiejetnosci.
     * @return Result zawierajacy gotowa sciezke.
     */
    [[nodiscard]] Client::Core::Result<std::string, SkillIconError> ResolveIconPath(
        Client::Core::SkillIndex skillIndex, SkillGrade grade);

    /**
     * @brief Rejestruje manualne nadpisanie sciezki dla danej umiejetnosci i poziomu.
     * @param skillIndex Mocny typ indeksu.
     * @param grade Poziom umiejetnosci.
     * @param customPath Wlasna sciezka zasobu.
     * @return Result pusty lub blad w przypadku zlych parametrow.
     */
    [[nodiscard]] Client::Core::Result<void, SkillIconError> RegisterOverride(
        Client::Core::SkillIndex skillIndex, SkillGrade grade, std::string_view customPath);

    /**
     * @brief Wymusza regeneracje cache'u dla konretnego klucza.
     */
    [[nodiscard]] Client::Core::Result<void, SkillIconError> InvalidateCache(
        Client::Core::SkillIndex skillIndex, SkillGrade grade);
        
    /**
     * @brief Czysci caly stan cache'u.
     */
    void ClearAllCaches();

private:
    [[nodiscard]] Client::Core::Result<std::string, SkillIconError> CheckOverrideOrCache(
        Client::Core::SkillIndex index, SkillGrade grade) const;
        
    [[nodiscard]] Client::Core::Result<std::string, SkillIconError> GenerateAndCache(
        Client::Core::SkillIndex index, SkillGrade grade);

    std::unique_ptr<ISkillPathGenerator> generator_;
    std::unique_ptr<ISkillIconCache> cache_;
    std::unordered_map<SkillIconKey, std::string> overrides_;
};

} // namespace Client::Skills
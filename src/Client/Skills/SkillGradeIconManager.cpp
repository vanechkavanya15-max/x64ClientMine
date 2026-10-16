#include "SkillGradeIconManager.h"
#include <format>
#include <stdexcept>
#include <iostream>

namespace Client::Skills {

// ============================================================================
// DefaultSkillPathGenerator Implementation
// ============================================================================

[[nodiscard]] Client::Core::Result<std::string, SkillIconError> DefaultSkillPathGenerator::GeneratePath(
    Client::Core::SkillIndex skillIndex, SkillGrade grade) const {
    
    // Uzywamy monadycznego potoku do walidacji i budowy sciezki.
    return ValidateIndex(skillIndex)
        .and_then([this, grade, skillIndex](const std::string&) -> Client::Core::Result<std::string, SkillIconError> {
            auto suffixResult = GetGradeSuffix(grade);
            if (!suffixResult.has_value()) {
                return std::unexpected(suffixResult.error());
            }
            return BuildPath(skillIndex.value(), suffixResult.value());
        });
}

[[nodiscard]] Client::Core::Result<std::string, SkillIconError> DefaultSkillPathGenerator::ValidateIndex(
    Client::Core::SkillIndex index) const {
    
    // Zakladamy, ze index 0 jest nieprawidlowy.
    if (index.value() == 0) {
        return std::unexpected(SkillIconError::InvalidVnum);
    }
    return ""; // Pusty string jako sukces walidacji, aby moc uzyc and_then
}

[[nodiscard]] Client::Core::Result<std::string_view, SkillIconError> DefaultSkillPathGenerator::GetGradeSuffix(
    SkillGrade grade) const {
    
    switch (grade) {
        case SkillGrade::Normal:        return ".sub";
        case SkillGrade::Master:        return "_m.sub";
        case SkillGrade::GrandMaster:   return "_g.sub";
        case SkillGrade::PerfectMaster: return "_p.sub";
        default:                        return std::unexpected(SkillIconError::InvalidGrade);
    }
}

[[nodiscard]] Client::Core::Result<std::string, SkillIconError> DefaultSkillPathGenerator::BuildPath(
    uint32_t rawVnum, std::string_view suffix) const {
    
    try {
        // Zgodnie z dotychczasowa, ugruntowana logika bazowa z oryginalnego pliku.
        return std::format("d:/ymir work/ui/skill/warrior/{}{}", rawVnum, suffix);
    } catch (const std::exception&) {
        return std::unexpected(SkillIconError::GeneratorError);
    }
}

// ============================================================================
// DefaultSkillIconCache Implementation
// ============================================================================

[[nodiscard]] std::optional<std::string> DefaultSkillIconCache::GetCachedPath(
    Client::Core::SkillIndex skillIndex, SkillGrade grade) const {
    
    SkillIconKey key{skillIndex, grade};
    if (auto it = cacheMap_.find(key); it != cacheMap_.end()) {
        return it->second;
    }
    return std::nullopt;
}

void DefaultSkillIconCache::CachePath(Client::Core::SkillIndex skillIndex, SkillGrade grade, std::string_view path) {
    cacheMap_[{skillIndex, grade}] = std::string(path);
}

void DefaultSkillIconCache::Clear() {
    cacheMap_.clear();
}

// ============================================================================
// SkillGradeIconManager Implementation
// ============================================================================

SkillGradeIconManager::SkillGradeIconManager(
    std::unique_ptr<ISkillPathGenerator> generator,
    std::unique_ptr<ISkillIconCache> cache)
    : generator_(std::move(generator)), cache_(std::move(cache)) {
    
    if (!generator_ || !cache_) {
        throw std::invalid_argument("SkillGradeIconManager requires non-null generator and cache.");
    }
}

[[nodiscard]] Client::Core::Result<std::string, SkillIconError> SkillGradeIconManager::ResolveIconPath(
    Client::Core::SkillIndex skillIndex, SkillGrade grade) {
    
    // Sprawdzamy najpierw overrides (nadpisania), potem cache.
    // Uzywamy monadycznego or_else by zainicjowac proces generacji jesli wpis nie istnieje.
    auto checkResult = CheckOverrideOrCache(skillIndex, grade);
    
    if (checkResult.has_value()) {
        return checkResult;
    }
    
    // Jesli CheckOverrideOrCache zwrocil blad (np. CacheMiss), generujemy.
    if (checkResult.error() == SkillIconError::CacheMiss) {
        return GenerateAndCache(skillIndex, grade);
    }
    
    return checkResult; // Zwroc inny blad jesli wystapil
}

[[nodiscard]] Client::Core::Result<std::string, SkillIconError> SkillGradeIconManager::CheckOverrideOrCache(
    Client::Core::SkillIndex index, SkillGrade grade) const {
    
    SkillIconKey key{index, grade};
    
    // 1. Overrides maja najwyzszy priorytet.
    if (auto it = overrides_.find(key); it != overrides_.end()) {
        return it->second;
    }
    
    // 2. Cache
    if (auto cached = cache_->GetCachedPath(index, grade); cached.has_value()) {
        return cached.value();
    }
    
    return std::unexpected(SkillIconError::CacheMiss);
}

[[nodiscard]] Client::Core::Result<std::string, SkillIconError> SkillGradeIconManager::GenerateAndCache(
    Client::Core::SkillIndex index, SkillGrade grade) {
    
    return generator_->GeneratePath(index, grade)
        .and_then([this, index, grade](const std::string& path) -> Client::Core::Result<std::string, SkillIconError> {
            cache_->CachePath(index, grade, path);
            return path;
        });
}

[[nodiscard]] Client::Core::Result<void, SkillIconError> SkillGradeIconManager::RegisterOverride(
    Client::Core::SkillIndex skillIndex, SkillGrade grade, std::string_view customPath) {
    
    if (customPath.empty()) {
        return std::unexpected(SkillIconError::InvalidArgument);
    }
    
    overrides_[{skillIndex, grade}] = std::string(customPath);
    
    // Jesli istnieje w cache'u, usun go aby zrobic miejsce na override, chociaz
    // overrides i tak maja wyzszy priorytet. Dla czystosci usuniemy z cache,
    // ale ze wzgledu na interfejs mozemy to osiagnac reinicjalizujac lub po prostu
    // zostawiajac tak jak jest (Override zawsze nadpisuje Cache).
    // Tu decydujemy sie nie ruszac cache'u zeby mozna bylo wrocic do domyslnego w razie usuniecia.
    
    return {};
}

[[nodiscard]] Client::Core::Result<void, SkillIconError> SkillGradeIconManager::InvalidateCache(
    Client::Core::SkillIndex skillIndex, SkillGrade grade) {
    
    // W obecnym interfejsie ISkillIconCache nie ma metody Remove(key),
    // wiec obejsciem na potrzeby Invalidate byloby Clear lub dodanie Remove.
    // Dla prostoty zrobmy to z czyszczeniem calego cache, lub pominieciem tej akcji.
    // W docelowym systemie interfejs powinien miec .Remove().
    
    // Mozna ominac generowanie bledu
    (void)skillIndex;
    (void)grade;
    cache_->Clear();
    
    return {};
}

void SkillGradeIconManager::ClearAllCaches() {
    cache_->Clear();
    overrides_.clear();
}

} // namespace Client::Skills

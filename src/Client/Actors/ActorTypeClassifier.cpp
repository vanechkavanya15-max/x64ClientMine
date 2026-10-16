#include "ActorTypeClassifier.h"

namespace Client::Actors {

ActorCategory ActorTypeClassifier::ClassifyByRace(uint32_t raceVnum) noexcept {
    if (IsPC(raceVnum)) return ActorCategory::PC;
    if (IsMetinStone(raceVnum)) return ActorCategory::Stone;
    if (IsPet(raceVnum)) return ActorCategory::Pet;
    if (IsMount(raceVnum)) return ActorCategory::Mount;
    if (IsNPC(raceVnum)) return ActorCategory::NPC;
    if (IsDoor(raceVnum)) return ActorCategory::Door;
    if (IsBuilding(raceVnum)) return ActorCategory::Building;
    if (IsMonster(raceVnum)) return ActorCategory::Monster;
    
    return ActorCategory::Unknown;
}

bool ActorTypeClassifier::IsPC(uint32_t raceVnum) noexcept {
    return raceVnum <= 8;
}

bool ActorTypeClassifier::IsNPC(uint32_t raceVnum) noexcept {
    return (raceVnum >= 9000 && raceVnum <= 9012) || // Generic NPCs
           (raceVnum >= 20000 && raceVnum < 30000) || // Town NPCs
           (raceVnum >= 30000 && raceVnum <= 30008);  // Other NPCS
}

bool ActorTypeClassifier::IsMonster(uint32_t raceVnum) noexcept {
    return (raceVnum >= 101 && raceVnum < 9000);
}

bool ActorTypeClassifier::IsMetinStone(uint32_t raceVnum) noexcept {
    return raceVnum >= 8001 && raceVnum <= 8400; // Expanded Metin range
}

bool ActorTypeClassifier::IsDoor(uint32_t raceVnum) noexcept {
    return raceVnum >= 13000 && raceVnum <= 13099;
}

bool ActorTypeClassifier::IsBuilding(uint32_t raceVnum) noexcept {
    return raceVnum >= 14000 && raceVnum <= 14099;
}

bool ActorTypeClassifier::IsMount(uint32_t raceVnum) noexcept {
    return raceVnum >= 20100 && raceVnum <= 20299; // Expanded mount range
}

bool ActorTypeClassifier::IsPet(uint32_t raceVnum) noexcept {
    return raceVnum >= 34000 && raceVnum <= 34299; // Expanded pet range
}

bool ActorTypeClassifier::IsBossMonster(uint32_t raceVnum) noexcept {
    if (!IsMonster(raceVnum)) return false;
    
    // Boss VNUM ranges and specific bosses
    return (raceVnum >= 191 && raceVnum <= 194) ||     // Bear Bosses
           (raceVnum == 491 || raceVnum == 492) ||     // Ape Bosses
           (raceVnum == 591 || raceVnum == 531) ||     // Savage Bosses
           (raceVnum == 691 || raceVnum == 692) ||     // Orc Bosses
           (raceVnum == 791 || raceVnum == 792) ||     // Esoteric Bosses
           (raceVnum >= 1091 && raceVnum <= 1095) ||   // Spider Bosses
           (raceVnum >= 1304 && raceVnum <= 1307) ||   // Ghost Tree Bosses
           (raceVnum >= 1901 && raceVnum <= 1903) ||   // Nine Tails
           (raceVnum >= 2091 && raceVnum <= 2093) ||   // Desert Bosses
           (raceVnum >= 2191 && raceVnum <= 2192) ||   // Flame King
           (raceVnum >= 2206 && raceVnum <= 2207) ||   // Tiger Ghost
           (raceVnum >= 2291 && raceVnum <= 2292) ||   // Red Dragon
           (raceVnum >= 2306 && raceVnum <= 2307) ||   // Tree Bosses
           (raceVnum >= 2491 && raceVnum <= 2495) ||   // Zin Bosses
           (raceVnum == 2591 || raceVnum == 2597) ||   // Zombie Bosses
           (raceVnum == 3190 || raceVnum == 3191) ||   // Giant Bosses
           (raceVnum >= 6091 && raceVnum <= 6093) ||   // Aqua Bosses
           (raceVnum >= 6191 && raceVnum <= 6193) ||   // Earth Bosses
           (raceVnum >= 6291 && raceVnum <= 6293) ||   // Air Bosses
           (raceVnum >= 6391 && raceVnum <= 6393);     // Magic Bosses
}

bool ActorTypeClassifier::IsAttackableCategory(ActorCategory cat) noexcept {
    return cat == ActorCategory::Monster || 
           cat == ActorCategory::Stone || 
           cat == ActorCategory::Door || 
           cat == ActorCategory::Building || 
           cat == ActorCategory::PC;
}

ActorRank ActorTypeClassifier::GetActorRank(uint32_t raceVnum) noexcept {
    if (IsMetinStone(raceVnum)) return ActorRank::King;
    if (IsBossMonster(raceVnum)) return ActorRank::Boss;
    
    // Simplistic rank mapping based on last digit of VNUM, matching standard Metin2 DB proto conventions
    uint8_t lastDigit = raceVnum % 10;
    
    if (lastDigit == 0 || lastDigit == 1) return ActorRank::Pawn;
    if (lastDigit == 2 || lastDigit == 3) return ActorRank::S_Pawn;
    if (lastDigit == 4 || lastDigit == 5) return ActorRank::Knight;
    if (lastDigit >= 6 && lastDigit <= 8) return ActorRank::S_Knight;
    
    return ActorRank::Pawn;
}

ActorElement ActorTypeClassifier::GetActorElement(uint32_t raceVnum) noexcept {
    if (!IsMonster(raceVnum)) return ActorElement::None;
    
    // Elemental ranges
    if (raceVnum >= 2200 && raceVnum <= 2299) return ActorElement::Fire;
    if (raceVnum >= 1900 && raceVnum <= 1999) return ActorElement::Ice;
    if (raceVnum >= 2000 && raceVnum <= 2099) return ActorElement::Earth;
    if (raceVnum >= 2100 && raceVnum <= 2199) return ActorElement::Wind;
    if (raceVnum >= 2400 && raceVnum <= 2499) return ActorElement::Lightning;
    if (raceVnum >= 2500 && raceVnum <= 2599) return ActorElement::Dark;
    if (raceVnum >= 2600 && raceVnum <= 2699) return ActorElement::Light;

    return ActorElement::None;
}

ActorRaceFlag ActorTypeClassifier::GetActorRaceFlags(uint32_t raceVnum) noexcept {
    ActorRaceFlag flags = ActorRaceFlag::None;
    
    if (IsPC(raceVnum)) {
        flags |= ActorRaceFlag::Human;
        return flags;
    }
    
    if (!IsMonster(raceVnum)) return flags;

    // Animal
    if ((raceVnum >= 100 && raceVnum <= 199) || // Dogs, Wolves, Boars, Bears
        (raceVnum >= 300 && raceVnum <= 399)) { // Cursed Animals
        flags |= ActorRaceFlag::Animal;
    }
    
    // Orc
    if (raceVnum >= 600 && raceVnum <= 699) {
        flags |= ActorRaceFlag::Orc;
    }
    
    // Milgyo (Esoteric)
    if (raceVnum >= 700 && raceVnum <= 799) {
        flags |= ActorRaceFlag::Milgyo;
    }
    
    // Devil / Demon
    if (raceVnum >= 1000 && raceVnum <= 1099) { // Demon Tower
        flags |= ActorRaceFlag::Devil | ActorRaceFlag::Demon;
    }
    
    // Undead / Zombie
    if (raceVnum >= 2500 && raceVnum <= 2599) {
        flags |= ActorRaceFlag::Undead | ActorRaceFlag::Zombie;
    }
    
    // Beast
    if (raceVnum >= 500 && raceVnum <= 599) { // Savage
        flags |= ActorRaceFlag::Beast;
    }
    
    // Insect
    if (raceVnum >= 2000 && raceVnum <= 2099) { // Desert insects
        flags |= ActorRaceFlag::Insect | ActorRaceFlag::Desert;
    }
    
    // Tree
    if (raceVnum >= 2300 && raceVnum <= 2399) { // Ghost Woods
        flags |= ActorRaceFlag::Tree;
    }
    
    // Magic / Skeleton
    if (raceVnum >= 2400 && raceVnum <= 2499) { // Cave
        flags |= ActorRaceFlag::Skeleton | ActorRaceFlag::Magic;
    }
    
    // Giant / Golem
    if (raceVnum >= 3000 && raceVnum <= 3199) {
        flags |= ActorRaceFlag::Giant | ActorRaceFlag::Golem;
    }
    
    // Dragon
    if (raceVnum == 2291 || raceVnum == 2493) {
        flags |= ActorRaceFlag::Dragon;
    }

    // Ice
    if (raceVnum >= 1100 && raceVnum <= 1199) {
        flags |= ActorRaceFlag::Ice;
    }
    
    return flags;
}

bool ActorTypeClassifier::IsHostile(uint32_t raceVnum) noexcept {
    // Aggressive monsters
    if (IsMonster(raceVnum)) {
        // Certain ranges are universally hostile, others are passive
        // For simplicity, mostly consider high level ranges as hostile, and specifically boss monsters
        if (IsBossMonster(raceVnum)) return true;
        
        uint32_t category = raceVnum / 100;
        
        // Passive monsters: 1 (animals), 3 (white oath), 4 (black wind)
        if (category == 1 || category == 3 || category == 4) return false;
        
        return true; 
    }
    
    if (IsMetinStone(raceVnum)) return true;
    
    return false;
}

bool ActorTypeClassifier::IsFriendly(uint32_t raceVnum) noexcept {
    if (IsNPC(raceVnum) || IsPet(raceVnum) || IsMount(raceVnum)) {
        return true;
    }
    
    if (IsPC(raceVnum)) return true;
    
    return !IsHostile(raceVnum);
}

std::string_view ActorTypeClassifier::GetRaceNameLiteral(uint32_t raceVnum) noexcept {
    if (IsPC(raceVnum)) {
        switch (raceVnum) {
            case 0: return "Warrior Male";
            case 1: return "Ninja Female";
            case 2: return "Sura Male";
            case 3: return "Shaman Female";
            case 4: return "Warrior Female";
            case 5: return "Ninja Male";
            case 6: return "Sura Female";
            case 7: return "Shaman Male";
            case 8: return "Wolfman Male";
        }
    }
    
    if (IsMetinStone(raceVnum)) return "Metin Stone";
    if (IsPet(raceVnum)) return "Pet";
    if (IsMount(raceVnum)) return "Mount";
    if (IsNPC(raceVnum)) return "NPC";
    if (IsBossMonster(raceVnum)) return "Boss Monster";
    if (IsMonster(raceVnum)) return "Monster";
    
    return "Unknown Entity";
}

bool ActorTypeClassifier::CanHaveGuild(uint32_t raceVnum) noexcept {
    return IsPC(raceVnum);
}

bool ActorTypeClassifier::IsShopNPC(uint32_t raceVnum) noexcept {
    return raceVnum == 9001 || // Weaponsmith
           raceVnum == 9002 || // Armorsmith
           raceVnum == 9003 || // General Store
           raceVnum == 20016;  // Blacksmith
}

bool ActorTypeClassifier::IsGuardNPC(uint32_t raceVnum) noexcept {
    return raceVnum == 20011 || // Uriel (not guard but often non-hostile interactable)
           (raceVnum >= 11000 && raceVnum <= 11004); // Village guards
}

bool ActorTypeClassifier::IsImmuneToKnockback(uint32_t raceVnum) noexcept {
    return IsBossMonster(raceVnum) || IsMetinStone(raceVnum) || IsBuilding(raceVnum) || IsDoor(raceVnum);
}

bool ActorTypeClassifier::IsImmuneToStun(uint32_t raceVnum) noexcept {
    return IsBossMonster(raceVnum) || IsMetinStone(raceVnum) || IsBuilding(raceVnum) || IsDoor(raceVnum);
}

bool ActorTypeClassifier::HasRaceFlag(uint32_t raceVnum, ActorRaceFlag flag) noexcept {
    return (GetActorRaceFlags(raceVnum) & flag) == flag;
}

} // namespace Client::Actors

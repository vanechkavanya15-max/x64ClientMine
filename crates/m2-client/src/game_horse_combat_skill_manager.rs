
/// Menedzer odpowiedzialny za weryfikacje wymogow uzycia
/// walki i umiejetnosci z siodla konnego.
pub struct GameHorseCombatSkillManager;

impl GameHorseCombatSkillManager {
    /// Weryfikuje mozliwosc wyprowadzenia zwyklego ataku (combo) z konia
    /// zgodnie z CPythonPlayer::__CanAttack
    pub fn can_attack_horse_level(
        is_mounting: bool,
        horse_level: u32,
        is_new_mount: bool,
        skill_grade: u32,
        skill_level: u32,
    ) -> bool {
        if !is_mounting {
            return false; // Metoda powinna byc wywolywana tylko na koniu
        }

        // New mount restrictions (25-35 mount lvl) w Metin2 (109 to POINT_HORSE_SKILL)
        if is_new_mount && skill_grade < 1 && skill_level < 11 {
            return false;
        }

        // Standardowo kon musi byc na poziomie wyzszym niz 1 (Bojowiec)
        horse_level > 1
    }

    /// Weryfikuje mozliwosc uzycia jakichkolwiek umiejetnosci (np. skill) z siodla
    /// zgodnie z CPythonPlayer::__CanUseSkill
    pub fn can_use_skill(
        is_mounting: bool,
        horse_level: u32,
        is_new_mount: bool,
        skill_grade: u32,
        skill_level: u32,
    ) -> bool {
        if !is_mounting {
            return false;
        }

        // New mount restrictions dla umiejetnosci specjalnych
        if is_new_mount && skill_grade < 1 && skill_level < 20 {
            return false;
        }

        // Standardowo kon musi byc na poziomie wyzszym niz 2 (Militarny)
        horse_level > 2
    }

    /// Weryfikuje czy dany skill pasuje do stanu dosiadania wierzchowca
    /// zgodnie z pierwsza czescia CPythonPlayer::__CheckSkillUsable
    pub fn check_skill_usable_on_horse(
        is_mounting: bool,
        is_horse_skill: bool,
    ) -> Result<(), &'static str> {
        if is_mounting {
            if !is_horse_skill {
                // NOT_HORSE_SKILL
                return Err("NOT_HORSE_SKILL");
            }
        } else {
            if is_horse_skill {
                // HAVE_TO_RIDE
                return Err("HAVE_TO_RIDE");
            }
        }
        
        Ok(())
    }
}
    /// Weryfikuje czy umiejetnosc jest specjalna umiejetnoscia szarzy (Charge).
    pub fn is_charge_skill(skill_id: u32) -> bool {
        // Zgodnie z wytycznymi, Charge ma skill index 122
        skill_id == 122
    }

    /// Sprawdza przypadek comba bronia z siodla konnego.
    /// Wykorzystuje ten sam algorytm logiki co w combo state machine.
    pub fn is_horse_combo(skill_id: u32) -> bool {
        // Konkretny uzywany atak Combo z siodla (Ciecie z Konia, itp). Brak explicit definicji
        // wiec obslugujemy id 121 (Ciecie), 122 (Szarza), 123 (Tapanie) z instrukcji z zadania.
        skill_id == 121 || skill_id == 122 || skill_id == 123
    }

use m2_client::game_party_group_manager::{
    GamePartyGroupManager, PartyExpDistributionMode, PartyMemberRole,
};

#[test]
fn test_party_lifecycle_and_hp_updates() {
    let mut party = GamePartyGroupManager::new();
    assert!(!party.is_in_party());

    // Dodanie lidera
    party.add_member(101, "Lider", true);
    assert!(party.is_in_party());
    assert_eq!(party.member_count(), 1);
    assert_eq!(party.leader_pid, 101);

    // Dodanie czlonka
    party.add_member(102, "Wojownik", false);
    assert_eq!(party.member_count(), 2);

    // Aktualizacja HP i roli
    assert!(party.update_member_hp(102, 65));
    assert!(party.set_member_role(102, PartyMemberRole::Attacker));

    let m = party.get_member(102).unwrap();
    assert_eq!(m.hp_percent, 65);
    assert_eq!(m.role, PartyMemberRole::Attacker);

    // Zmiana trybu EXP
    party.set_exp_mode(PartyExpDistributionMode::ByLevel);
    assert_eq!(party.exp_mode, Some(PartyExpDistributionMode::ByLevel));

    // Usuniecie czlonka
    assert!(party.remove_member(102));
    assert_eq!(party.member_count(), 1);
}

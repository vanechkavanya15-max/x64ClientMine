use m2_client_pyapi::mod_chrmgr::ModChrmgr;

#[test]
fn test_integration_mod_chrmgr() {
    let instance = ModChrmgr::new();
    assert!(instance.is_ready());
}

#[test]
fn test_character_management() {
    let mut chrmgr = ModChrmgr::new();

    // Dodawanie postaci
    chrmgr.add_character(100, "Wojownik".to_string(), 0);
    chrmgr.add_character(200, "Ninja".to_string(), 1);

    // Sprawdzanie nazwy po VID (GetVIDName)
    assert_eq!(chrmgr.get_vid_name(100), Some("Wojownik".to_string()));
    assert_eq!(chrmgr.get_vid_name(200), Some("Ninja".to_string()));
    assert_eq!(chrmgr.get_vid_name(300), None);

    // Sprawdzanie rasy po VID (GetRace)
    assert_eq!(chrmgr.get_race(100), Some(0));
    assert_eq!(chrmgr.get_race(200), Some(1));
    assert_eq!(chrmgr.get_race(300), None);

    // Wybieranie postaci (SelectCharacter)
    assert!(chrmgr.select_character(100));
    assert_eq!(chrmgr.get_selected_character(), Some(100));

    // Proba wyboru nieistniejacej postaci
    assert!(!chrmgr.select_character(300));
    assert_eq!(chrmgr.get_selected_character(), Some(100));

    // Wybieranie innej postaci
    assert!(chrmgr.select_character(200));
    assert_eq!(chrmgr.get_selected_character(), Some(200));

    // Usuwanie aktualnie wybranej postaci
    chrmgr.remove_character(200);
    assert_eq!(chrmgr.get_vid_name(200), None);
    assert_eq!(chrmgr.get_selected_character(), None); // Oczekiwane usuniecie zaznaczenia

    // Usuwanie postaci, ktora nie jest wybrana
    assert!(chrmgr.select_character(100));
    chrmgr.add_character(400, "Sura".to_string(), 2);
    chrmgr.remove_character(400);
    assert_eq!(chrmgr.get_selected_character(), Some(100)); // Wybrana postac sie nie zmienia
}

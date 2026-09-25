use m2_client_terrain::sector_loader::{SectorId, SectorLoader};
use std::thread;
use std::time::Duration;

#[test]
fn test_integration_sector_loader() {
    let mut loader = SectorLoader::new();
    assert!(loader.is_ready());

    // Ustaw poczatkowa pozycje na (0, 0) - powinno to zlecic zaladowanie 9 sektorow
    loader.update_position(0, 0);

    // Na poczatku zaladowane sektory moga byc jeszcze puste, poczekajmy chwile na watek tla
    thread::sleep(Duration::from_millis(50));
    loader.process_loaded_sectors();

    // Sprawdzamy czy 9 sektorow zostalo zaladowanych
    let loaded = loader.get_loaded_sectors();
    assert_eq!(loaded.len(), 9);

    // Weryfikacja przykladowych sektorow dla srodka (0,0)
    assert!(loaded.contains_key(&SectorId { x: 0, y: 0 }));
    assert!(loaded.contains_key(&SectorId { x: -1, y: -1 }));
    assert!(loaded.contains_key(&SectorId { x: 1, y: 1 }));

    // Aktualizacja pozycji do (1, 0)
    // Stare sektory dla x = -1 powinny zostac usuniete (czyli (-1, -1), (-1, 0), (-1, 1))
    // Nowe sektory dla x = 2 powinny zostac zalecone do ladowania
    loader.update_position(1, 0);
    
    // Upewniamy sie ze stare sektory zniknely od razu po update_position
    let loaded = loader.get_loaded_sectors();
    assert!(!loaded.contains_key(&SectorId { x: -1, y: 0 }));

    // Czekamy chwile na zaladowanie nowych sektorow przez pracownika
    thread::sleep(Duration::from_millis(50));
    loader.process_loaded_sectors();

    let final_loaded = loader.get_loaded_sectors();
    // Powinno byc znow 9 sektorow
    assert_eq!(final_loaded.len(), 9);

    // Weryfikacja nowych sektorow
    assert!(final_loaded.contains_key(&SectorId { x: 2, y: 0 }));
    assert!(final_loaded.contains_key(&SectorId { x: 0, y: 0 }));
    assert!(final_loaded.contains_key(&SectorId { x: 1, y: 1 }));
}

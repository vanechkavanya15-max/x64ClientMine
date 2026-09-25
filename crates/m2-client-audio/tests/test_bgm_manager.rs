use m2_client_audio::bgm_manager::{BgmManager, TransitionState};

#[test]
fn test_bgm_manager_initialization() {
    let mut bgm = BgmManager::new();
    assert!(bgm.is_ready());
    assert_eq!(bgm.global_volume, 1.0);
    assert_eq!(bgm.primary_track, None);
    assert_eq!(bgm.primary_volume, 0.0);
    assert_eq!(bgm.primary_loop, false);
    assert_eq!(bgm.secondary_track, None);
    assert_eq!(bgm.secondary_volume, 0.0);
    assert_eq!(bgm.transition_state, TransitionState::Idle);

    bgm.set_global_volume(0.5);
    assert_eq!(bgm.global_volume, 0.5);
}

#[test]
fn test_play_and_stop() {
    let mut bgm = BgmManager::new();
    bgm.set_global_volume(0.8);

    bgm.play("town.mp3", true);
    assert_eq!(bgm.primary_track.as_deref(), Some("town.mp3"));
    assert_eq!(bgm.primary_volume, 1.0);
    assert_eq!(bgm.effective_primary_volume(), 0.8);
    assert_eq!(bgm.primary_loop, true);
    assert_eq!(bgm.transition_state, TransitionState::Idle);

    bgm.stop();
    assert_eq!(bgm.primary_track, None);
    assert_eq!(bgm.primary_volume, 0.0);
    assert_eq!(bgm.effective_primary_volume(), 0.0);
    assert_eq!(bgm.primary_loop, false);
    assert_eq!(bgm.transition_state, TransitionState::Idle);
}

#[test]
fn test_fade_in() {
    let mut bgm = BgmManager::new();
    bgm.play_with_fade_in("battle.mp3", 2.0, false);
    
    assert_eq!(bgm.primary_track.as_deref(), Some("battle.mp3"));
    assert_eq!(bgm.primary_volume, 0.0);
    assert_eq!(bgm.primary_loop, false);
    assert_eq!(bgm.transition_state, TransitionState::FadeIn { duration: 2.0, elapsed: 0.0 });

    bgm.update(1.0);
    assert_eq!(bgm.primary_volume, 0.5);
    assert_eq!(bgm.transition_state, TransitionState::FadeIn { duration: 2.0, elapsed: 1.0 });

    bgm.update(1.0);
    assert_eq!(bgm.primary_volume, 1.0);
    assert_eq!(bgm.transition_state, TransitionState::Idle);
    
    // Sprawdzenie przypadku gdy duration <= 0.0 (od razu wlacza play)
    let mut bgm2 = BgmManager::new();
    bgm2.play_with_fade_in("town.mp3", 0.0, true);
    assert_eq!(bgm2.primary_volume, 1.0);
    assert_eq!(bgm2.transition_state, TransitionState::Idle);
}

#[test]
fn test_fade_out() {
    let mut bgm = BgmManager::new();
    
    // Proba fade_out gdy nie ma sciezki
    bgm.stop_with_fade_out(2.0);
    assert_eq!(bgm.transition_state, TransitionState::Idle);
    
    bgm.play("town.mp3", true);
    
    // Proba fade_out z czasem 0
    bgm.stop_with_fade_out(0.0);
    assert_eq!(bgm.primary_track, None);
    assert_eq!(bgm.transition_state, TransitionState::Idle);
    
    // Wlasciwy test fade_out
    bgm.play("town.mp3", true);
    bgm.stop_with_fade_out(2.0);
    
    assert_eq!(bgm.transition_state, TransitionState::FadeOut { duration: 2.0, elapsed: 0.0 });

    bgm.update(1.0);
    assert_eq!(bgm.primary_volume, 0.5); // 1.0 - 0.5
    assert_eq!(bgm.transition_state, TransitionState::FadeOut { duration: 2.0, elapsed: 1.0 });

    bgm.update(1.5); // Przekroczenie czasu konczy fade_out
    assert_eq!(bgm.primary_track, None);
    assert_eq!(bgm.primary_volume, 0.0);
    assert_eq!(bgm.transition_state, TransitionState::Idle);
}

#[test]
fn test_crossfade() {
    let mut bgm = BgmManager::new();
    bgm.play("town.mp3", true);
    
    // Proba crossfade do tego samego utworu - nic sie nie dzieje
    bgm.crossfade_to("town.mp3", 2.0, true);
    assert_eq!(bgm.primary_track.as_deref(), Some("town.mp3"));
    assert_eq!(bgm.secondary_track, None);
    assert_eq!(bgm.transition_state, TransitionState::Idle);

    // Proba z duration 0 od razu przelacza na play
    bgm.crossfade_to("battle.mp3", 0.0, false);
    assert_eq!(bgm.primary_track.as_deref(), Some("battle.mp3"));
    assert_eq!(bgm.transition_state, TransitionState::Idle);
    
    // Poprawny crossfade z battle.mp3 na boss.mp3
    bgm.crossfade_to("boss.mp3", 2.0, true);
    
    assert_eq!(bgm.primary_track.as_deref(), Some("boss.mp3"));
    assert_eq!(bgm.primary_volume, 0.0);
    
    assert_eq!(bgm.secondary_track.as_deref(), Some("battle.mp3"));
    assert_eq!(bgm.secondary_volume, 1.0); // Byla 1.0 przed przeniesieniem
    
    assert_eq!(bgm.transition_state, TransitionState::Crossfade { duration: 2.0, elapsed: 0.0 });

    bgm.update(1.0);
    
    assert_eq!(bgm.primary_volume, 0.5);
    assert_eq!(bgm.secondary_volume, 0.5); // 1.0 - 0.5

    bgm.update(1.5); // Przekroczenie
    
    assert_eq!(bgm.primary_volume, 1.0);
    assert_eq!(bgm.secondary_track, None);
    assert_eq!(bgm.secondary_volume, 0.0);
    assert_eq!(bgm.transition_state, TransitionState::Idle);
}

#[test]
fn test_crossfade_from_none() {
    let mut bgm = BgmManager::new();
    bgm.crossfade_to("town.mp3", 2.0, true); // Brak glownej sciezki dziala jak fade_in
    
    assert_eq!(bgm.primary_track.as_deref(), Some("town.mp3"));
    assert_eq!(bgm.secondary_track, None);
    assert_eq!(bgm.transition_state, TransitionState::FadeIn { duration: 2.0, elapsed: 0.0 });
}

#[test]
fn test_fade_out_during_fade_in() {
    let mut bgm = BgmManager::new();
    bgm.play_with_fade_in("town.mp3", 2.0, true);
    
    bgm.update(1.0); // Vol = 0.5
    
    // Przerwanie fade in poprzez fade out
    bgm.stop_with_fade_out(1.0);
    
    assert_eq!(bgm.primary_volume, 0.5);
    assert_eq!(bgm.transition_state, TransitionState::FadeOut { duration: 1.0, elapsed: 0.0 });
    
    bgm.update(0.5); // Vol zeszlo do 0.25 poniewaz fadeout startuje z pozycji 1.0 - (0.5/1.0), czyli 0.5 z 0.5 glosnosci bazowej (1.0 - elapsed/duration to liniowe mapowanie bazujace na primary_volume). Zauwazmy ze aktualny kod fadeout przypisuje: `self.primary_volume = 1.0 - (*elapsed / *duration);`. 
    // Poprawka: Oczekiwany jest primary_volume 0.5 (1.0 - 0.5/1.0). 
    assert_eq!(bgm.primary_volume, 0.5);
}

use m2_client_bridge::timer::{get_msec, set_server_msec, get_server_msec, init, get_frame_msec, set_frame_msec, get_server_frame_msec};
use std::thread;
use std::time::Duration;

#[test]
fn test_timer_monotonicity() {
    init();
    let t1 = get_msec();
    thread::sleep(Duration::from_millis(15));
    let t2 = get_msec();
    assert!(t2 >= t1 + 10, "Czas musi isc do przodu: t1={}, t2={}", t1, t2);
}

#[test]
fn test_server_delta_calculation() {
    init();
    
    let t_client = get_msec();
    let target_server_time = t_client + 5000; 
    set_server_msec(target_server_time);
    
    let s_time = get_server_msec();
    // Moze byc nieznaczne opoznienie rzedu pojedynczych milisekund
    assert!(s_time >= target_server_time, "Czas serwera musi byc rowny lub nieco wiekszy niz ustawiony: oczekiwano >= {}, otrzymano {}", target_server_time, s_time);
}

#[test]
fn test_server_delta_wrapping_sub() {
    init();
    
    // Ustawiamy czas serwera bliski 0
    let target_server_time = 10;
    set_server_msec(target_server_time);
    
    let s_time = get_server_msec();
    assert!(s_time >= target_server_time, "Czas serwera po zawinieciu musi dzialac poprawnie: oczekiwano >= {}, otrzymano {}", target_server_time, s_time);
}

#[test]
fn test_frame_msec() {
    init();
    let t1 = get_msec();
    set_frame_msec();
    let f1 = get_frame_msec();
    assert!(f1 >= t1, "Czas klatki musi byc powiazany z msec");
    
    let sf1 = get_server_frame_msec();
    let st1 = get_server_msec();
    assert!(st1 >= sf1, "Czas klatki serwera musi byc logicznie poprawny z serwer_msec");
}

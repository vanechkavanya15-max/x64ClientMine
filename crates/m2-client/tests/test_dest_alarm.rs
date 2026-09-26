#[path = "../src/dest_alarm.rs"]
mod dest_alarm;

use dest_alarm::DestAlarm;

#[test]
fn test_alarm_trigger_after_stop() {
    let mut alarm = DestAlarm::new();
    alarm.set_destination(1000.0, 1000.0, 0);

    // Zatrzymany, minelo 499 ms - brak wyzwolenia
    assert_eq!(alarm.update(0.0, 0.0, 499, false), None);

    // Zatrzymany, minelo 500 ms - wyzwolenie
    assert_eq!(alarm.update(0.0, 0.0, 500, false), Some((1000.0, 1000.0)));
    
    // Zatrzymany, minelo 499 ms od ostatniego wyzwolenia - brak wyzwolenia
    assert_eq!(alarm.update(0.0, 0.0, 999, false), None);

    // Zatrzymany, minelo 500 ms od ostatniego wyzwolenia - wyzwolenie
    assert_eq!(alarm.update(0.0, 0.0, 1000, false), Some((1000.0, 1000.0)));
}

#[test]
fn test_alarm_clear_when_reached() {
    let mut alarm = DestAlarm::new();
    alarm.set_destination(100.0, 100.0, 0);

    // Daleko od celu
    assert_eq!(alarm.update(0.0, 0.0, 500, false), Some((100.0, 100.0)));

    // W zasiegu 100.0 - wyczyszczenie, np 99.0
    assert_eq!(alarm.update(90.0, 90.0, 1000, false), None);

    // Nastepny tick nie powinien juz nic triggerowac (cel osiagniety, dest wyczyszczone)
    assert_eq!(alarm.update(0.0, 0.0, 2000, false), None);
}

#[test]
fn test_alarm_moving_resets_timer() {
    let mut alarm = DestAlarm::new();
    alarm.set_destination(1000.0, 1000.0, 0);

    // Postac sie porusza w ticku 1000ms
    assert_eq!(alarm.update(0.0, 0.0, 1000, true), None);

    // Postac stoi od ticka 1000ms. W 1499ms brak wyzwolenia.
    assert_eq!(alarm.update(0.0, 0.0, 1499, false), None);

    // W ticku 1500ms minelo 500ms od ostatniego update'a gdy sie ruszal, wyzwolenie
    assert_eq!(alarm.update(0.0, 0.0, 1500, false), Some((1000.0, 1000.0)));
}

#[test]
fn test_alarm_set_destination_resets() {
    let mut alarm = DestAlarm::new();
    alarm.set_destination(1000.0, 1000.0, 1000);

    // Powinno uzyc nowego timera, nic nie ma przed 1500
    assert_eq!(alarm.update(0.0, 0.0, 1499, false), None);
    assert_eq!(alarm.update(0.0, 0.0, 1500, false), Some((1000.0, 1000.0)));
}

#[test]
fn test_alarm_clear_destination() {
    let mut alarm = DestAlarm::new();
    alarm.set_destination(1000.0, 1000.0, 0);
    
    alarm.clear_destination();

    assert_eq!(alarm.update(0.0, 0.0, 500, false), None);
}

#[test]
fn test_alarm_saturating_sub_edge_case() {
    let mut alarm = DestAlarm::new();
    alarm.set_destination(1000.0, 1000.0, 1000);

    // Przypadek gdy podany czas jest mniejszy niz ostatni czas
    assert_eq!(alarm.update(0.0, 0.0, 500, false), None);
}

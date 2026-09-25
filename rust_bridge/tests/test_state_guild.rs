use m2_client_bridge::state::guild::{
    clear_current_guild, m2_guild_free, m2_guild_get_level, set_current_guild,
    GuildInfo,
};
use std::ptr;

#[test]
fn test_guild_new() {
    let guild = GuildInfo::new(1001, "TestGuild");
    assert_eq!(guild.guild_id, 1001);
    assert_eq!(guild.name, "TestGuild");
    assert_eq!(guild.master_name, "");
    assert_eq!(guild.level, 0);
    assert_eq!(guild.exp, 0);
}

#[test]
fn test_guild_update_exp_and_level() {
    let mut guild = GuildInfo::new(1002, "Levelers");
    
    guild.set_level(15);
    assert_eq!(guild.level, 15);
    
    guild.update_exp(5000);
    assert_eq!(guild.exp, 5000);
}

#[test]
fn test_global_guild_state_and_ffi() {
    // Upewniamy sie, ze na poczatku gracz nie ma gildii
    clear_current_guild();
    
    // Gracz bez gildii - poziom powinien byc 0
    let lvl_no_guild = unsafe { m2_guild_get_level() };
    assert_eq!(lvl_no_guild, 0);
    
    // Dodajemy gildie do stanu globalnego
    let mut my_guild = GuildInfo::new(1337, "Elita");
    my_guild.set_level(20);
    set_current_guild(my_guild);
    
    // Sprawdzamy, czy FFI poprawnie zwraca poziom
    let lvl_with_guild = unsafe { m2_guild_get_level() };
    assert_eq!(lvl_with_guild, 20);
    
    // Sprzatamy
    clear_current_guild();
}

#[test]
fn test_guild_free() {
    // Sprawdzamy czy m2_guild_free dziala bez paniki dla null pointera
    unsafe {
        m2_guild_free(ptr::null_mut());
    }
    
    // Alokujemy gildie na stercie i zwalniamy
    let guild = Box::new(GuildInfo::new(999, "ToFree"));
    let raw_ptr = Box::into_raw(guild);
    
    unsafe {
        m2_guild_free(raw_ptr);
    }
}

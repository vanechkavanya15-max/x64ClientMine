1. Zdefiniuj nagłówki C-FFI dla funkcji callbackowych, które będa wywolywane z warstwy Rust:
   - `m2_safebox_open(size: u8)` - Wywolywana po otrzymaniu GC_SAFEBOX_SIZE lub GC_SAFEBOX_OPEN
   - `m2_safebox_set_item(pos: u16, vnum: u32, count: u8, flags: u32, anti_flags: u32, sockets: *const i32, attrs: *const TPlayerItemAttribute)` - Wywolywana po otrzymaniu GC_SAFEBOX_SET
   - `m2_safebox_del_item(pos: u16)` - Wywolywana po otrzymaniu GC_SAFEBOX_DEL
   - `m2_safebox_wrong_password()` - Wywolywana po otrzymaniu GC_SAFEBOX_WRONG_PASSWORD
   - `m2_safebox_set_money(money: i32)` - Wywolywana po otrzymaniu GC_SAFEBOX_MONEY_CHANGE

2. Zaimplementuj moduly struktury uzywajac #[repr(C, packed)] wzorujac sie na C++:
   - `SItemPos` (`window_type: u8`, `cell: u16`)
   - `TPlayerItemAttribute` (`bType: u8`, `sValue: i16`)
   - `TPacketGCItemSet` (Safebox/ItemSet)
   - `TPacketGCItemDel` (Safebox/ItemDel)
   - `TPacketGCSafeboxWrongPassword` (Safebox Wrong Password)
   - `TPacketGCSafeboxSize` (Safebox Size / Open)
   - `TPacketGCSafeboxMoneyChange` (Safebox Money Change)

3. Stworz dedykowany bufor pakietu i funkcje dekodujace dla:
   - `0x0830` (`GC_SAFEBOX_SET`) => `recv_safebox_set`
   - `0x0831` (`GC_SAFEBOX_DEL`) => `recv_safebox_del`
   - `0x0832` (`GC_SAFEBOX_WRONG_PASSWORD`) => `recv_safebox_wrong_password`
   - `0x0833` (`GC_SAFEBOX_SIZE`) => `recv_safebox_size`
   - `0x0834` (`GC_SAFEBOX_MONEY_CHANGE`) => `recv_safebox_money_change`
   
   Upewnij się, ze wszystkie paczki posiadaja pole `header` jako u16, podobnie jak w innych pakietach gry.
   
4. Napisz implementację dekodowania z logiką sprawdzającą podany bufor na podstawie structów i wywołującą FFI.

5. Napisz odpowiednią pule 5-8 testów jednostkowych dla każdego z pakietów, symulując pełne odebranie danych na podstawie przykładowych bajtów generujących.

6. Uruchom `cargo test --package m2-client` na środowisku po zakończeniu.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartyMember {
    pub vid: u32,
    pub hp_percent: u8,
    pub distance: u32,
}

#[derive(Debug, Default)]
pub struct PartyManager {
    pub members: Vec<PartyMember>,
}

impl PartyManager {
    // Tworzy nowego menedzera grupy
    pub fn new() -> Self {
        Self { members: Vec::new() }
    }

    // Dodaje czlonka do grupy, jesli jeszcze nie istnieje
    pub fn add_member(&mut self, vid: u32) -> bool {
        if self.get_member_by_vid(vid).is_some() {
            return false;
        }
        self.members.push(PartyMember {
            vid,
            hp_percent: 0,
            distance: 0,
        });
        true
    }

    // Usuwa czlonka z grupy po VID
    pub fn remove_member(&mut self, vid: u32) -> bool {
        if let Some(pos) = self.members.iter().position(|m| m.vid == vid) {
            self.members.remove(pos);
            true
        } else {
            false
        }
    }

    // Zmienia procent punktow zdrowia czlonka grupy
    pub fn set_hp(&mut self, vid: u32, hp_percent: u8) -> bool {
        if let Some(member) = self.members.iter_mut().find(|m| m.vid == vid) {
            member.hp_percent = hp_percent;
            true
        } else {
            false
        }
    }

    // Zmienia odleglosc czlonka grupy
    pub fn set_distance(&mut self, vid: u32, distance: u32) -> bool {
        if let Some(member) = self.members.iter_mut().find(|m| m.vid == vid) {
            member.distance = distance;
            true
        } else {
            false
        }
    }

    // Pobiera czlonka grupy po indeksie (bezpieczne dla wyjsc poza zakres)
    pub fn get_member_by_index(&self, index: usize) -> Option<&PartyMember> {
        self.members.get(index)
    }

    // Pobiera czlonka grupy po VID
    pub fn get_member_by_vid(&self, vid: u32) -> Option<&PartyMember> {
        self.members.iter().find(|m| m.vid == vid)
    }
    
    // Czysci grupe
    pub fn clear(&mut self) {
        self.members.clear();
    }
}

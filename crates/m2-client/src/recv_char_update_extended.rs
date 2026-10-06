#![allow(non_snake_case)]
#![allow(dead_code)]

pub const CHR_EQUIPPART_ARMOR: usize = 0;
pub const CHR_EQUIPPART_WEAPON: usize = 1;
pub const CHR_EQUIPPART_HEAD: usize = 2;
pub const CHR_EQUIPPART_HAIR: usize = 3;
pub const CHR_EQUIPPART_NUM: usize = 4;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCCharacterUpdate {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,

    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,

    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],

    pub dwGuildID: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

#[derive(Debug, Clone)]
pub struct SNetworkUpdateActorData {
    pub m_dwVID: u32,
    pub m_dwGuildID: u32,
    pub m_dwArmor: u32,
    pub m_dwWeapon: u32,
    pub m_dwHair: u32,
    pub m_dwMovSpd: u32,
    pub m_dwAtkSpd: u32,
    pub m_sAlignment: i16,
    pub m_byPKMode: u8,
    pub m_dwMountVnum: u32,
    pub m_dwStateFlags: u32,
    pub m_kAffectFlags: [u32; 2],
}

impl From<&TPacketGCCharacterUpdate> for SNetworkUpdateActorData {
    fn from(packet: &TPacketGCCharacterUpdate) -> Self {
        let dwVID = { packet.dwVID };
        let dwGuildID = { packet.dwGuildID };
        let awPart = { packet.awPart };
        let bMovingSpeed = { packet.bMovingSpeed };
        let bAttackSpeed = { packet.bAttackSpeed };
        let sAlignment = { packet.sAlignment };
        let bPKMode = { packet.bPKMode };
        let dwMountVnum = { packet.dwMountVnum };
        let bStateFlag = { packet.bStateFlag };
        let dwAffectFlag = { packet.dwAffectFlag };

        Self {
            m_dwVID: dwVID,
            m_dwGuildID: dwGuildID,
            m_dwArmor: awPart[CHR_EQUIPPART_ARMOR] as u32,
            m_dwWeapon: awPart[CHR_EQUIPPART_WEAPON] as u32,
            m_dwHair: awPart[CHR_EQUIPPART_HAIR] as u32,
            m_dwMovSpd: bMovingSpeed as u32,
            m_dwAtkSpd: bAttackSpeed as u32,
            m_sAlignment: sAlignment,
            m_byPKMode: bPKMode,
            m_dwMountVnum: dwMountVnum,
            m_dwStateFlags: bStateFlag as u32,
            m_kAffectFlags: dwAffectFlag,
        }
    }
}

pub trait NetworkActorManager {
    fn update_actor(&mut self, data: &SNetworkUpdateActorData);
}

pub trait AbstractPlayer {
    fn is_main_character_index(&self, vid: u32) -> bool;
    fn notify_character_update(&mut self, vid: u32);
    fn set_weapon_power(&mut self, weapon: u32);
}

pub trait NetworkStream {
    fn set_guild_id(&mut self, guild_id: u32);
    fn refresh_status(&mut self);
    fn refresh_alignment_window(&mut self);
    fn refresh_equipment_window(&mut self);
    fn refresh_inventory_window(&mut self);
    fn update_simple_player_info(&mut self, index: usize, hair: u32, armor: u32);
    fn get_selected_character_index(&self) -> usize;
}

#[derive(Debug, Clone, Default)]
pub struct SimplePlayerInfo {
    pub wHairPart: u32,
    pub wMainPart: u32,
}

pub struct CharacterUpdateHandler<'a, M: NetworkActorManager, P: AbstractPlayer, S: NetworkStream> {
    pub actor_manager: &'a mut M,
    pub player: &'a mut P,
    pub network_stream: &'a mut S,
}

impl<'a, M: NetworkActorManager, P: AbstractPlayer, S: NetworkStream> CharacterUpdateHandler<'a, M, P, S> {
    pub fn new(actor_manager: &'a mut M, player: &'a mut P, network_stream: &'a mut S) -> Self {
        Self {
            actor_manager,
            player,
            network_stream,
        }
    }

    pub fn handle_packet(&mut self, packet_bytes: &[u8]) -> Result<(), &'static str> {
        let size = core::mem::size_of::<TPacketGCCharacterUpdate>();
        if packet_bytes.len() < size {
            return Err("Packet too small");
        }

        let packet: TPacketGCCharacterUpdate = unsafe {
            core::ptr::read_unaligned(packet_bytes.as_ptr() as *const TPacketGCCharacterUpdate)
        };

        let actor_data = SNetworkUpdateActorData::from(&packet);
        self.recv_character_update_packet(&actor_data);
        Ok(())
    }

    pub fn recv_character_update_packet(&mut self, net_update_actor_data: &SNetworkUpdateActorData) {
        self.actor_manager.update_actor(net_update_actor_data);

        if self.player.is_main_character_index(net_update_actor_data.m_dwVID) {
            self.network_stream.set_guild_id(net_update_actor_data.m_dwGuildID);
            self.player.set_weapon_power(net_update_actor_data.m_dwWeapon);

            self.network_stream.refresh_status();
            self.network_stream.refresh_alignment_window();
            self.network_stream.refresh_equipment_window();
            self.network_stream.refresh_inventory_window();

            let index = self.network_stream.get_selected_character_index();
            self.network_stream.update_simple_player_info(
                index,
                net_update_actor_data.m_dwHair,
                net_update_actor_data.m_dwArmor,
            );
        } else {
            self.player.notify_character_update(net_update_actor_data.m_dwVID);
        }
    }
}

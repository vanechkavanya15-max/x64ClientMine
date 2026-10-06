#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::c_char;

pub const CREATE_TARGET_TYPE_NONE: u8 = 0;
pub const CREATE_TARGET_TYPE_LOCATION: u8 = 1;
pub const CREATE_TARGET_TYPE_CHARACTER: u8 = 2;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCTarget {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub bHPPercent: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCTargetCreate {
    pub header: u16,
    pub length: u16,
    pub lID: i32,
    pub szTargetName: [u8; 33],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCTargetCreateNew {
    pub header: u16,
    pub length: u16,
    pub lID: i32,
    pub szTargetName: [u8; 33],
    pub dwVID: u32,
    pub byType: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCTargetUpdate {
    pub header: u16,
    pub length: u16,
    pub lID: i32,
    pub lX: i32,
    pub lY: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCTargetDelete {
    pub header: u16,
    pub length: u16,
    pub lID: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TargetBoardAction {
    CloseTargetBoard,
    CloseTargetBoardIfDifferent(u32),
    SetHPTargetBoard { vid: u32, hp_percent: u8 },
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TargetPacketResult {
    pub action: TargetBoardAction,
}

pub fn handle_target_packet(
    packet: &TPacketGCTarget,
    is_dead: bool,
    is_pc_or_building: bool,
    can_view_hp: bool,
) -> TargetPacketResult {
    if is_dead {
        return TargetPacketResult {
            action: TargetBoardAction::CloseTargetBoard,
        };
    }

    if is_pc_or_building {
        return TargetPacketResult {
            action: TargetBoardAction::CloseTargetBoardIfDifferent(packet.dwVID),
        };
    } else if can_view_hp {
        return TargetPacketResult {
            action: TargetBoardAction::SetHPTargetBoard {
                vid: packet.dwVID,
                hp_percent: packet.bHPPercent,
            },
        };
    } else {
        return TargetPacketResult {
            action: TargetBoardAction::CloseTargetBoard,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TargetCreateResult {
    pub lID: i32,
    pub szTargetName: [u8; 33],
    pub open_atlas_window: bool,
}

pub fn handle_target_create_packet(packet: &TPacketGCTargetCreate) -> TargetCreateResult {
    TargetCreateResult {
        lID: packet.lID,
        szTargetName: packet.szTargetName,
        open_atlas_window: true,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TargetCreateNewAction {
    Location {
        lID: i32,
        szTargetName: [u8; 33],
    },
    Character {
        lID: i32,
        szTargetName: [u8; 33],
        dwVID: u32,
        create_target_effect: bool,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct TargetCreateNewResult {
    pub action: TargetCreateNewAction,
    pub open_atlas_window: bool,
}

pub fn handle_target_create_packet_new(packet: &TPacketGCTargetCreateNew) -> TargetCreateNewResult {
    let action = if packet.byType == CREATE_TARGET_TYPE_LOCATION {
        TargetCreateNewAction::Location {
            lID: packet.lID,
            szTargetName: packet.szTargetName,
        }
    } else {
        TargetCreateNewAction::Character {
            lID: packet.lID,
            szTargetName: packet.szTargetName,
            dwVID: packet.dwVID,
            create_target_effect: true,
        }
    };

    TargetCreateNewResult {
        action,
        open_atlas_window: true,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TargetUpdateResult {
    pub lID: i32,
    pub lX: i32,
    pub lY: i32,
    pub update_target: bool,
    pub create_target_effect: bool,
}

pub fn handle_target_update_packet(packet: &TPacketGCTargetUpdate) -> TargetUpdateResult {
    TargetUpdateResult {
        lID: packet.lID,
        lX: packet.lX,
        lY: packet.lY,
        update_target: true,
        create_target_effect: true,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TargetDeleteResult {
    pub lID: i32,
    pub delete_target: bool,
    pub delete_target_effect: bool,
}

pub fn handle_target_delete_packet(packet: &TPacketGCTargetDelete) -> TargetDeleteResult {
    TargetDeleteResult {
        lID: packet.lID,
        delete_target: true,
        delete_target_effect: true,
    }
}

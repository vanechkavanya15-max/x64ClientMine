// Stale protokolu sieciowego m2dev na podstawie Packet.h i ControlPackets.h
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

pub const PACKET_HEADER_SIZE: usize = 4;

pub type TPacketHeader = u16;

// ============================================================================
// CG -- Client -> Game
// ============================================================================
pub mod cg {
    // Control (ControlPackets.h)
    pub const PONG: u16 = 0x0006;
    pub const KEY_RESPONSE: u16 = 0x000A;

    // Control (Packet.h)
    pub const CLIENT_VERSION: u16 = 0x000D;
    pub const STATE_CHECKER: u16 = 0x000F;
    pub const TEXT: u16 = 0x0011;

    // Authentication
    pub const LOGIN2: u16 = 0x0101;
    pub const LOGIN3: u16 = 0x0102;
    pub const LOGIN_SECURE: u16 = 0x0103;
    pub const EMPIRE: u16 = 0x010A;
    pub const CHANGE_NAME: u16 = 0x010B;

    // Character
    pub const CHARACTER_CREATE: u16 = 0x0201;
    pub const CHARACTER_DELETE: u16 = 0x0202;
    pub const CHARACTER_SELECT: u16 = 0x0203;
    pub const ENTERGAME: u16 = 0x0204;

    // Movement
    pub const MOVE: u16 = 0x0301;
    pub const SYNC_POSITION: u16 = 0x0303;
    pub const WARP: u16 = 0x0305;

    // Combat
    pub const ATTACK: u16 = 0x0401;
    pub const USE_SKILL: u16 = 0x0402;
    pub const SHOOT: u16 = 0x0403;
    pub const FLY_TARGETING: u16 = 0x0404;
    pub const ADD_FLY_TARGETING: u16 = 0x0405;

    // Items
    pub const ITEM_USE: u16 = 0x0501;
    pub const ITEM_DROP: u16 = 0x0502;
    pub const ITEM_DROP2: u16 = 0x0503;
    pub const ITEM_MOVE: u16 = 0x0504;
    pub const ITEM_PICKUP: u16 = 0x0505;
    pub const ITEM_USE_TO_ITEM: u16 = 0x0506;
    pub const ITEM_GIVE: u16 = 0x0507;
    pub const EXCHANGE: u16 = 0x0508;
    pub const QUICKSLOT_ADD: u16 = 0x0509;
    pub const QUICKSLOT_DEL: u16 = 0x050A;
    pub const QUICKSLOT_SWAP: u16 = 0x050B;
    pub const REFINE: u16 = 0x050C;
    pub const DRAGON_SOUL_REFINE: u16 = 0x050D;

    // Chat
    pub const CHAT: u16 = 0x0601;
    pub const WHISPER: u16 = 0x0602;

    // Social
    pub const PARTY_INVITE: u16 = 0x0701;
    pub const PARTY_INVITE_ANSWER: u16 = 0x0702;
    pub const PARTY_REMOVE: u16 = 0x0703;
    pub const PARTY_SET_STATE: u16 = 0x0704;
    pub const PARTY_USE_SKILL: u16 = 0x0705;
    pub const PARTY_PARAMETER: u16 = 0x0706;
    pub const GUILD: u16 = 0x0720;
    pub const ANSWER_MAKE_GUILD: u16 = 0x0721;
    pub const GUILD_SYMBOL_UPLOAD: u16 = 0x0722;
    pub const SYMBOL_CRC: u16 = 0x0723;
    pub const MESSENGER: u16 = 0x0740;

    // Shop / Safebox / Mall
    pub const SHOP: u16 = 0x0801;
    pub const MYSHOP: u16 = 0x0802;
    pub const SAFEBOX_CHECKIN: u16 = 0x0820;
    pub const SAFEBOX_CHECKOUT: u16 = 0x0821;
    pub const SAFEBOX_ITEM_MOVE: u16 = 0x0822;
    pub const MALL_CHECKOUT: u16 = 0x0840;

    // Quest
    pub const SCRIPT_ANSWER: u16 = 0x0901;
    pub const SCRIPT_BUTTON: u16 = 0x0902;
    pub const SCRIPT_SELECT_ITEM: u16 = 0x0903;
    pub const QUEST_INPUT_STRING: u16 = 0x0904;
    pub const QUEST_CONFIRM: u16 = 0x0905;
    pub const QUEST_CANCEL: u16 = 0x0906;

    // UI / Targeting
    pub const TARGET: u16 = 0x0A01;
    pub const ON_CLICK: u16 = 0x0A02;
    pub const CHARACTER_POSITION: u16 = 0x0A60;

    // World
    pub const FISHING: u16 = 0x0B01;
    pub const DUNGEON: u16 = 0x0B02;
    pub const HACK: u16 = 0x0B03;

    // Guild Marks
    pub const MARK_LOGIN: u16 = 0x0C01;
    pub const MARK_CRCLIST: u16 = 0x0C02;
    pub const MARK_UPLOAD: u16 = 0x0C03;
    pub const MARK_IDXLIST: u16 = 0x0C04;
}

// ============================================================================
// GC -- Game -> Client
// ============================================================================
pub mod gc {
    // Control (ControlPackets.h)
    pub const PING: u16 = 0x0007;
    pub const PHASE: u16 = 0x0008;
    pub const KEY_CHALLENGE: u16 = 0x000B;
    pub const KEY_COMPLETE: u16 = 0x000C;

    // Control (Packet.h)
    pub const RESPOND_CHANNELSTATUS: u16 = 0x0010;

    // Authentication
    pub const LOGIN_SUCCESS3: u16 = 0x0104;
    pub const LOGIN_SUCCESS4: u16 = 0x0105;
    pub const LOGIN_FAILURE: u16 = 0x0106;
    pub const LOGIN_KEY: u16 = 0x0107;
    pub const AUTH_SUCCESS: u16 = 0x0108;
    pub const EMPIRE: u16 = 0x0109;
    pub const CHANGE_NAME: u16 = 0x010C;

    // Character
    pub const CHARACTER_ADD: u16 = 0x0205;
    pub const CHARACTER_ADD2: u16 = 0x0206;
    pub const CHAR_ADDITIONAL_INFO: u16 = 0x0207;
    pub const CHARACTER_DEL: u16 = 0x0208;
    pub const CHARACTER_UPDATE: u16 = 0x0209;
    pub const CHARACTER_UPDATE2: u16 = 0x020A;
    pub const CHARACTER_DELETE: u16 = 0x020A;
    pub const CHARACTER_POSITION: u16 = 0x020B;
    pub const PLAYER_CREATE_SUCCESS: u16 = 0x020C;
    pub const PLAYER_CREATE_FAILURE: u16 = 0x020D;
    pub const PLAYER_DELETE_SUCCESS: u16 = 0x020E;
    pub const PLAYER_DELETE_WRONG_SOCIAL_ID: u16 = 0x020F;
    pub const MAIN_CHARACTER: u16 = 0x0210;
    pub const PLAYER_POINTS: u16 = 0x0214;
    pub const PLAYER_POINT_CHANGE: u16 = 0x0215;
    pub const STUN: u16 = 0x0216;
    pub const DEAD: u16 = 0x0217;
    pub const CHANGE_SPEED: u16 = 0x0218;
    pub const WALK_MODE: u16 = 0x0219;
    pub const SKILL_LEVEL: u16 = 0x021A;
    pub const SKILL_LEVEL_NEW: u16 = 0x021B;
    pub const SKILL_COOLTIME_END: u16 = 0x021C;
    pub const CHANGE_SKILL_GROUP: u16 = 0x021D;
    pub const VIEW_EQUIP: u16 = 0x021E;

    // Movement
    pub const MOVE: u16 = 0x0302;
    pub const SYNC_POSITION: u16 = 0x0304;
    pub const WARP: u16 = 0x0306;
    pub const MOTION: u16 = 0x0307;
    pub const DIG_MOTION: u16 = 0x0308;

    // Combat
    pub const DAMAGE_INFO: u16 = 0x0410;
    pub const FLY_TARGETING: u16 = 0x0411;
    pub const ADD_FLY_TARGETING: u16 = 0x0412;
    pub const CREATE_FLY: u16 = 0x0413;
    pub const PVP: u16 = 0x0414;
    pub const DUEL_START: u16 = 0x0415;

    // Items
    pub const ITEM_DEL: u16 = 0x0510;
    pub const ITEM_SET: u16 = 0x0511;
    pub const ITEM_USE: u16 = 0x0512;
    pub const ITEM_DROP: u16 = 0x0513;
    pub const ITEM_UPDATE: u16 = 0x0514;
    pub const ITEM_GROUND_ADD: u16 = 0x0515;
    pub const ITEM_GROUND_DEL: u16 = 0x0516;
    pub const ITEM_OWNERSHIP: u16 = 0x0517;
    pub const ITEM_GET: u16 = 0x0518;
    pub const QUICKSLOT_ADD: u16 = 0x0519;
    pub const QUICKSLOT_DEL: u16 = 0x051A;
    pub const QUICKSLOT_SWAP: u16 = 0x051B;
    pub const EXCHANGE: u16 = 0x051C;
    pub const REFINE_INFORMATION: u16 = 0x051D;
    pub const REFINE_INFORMATION_NEW: u16 = 0x051E;
    pub const DRAGON_SOUL_REFINE: u16 = 0x051F;

    // Chat
    pub const CHAT: u16 = 0x0603;
    pub const WHISPER: u16 = 0x0604;

    // Social
    pub const PARTY_INVITE: u16 = 0x0710;
    pub const PARTY_ADD: u16 = 0x0711;
    pub const PARTY_UPDATE: u16 = 0x0712;
    pub const PARTY_REMOVE: u16 = 0x0713;
    pub const PARTY_LINK: u16 = 0x0714;
    pub const PARTY_UNLINK: u16 = 0x0715;
    pub const PARTY_PARAMETER: u16 = 0x0716;
    pub const GUILD: u16 = 0x0730;
    pub const REQUEST_MAKE_GUILD: u16 = 0x0731;
    pub const SYMBOL_DATA: u16 = 0x0732;
    pub const MESSENGER: u16 = 0x0741;
    pub const LOVER_INFO: u16 = 0x0750;
    pub const LOVE_POINT_UPDATE: u16 = 0x0751;

    // Shop / Safebox / Mall
    pub const SHOP: u16 = 0x0810;
    pub const SHOP_SIGN: u16 = 0x0811;
    pub const SAFEBOX_SET: u16 = 0x0830;
    pub const SAFEBOX_DEL: u16 = 0x0831;
    pub const SAFEBOX_WRONG_PASSWORD: u16 = 0x0832;
    pub const SAFEBOX_SIZE: u16 = 0x0833;
    pub const SAFEBOX_MONEY_CHANGE: u16 = 0x0834;
    pub const MALL_OPEN: u16 = 0x0841;
    pub const MALL_SET: u16 = 0x0842;
    pub const MALL_DEL: u16 = 0x0843;

    // Quest
    pub const SCRIPT: u16 = 0x0910;
    pub const QUEST_CONFIRM: u16 = 0x0911;
    pub const QUEST_INFO: u16 = 0x0912;

    // UI / Effects / Targeting
    pub const TARGET: u16 = 0x0A10;
    pub const TARGET_UPDATE: u16 = 0x0A11;
    pub const TARGET_DELETE: u16 = 0x0A12;
    pub const TARGET_CREATE_NEW: u16 = 0x0A13;
    pub const AFFECT_ADD: u16 = 0x0A20;
    pub const AFFECT_REMOVE: u16 = 0x0A21;
    pub const SEPCIAL_EFFECT: u16 = 0x0A30;
    pub const SPECIAL_EFFECT: u16 = 0x0A30;
    pub const SPECIFIC_EFFECT: u16 = 0x0A31;
    pub const MOUNT: u16 = 0x0A40;
    pub const OWNERSHIP: u16 = 0x0A41;
    pub const NPC_POSITION: u16 = 0x0A50;

    // World
    pub const FISHING: u16 = 0x0B10;
    pub const DUNGEON: u16 = 0x0B11;
    pub const LAND_LIST: u16 = 0x0B12;
    pub const TIME: u16 = 0x0B13;
    pub const CHANNEL: u16 = 0x0B14;
    pub const MARK_UPDATE: u16 = 0x0B15;
    pub const OBSERVER_ADD: u16 = 0x0B20;
    pub const OBSERVER_REMOVE: u16 = 0x0B21;
    pub const OBSERVER_MOVE: u16 = 0x0B22;

    // Guild Marks
    pub const MARK_BLOCK: u16 = 0x0C10;
    pub const MARK_IDXLIST: u16 = 0x0C11;
    pub const MARK_DIFF_DATA: u16 = 0x0C12;
}

// Aliasy z duzych liter dla wygody
pub use cg as CG;
pub use gc as GC;

// Fazy gry zgodnie z EPhases
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EPhases {
    Close = 0,
    Handshake = 1,
    Login = 2,
    Select = 3,
    Loading = 4,
    Game = 5,
    Dead = 6,
    ClientConnecting = 7,
    DbClient = 8,
    P2P = 9,
    Auth = 10,
}

// Subheader enums
pub mod guild_sub {
    pub mod cg {
        pub const ADD_MEMBER: u8 = 0;
        pub const REMOVE_MEMBER: u8 = 1;
        pub const CHANGE_GRADE_NAME: u8 = 2;
        pub const CHANGE_GRADE_AUTHORITY: u8 = 3;
        pub const OFFER: u8 = 4;
        pub const POST_COMMENT: u8 = 5;
        pub const DELETE_COMMENT: u8 = 6;
        pub const REFRESH_COMMENT: u8 = 7;
        pub const CHANGE_MEMBER_GRADE: u8 = 8;
        pub const USE_SKILL: u8 = 9;
        pub const CHANGE_MEMBER_GENERAL: u8 = 10;
        pub const GUILD_INVITE_ANSWER: u8 = 11;
        pub const CHARGE_GSP: u8 = 12;
        pub const DEPOSIT_MONEY: u8 = 13;
        pub const WITHDRAW_MONEY: u8 = 14;
    }
    pub mod gc {
        pub const LOGIN: u8 = 0;
        pub const LOGOUT: u8 = 1;
        pub const LIST: u8 = 2;
        pub const GRADE: u8 = 3;
        pub const ADD: u8 = 4;
        pub const REMOVE: u8 = 5;
        pub const GRADE_NAME: u8 = 6;
        pub const GRADE_AUTH: u8 = 7;
        pub const INFO: u8 = 8;
        pub const COMMENTS: u8 = 9;
        pub const CHANGE_EXP: u8 = 10;
        pub const CHANGE_MEMBER_GRADE: u8 = 11;
        pub const SKILL_INFO: u8 = 12;
        pub const CHANGE_MEMBER_GENERAL: u8 = 13;
        pub const GUILD_INVITE: u8 = 14;
        pub const WAR: u8 = 15;
        pub const GUILD_NAME: u8 = 16;
        pub const GUILD_WAR_LIST: u8 = 17;
        pub const GUILD_WAR_END_LIST: u8 = 18;
        pub const WAR_POINT: u8 = 19;
        pub const MONEY_CHANGE: u8 = 20;
    }
}

pub mod shop_sub {
    pub mod cg {
        pub const END: u8 = 0;
        pub const BUY: u8 = 1;
        pub const SELL: u8 = 2;
        pub const SELL2: u8 = 3;
    }
    pub mod gc {
        pub const START: u8 = 0;
        pub const END: u8 = 1;
        pub const UPDATE_ITEM: u8 = 2;
        pub const UPDATE_PRICE: u8 = 3;
        pub const OK: u8 = 4;
        pub const NOT_ENOUGH_MONEY: u8 = 5;
        pub const SOLDOUT: u8 = 6;
        pub const INVENTORY_FULL: u8 = 7;
        pub const INVALID_POS: u8 = 8;
        pub const SOLD_OUT: u8 = 9;
        pub const START_EX: u8 = 10;
        pub const NOT_ENOUGH_MONEY_EX: u8 = 11;
    }
}

pub mod exchange_sub {
    pub mod cg {
        pub const START: u8 = 0;
        pub const ITEM_ADD: u8 = 1;
        pub const ITEM_DEL: u8 = 2;
        pub const ELK_ADD: u8 = 3;
        pub const ACCEPT: u8 = 4;
        pub const CANCEL: u8 = 5;
    }
    pub mod gc {
        pub const START: u8 = 0;
        pub const ITEM_ADD: u8 = 1;
        pub const ITEM_DEL: u8 = 2;
        pub const ELK_ADD: u8 = 3;
        pub const ACCEPT: u8 = 4;
        pub const END: u8 = 5;
        pub const ALREADY: u8 = 6;
        pub const LESS_ELK: u8 = 7;
    }
}

pub mod messenger_sub {
    pub mod cg {
        pub const ADD_BY_VID: u8 = 0;
        pub const ADD_BY_NAME: u8 = 1;
        pub const REMOVE: u8 = 2;
        pub const INVITE_ANSWER: u8 = 3;
    }
    pub mod gc {
        pub const LIST: u8 = 0;
        pub const LOGIN: u8 = 1;
        pub const LOGOUT: u8 = 2;
        pub const INVITE: u8 = 3;
        pub const REMOVE_FRIEND: u8 = 4;
    }
}

pub mod fishing_sub {
    pub mod gc {
        pub const START: u8 = 0;
        pub const STOP: u8 = 1;
        pub const REACT: u8 = 2;
        pub const SUCCESS: u8 = 3;
        pub const FAIL: u8 = 4;
        pub const FISH: u8 = 5;
    }
}

pub mod dungeon_sub {
    pub mod gc {
        pub const TIME_ATTACK_START: u8 = 0;
        pub const DESTINATION_POSITION: u8 = 1;
    }
}

pub mod dragon_soul_sub {
    pub const OPEN: u8 = 0;
    pub const CLOSE: u8 = 1;
    pub const DO_UPGRADE: u8 = 2;
    pub const DO_IMPROVEMENT: u8 = 3;
    pub const DO_REFINE: u8 = 4;
    pub const REFINE_FAIL: u8 = 5;
    pub const REFINE_FAIL_MAX_REFINE: u8 = 6;
    pub const REFINE_FAIL_INVALID_MATERIAL: u8 = 7;
    pub const REFINE_FAIL_NOT_ENOUGH_MONEY: u8 = 8;
    pub const REFINE_FAIL_NOT_ENOUGH_MATERIAL: u8 = 9;
    pub const REFINE_FAIL_TOO_MUCH_MATERIAL: u8 = 10;
    pub const REFINE_SUCCEED: u8 = 11;
}

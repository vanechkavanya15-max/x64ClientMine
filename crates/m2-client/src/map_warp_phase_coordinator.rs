#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCWarp {
    pub header: u16,
    pub length: u16,
    pub lX: i32,
    pub lY: i32,
    pub lAddr: u32,
    pub wPort: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileBuffer {
    pub x_index: i32,
    pub y_index: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum WarpPhaseError {
    NotLoading,
    InvalidPacket,
    InvalidTileData,
    AlreadyReady,
}

pub struct MapWarpPhaseCoordinator {
    pub tile_buffers: Vec<TileBuffer>,
    pub is_loading: bool,
    pub is_ready: bool,
    pub map_base_x: i32,
    pub map_base_y: i32,
    pub server_addr: u32,
    pub server_port: u16,
}

impl MapWarpPhaseCoordinator {
    pub fn new() -> Self {
        Self {
            tile_buffers: Vec::new(),
            is_loading: false,
            is_ready: false,
            map_base_x: 0,
            map_base_y: 0,
            server_addr: 0,
            server_port: 0,
        }
    }

    /// Odbiera pakiet Warp i inicjalizuje proces przeładowania mapy.
    pub fn handle_warp_packet(&mut self, packet: &TPacketGCWarp) -> Result<(), WarpPhaseError> {
        let x = { packet.lX };
        let y = { packet.lY };
        let addr = { packet.lAddr };
        let port = { packet.wPort };

        self.reset_tile_buffers();
        self.initialize_new_map(x, y, addr, port);
        Ok(())
    }

    /// Resetuje bufory kafelkow.
    pub fn reset_tile_buffers(&mut self) {
        self.tile_buffers.clear();
        self.is_ready = false;
        self.is_loading = false;
    }

    /// Inicjalizuje nowa mape bazowa.
    pub fn initialize_new_map(&mut self, base_x: i32, base_y: i32, addr: u32, port: u16) {
        self.map_base_x = base_x;
        self.map_base_y = base_y;
        self.server_addr = addr;
        self.server_port = port;
        self.is_loading = true;
    }

    /// Dodaje bufor kafelka dla zaladowanego sektora mapy.
    pub fn add_tile_buffer(&mut self, tile: TileBuffer) -> Result<(), WarpPhaseError> {
        if !self.is_loading {
            return Err(WarpPhaseError::NotLoading);
        }
        if tile.data.is_empty() {
            return Err(WarpPhaseError::InvalidTileData);
        }
        self.tile_buffers.push(tile);
        Ok(())
    }

    /// Konczy proces ladowania mapy.
    pub fn finalize_map_load(&mut self) -> Result<(), WarpPhaseError> {
        if !self.is_loading {
            return Err(WarpPhaseError::NotLoading);
        }
        if self.is_ready {
            return Err(WarpPhaseError::AlreadyReady);
        }
        
        self.is_loading = false;
        self.is_ready = true;
        Ok(())
    }
}

impl Default for MapWarpPhaseCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

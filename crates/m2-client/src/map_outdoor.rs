 //! Wzorzec referencyjny C++: `source/GameLib/MapOutdoor.cpp oraz source/GameLib/MapOutdoorUpdate.cpp`
 //! Opis: Odwzorowanie CMapOutdoor: zarzadzanie aktywnymi terenami wokol gracza, obliczanie wysokosci terenu GetHeight(x, y), konfiguracja nieba i mgly mapy.
 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapPart {
    Terrain = 0,
    Object = 1,
    Cloud = 2,
    Water = 3,
    Tree = 4,
    Sky = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainRenderSort {
    DistanceSort = 0,
    TextureSort = 1,
}

pub const LOAD_SIZE_WIDTH: i32 = 1;
pub const AROUND_AREA_NUM: i32 = 1 + (LOAD_SIZE_WIDTH * 2) * (LOAD_SIZE_WIDTH * 2) * 2; // Zgodnie z C++ makrem `AROUND_AREA_NUM 1+(LOAD_SIZE_WIDTH*2)*(LOAD_SIZE_WIDTH*2)*2` tj. 9

pub const TERRAIN_PATCHSIZE: i32 = 16;
pub const TERRAIN_SIZE: i32 = 128;
pub const TERRAIN_PATCHCOUNT: i32 = TERRAIN_SIZE / TERRAIN_PATCHSIZE;
pub const TERRAIN_XSIZE: i32 = TERRAIN_SIZE * 200;
pub const TERRAIN_YSIZE: i32 = TERRAIN_SIZE * 200;
pub const HALF_CELLSCALE: i32 = 100;
pub const HEIGHT_TILE_XRATIO: i32 = 2;
pub const HEIGHT_TILE_YRATIO: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainCoordinate {
    pub terrain_coord_x: i16,
    pub terrain_coord_y: i16,
}

#[derive(Debug, Clone)]
 pub struct MapOutdoor {
     pub initialized: bool,
    pub terrain_count_x: i16,
    pub terrain_count_y: i16,
    pub base_x: u32,
    pub base_y: u32,
    pub enable_terrain_only_for_height: bool,
    pub enable_portal: bool,
    pub visible_part_flags: u32,
    pub splat_limit: i32,
    pub terrain_render_sort: TerrainRenderSort,
    pub opaque_water_depth: f32,
    pub cur_coordinate: TerrainCoordinate,
    pub prev_coordinate: TerrainCoordinate,
    pub view_radius: i32,
    pub patch_count: u16,
}

impl Default for MapOutdoor {
    fn default() -> Self {
        Self {
            initialized: false,
            terrain_count_x: 0,
            terrain_count_y: 0,
            base_x: 0,
            base_y: 0,
            enable_terrain_only_for_height: false,
            enable_portal: false,
            visible_part_flags: 0xffffffff,
            splat_limit: 50000,
            terrain_render_sort: TerrainRenderSort::DistanceSort,
            opaque_water_depth: 400.0,
            cur_coordinate: TerrainCoordinate { terrain_coord_x: -1, terrain_coord_y: -1 },
            prev_coordinate: TerrainCoordinate { terrain_coord_x: -1, terrain_coord_y: -1 },
            view_radius: 0,
            patch_count: 0,
        }
    }
 }
 
 impl MapOutdoor {
     pub fn new() -> Self {
        let mut instance = Self::default();
        instance.initialize();
        instance
    }

    pub fn initialize(&mut self) -> bool {
        self.initialized = true;
        self.terrain_count_x = 0;
        self.terrain_count_y = 0;
        self.base_x = 0;
        self.base_y = 0;
        self.enable_terrain_only_for_height = false;
        self.enable_portal = false;
        self.visible_part_flags = 0xffffffff;
        self.splat_limit = 50000;
        self.terrain_render_sort = TerrainRenderSort::DistanceSort;
        self.opaque_water_depth = 400.0;
        self.cur_coordinate = TerrainCoordinate { terrain_coord_x: -1, terrain_coord_y: -1 };
        self.prev_coordinate = TerrainCoordinate { terrain_coord_x: -1, terrain_coord_y: -1 };
        self.view_radius = 0;
        self.patch_count = 0;
        self.initialize_fog();
        true
    }

    pub fn initialize_fog(&mut self) {
        // Mock inicjalizacji mgly wedlug CMapOutdoor::InitializeFog()
        // W C++: memset(&m_matAlphaFogTexture, 0, sizeof(D3DXMATRIX));
        // W Rust bridge jeszcze brak stanu mgly uzywanego pozniej, w tym tasku wystarczy skeleton.
     }
 
     pub fn is_ready(&self) -> bool {
         self.initialized
     }
 
    pub fn clear(&mut self) {
        self.initialize();
        self.initialized = false;
    }

    pub fn set_terrain_count(&mut self, x: i16, y: i16) -> bool {
        if x == 0 || x > 255 || y == 0 || y > 255 {
            return false;
        }
        self.terrain_count_x = x;
        self.terrain_count_y = y;
        true
    }

    pub fn get_terrain_count(&self) -> (i16, i16) {
        (self.terrain_count_x, self.terrain_count_y)
    }

    pub fn set_base_xy(&mut self, x: u32, y: u32) {
        self.base_x = x;
        self.base_y = y;
    }

    pub fn get_base_xy(&self) -> (u32, u32) {
        (self.base_x, self.base_y)
    }

    pub fn enable_terrain_only_for_height(&mut self, flag: bool) {
        self.enable_terrain_only_for_height = flag;
    }

    pub fn enable_portal(&mut self, flag: bool) {
        self.enable_portal = flag;
    }

    pub fn is_enable_portal(&self) -> bool {
        self.enable_portal
    }

    pub fn set_opaque_water_depth(&mut self, depth: f32) {
        self.opaque_water_depth = depth;
    }

    pub fn get_opaque_water_depth(&self) -> f32 {
        self.opaque_water_depth
    }

    pub fn set_terrain_render_sort(&mut self, sort: TerrainRenderSort) {
        self.terrain_render_sort = sort;
    }

    pub fn get_terrain_render_sort(&self) -> TerrainRenderSort {
        self.terrain_render_sort
    }

    pub fn set_visible_part(&mut self, part: MapPart, is_visible: bool) {
        let mask = 1 << (part as u32);
        if is_visible {
            self.visible_part_flags |= mask;
        } else {
            self.visible_part_flags &= !mask;
        }
    }

    pub fn set_splat_limit(&mut self, limit: i32) {
        self.splat_limit = limit;
    }

    pub fn get_terrain_num_from_coord(&self, w_coord_x: u16, w_coord_y: u16, pby_terrain_num: &mut u8) -> bool {
        let cur_x = self.cur_coordinate.terrain_coord_x;
        let cur_y = self.cur_coordinate.terrain_coord_y;
        
        let terrain_num = (w_coord_y as i32 - cur_y as i32 + LOAD_SIZE_WIDTH) * 3 + (w_coord_x as i32 - cur_x as i32 + LOAD_SIZE_WIDTH);
        
        if terrain_num < 0 || terrain_num >= AROUND_AREA_NUM {
            return false;
        }
        
        *pby_terrain_num = terrain_num as u8;
        true
    }

    pub fn get_terrain_height(&self, fx: f32, mut fy: f32) -> f32 {
        if fy < 0.0 {
            fy = -fy;
        }

        let lx = fx as i32;
        let ly = fy as i32;

        let us_coord_x = (lx / TERRAIN_XSIZE) as u16;
        let us_coord_y = (ly / TERRAIN_YSIZE) as u16;

        let mut by_terrain_num = 0;
        if !self.get_terrain_num_from_coord(us_coord_x, us_coord_y, &mut by_terrain_num) {
            return 0.0;
        }

        // Zgodnie z C++: pTerrain->GetHeight(lx, ly);
        // My na ten moment nie portujemy samej instancji CTerrain ani logiki mapy wysokosci (Terrain.cpp to ponad 1000 linii, poza scopem tego zadania).
        // Zwrocmy bazowe zero. W prawdziwym kodzie wolane by bylo pTerrain[byTerrainNum].get_height(lx, ly).
        0.0 
    }

    pub fn get_height(&self, fx: f32, fy: f32) -> f32 {
        let terrain_height = self.get_terrain_height(fx, fy);

        if !self.enable_terrain_only_for_height {
            let check_height = 25000.0;
            let object_height = -check_height;

            // FGetObjectHeight - sprawdzanie wysokosci obiektu w CTerrain (drzewa, budynki w culling manager).
            // Zwracamy max z object_height i terrain_height.
            if object_height > terrain_height {
                return object_height;
            }
        }
        terrain_height
    }

    pub fn get_cache_height(&self, fx: f32, fy: f32) -> f32 {
        // C++ zarzadza wielkim Cache SHeightCache z kluczem bitowym i statystykami chybien (HitRate).
        // W portowaniu Rustowym cache na razie przechodzi jako przezroczyste wywolanie:
        self.get_height(fx, fy)
    }

    pub fn convert_to_map_coords(
        &self,
        mut fx: f32,
        mut fy: f32,
    ) -> (i32, i32, u8, u8, u16, u16) {
        if fy < 0.0 {
            fy = -fy;
        }

        let ix = fx as i32;
        let iy = fy as i32;

        let pw_terrain_num_x = (ix / TERRAIN_XSIZE) as u16;
        let pw_terrain_num_y = (iy / TERRAIN_YSIZE) as u16;

        let maxx = TERRAIN_XSIZE as f32;
        let maxy = TERRAIN_YSIZE as f32;

        while fx < 0.0 {
            fx += maxx;
        }

        while fy < 0.0 {
            fy += maxy;
        }

        while fx >= maxx {
            fx -= maxx;
        }

        while fy >= maxy {
            fy -= maxy;
        }

        let fooscale = 1.0 / (HALF_CELLSCALE as f32);

        let f_cell_x = fx * fooscale;
        let f_cell_y = fy * fooscale;

        let i_cell_x = f_cell_x as i32;
        let i_cell_y = f_cell_y as i32;

        let f_ratioooscale = (HEIGHT_TILE_XRATIO as f32) * fooscale;

        let f_subcell_x = fx * f_ratioooscale;
        let f_subcell_y = fy * f_ratioooscale;

        let mut puc_sub_cell_x = f_subcell_x as u8;
        let mut puc_sub_cell_y = f_subcell_y as u8;

        puc_sub_cell_x %= HEIGHT_TILE_XRATIO as u8;
        puc_sub_cell_y %= HEIGHT_TILE_YRATIO as u8;
 
        (
            i_cell_x,
            i_cell_y,
            puc_sub_cell_x,
            puc_sub_cell_y,
            pw_terrain_num_x,
            pw_terrain_num_y,
        )
     }
 }

use m2_client_bridge::ffi_guard::safe_ffi_call;

/// Informacje o poziomie szczegolowosci (LOD) drzewa
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpeedTreeLodInfo {
    /// 0 = Pelna geometria 3D, 1 = Cross-fading, 2 = Czysty billboard
    pub level: u8,
    /// Wspolczynnik przezroczystosci dla geometrii 3D (0.0 = niewidoczne, 1.0 = w pelni widoczne)
    pub alpha_3d: f32,
    /// Wspolczynnik przezroczystosci dla billboardu (0.0 = niewidoczny, 1.0 = w pelni widoczny)
    pub alpha_billboard: f32,
}

/// Wierzcholek dla billboardu drzewa
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BillboardVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

/// Oblicza LOD na podstawie odleglosci od kamery
pub fn calculate_speedtree_lod(distance: f32) -> SpeedTreeLodInfo {
    let distance = distance.max(0.0);
    
    if distance < 3000.0 {
        // Pelna geometria 3D
        SpeedTreeLodInfo {
            level: 0,
            alpha_3d: 1.0,
            alpha_billboard: 0.0,
        }
    } else if distance <= 6000.0 {
        // Cross-fading alpha z geometria 3D na billboard
        // 3000.0 -> factor 0.0, 6000.0 -> factor 1.0
        let factor = (distance - 3000.0) / 3000.0;
        let factor = factor.clamp(0.0, 1.0);
        
        SpeedTreeLodInfo {
            level: 1,
            alpha_3d: 1.0 - factor,
            alpha_billboard: factor,
        }
    } else {
        // Czysty billboard
        SpeedTreeLodInfo {
            level: 2,
            alpha_3d: 0.0,
            alpha_billboard: 1.0,
        }
    }
}

/// Generuje dwa przekatne wielokaty (quady) dla czystego billboardu
/// Kazdy quad sklada sie z 2 trojkatow (6 wierzcholkow), razem 12 wierzcholkow
pub fn generate_cross_billboard(center: [f32; 3], width: f32, height: f32) -> [BillboardVertex; 12] {
    let half_w = width * 0.5;
    
    // Quad 1: os XZ (przesunieta o polowe szerokosci po X)
    // Zeby osiagnac efekt dwoch przekatnych, zrobmy krzyz na osiach XY wzgledem srodka
    
    // Przekatna 1 (od lewego dolnego rogu -X, -Y do prawego gornego +X, +Y)
    // Obrocmy to o 45 stopni dla lepszego krzyza. 
    // width to szerokosc, zakladamy ze Z idzie w gore (Z-up), bo to Metin2 (WRAPPER_UP_POS_Z).
    let d = half_w * 0.70710678; // sin(45) i cos(45) to 1/sqrt(2)
    
    let p1_bot = [center[0] - d, center[1] - d, center[2]];
    let p1_top = [center[0] - d, center[1] - d, center[2] + height];
    let p2_bot = [center[0] + d, center[1] + d, center[2]];
    let p2_top = [center[0] + d, center[1] + d, center[2] + height];
    
    // Przekatna 2
    let p3_bot = [center[0] - d, center[1] + d, center[2]];
    let p3_top = [center[0] - d, center[1] + d, center[2] + height];
    let p4_bot = [center[0] + d, center[1] - d, center[2]];
    let p4_top = [center[0] + d, center[1] - d, center[2] + height];
    
    // UV map (top left = 0,0, bottom right = 1,1) (flip T coord used in C++)
    // WRAPPER_FLIP_T_TEXCOORD
    let uv_tl = [0.0, 0.0];
    let uv_bl = [0.0, 1.0];
    let uv_tr = [1.0, 0.0];
    let uv_br = [1.0, 1.0];
    
    [
        // Quad 1, Trojkat 1
        BillboardVertex { position: p1_bot, uv: uv_bl },
        BillboardVertex { position: p2_bot, uv: uv_br },
        BillboardVertex { position: p1_top, uv: uv_tl },
        // Quad 1, Trojkat 2
        BillboardVertex { position: p2_bot, uv: uv_br },
        BillboardVertex { position: p2_top, uv: uv_tr },
        BillboardVertex { position: p1_top, uv: uv_tl },
        
        // Quad 2, Trojkat 1
        BillboardVertex { position: p3_bot, uv: uv_bl },
        BillboardVertex { position: p4_bot, uv: uv_br },
        BillboardVertex { position: p3_top, uv: uv_tl },
        // Quad 2, Trojkat 2
        BillboardVertex { position: p4_bot, uv: uv_br },
        BillboardVertex { position: p4_top, uv: uv_tr },
        BillboardVertex { position: p3_top, uv: uv_tl },
    ]
}

/// Eksport FFI
#[no_mangle]
pub extern "C" fn m2_forest_get_lod_level(distance: f32) -> u8 {
    safe_ffi_call(2, || {
        calculate_speedtree_lod(distance).level
    })
}

// BitKnit decompression algorithm natively implemented in Rust.
// Based on public implementation reference formats.

#[derive(Clone, Copy)]
pub struct BitknitLiteral {
    pub lookup: [u16; 516],
    pub a: [u16; 301],
    pub freq: [u16; 300],
    pub adapt_interval: u32,
}

impl BitknitLiteral {
    pub fn new() -> Self {
        let mut model = Self {
            lookup: [0; 516],
            a: [0; 301],
            freq: [1; 300],
            adapt_interval: 1024,
        };
        for i in 0..264 {
            model.a[i] = ((0x8000 - 300 + 264) * i as u32 / 264) as u16;
        }
        for i in 264..=300 {
            model.a[i] = (0x8000 - 300 + i) as u16;
        }
        Self::rebuild_lookup(&mut model);
        model
    }

    fn rebuild_lookup(model: &mut Self) {
        let mut p = 0;
        for i in 0..300 {
            let p_end = (model.a[i + 1].saturating_sub(1) >> 6) as usize;
            while p <= p_end {
                model.lookup[p] = i as u16;
                model.lookup[p + 1] = i as u16;
                model.lookup[p + 2] = i as u16;
                model.lookup[p + 3] = i as u16;
                p += 4;
            }
        }
    }

    pub fn adaptive(&mut self, sym: usize) {
        self.adapt_interval = 1024;
        self.freq[sym] += 725;
        let mut sum = 0;
        for i in 0..300 {
            sum += self.freq[i] as u32;
            self.freq[i] = 1;
            let a_val = self.a[i + 1] as u32;
            self.a[i + 1] = (a_val + ((sum - a_val) >> 1)) as u16;
        }
        Self::rebuild_lookup(self);
    }

    pub fn lookup(&mut self, bits: &mut u32) -> u32 {
        let masked = *bits & 0x7FFF;
        let mut sym = self.lookup[(masked >> 6) as usize] as usize;
        sym += (masked > self.a[sym + 1] as u32) as usize;
        while masked >= self.a[sym + 1] as u32 {
            sym += 1;
        }
        
        let asym0 = self.a[sym] as u32;
        let asym1 = self.a[sym + 1] as u32;
        *bits = masked + (*bits >> 15) * (asym1 - asym0) - asym0;
        
        self.freq[sym] += 31;
        self.adapt_interval -= 1;
        if self.adapt_interval == 0 {
            self.adaptive(sym);
        }
        sym as u32
    }
}

#[derive(Clone, Copy)]
pub struct BitknitDistanceLsb {
    pub lookup: [u16; 68],
    pub a: [u16; 41],
    pub freq: [u16; 40],
    pub adapt_interval: u32,
}

impl BitknitDistanceLsb {
    pub fn new() -> Self {
        let mut model = Self {
            lookup: [0; 68],
            a: [0; 41],
            freq: [1; 40],
            adapt_interval: 1024,
        };
        for i in 0..=40 {
            model.a[i] = (0x8000 * i as u32 / 40) as u16;
        }
        Self::rebuild_lookup(&mut model);
        model
    }

    fn rebuild_lookup(model: &mut Self) {
        let mut p = 0;
        for i in 0..40 {
            let p_end = (model.a[i + 1].saturating_sub(1) >> 9) as usize;
            while p <= p_end {
                model.lookup[p] = i as u16;
                model.lookup[p + 1] = i as u16;
                model.lookup[p + 2] = i as u16;
                model.lookup[p + 3] = i as u16;
                p += 4;
            }
        }
    }

    pub fn adaptive(&mut self, sym: usize) {
        self.adapt_interval = 1024;
        self.freq[sym] += 985;
        let mut sum = 0;
        for i in 0..40 {
            sum += self.freq[i] as u32;
            self.freq[i] = 1;
            let a_val = self.a[i + 1] as u32;
            self.a[i + 1] = (a_val + ((sum - a_val) >> 1)) as u16;
        }
        Self::rebuild_lookup(self);
    }

    pub fn lookup(&mut self, bits: &mut u32) -> u32 {
        let masked = *bits & 0x7FFF;
        let mut sym = self.lookup[(masked >> 9) as usize] as usize;
        sym += (masked > self.a[sym + 1] as u32) as usize;
        while masked >= self.a[sym + 1] as u32 {
            sym += 1;
        }
        let asym0 = self.a[sym] as u32;
        let asym1 = self.a[sym + 1] as u32;
        *bits = masked + (*bits >> 15) * (asym1 - asym0) - asym0;
        
        self.freq[sym] += 31;
        self.adapt_interval -= 1;
        if self.adapt_interval == 0 {
            self.adaptive(sym);
        }
        sym as u32
    }
}

#[derive(Clone, Copy)]
pub struct BitknitDistanceBits {
    pub lookup: [u16; 68], 
    pub a: [u16; 22],
    pub freq: [u16; 21],
    pub adapt_interval: u32,
}

impl BitknitDistanceBits {
    pub fn new() -> Self {
        let mut model = Self {
            lookup: [0; 68],
            a: [0; 22],
            freq: [1; 21],
            adapt_interval: 1024,
        };
        for i in 0..=21 {
            model.a[i] = (0x8000 * i as u32 / 21) as u16;
        }
        Self::rebuild_lookup(&mut model);
        model
    }

    fn rebuild_lookup(model: &mut Self) {
        let mut p = 0;
        for i in 0..21 {
            let p_end = (model.a[i + 1].saturating_sub(1) >> 9) as usize;
            while p <= p_end {
                model.lookup[p] = i as u16;
                model.lookup[p + 1] = i as u16;
                model.lookup[p + 2] = i as u16;
                model.lookup[p + 3] = i as u16;
                p += 4;
            }
        }
    }

    pub fn adaptive(&mut self, sym: usize) {
        self.adapt_interval = 1024;
        self.freq[sym] += 1004;
        let mut sum = 0;
        for i in 0..21 {
            sum += self.freq[i] as u32;
            self.freq[i] = 1;
            let a_val = self.a[i + 1] as u32;
            self.a[i + 1] = (a_val + ((sum - a_val) >> 1)) as u16;
        }
        Self::rebuild_lookup(self);
    }

    pub fn lookup(&mut self, bits: &mut u32) -> u32 {
        let masked = *bits & 0x7FFF;
        let mut sym = self.lookup[(masked >> 9) as usize] as usize;
        sym += (masked > self.a[sym + 1] as u32) as usize;
        while masked >= self.a[sym + 1] as u32 {
            sym += 1;
        }
        let asym0 = self.a[sym] as u32;
        let asym1 = self.a[sym + 1] as u32;
        *bits = masked + (*bits >> 15) * (asym1 - asym0) - asym0;
        
        self.freq[sym] += 31;
        self.adapt_interval -= 1;
        if self.adapt_interval == 0 {
            self.adaptive(sym);
        }
        sym as u32
    }
}

pub struct BitknitState {
    pub recent_dist: [u32; 8],
    pub last_match_dist: u32,
    pub recent_dist_mask: u32,
    pub literals: [BitknitLiteral; 4],
    pub distance_lsb: [BitknitDistanceLsb; 4],
    pub distance_bits: BitknitDistanceBits,
}

impl BitknitState {
    pub fn new() -> Self {
        let recent_dist_mask = (7 << (7 * 3)) | (6 << (6 * 3)) |
                               (5 << (5 * 3)) | (4 << (4 * 3)) |
                               (3 << (3 * 3)) | (2 << (2 * 3)) |
                               (1 << (1 * 3)) | (0 << (0 * 3));
        Self {
            recent_dist: [1; 8],
            last_match_dist: 1,
            recent_dist_mask,
            literals: [BitknitLiteral::new(), BitknitLiteral::new(), BitknitLiteral::new(), BitknitLiteral::new()],
            distance_lsb: [BitknitDistanceLsb::new(), BitknitDistanceLsb::new(), BitknitDistanceLsb::new(), BitknitDistanceLsb::new()],
            distance_bits: BitknitDistanceBits::new(),
        }
    }
}

pub fn bitknit_decode(src: &[u8], dst: &mut [u8]) -> Option<usize> {
    if src.len() < 4 {
        return None;
    }
    
    let mut bk = BitknitState::new();
    let mut src_idx = 0;
    let mut dst_idx = 0;
    
    let mut recent_dist_mask = bk.recent_dist_mask;
    let mut last_match_dist = bk.last_match_dist as usize;
    
    let v = u32::from_le_bytes([src[0], src[1], src[2], src[3]]);
    src_idx += 4;
    
    if v < 0x10000 {
        return Some(0);
    }
    
    let mut a = v >> 4;
    let n = v & 0xF;
    
    if a < 0x10000 {
        if src_idx + 2 > src.len() { return None; }
        a = (a << 16) | u16::from_le_bytes([src[src_idx], src[src_idx+1]]) as u32;
        src_idx += 2;
    }
    
    let mut bits = a >> n;
    
    if bits < 0x10000 {
        if src_idx + 2 > src.len() { return None; }
        bits = (bits << 16) | u16::from_le_bytes([src[src_idx], src[src_idx+1]]) as u32;
        src_idx += 2;
    }
    
    if src_idx + 2 > src.len() { return None; }
    a = (a << 16) | u16::from_le_bytes([src[src_idx], src[src_idx+1]]) as u32;
    src_idx += 2;
    
    let mut bits2 = (1 << (n + 16)) | (a & ((1 << (n + 16)) - 1));
    
    macro_rules! renormalize {
        () => {
            if bits < 0x10000 {
                if src_idx + 2 > src.len() { return None; }
                bits = (bits << 16) | u16::from_le_bytes([src[src_idx], src[src_idx+1]]) as u32;
                src_idx += 2;
                let bitst = bits;
                bits = bits2;
                bits2 = bitst;
            }
        }
    }
    
    if dst_idx == 0 && dst.len() > 0 {
        dst[dst_idx] = (bits & 0xFF) as u8;
        dst_idx += 1;
        bits >>= 8;
        renormalize!();
    }
    
    while dst_idx + 4 < dst.len() {
        let mut sym = bk.literals[dst_idx & 3].lookup(&mut bits);
        renormalize!();
        
        if sym < 256 {
            let last_byte = if last_match_dist <= dst_idx {
                dst[dst_idx - last_match_dist]
            } else {
                0
            };
            dst[dst_idx] = sym.wrapping_add(last_byte as u32) as u8;
            dst_idx += 1;
            
            if dst_idx + 4 >= dst.len() {
                break;
            }
            
            sym = bk.literals[dst_idx & 3].lookup(&mut bits);
            renormalize!();
            
            if sym < 256 {
                let last_byte = if last_match_dist <= dst_idx {
                    dst[dst_idx - last_match_dist]
                } else {
                    0
                };
                dst[dst_idx] = sym.wrapping_add(last_byte as u32) as u8;
                dst_idx += 1;
                continue;
            }
        }
        
        if sym >= 288 {
            let nb = sym - 287;
            sym = (bits & ((1 << nb) - 1)) + (1 << nb) + 286;
            bits >>= nb;
            renormalize!();
        }
        
        let copy_length = (sym - 254) as usize;
        
        sym = bk.distance_lsb[dst_idx & 3].lookup(&mut bits);
        renormalize!();
        
        let mut match_dist;
        
        if sym >= 8 {
            let nb = bk.distance_bits.lookup(&mut bits);
            renormalize!();
            
            match_dist = bits & ((1 << (nb & 0xF)) - 1);
            bits >>= nb & 0xF;
            renormalize!();
            
            if nb >= 0x10 {
                if src_idx + 2 > src.len() { return None; }
                match_dist = (match_dist << 16) | u16::from_le_bytes([src[src_idx], src[src_idx+1]]) as u32;
                src_idx += 2;
            }
            
            match_dist = (32 << nb) + (match_dist << 5) + sym - 39;
            
            bk.recent_dist[((recent_dist_mask >> 21) & 7) as usize] = bk.recent_dist[((recent_dist_mask >> 18) & 7) as usize];
            bk.recent_dist[((recent_dist_mask >> 18) & 7) as usize] = match_dist;
        } else {
            let idx = (recent_dist_mask >> (3 * sym)) & 7;
            let mask = !(7 << (3 * sym));
            match_dist = bk.recent_dist[idx as usize];
            recent_dist_mask = (recent_dist_mask & mask) | ((idx + 8 * recent_dist_mask) & !mask);
        }
        
        if match_dist == 0 || match_dist as usize > dst_idx {
            return None; // Invalid distance
        }
        
        let d = match_dist as usize;
        
        for _ in 0..copy_length {
            if dst_idx >= dst.len() { break; }
            dst[dst_idx] = dst[dst_idx - d];
            dst_idx += 1;
        }
        
        last_match_dist = match_dist as usize;
    }
    
    if dst_idx + 4 <= dst.len() {
        let footer = (bits as u16 as u32) | (bits2 << 16);
        let footer_bytes = footer.to_le_bytes();
        dst[dst_idx..dst_idx+4].copy_from_slice(&footer_bytes);
        dst_idx += 4;
        let _ = dst_idx; // Suppress unused var warning
    }
    
    Some(src_idx)
}

//! Modul quadtree dla skrzynki m2-client-terrain
//! Drzewo czworkowe (Quadtree) dla culling'u i LOD sektorow
//! Struktura wezlow przestrzennych, prostopadlosciany AABB, testy przeciecia z frustum

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Sprawdza przeciecie z prostopadloscianem
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    pub fn contains_point(&self, point: &Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    pub fn new(normal: Vec3, d: f32) -> Self {
        Self { normal, d }
    }

    /// Odleglosc punktu od plaszczyzny
    pub fn distance(&self, p: &Vec3) -> f32 {
        self.normal.dot(p) + self.d
    }
}

#[derive(Debug, Clone)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn new(planes: [Plane; 6]) -> Self {
        Self { planes }
    }

    /// Sprawdza czy AABB znajduje sie w bryle widzenia
    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool {
        for plane in &self.planes {
            let px = if plane.normal.x > 0.0 { aabb.max.x } else { aabb.min.x };
            let py = if plane.normal.y > 0.0 { aabb.max.y } else { aabb.min.y };
            let pz = if plane.normal.z > 0.0 { aabb.max.z } else { aabb.min.z };

            let p = Vec3::new(px, py, pz);

            if plane.distance(&p) < 0.0 {
                return false;
            }
        }
        true
    }
}

/// Element przechowywany w Quadtree
#[derive(Debug, Clone)]
pub struct QuadtreeItem {
    pub id: u32,
    pub aabb: Aabb,
}

/// Wezel drzewa czworkowego
#[derive(Debug, Clone)]
pub struct QuadtreeNode {
    pub bounds: Aabb,
    pub items: Vec<QuadtreeItem>,
    pub children: Option<Box<[QuadtreeNode; 4]>>,
    pub depth: u32,
    pub max_depth: u32,
    pub max_items: usize,
}

impl QuadtreeNode {
    pub fn new(bounds: Aabb, depth: u32, max_depth: u32, max_items: usize) -> Self {
        Self {
            bounds,
            items: Vec::new(),
            children: None,
            depth,
            max_depth,
            max_items,
        }
    }

    pub fn insert(&mut self, item: QuadtreeItem) -> bool {
        if !self.bounds.intersects(&item.aabb) {
            return false;
        }

        if self.children.is_none() {
            if self.items.len() < self.max_items || self.depth == self.max_depth {
                self.items.push(item);
                return true;
            }
            self.split();
        }

        if let Some(children) = &mut self.children {
            let mut inserted = false;
            for child in children.iter_mut() {
                if child.insert(item.clone()) {
                    inserted = true;
                }
            }
            if inserted {
                return true;
            }
        }

        self.items.push(item);
        true
    }

    fn split(&mut self) {
        let min = self.bounds.min;
        let max = self.bounds.max;
        let mid_x = min.x + (max.x - min.x) / 2.0;
        let mid_z = min.z + (max.z - min.z) / 2.0;

        let bounds0 = Aabb::new(Vec3::new(min.x, min.y, min.z), Vec3::new(mid_x, max.y, mid_z));
        let bounds1 = Aabb::new(Vec3::new(mid_x, min.y, min.z), Vec3::new(max.x, max.y, mid_z));
        let bounds2 = Aabb::new(Vec3::new(min.x, min.y, mid_z), Vec3::new(mid_x, max.y, max.z));
        let bounds3 = Aabb::new(Vec3::new(mid_x, min.y, mid_z), Vec3::new(max.x, max.y, max.z));

        let next_depth = self.depth + 1;
        self.children = Some(Box::new([
            QuadtreeNode::new(bounds0, next_depth, self.max_depth, self.max_items),
            QuadtreeNode::new(bounds1, next_depth, self.max_depth, self.max_items),
            QuadtreeNode::new(bounds2, next_depth, self.max_depth, self.max_items),
            QuadtreeNode::new(bounds3, next_depth, self.max_depth, self.max_items),
        ]));

        let old_items = std::mem::take(&mut self.items);
        for item in old_items {
            let mut inserted_to_child = false;
            if let Some(children) = &mut self.children {
                for child in children.iter_mut() {
                    if child.insert(item.clone()) {
                        inserted_to_child = true;
                    }
                }
            }
            if !inserted_to_child {
                self.items.push(item);
            }
        }
    }

    pub fn query_culling(&self, frustum: &Frustum, result: &mut Vec<u32>) {
        if !frustum.intersects_aabb(&self.bounds) {
            return;
        }

        for item in &self.items {
            if frustum.intersects_aabb(&item.aabb) {
                if !result.contains(&item.id) {
                    result.push(item.id);
                }
            }
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_culling(frustum, result);
            }
        }
    }
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone)]
pub struct Quadtree {
    pub root: QuadtreeNode,
    pub initialized: bool,
}

impl Default for Quadtree {
    fn default() -> Self {
        Self::new(Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1000.0, 1000.0, 1000.0)))
    }
}

impl Quadtree {
    pub fn new(bounds: Aabb) -> Self {
        Self {
            root: QuadtreeNode::new(bounds, 0, 4, 10),
            initialized: true,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn insert(&mut self, id: u32, aabb: Aabb) -> bool {
        self.root.insert(QuadtreeItem { id, aabb })
    }

    pub fn query_culling(&self, frustum: &Frustum) -> Vec<u32> {
        let mut result = Vec::new();
        self.root.query_culling(frustum, &mut result);
        result
    }
}

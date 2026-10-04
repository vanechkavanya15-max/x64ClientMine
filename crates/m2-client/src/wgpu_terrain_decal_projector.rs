use glam::{Vec2, Vec3};

/// A plane defined by its normal and distance from origin.
/// Plane equation: dot(N, P) - d = 0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    pub fn new(normal: Vec3, d: f32) -> Self {
        Self { normal, d }
    }
    
    pub fn distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) - self.d
    }
}

pub const DECAL_EPSILON: f32 = 0.25;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DecalVertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
}

pub struct TerrainDecalProjector {
    center: Vec3,
    normal: Vec3,
    tangent: Vec3,
    binormal: Vec3,
    width: f32,
    height: f32,
    depth: f32,
    planes: [Plane; 6],
}

impl TerrainDecalProjector {
    pub fn new(center: Vec3, normal: Vec3, tangent: Vec3, width: f32, height: f32, depth: f32) -> Self {
        let normal = normal.normalize();
        let tangent = tangent.normalize();
        let binormal = normal.cross(tangent).normalize();

        // Left and Right planes (tangent direction)
        let d_left = center.dot(tangent);
        let left_plane = Plane::new(tangent, d_left - width * 0.5);
        let right_plane = Plane::new(-tangent, -d_left - width * 0.5);

        // Bottom and Top planes (binormal direction)
        let d_bottom = center.dot(binormal);
        let bottom_plane = Plane::new(binormal, d_bottom - height * 0.5);
        let top_plane = Plane::new(-binormal, -d_bottom - height * 0.5);

        // Front and Back planes (normal direction)
        let d_front = center.dot(normal);
        let front_plane = Plane::new(-normal, -d_front - depth);
        let back_plane = Plane::new(normal, d_front - depth);

        Self {
            center,
            normal,
            tangent,
            binormal,
            width,
            height,
            depth,
            planes: [left_plane, right_plane, bottom_plane, top_plane, front_plane, back_plane],
        }
    }

    /// Clips a polygon against a single plane.
    pub fn clip_polygon_against_plane(plane: &Plane, in_vertices: &[Vec3]) -> Vec<Vec3> {
        let mut out_vertices = Vec::new();
        let count = in_vertices.len();
        
        if count == 0 {
            return out_vertices;
        }

        let mut distances = Vec::with_capacity(count);
        let mut all_negative = true;

        for v in in_vertices {
            let dist = plane.distance(*v);
            distances.push(dist);
            if dist >= 0.0 {
                all_negative = false;
            }
        }

        if all_negative {
            return out_vertices;
        }

        for i in 0..count {
            let prev = if i == 0 { count - 1 } else { i - 1 };
            
            let v_curr = in_vertices[i];
            let v_prev = in_vertices[prev];
            
            let dist_curr = distances[i];
            let dist_prev = distances[prev];

            let curr_inside = dist_curr >= 0.0;
            let prev_inside = dist_prev >= 0.0;

            if curr_inside != prev_inside {
                // Intersection
                let t = dist_prev / (dist_prev - dist_curr);
                let v_intersect = v_prev.lerp(v_curr, t);
                out_vertices.push(v_intersect);
            }

            if curr_inside {
                out_vertices.push(v_curr);
            }
        }

        out_vertices
    }

    pub fn clip_polygon(&self, vertices: &[Vec3]) -> Vec<Vec3> {
        let mut current_poly = vertices.to_vec();
        
        for plane in &self.planes {
            if current_poly.is_empty() {
                break;
            }
            current_poly = Self::clip_polygon_against_plane(plane, &current_poly);
        }
        
        current_poly
    }

    /// Extends terrain vertices inside the decal bounding box and generates clipped geometry
    pub fn generate_decal_geometry<F>(&self, cell_scale: f32, get_height: F) -> Vec<DecalVertex> 
    where F: Fn(f32, f32) -> f32
    {
        let mut out_vertices = Vec::new();

        let search_radius = self.width.max(self.height);
        
        let min_x = self.center.x - search_radius;
        let max_x = self.center.x + search_radius;
        
        // C++ uses fabs(v3Center.y), but in Metin2 engine y is often negative, 
        // we'll keep bounds check simple in world coords
        let min_y = self.center.y - search_radius;
        let max_y = self.center.y + search_radius;

        let start_x = (min_x / cell_scale).floor() * cell_scale;
        let end_x = (max_x / cell_scale).ceil() * cell_scale;
        
        let start_y = (min_y / cell_scale).floor() * cell_scale;
        let end_y = (max_y / cell_scale).ceil() * cell_scale;

        let mut y = start_y;
        while y < end_y {
            let mut x = start_x;
            while x < end_x {
                
                let h00 = get_height(x, y) + DECAL_EPSILON;
                let h10 = get_height(x + cell_scale, y) + DECAL_EPSILON;
                let h01 = get_height(x, y + cell_scale) + DECAL_EPSILON;
                let h11 = get_height(x + cell_scale, y + cell_scale) + DECAL_EPSILON;

                let v00 = Vec3::new(x, y, h00);
                let v10 = Vec3::new(x + cell_scale, y, h10);
                let v01 = Vec3::new(x, y + cell_scale, h01);
                let v11 = Vec3::new(x + cell_scale, y + cell_scale, h11);

                // Triangle 1
                let t1 = [v00, v10, v11];
                // C++ checks if face is pointing towards projection direction
                let normal1 = (v10 - v00).cross(v11 - v00).normalize();
                if self.normal.dot(normal1) > DECAL_EPSILON {
                    let clipped1 = self.clip_polygon(&t1);
                    self.triangulate_and_add(&clipped1, &mut out_vertices);
                }

                // Triangle 2
                let t2 = [v00, v11, v01];
                let normal2 = (v11 - v00).cross(v01 - v00).normalize();
                if self.normal.dot(normal2) > DECAL_EPSILON {
                    let clipped2 = self.clip_polygon(&t2);
                    self.triangulate_and_add(&clipped2, &mut out_vertices);
                }

                x += cell_scale;
            }
            y += cell_scale;
        }

        out_vertices
    }

    fn triangulate_and_add(&self, poly: &[Vec3], out_vertices: &mut Vec<DecalVertex>) {
        if poly.len() < 3 {
            return;
        }
        
        let inv_w = 1.0 / self.width;
        let inv_h = 1.0 / self.height;

        let get_uv = |v: Vec3| -> [f32; 2] {
            let diff = v - self.center;
            let u = -diff.dot(self.binormal) * inv_h + 0.5;
            let v_coord = -diff.dot(self.tangent) * inv_w + 0.5;
            [u, v_coord]
        };

        for i in 2..poly.len() {
            out_vertices.push(DecalVertex {
                position: poly[0].into(),
                tex_coord: get_uv(poly[0]),
            });
            out_vertices.push(DecalVertex {
                position: poly[i-1].into(),
                tex_coord: get_uv(poly[i-1]),
            });
            out_vertices.push(DecalVertex {
                position: poly[i].into(),
                tex_coord: get_uv(poly[i]),
            });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn multiply_vector(&self, v: &Vector4) -> Vector4 {
        let x = self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z + self.m[0][3] * v.w;
        let y = self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z + self.m[1][3] * v.w;
        let z = self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z + self.m[2][3] * v.w;
        let w = self.m[3][0] * v.x + self.m[3][1] * v.y + self.m[3][2] * v.z + self.m[3][3] * v.w;
        Vector4 { x, y, z, w }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Empire {
    None = 0,
    EmpireA = 1,
    EmpireB = 2,
    EmpireC = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentGrade {
    Grade0 = 0,
    Grade1 = 1,
    Grade2 = 2,
    Grade3 = 3,
    Grade4 = 4, // TITLE_NONE
    Grade5 = 5,
    Grade6 = 6,
    Grade7 = 7,
    Grade8 = 8,
}

impl AlignmentGrade {
    pub fn from_alignment(alignment: i32) -> Self {
        if alignment >= 12000 { AlignmentGrade::Grade0 }
        else if alignment >= 8000 { AlignmentGrade::Grade1 }
        else if alignment >= 4000 { AlignmentGrade::Grade2 }
        else if alignment >= 1000 { AlignmentGrade::Grade3 }
        else if alignment >= 0 { AlignmentGrade::Grade4 }
        else if alignment > -4000 { AlignmentGrade::Grade5 }
        else if alignment > -8000 { AlignmentGrade::Grade6 }
        else if alignment > -12000 { AlignmentGrade::Grade7 }
        else { AlignmentGrade::Grade8 }
    }

    pub fn to_title_string(&self) -> Option<String> {
        match self {
            AlignmentGrade::Grade0 => Some("Chivalric".to_string()),
            AlignmentGrade::Grade1 => Some("Noble".to_string()),
            AlignmentGrade::Grade2 => Some("Good".to_string()),
            AlignmentGrade::Grade3 => Some("Friendly".to_string()),
            AlignmentGrade::Grade4 => None,
            AlignmentGrade::Grade5 => Some("Aggressive".to_string()),
            AlignmentGrade::Grade6 => Some("Fraudulent".to_string()),
            AlignmentGrade::Grade7 => Some("Malicious".to_string()),
            AlignmentGrade::Grade8 => Some("Cruel".to_string()),
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            AlignmentGrade::Grade0 | AlignmentGrade::Grade1 | AlignmentGrade::Grade2 | AlignmentGrade::Grade3 => {
                Color::new(0.0, 0.0, 1.0, 1.0) // Niebieski dla dodatnich (wysokich) rang
            },
            AlignmentGrade::Grade4 => {
                Color::new(1.0, 1.0, 1.0, 1.0) // Bialy dla neutralnej rangi
            },
            AlignmentGrade::Grade5 | AlignmentGrade::Grade6 | AlignmentGrade::Grade7 | AlignmentGrade::Grade8 => {
                Color::new(0.6, 0.3, 0.0, 1.0) // Brazowy dla ujemnych (niskich) rang
            },
        }
    }
}

pub fn get_name_color(player_empire: Empire, target_empire: Empire, target_alignment: &AlignmentGrade) -> Color {
    // Jezeli wrogie krolestwo, to kolor czerwony
    if player_empire != target_empire && player_empire != Empire::None && target_empire != Empire::None {
        return Color::new(1.0, 0.0, 0.0, 1.0);
    }
    
    // W przeciwnym wypadku kolor wedlug rangi
    target_alignment.get_color()
}

#[derive(Debug)]
pub struct TextTail {
    pub name_string: String,
    pub guild_name: Option<String>,
    pub title_string: Option<String>,
    pub hp_bar_pct: Option<u8>,
    pub position: Vector3,
    pub height: f32,
    pub name_color: Color,
    pub title_color: Color,
}

impl TextTail {
    pub fn new(
        name: String,
        guild: Option<String>,
        alignment: i32,
        player_empire: Empire,
        target_empire: Empire,
        position: Vector3,
        height: f32,
        hp_pct: u8,
    ) -> Self {
        let grade = AlignmentGrade::from_alignment(alignment);
        let title_string = grade.to_title_string();
        let name_color = get_name_color(player_empire, target_empire, &grade);
        let title_color = grade.get_color();
        
        // Jezeli hp wynosi co najmniej 100%, to nie pokazujemy paska hp
        let hp_bar_pct = if hp_pct >= 100 {
            None
        } else {
            Some(hp_pct)
        };
        
        Self {
            name_string: name,
            guild_name: guild,
            title_string,
            hp_bar_pct,
            position,
            height,
            name_color,
            title_color,
        }
    }
    
    pub fn transform_to_screen(
        &self,
        view_proj: &Matrix4,
        screen_width: f32,
        screen_height: f32,
    ) -> Option<(f32, f32)> {
        let world_pos = Vector4 {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z + self.height,
            w: 1.0,
        };
        
        let clip_coords = view_proj.multiply_vector(&world_pos);
        
        // Wartosc `w` mniejsza badz rowna 0 oznacza ze punkt znajduje sie za kamera
        if clip_coords.w <= 0.0 {
            return None;
        }
        
        let ndc_x = clip_coords.x / clip_coords.w;
        let ndc_y = clip_coords.y / clip_coords.w;
        
        // Zrzut z NDC [-1, 1] do wspolrzednych ekranu
        // Zakladamy iz Y rosnie w dol ekranu:
        let screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
        let screen_y = (1.0 - ndc_y) * 0.5 * screen_height;
        
        Some((screen_x, screen_y))
    }
}

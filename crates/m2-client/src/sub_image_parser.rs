#[derive(Debug, PartialEq)]
pub struct SubImage {
    pub version: Option<String>,
    pub image: String,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl SubImage {
    pub fn parse(text: &str) -> Result<SubImage, &'static str> {
        let mut title_found = false;
        let mut version = None;
        let mut image = None;
        let mut left = None;
        let mut top = None;
        let mut right = None;
        let mut bottom = None;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.splitn(2, |c: char| c.is_whitespace());
            let key = parts.next().unwrap().to_lowercase();
            let value = parts.next().map(|s| s.trim()).unwrap_or("");

            match key.as_str() {
                "title" => {
                    if value.to_lowercase() != "subimage" {
                        return Err("invalid title");
                    }
                    title_found = true;
                }
                "version" => {
                    version = Some(value.to_string());
                }
                "image" => {
                    let clean = value.trim().trim_matches('"').trim();
                    image = Some(clean.to_string());
                }
                "left" => {
                    left = Some(value.parse::<i32>().map_err(|_| "invalid left value")?);
                }
                "top" => {
                    top = Some(value.parse::<i32>().map_err(|_| "invalid top value")?);
                }
                "right" => {
                    right = Some(value.parse::<i32>().map_err(|_| "invalid right value")?);
                }
                "bottom" => {
                    bottom = Some(value.parse::<i32>().map_err(|_| "invalid bottom value")?);
                }
                _ => {}
            }
        }

        if !title_found {
            return Err("missing title");
        }

        Ok(SubImage {
            version,
            image: image.ok_or("missing image")?,
            left: left.ok_or("missing left")?,
            top: top.ok_or("missing top")?,
            right: right.ok_or("missing right")?,
            bottom: bottom.ok_or("missing bottom")?,
        })
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn get_uv(&self, tex_w: u32, tex_h: u32) -> (f32, f32, f32, f32) {
        if tex_w == 0 || tex_h == 0 {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let u0 = self.left as f32 / tex_w as f32;
        let v0 = self.top as f32 / tex_h as f32;
        let u1 = self.right as f32 / tex_w as f32;
        let v1 = self.bottom as f32 / tex_h as f32;
        (u0, v0, u1, v1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BannerType {
    Notice,
    BigNotice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationState {
    FadeIn,
    Display,
    FadeOut,
    Finished,
}

#[derive(Debug, Clone)]
pub struct BannerAnnouncement {
    pub banner_type: BannerType,
    pub text: String,
    pub timestamp: u32,
    pub display_duration_ms: u32,
    pub opacity: f32,
    pub position_y: f32,
    pub state: AnimationState,
    pub fade_in_duration: u32,
    pub fade_out_duration: u32,
}

impl BannerAnnouncement {
    pub fn new(banner_type: BannerType, text: String, timestamp: u32, duration: u32) -> Self {
        let (fade_in_duration, fade_out_duration) = match banner_type {
            BannerType::Notice => (500, 500),
            BannerType::BigNotice => (1000, 1000),
        };
        
        Self {
            banner_type: banner_type.clone(),
            text,
            timestamp,
            display_duration_ms: duration,
            opacity: 0.0,
            position_y: if banner_type == BannerType::Notice { -20.0 } else { 0.0 },
            state: AnimationState::FadeIn,
            fade_in_duration,
            fade_out_duration,
        }
    }

    pub fn is_expired(&self, current_timestamp: u32) -> bool {
        self.state == AnimationState::Finished || current_timestamp >= self.timestamp + self.display_duration_ms + self.fade_in_duration + self.fade_out_duration
    }
    
    pub fn update(&mut self, current_timestamp: u32) {
        let elapsed = current_timestamp.saturating_sub(self.timestamp);
        
        match self.state {
            AnimationState::FadeIn => {
                if elapsed >= self.fade_in_duration {
                    self.opacity = 1.0;
                    self.position_y = if self.banner_type == BannerType::Notice { 0.0 } else { 0.0 };
                    self.state = AnimationState::Display;
                } else {
                    let progress = elapsed as f32 / self.fade_in_duration as f32;
                    self.opacity = progress;
                    if self.banner_type == BannerType::Notice {
                        self.position_y = -20.0 * (1.0 - progress);
                    }
                }
            }
            AnimationState::Display => {
                if elapsed >= self.fade_in_duration + self.display_duration_ms {
                    self.state = AnimationState::FadeOut;
                }
            }
            AnimationState::FadeOut => {
                let fade_out_start = self.fade_in_duration + self.display_duration_ms;
                if elapsed >= fade_out_start + self.fade_out_duration {
                    self.opacity = 0.0;
                    self.state = AnimationState::Finished;
                } else {
                    let progress = (elapsed - fade_out_start) as f32 / self.fade_out_duration as f32;
                    self.opacity = 1.0 - progress;
                    if self.banner_type == BannerType::Notice {
                        self.position_y = -20.0 * progress;
                    }
                }
            }
            AnimationState::Finished => {}
        }
    }
}

pub struct NoticeBannerSystem {
    pub active_banners: Vec<BannerAnnouncement>,
    pub notice_queue: std::collections::VecDeque<BannerAnnouncement>,
    pub big_notice_queue: std::collections::VecDeque<BannerAnnouncement>,
    pub max_active_notices: usize,
    pub max_active_big_notices: usize,
}

impl NoticeBannerSystem {
    pub fn new() -> Self {
        Self {
            active_banners: Vec::new(),
            notice_queue: std::collections::VecDeque::new(),
            big_notice_queue: std::collections::VecDeque::new(),
            max_active_notices: 5,
            max_active_big_notices: 1,
        }
    }

    pub fn append_notice(&mut self, text: String, timestamp: u32) {
        self.notice_queue.push_back(BannerAnnouncement::new(
            BannerType::Notice,
            text,
            timestamp,
            5000,
        ));
        self.update_active_banners(timestamp);
    }

    pub fn append_big_notice(&mut self, text: String, timestamp: u32) {
        self.big_notice_queue.push_back(BannerAnnouncement::new(
            BannerType::BigNotice,
            text,
            timestamp,
            7000,
        ));
        self.update_active_banners(timestamp);
    }

    pub fn update(&mut self, current_timestamp: u32) {
        for banner in &mut self.active_banners {
            banner.update(current_timestamp);
        }
        self.active_banners.retain(|b| !b.is_expired(current_timestamp));
        self.update_active_banners(current_timestamp);
    }

    pub fn clear(&mut self) {
        self.active_banners.clear();
        self.notice_queue.clear();
        self.big_notice_queue.clear();
    }
    
    pub fn get_active_notices(&self) -> Vec<&BannerAnnouncement> {
        self.active_banners.iter().filter(|b| b.banner_type == BannerType::Notice).collect()
    }
    
    pub fn get_active_big_notices(&self) -> Vec<&BannerAnnouncement> {
        self.active_banners.iter().filter(|b| b.banner_type == BannerType::BigNotice).collect()
    }

    fn update_active_banners(&mut self, current_timestamp: u32) {
        // Process standard notices
        while !self.notice_queue.is_empty() {
            let current_count = self.active_banners.iter().filter(|b| b.banner_type == BannerType::Notice).count();
            if current_count >= self.max_active_notices {
                break;
            }
            if let Some(mut banner) = self.notice_queue.pop_front() {
                banner.timestamp = current_timestamp;
                self.active_banners.push(banner);
            }
        }
        
        // Process big notices
        while !self.big_notice_queue.is_empty() {
            let current_count = self.active_banners.iter().filter(|b| b.banner_type == BannerType::BigNotice).count();
            if current_count >= self.max_active_big_notices {
                break;
            }
            if let Some(mut banner) = self.big_notice_queue.pop_front() {
                banner.timestamp = current_timestamp;
                self.active_banners.push(banner);
            }
        }
    }
}

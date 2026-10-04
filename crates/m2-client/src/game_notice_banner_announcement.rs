#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BannerType {
    Notice,
    BigNotice,
}

#[derive(Debug, Clone)]
pub struct BannerAnnouncement {
    pub banner_type: BannerType,
    pub text: String,
    pub timestamp: u32,
    pub display_duration_ms: u32,
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
        self.notice_queue.push_back(BannerAnnouncement {
            banner_type: BannerType::Notice,
            text,
            timestamp,
            display_duration_ms: 5000, // 5 seconds by default
        });
        self.update_active_banners(timestamp);
    }

    pub fn append_big_notice(&mut self, text: String, timestamp: u32) {
        self.big_notice_queue.push_back(BannerAnnouncement {
            banner_type: BannerType::BigNotice,
            text,
            timestamp,
            display_duration_ms: 7000, // 7 seconds by default
        });
        self.update_active_banners(timestamp);
    }

    pub fn update(&mut self, current_timestamp: u32) {
        self.active_banners.retain(|b| current_timestamp < b.timestamp + b.display_duration_ms);
        self.update_active_banners(current_timestamp);
    }

    fn update_active_banners(&mut self, current_timestamp: u32) {
        // Process standard notices
        while let Some(_) = self.notice_queue.front() {
            let current_count = self.active_banners.iter().filter(|b| b.banner_type == BannerType::Notice).count();
            if current_count < self.max_active_notices {
                if let Some(mut banner) = self.notice_queue.pop_front() {
                    banner.timestamp = current_timestamp;
                    self.active_banners.push(banner);
                }
            } else {
                break;
            }
        }
        
        // Process big notices
        while let Some(_) = self.big_notice_queue.front() {
            let current_count = self.active_banners.iter().filter(|b| b.banner_type == BannerType::BigNotice).count();
            if current_count < self.max_active_big_notices {
                if let Some(mut banner) = self.big_notice_queue.pop_front() {
                    banner.timestamp = current_timestamp;
                    self.active_banners.push(banner);
                }
            } else {
                break;
            }
        }
    }
}

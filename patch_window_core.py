import re

with open("crates/m2-client/src/ui_window_core.rs", "r") as f:
    text = f.read()

struct_patch = """
    pub slot_window: Option<crate::ui_widget_slotwindow::SlotWindow>,
    pub py_handler: usize,
    pub is_flag_pick_always: bool,
    pub flags: u32,
    pub clip_rect: Option<[f32; 4]>,
    pub scissor_enabled: bool,
    
    // Geometry extensions
    pub show_cursor: bool,
    pub stored_position: Option<(i32, i32)>,
}
"""

text = re.sub(
    r'pub slot_window: Option<crate::ui_widget_slotwindow::SlotWindow>,\s*pub py_handler: usize,\s*pub is_flag_pick_always: bool,\s*pub flags: u32,\s*pub clip_rect: Option<\[f32; 4\]>,\s*pub scissor_enabled: bool,\s*}',
    struct_patch.strip(),
    text
)

impl_patch = """
            is_hover: false,
            slot_window: None,
            py_handler: 0,
            is_flag_pick_always: false,
            flags: 0,
            clip_rect: None,
            scissor_enabled: false,
            show_cursor: false,
            stored_position: None,
        }
"""

text = re.sub(
    r'is_hover: false,\s*slot_window: None,\s*py_handler: 0,\s*is_flag_pick_always: false,\s*flags: 0,\s*clip_rect: None,\s*scissor_enabled: false,\s*}',
    impl_patch.strip(),
    text
)

with open("crates/m2-client/src/ui_window_core.rs", "w") as f:
    f.write(text)

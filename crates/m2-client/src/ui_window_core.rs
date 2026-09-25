//! Modul `ui_window_core` — Klasa bazowa elementu interfejsu UI::CWindow
//! Wzorzec referencyjny C++: `source/EterPythonLib/PythonWindow.cpp`
//! Opis: Odwzorowanie UI::CWindow: hierarchia rodzic-dzieci, pozycja lokalna i globalna (x, y, width, height), widocznosc, flagi (MOVABLE, LIMIT), test trafienia kursorem IsIn(mx, my).

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlign {
    Left = 0,
    Center = 1,
    Right = 2,
}

impl Default for HorizontalAlign {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlign {
    Top = 0,
    Center = 1,
    Bottom = 2,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub mod window_flags {
    pub const MOVABLE: u32 = 1 << 0;
    pub const LIMIT: u32 = 1 << 1;
    pub const SNAP: u32 = 1 << 2;
    pub const DRAGABLE: u32 = 1 << 3;
    pub const ATTACH: u32 = 1 << 4;
    pub const RESTRICT_X: u32 = 1 << 5;
    pub const RESTRICT_Y: u32 = 1 << 6;
    pub const NOT_CAPTURE: u32 = 1 << 7;
    pub const FLOAT: u32 = 1 << 8;
    pub const NOT_PICK: u32 = 1 << 9;
    pub const IGNORE_SIZE: u32 = 1 << 10;
    pub const RTL: u32 = 1 << 11;
}

#[derive(Debug, Clone)]
pub struct UiWindowCore {
    pub initialized: bool,
    
    // Pola przeniesione z C++
    pub name: String,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub rect: Rect,
    pub limit_bias_rect: Rect,
    
    pub is_movable: bool,
    pub is_show: bool,
    pub flags: u32,
    
    pub parent: Option<Weak<RefCell<UiWindowCore>>>,
    pub children: Vec<Rc<RefCell<UiWindowCore>>>,
    
    pub is_updating_children: bool,
    pub reserve_child_list: Vec<Rc<RefCell<UiWindowCore>>>,
}

impl Default for UiWindowCore {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindowCore {
    pub fn new() -> Self {
        Self {
            initialized: true,
            name: String::new(),
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Top,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            rect: Rect::default(),
            limit_bias_rect: Rect::default(),
            is_movable: false,
            is_show: false,
            flags: 0,
            parent: None,
            children: Vec::new(),
            is_updating_children: false,
            reserve_child_list: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    // --- Gettery i Settery ---
    
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.rect.right = self.rect.left + self.width;
        self.rect.bottom = self.rect.top + self.height;
    }
    
    pub fn get_width(&self) -> i32 {
        self.width
    }
    
    pub fn get_height(&self) -> i32 {
        self.height
    }
    
    pub fn set_horizontal_align(&mut self, align: HorizontalAlign) {
        self.horizontal_align = align;
        // Oryginalny kod C++ wywoluje UpdateRect natychmiast, ale my to odkladamy.
        // Metoda update_rect zrobi to w odpowiednim kontekscie hierarchii.
    }
    
    pub fn set_vertical_align(&mut self, align: VerticalAlign) {
        self.vertical_align = align;
    }
    
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    
    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    pub fn get_position_x(&self) -> i32 {
        self.x
    }
    
    pub fn get_position_y(&self) -> i32 {
        self.y
    }
    
    pub fn get_rect(&self) -> Rect {
        self.rect
    }
    
    pub fn get_local_position(&self, mut x: i32, mut y: i32) -> (i32, i32) {
        x -= self.rect.left;
        y -= self.rect.top;
        (x, y)
    }
    
    // -- Margines Ograniczen --
    
    pub fn set_limit_bias(&mut self, left: i32, right: i32, top: i32, bottom: i32) {
        self.limit_bias_rect.left = left;
        self.limit_bias_rect.right = right;
        self.limit_bias_rect.top = top;
        self.limit_bias_rect.bottom = bottom;
    }
    
    pub fn get_limit_bias(&self) -> Rect {
        self.limit_bias_rect
    }
    
    // --- Widocznosc ---
    
    pub fn show(&mut self) {
        self.is_show = true;
    }
    
    pub fn hide(&mut self) {
        self.is_show = false;
    }
    
    pub fn is_show(&self) -> bool {
        self.is_show
    }
    
    // --- Flagi ---
    
    pub fn add_flag(&mut self, flag: u32) {
        self.flags |= flag;
    }
    
    pub fn remove_flag(&mut self, flag: u32) {
        self.flags &= !flag;
    }
    
    pub fn is_flag(&self, flag: u32) -> bool {
        (self.flags & flag) != 0
    }
    
    // --- Test Trafienia ---
    
    pub fn is_in(&self, x: i32, y: i32) -> bool {
        if x >= self.rect.left && x <= self.rect.right {
            if y >= self.rect.top && y <= self.rect.bottom {
                return true;
            }
        }
        false
    }
    
    // --- Relacje Rodzic/Dziecko ---
    
    pub fn add_child(&mut self, child: Rc<RefCell<UiWindowCore>>, parent_weak: Weak<RefCell<UiWindowCore>>) {
        child.borrow_mut().parent = Some(parent_weak);
        self.children.push(child);
    }
    
    pub fn get_parent(&self) -> Option<Rc<RefCell<UiWindowCore>>> {
        self.parent.as_ref().and_then(|weak| weak.upgrade())
    }
    
    pub fn has_parent(&self) -> bool {
        self.parent.as_ref().and_then(|weak| weak.upgrade()).is_some()
    }
    
    pub fn has_child(&self) -> bool {
        !self.children.is_empty()
    }
    
    pub fn get_child_count(&self) -> usize {
        self.children.len()
    }
    
    pub fn delete_child(&mut self, child: Rc<RefCell<UiWindowCore>>) {
        if self.is_updating_children {
            self.reserve_child_list.push(child);
        } else {
            self.children.retain(|c| !Rc::ptr_eq(c, &child));
        }
    }
    
    pub fn remove_reserve_children(&mut self) {
        if self.reserve_child_list.is_empty() {
            return;
        }
        
        let reserves = std::mem::take(&mut self.reserve_child_list);
        for res in reserves {
            self.children.retain(|c| !Rc::ptr_eq(c, &res));
        }
    }
    
    // --- Aktualizacja Globalnego Prostokata ---
    
    pub fn update_rect(&mut self) {
        self.rect.top = self.y;
        
        if let Some(parent_rc) = self.get_parent() {
            let parent = parent_rc.borrow();
            
            match self.vertical_align {
                VerticalAlign::Bottom => {
                    self.rect.top = parent.get_height() - self.rect.top;
                }
                VerticalAlign::Center => {
                    self.rect.top = (parent.get_height() - self.get_height()) / 2 + self.rect.top;
                }
                VerticalAlign::Top => {}
            }
            self.rect.top += parent.rect.top;
            self.rect.bottom = self.rect.top + self.height;
            
            if parent.is_flag(window_flags::RTL) {
                self.rect.left = parent.get_width() - self.width - self.x;
                match self.horizontal_align {
                    HorizontalAlign::Right => {
                        self.rect.left = -self.x;
                    }
                    HorizontalAlign::Center => {
                        self.rect.left = parent.get_width() / 2 - self.get_width() - self.x;
                    }
                    HorizontalAlign::Left => {}
                }
                self.rect.left += parent.rect.left;
                self.rect.right = self.rect.left + self.width;
            } else {
                self.rect.left = self.x;
                match self.horizontal_align {
                    HorizontalAlign::Right => {
                        self.rect.left = parent.get_width() - self.rect.left;
                    }
                    HorizontalAlign::Center => {
                        self.rect.left = (parent.get_width() - self.get_width()) / 2 + self.rect.left;
                    }
                    HorizontalAlign::Left => {}
                }
                self.rect.left += parent.rect.left;
                self.rect.right = self.rect.left + self.width;
            }
            
        } else {
            self.rect.bottom = self.rect.top + self.height;
            self.rect.left = self.x;
            self.rect.right = self.rect.left + self.width;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_window_core() {
        let instance = UiWindowCore::new();
        assert!(instance.is_ready());
    }
}

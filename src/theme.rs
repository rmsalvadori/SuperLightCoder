use egui::Color32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Clone)]
pub struct ThemeColors {
    pub bg_editor: Color32,
    pub bg_bottom_bar: Color32,
    pub text_editor: Color32,
    pub text_gutter: Color32,
    pub text_gutter_active: Color32,
    pub text_bottom_bar: Color32,
    pub bottom_bar_border: Color32,
    pub selection_bg: Color32,
    pub cursor_color: Color32,
    
    // Syntax highlight colors
    pub syn_keyword: Color32,
    pub syn_type: Color32,
    pub syn_string: Color32,
    pub syn_number: Color32,
    pub syn_comment: Color32,
    pub syn_function: Color32,
    pub syn_operator: Color32,
    pub syn_tag: Color32,
    pub syn_attribute: Color32,
    pub syn_punctuation: Color32,
}

impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            bg_editor: Color32::from_rgb(30, 30, 30),
            bg_bottom_bar: Color32::from_rgb(40, 40, 40),
            text_editor: Color32::from_rgb(220, 220, 220),
            text_gutter: Color32::from_rgb(110, 110, 110),
            text_gutter_active: Color32::from_rgb(240, 240, 240),
            text_bottom_bar: Color32::from_rgb(200, 200, 200),
            bottom_bar_border: Color32::from_rgb(55, 55, 55),
            selection_bg: Color32::from_rgba_premultiplied(38, 79, 120, 180),
            cursor_color: Color32::from_rgb(255, 255, 255),

            syn_keyword: Color32::from_rgb(86, 156, 214),     // Light Blue
            syn_type: Color32::from_rgb(78, 201, 176),        // Teal / Mint
            syn_string: Color32::from_rgb(206, 145, 120),     // Soft Peach / Orange
            syn_number: Color32::from_rgb(181, 206, 168),     // Light Green
            syn_comment: Color32::from_rgb(106, 153, 85),     // Olive Green
            syn_function: Color32::from_rgb(220, 220, 170),   // Light Yellow
            syn_operator: Color32::from_rgb(212, 212, 212),   // Off-white
            syn_tag: Color32::from_rgb(86, 156, 214),         // Blue
            syn_attribute: Color32::from_rgb(156, 220, 254),   // Cyan
            syn_punctuation: Color32::from_rgb(180, 180, 180), // Subtle Gray
        }
    }

    pub fn light() -> Self {
        Self {
            bg_editor: Color32::from_rgb(255, 255, 255),
            bg_bottom_bar: Color32::from_rgb(238, 238, 238),
            text_editor: Color32::from_rgb(20, 20, 20),
            text_gutter: Color32::from_rgb(140, 140, 140),
            text_gutter_active: Color32::from_rgb(20, 20, 20),
            text_bottom_bar: Color32::from_rgb(60, 60, 60),
            bottom_bar_border: Color32::from_rgb(215, 215, 215),
            selection_bg: Color32::from_rgba_premultiplied(173, 214, 255, 180),
            cursor_color: Color32::from_rgb(0, 0, 0),

            syn_keyword: Color32::from_rgb(0, 0, 255),        // Classic Blue
            syn_type: Color32::from_rgb(38, 127, 153),        // Teal
            syn_string: Color32::from_rgb(163, 21, 21),       // Crimson Red
            syn_number: Color32::from_rgb(9, 136, 90),        // Dark Green
            syn_comment: Color32::from_rgb(0, 128, 0),         // Green
            syn_function: Color32::from_rgb(121, 94, 38),     // Warm Ochre
            syn_operator: Color32::from_rgb(40, 40, 40),      // Dark Gray
            syn_tag: Color32::from_rgb(128, 0, 0),            // Dark Red
            syn_attribute: Color32::from_rgb(255, 0, 0),      // Red
            syn_punctuation: Color32::from_rgb(80, 80, 80),   // Medium Gray
        }
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }
}

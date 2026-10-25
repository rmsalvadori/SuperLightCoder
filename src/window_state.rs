use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub maximized: bool,
    pub last_file: Option<PathBuf>,
    pub word_wrap: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 980.0,
            height: 680.0,
            maximized: false,
            last_file: None,
            word_wrap: false,
        }
    }
}

fn config_dir() -> Option<PathBuf> {
    let mut path = std::env::var_os("LOCALAPPDATA").map(PathBuf::from)?;
    path.push("SuperLightCoder");
    Some(path)
}

pub fn load_window_state() -> WindowConfig {
    let mut config = WindowConfig::default();
    if let Some(mut path) = config_dir() {
        path.push("window_state.txt");
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "width" => {
                            if let Ok(w) = parts[1].parse::<f32>() {
                                if w >= 550.0 {
                                    config.width = w;
                                }
                            }
                        }
                        "height" => {
                            if let Ok(h) = parts[1].parse::<f32>() {
                                if h >= 350.0 {
                                    config.height = h;
                                }
                            }
                        }
                        "maximized" => {
                            if let Ok(m) = parts[1].parse::<bool>() {
                                config.maximized = m;
                            }
                        }
                        "last_file" => {
                            if !parts[1].is_empty() {
                                let p = PathBuf::from(parts[1]);
                                if p.is_file() {
                                    config.last_file = Some(p);
                                }
                            }
                        }
                        "word_wrap" => {
                            if let Ok(b) = parts[1].parse::<bool>() {
                                config.word_wrap = b;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    config
}

pub fn save_window_state(config: &WindowConfig) {
    if let Some(dir) = config_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let mut path = dir;
        path.push("window_state.txt");
        let mut content = format!(
            "width={}\nheight={}\nmaximized={}\nword_wrap={}\n",
            config.width.round() as i32,
            config.height.round() as i32,
            config.maximized,
            config.word_wrap
        );
        if let Some(ref lf) = config.last_file {
            content.push_str(&format!("last_file={}\n", lf.to_string_lossy()));
        }
        let _ = std::fs::write(path, content);
    }
}

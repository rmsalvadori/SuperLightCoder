use eframe::egui;
use std::path::PathBuf;
use std::time::Instant;

use crate::formatter::format_code;
use crate::registry;
use crate::syntax::Language;
use crate::theme::{ThemeColors, ThemeMode};

#[derive(Clone, PartialEq, Eq)]
pub enum PendingAction {
    CloseWindow,
    NewFile,
    OpenFile,
    OpenPath(PathBuf),
}

#[derive(Clone, Debug)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

impl FileItem {
    pub fn scan_dir(dir: &std::path::Path) -> Vec<FileItem> {
        let mut items = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue;
                }
                let is_dir = path.is_dir();
                if is_dir {
                    dirs.push(FileItem {
                        name,
                        path,
                        is_dir: true,
                    });
                } else {
                    files.push(FileItem {
                        name,
                        path,
                        is_dir: false,
                    });
                }
            }

            dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            items.extend(dirs);
            items.extend(files);
        }
        items
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchMatch {
    pub byte_start: usize,
    pub byte_end: usize,
    pub char_start: usize,
    pub char_end: usize,
}

#[derive(Clone, Debug, Default)]
pub struct FindReplaceState {
    pub is_open: bool,
    pub show_replace: bool,
    pub find_query: String,
    pub replace_query: String,
    pub case_sensitive: bool,
    pub matches: Vec<SearchMatch>,
    pub current_match: usize,
    pub should_focus_find: bool,
}

impl FindReplaceState {
    pub fn open_find(&mut self) {
        self.is_open = true;
        self.should_focus_find = true;
    }

    pub fn open_replace(&mut self) {
        self.is_open = true;
        self.show_replace = true;
        self.should_focus_find = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn update_matches(&mut self, text: &str) {
        self.matches.clear();
        if self.find_query.is_empty() || text.is_empty() {
            self.current_match = 0;
            return;
        }

        let q_char_len = self.find_query.chars().count();

        if self.case_sensitive {
            let mut cur_char = 0;
            let mut prev_byte = 0;
            for (b_idx, _) in text.match_indices(&self.find_query) {
                cur_char += text[prev_byte..b_idx].chars().count();
                prev_byte = b_idx;
                self.matches.push(SearchMatch {
                    byte_start: b_idx,
                    byte_end: b_idx + self.find_query.len(),
                    char_start: cur_char,
                    char_end: cur_char + q_char_len,
                });
            }
        } else {
            let text_chars: Vec<char> = text.chars().collect();
            let query_chars: Vec<char> = self.find_query.to_lowercase().chars().collect();
            let q_len = query_chars.len();
            if text_chars.len() >= q_len {
                let mut byte_offset = 0;
                for i in 0..=(text_chars.len() - q_len) {
                    let matched = text_chars[i..i + q_len]
                        .iter()
                        .zip(query_chars.iter())
                        .all(|(tc, qc)| tc.to_lowercase().eq(std::iter::once(*qc)));
                    if matched {
                        let m_str: String = text_chars[i..i + q_len].iter().collect();
                        let b_start = byte_offset;
                        let b_end = byte_offset + m_str.len();
                        self.matches.push(SearchMatch {
                            byte_start: b_start,
                            byte_end: b_end,
                            char_start: i,
                            char_end: i + q_len,
                        });
                    }
                    byte_offset += text_chars[i].len_utf8();
                }
            }
        }

        if self.matches.is_empty() {
            self.current_match = 0;
        } else if self.current_match >= self.matches.len() {
            self.current_match = 0;
        }
    }

    pub fn next_match(&mut self) -> Option<SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        self.current_match = (self.current_match + 1) % self.matches.len();
        Some(self.matches[self.current_match])
    }

    pub fn prev_match(&mut self) -> Option<SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        if self.current_match == 0 {
            self.current_match = self.matches.len() - 1;
        } else {
            self.current_match -= 1;
        }
        Some(self.matches[self.current_match])
    }

    pub fn get_current_match(&self) -> Option<SearchMatch> {
        if self.matches.is_empty() {
            None
        } else {
            Some(self.matches[self.current_match])
        }
    }

    pub fn replace_current(&mut self, text: &mut String) -> bool {
        if self.matches.is_empty() {
            return false;
        }
        let m = self.matches[self.current_match];
        if m.byte_start <= text.len() && m.byte_end <= text.len() {
            text.replace_range(m.byte_start..m.byte_end, &self.replace_query);
            self.update_matches(text);
            true
        } else {
            false
        }
    }

    pub fn replace_all(&mut self, text: &mut String) -> usize {
        if self.matches.is_empty() {
            return 0;
        }
        let mut count = 0;
        for m in self.matches.iter().rev() {
            if m.byte_start <= text.len() && m.byte_end <= text.len() {
                text.replace_range(m.byte_start..m.byte_end, &self.replace_query);
                count += 1;
            }
        }
        self.update_matches(text);
        count
    }
}

#[derive(Clone, Debug, Default)]
pub struct MultiSelectionState {
    pub is_column_mode: bool,
    pub is_line_end_mode: bool, // Alt + Ctrl: cursor placed at the end of each selected line
    pub column_start_line: usize,
    pub column_end_line: usize,
    pub column_col: usize,
    pub free_cursors: Vec<(usize, usize)>, // (line, col)
}

impl MultiSelectionState {
    pub fn clear(&mut self) {
        self.is_column_mode = false;
        self.is_line_end_mode = false;
        self.column_start_line = 0;
        self.column_end_line = 0;
        self.column_col = 0;
        self.free_cursors.clear();
    }

    pub fn is_active(&self) -> bool {
        self.is_column_mode || !self.free_cursors.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct UndoSnapshot {
    pub text: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

#[derive(Default)]
pub struct HistoryState {
    pub undo_stack: Vec<UndoSnapshot>,
    pub redo_stack: Vec<UndoSnapshot>,
    pub base_snapshot: Option<UndoSnapshot>,
    pub last_edit_time: Option<Instant>,
    pub request_cursor_sync: bool,
}

impl HistoryState {
    pub fn push_undo(&mut self, text: String, line: usize, col: usize) {
        if let Some(last) = self.undo_stack.last() {
            if last.text == text {
                return;
            }
        }
        self.undo_stack.push(UndoSnapshot {
            text,
            cursor_line: line,
            cursor_col: col,
        });
        if self.undo_stack.len() > 300 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
        self.last_edit_time = Some(Instant::now());
    }

    pub fn undo(&mut self, current_text: &str, cur_line: usize, cur_col: usize) -> Option<UndoSnapshot> {
        if let Some(base) = self.base_snapshot.take() {
            if base.text != current_text {
                self.redo_stack.push(UndoSnapshot {
                    text: current_text.to_string(),
                    cursor_line: cur_line,
                    cursor_col: cur_col,
                });
                self.request_cursor_sync = true;
                self.last_edit_time = Some(Instant::now());
                return Some(base);
            }
        }
        let prev = self.undo_stack.pop()?;
        self.redo_stack.push(UndoSnapshot {
            text: current_text.to_string(),
            cursor_line: cur_line,
            cursor_col: cur_col,
        });
        self.base_snapshot = None;
        self.request_cursor_sync = true;
        self.last_edit_time = Some(Instant::now());
        Some(prev)
    }

    pub fn redo(&mut self, current_text: &str, cur_line: usize, cur_col: usize) -> Option<UndoSnapshot> {
        let next = self.redo_stack.pop()?;
        self.undo_stack.push(UndoSnapshot {
            text: current_text.to_string(),
            cursor_line: cur_line,
            cursor_col: cur_col,
        });
        self.base_snapshot = None;
        self.request_cursor_sync = true;
        self.last_edit_time = Some(Instant::now());
        Some(next)
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.base_snapshot = None;
        self.last_edit_time = None;
        self.request_cursor_sync = false;
    }
}

pub struct SuperLightCoderApp {
    pub text: String,
    pub file_path: Option<PathBuf>,
    pub is_modified: bool,
    pub language: Language,
    pub theme_mode: ThemeMode,
    pub font_size: f32,
    pub zoom_level: f32,
    pub crlf: bool,
    pub encoding: String,

    pub cursor_line: usize,
    pub cursor_col: usize,
    pub selection_count: Option<usize>,

    pub show_settings: bool,
    pub show_unsaved_dialog: bool,
    pub pending_action: Option<PendingAction>,

    pub show_sidebar: bool,
    pub current_folder: Option<PathBuf>,
    pub folder_items: Vec<FileItem>,

    pub find_replace: FindReplaceState,
    pub multi_sel: MultiSelectionState,
    pub history: HistoryState,
    pub request_select_all: bool,

    pub status_message: Option<(String, Instant)>,
    pub context_menu_active: bool,
    pub win_config: crate::window_state::WindowConfig,
}

impl SuperLightCoderApp {
    pub fn new(initial_file: Option<PathBuf>, win_config: crate::window_state::WindowConfig) -> Self {
        let current_folder = std::env::current_dir().ok();
        let folder_items = current_folder
            .as_ref()
            .map(|f| FileItem::scan_dir(f))
            .unwrap_or_default();

        let mut app = Self {
            text: String::new(),
            file_path: None,
            is_modified: false,
            language: Language::PlainText,
            theme_mode: ThemeMode::Dark,
            font_size: 14.0,
            zoom_level: 1.0,
            crlf: false,
            encoding: "UTF-8".to_string(),

            cursor_line: 1,
            cursor_col: 1,
            selection_count: None,

            show_settings: false,
            show_unsaved_dialog: false,
            pending_action: None,

            show_sidebar: false,
            current_folder,
            folder_items,

            find_replace: FindReplaceState::default(),
            multi_sel: MultiSelectionState::default(),
            history: HistoryState::default(),
            request_select_all: false,

            status_message: None,
            context_menu_active: registry::is_context_menu_registered(),
            win_config,
        };

        if let Some(path) = initial_file {
            app.load_file(path);
        }

        app
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some((msg.into(), Instant::now()));
    }

    pub fn load_file(&mut self, path: PathBuf) {
        match std::fs::read(&path) {
            Ok(bytes) => {
                let content = String::from_utf8_lossy(&bytes).to_string();
                self.crlf = content.contains("\r\n");
                self.text = content.replace("\r\n", "\n");
                self.language = Language::from_path(&path);
                self.file_path = Some(path.clone());
                self.is_modified = false;
                self.history.clear();
                self.set_status(format!("Arquivo aberto: {}", path.display()));
            }
            Err(e) => {
                self.set_status(format!("Erro ao abrir arquivo: {}", e));
            }
        }
    }

    pub fn save_undo_snapshot(&mut self) {
        self.history.push_undo(self.text.clone(), self.cursor_line, self.cursor_col);
        self.history.base_snapshot = None;
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.history.undo(&self.text, self.cursor_line, self.cursor_col) {
            self.text = prev.text;
            self.cursor_line = prev.cursor_line;
            self.cursor_col = prev.cursor_col;
            self.is_modified = true;
            if self.find_replace.is_open {
                self.find_replace.update_matches(&self.text);
            }
            self.set_status("Desfazer (Ctrl+Z)");
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.history.redo(&self.text, self.cursor_line, self.cursor_col) {
            self.text = next.text;
            self.cursor_line = next.cursor_line;
            self.cursor_col = next.cursor_col;
            self.is_modified = true;
            if self.find_replace.is_open {
                self.find_replace.update_matches(&self.text);
            }
            self.set_status("Refazer (Ctrl+Y)");
        }
    }

    pub fn request_new_file(&mut self) {
        if self.is_modified {
            self.pending_action = Some(PendingAction::NewFile);
            self.show_unsaved_dialog = true;
        } else {
            self.new_file();
        }
    }

    pub fn new_file(&mut self) {
        self.text.clear();
        self.file_path = None;
        self.is_modified = false;
        self.language = Language::PlainText;
        self.cursor_line = 1;
        self.cursor_col = 1;
        self.selection_count = None;
        self.history.clear();
        self.set_status("Novo arquivo criado");
    }

    pub fn request_open_file(&mut self) {
        if self.is_modified {
            self.pending_action = Some(PendingAction::OpenFile);
            self.show_unsaved_dialog = true;
        } else {
            self.open_file_dialog();
        }
    }

    pub fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.load_file(path);
        }
    }

    pub fn open_folder_dialog(&mut self) {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            self.folder_items = FileItem::scan_dir(&folder);
            self.current_folder = Some(folder.clone());
            self.show_sidebar = true;
            self.set_status(format!("Pasta aberta: {}", folder.display()));
        }
    }

    pub fn refresh_folder(&mut self) {
        if let Some(ref folder) = self.current_folder {
            self.folder_items = FileItem::scan_dir(folder);
        }
    }

    pub fn save_file(&mut self) {
        if let Some(path) = self.file_path.clone() {
            let to_write = if self.crlf {
                self.text.replace('\n', "\r\n")
            } else {
                self.text.clone()
            };

            match std::fs::write(&path, to_write) {
                Ok(_) => {
                    self.is_modified = false;
                    self.set_status(format!("Arquivo salvo: {}", path.display()));
                }
                Err(e) => {
                    self.set_status(format!("Erro ao salvar arquivo: {}", e));
                }
            }
        } else {
            self.save_file_as_dialog();
        }
    }

    pub fn save_file_as_dialog(&mut self) {
        let mut dialog = rfd::FileDialog::new();
        if let Some(path) = &self.file_path {
            if let Some(parent) = path.parent() {
                dialog = dialog.set_directory(parent);
            }
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                dialog = dialog.set_file_name(name);
            }
        }

        if let Some(new_path) = dialog.save_file() {
            let to_write = if self.crlf {
                self.text.replace('\n', "\r\n")
            } else {
                self.text.clone()
            };

            match std::fs::write(&new_path, to_write) {
                Ok(_) => {
                    self.language = Language::from_path(&new_path);
                    self.file_path = Some(new_path.clone());
                    self.is_modified = false;
                    self.set_status(format!("Arquivo salvo como: {}", new_path.display()));
                }
                Err(e) => {
                    self.set_status(format!("Erro ao salvar: {}", e));
                }
            }
        }
    }

    pub fn format_document(&mut self) {
        let before_len = self.text.len();
        let formatted = format_code(&self.text, self.language);
        if formatted != self.text {
            self.save_undo_snapshot();
            self.text = formatted;
            self.is_modified = true;
            self.set_status(format!(
                "Código {} formatado com sucesso! ({} caracteres)",
                self.language.name(),
                self.text.len()
            ));
        } else if before_len == 0 {
            self.set_status("O documento está vazio.");
        } else {
            self.set_status(format!("O código {} já está formatado.", self.language.name()));
        }
    }

    pub fn duplicate_line(&mut self) {
        let lines: Vec<&str> = self.text.split('\n').collect();
        if lines.is_empty() {
            return;
        }
        let target_idx = (self.cursor_line.max(1) - 1).min(lines.len() - 1);
        let line_to_dup = lines[target_idx].to_string();
        drop(lines);

        self.save_undo_snapshot();

        let mut cur_line = 0;
        let mut insert_pos = self.text.len();
        for (i, b) in self.text.bytes().enumerate() {
            if cur_line == target_idx && b == b'\n' {
                insert_pos = i + 1;
                break;
            }
            if b == b'\n' {
                cur_line += 1;
            }
        }

        if insert_pos < self.text.len() {
            self.text.insert_str(insert_pos, &format!("{}\n", line_to_dup));
        } else {
            if !self.text.is_empty() && !self.text.ends_with('\n') {
                self.text.push('\n');
            }
            self.text.push_str(&line_to_dup);
        }

        self.cursor_line += 1;
        self.is_modified = true;
        self.set_status("Linha duplicada (Ctrl+D)");
    }

    pub fn move_line_up(&mut self) {
        if self.cursor_line <= 1 {
            return;
        }
        let mut lines: Vec<String> = self.text.split('\n').map(|s| s.to_string()).collect();
        let idx = self.cursor_line - 1;
        if idx < lines.len() {
            self.save_undo_snapshot();
            lines.swap(idx, idx - 1);
            self.text = lines.join("\n");
            self.cursor_line -= 1;
            self.is_modified = true;
        }
    }

    pub fn move_line_down(&mut self) {
        let mut lines: Vec<String> = self.text.split('\n').map(|s| s.to_string()).collect();
        let idx = self.cursor_line - 1;
        if idx + 1 < lines.len() {
            self.save_undo_snapshot();
            lines.swap(idx, idx + 1);
            self.text = lines.join("\n");
            self.cursor_line += 1;
            self.is_modified = true;
        }
    }

    pub fn apply_multi_insert(&mut self, text_to_insert: &str) {
        if text_to_insert.is_empty() {
            return;
        }
        self.save_undo_snapshot();
        let mut lines: Vec<String> = self.text.split('\n').map(|s| s.to_string()).collect();

        if self.multi_sel.is_line_end_mode {
            let min_l = self.multi_sel.column_start_line.min(self.multi_sel.column_end_line).max(1);
            let max_l = self.multi_sel.column_start_line.max(self.multi_sel.column_end_line).min(lines.len());
            for l in min_l..=max_l {
                let line_idx = l - 1;
                if line_idx < lines.len() {
                    lines[line_idx].push_str(text_to_insert);
                }
            }
            self.text = lines.join("\n");
            self.is_modified = true;
        } else if self.multi_sel.is_column_mode {
            let min_l = self.multi_sel.column_start_line.min(self.multi_sel.column_end_line).max(1);
            let max_l = self.multi_sel.column_start_line.max(self.multi_sel.column_end_line).min(lines.len());
            let col = self.multi_sel.column_col.max(1);

            for l in min_l..=max_l {
                let line_idx = l - 1;
                if line_idx < lines.len() {
                    let mut line_chars: Vec<char> = lines[line_idx].chars().collect();
                    while line_chars.len() < col - 1 {
                        line_chars.push(' ');
                    }
                    let insert_at = (col - 1).min(line_chars.len());
                    for (offset, ch) in text_to_insert.chars().enumerate() {
                        line_chars.insert(insert_at + offset, ch);
                    }
                    lines[line_idx] = line_chars.into_iter().collect();
                }
            }
            self.multi_sel.column_col += text_to_insert.chars().count();
            self.text = lines.join("\n");
            self.is_modified = true;
        } else if !self.multi_sel.free_cursors.is_empty() {
            let mut cursors = self.multi_sel.free_cursors.clone();
            cursors.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

            for (l, c) in &cursors {
                let line_idx = l.max(&1) - 1;
                if line_idx < lines.len() {
                    let mut line_chars: Vec<char> = lines[line_idx].chars().collect();
                    while line_chars.len() < c - 1 {
                        line_chars.push(' ');
                    }
                    let insert_at = (c - 1).min(line_chars.len());
                    for (offset, ch) in text_to_insert.chars().enumerate() {
                        line_chars.insert(insert_at + offset, ch);
                    }
                    lines[line_idx] = line_chars.into_iter().collect();
                }
            }

            let added_chars = text_to_insert.chars().count();
            for cur in &mut self.multi_sel.free_cursors {
                cur.1 += added_chars;
            }
            self.text = lines.join("\n");
            self.is_modified = true;
        }
    }

    pub fn apply_multi_backspace(&mut self) {
        self.save_undo_snapshot();
        let mut lines: Vec<String> = self.text.split('\n').map(|s| s.to_string()).collect();

        if self.multi_sel.is_line_end_mode {
            let min_l = self.multi_sel.column_start_line.min(self.multi_sel.column_end_line).max(1);
            let max_l = self.multi_sel.column_start_line.max(self.multi_sel.column_end_line).min(lines.len());
            for l in min_l..=max_l {
                let line_idx = l - 1;
                if line_idx < lines.len() {
                    lines[line_idx].pop();
                }
            }
            self.text = lines.join("\n");
            self.is_modified = true;
        } else if self.multi_sel.is_column_mode && self.multi_sel.column_col > 1 {
            let min_l = self.multi_sel.column_start_line.min(self.multi_sel.column_end_line).max(1);
            let max_l = self.multi_sel.column_start_line.max(self.multi_sel.column_end_line).min(lines.len());
            let col = self.multi_sel.column_col;

            for l in min_l..=max_l {
                let line_idx = l - 1;
                if line_idx < lines.len() {
                    let mut line_chars: Vec<char> = lines[line_idx].chars().collect();
                    if line_chars.len() >= col - 1 && col - 1 > 0 {
                        line_chars.remove(col - 2);
                        lines[line_idx] = line_chars.into_iter().collect();
                    }
                }
            }
            self.multi_sel.column_col -= 1;
            self.text = lines.join("\n");
            self.is_modified = true;
        } else if !self.multi_sel.free_cursors.is_empty() {
            let mut cursors = self.multi_sel.free_cursors.clone();
            cursors.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

            for (l, c) in &cursors {
                let line_idx = l.max(&1) - 1;
                if line_idx < lines.len() && *c > 1 {
                    let mut line_chars: Vec<char> = lines[line_idx].chars().collect();
                    if line_chars.len() >= c - 1 {
                        line_chars.remove(c - 2);
                        lines[line_idx] = line_chars.into_iter().collect();
                    }
                }
            }
            for cur in &mut self.multi_sel.free_cursors {
                if cur.1 > 1 {
                    cur.1 -= 1;
                }
            }
            self.text = lines.join("\n");
            self.is_modified = true;
        }
    }

    pub fn apply_multi_delete(&mut self) {
        self.save_undo_snapshot();
        let mut lines: Vec<String> = self.text.split('\n').map(|s| s.to_string()).collect();

        if self.multi_sel.is_line_end_mode {
            // At the end of line, forward delete does nothing
        } else if self.multi_sel.is_column_mode {
            let min_l = self.multi_sel.column_start_line.min(self.multi_sel.column_end_line).max(1);
            let max_l = self.multi_sel.column_start_line.max(self.multi_sel.column_end_line).min(lines.len());
            let col = self.multi_sel.column_col;

            for l in min_l..=max_l {
                let line_idx = l - 1;
                if line_idx < lines.len() {
                    let mut line_chars: Vec<char> = lines[line_idx].chars().collect();
                    if col <= line_chars.len() {
                        line_chars.remove(col - 1);
                        lines[line_idx] = line_chars.into_iter().collect();
                    }
                }
            }
            self.text = lines.join("\n");
            self.is_modified = true;
        } else if !self.multi_sel.free_cursors.is_empty() {
            let mut cursors = self.multi_sel.free_cursors.clone();
            cursors.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

            for (l, c) in &cursors {
                let line_idx = l.max(&1) - 1;
                if line_idx < lines.len() {
                    let mut line_chars: Vec<char> = lines[line_idx].chars().collect();
                    if *c <= line_chars.len() {
                        line_chars.remove(*c - 1);
                        lines[line_idx] = line_chars.into_iter().collect();
                    }
                }
            }
            self.text = lines.join("\n");
            self.is_modified = true;
        }
    }

    fn execute_pending_action(&mut self, ctx: &egui::Context, action: Option<PendingAction>) {
        match action {
            Some(PendingAction::CloseWindow) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            Some(PendingAction::NewFile) => {
                self.new_file();
            }
            Some(PendingAction::OpenFile) => {
                self.open_file_dialog();
            }
            Some(PendingAction::OpenPath(path)) => {
                self.load_file(path);
            }
            None => {}
        }
    }

    pub fn request_open_path(&mut self, path: PathBuf) {
        if self.is_modified {
            self.pending_action = Some(PendingAction::OpenPath(path));
            self.show_unsaved_dialog = true;
        } else {
            self.load_file(path);
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            let ctrl = i.modifiers.command || i.modifiers.ctrl;
            let shift = i.modifiers.shift;
            let alt = i.modifiers.alt;

            // Ctrl + N: Novo
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::N) {
                self.request_new_file();
            }

            // Ctrl + O: Abrir
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::O) {
                self.request_open_file();
            }

            // Ctrl + Shift + S: Salvar Como
            if ctrl && shift && !alt && i.key_pressed(egui::Key::S) {
                self.save_file_as_dialog();
            }
            // Ctrl + S: Salvar
            else if ctrl && !shift && !alt && i.key_pressed(egui::Key::S) {
                self.save_file();
            }

            // Ctrl + Z: Desfazer
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::Z) {
                self.undo();
            }

            // Ctrl + Y ou Ctrl + Shift + Z: Refazer
            if (ctrl && !shift && !alt && i.key_pressed(egui::Key::Y))
                || (ctrl && shift && !alt && i.key_pressed(egui::Key::Z))
            {
                self.redo();
            }

            // Ctrl + D: Duplicar Linha
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::D) {
                self.duplicate_line();
            }

            // Mover Linha para Cima: Alt + Up ou Ctrl + Shift + Up
            if (alt && !ctrl && i.key_pressed(egui::Key::ArrowUp))
                || (ctrl && shift && i.key_pressed(egui::Key::ArrowUp))
            {
                self.move_line_up();
            }

            // Mover Linha para Baixo: Alt + Down ou Ctrl + Shift + Down
            if (alt && !ctrl && i.key_pressed(egui::Key::ArrowDown))
                || (ctrl && shift && i.key_pressed(egui::Key::ArrowDown))
            {
                self.move_line_down();
            }

            // Ctrl + F: Localizar texto (Find bar estilo VS Code)
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::F) {
                self.find_replace.open_find();
                self.find_replace.update_matches(&self.text);
            }

            // Ctrl + H: Localizar e Substituir (Find & Replace bar)
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::H) {
                self.find_replace.open_replace();
                self.find_replace.update_matches(&self.text);
            }

            // Ctrl + Shift + F ou Alt + Shift + F: Formatar código
            if (ctrl && shift && !alt && i.key_pressed(egui::Key::F))
                || (alt && shift && !ctrl && i.key_pressed(egui::Key::F))
            {
                self.format_document();
            }

            // Escape: Fechar busca ou limpar seleção múltipla
            if i.key_pressed(egui::Key::Escape) {
                if self.find_replace.is_open {
                    self.find_replace.close();
                }
                self.multi_sel.clear();
            }

            // Ctrl + ,: Abrir configurações
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::Comma) {
                self.show_settings = !self.show_settings;
            }

            // Ctrl + +: Zoom In
            if ctrl && (i.key_pressed(egui::Key::Equals) || i.key_pressed(egui::Key::Plus)) {
                self.zoom_level = (self.zoom_level + 0.1).min(3.0);
            }

            // Ctrl + -: Zoom Out
            if ctrl && i.key_pressed(egui::Key::Minus) {
                self.zoom_level = (self.zoom_level - 0.1).max(0.5);
            }

            // Ctrl + B: Alternar Barra Lateral do Explorador de Arquivos
            if ctrl && !shift && !alt && i.key_pressed(egui::Key::B) {
                self.show_sidebar = !self.show_sidebar;
                if self.show_sidebar && self.folder_items.is_empty() {
                    self.refresh_folder();
                }
            }

            // Ctrl + 0: Zoom Reset
            if ctrl && i.key_pressed(egui::Key::Num0) {
                self.zoom_level = 1.0;
            }
        });
    }

    fn render_find_replace_bar(&mut self, ctx: &egui::Context, colors: &ThemeColors) {
        if !self.find_replace.is_open {
            return;
        }

        let mut close_requested = false;
        let mut jump_to_match = false;

        egui::Area::new(egui::Id::new("find_replace_floating_bar"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-20.0, 10.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::window(ui.style())
                    .fill(colors.bg_bottom_bar)
                    .stroke(egui::Stroke::new(1.0_f32, colors.bottom_bar_border))
                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                    .rounding(6.0)
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 4.0),
                        blur: 8.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(100),
                    })
                    .show(ui, |ui| {
                        ui.set_max_width(380.0);

                        // Row 1: Find Row
                        ui.horizontal(|ui| {
                            let arrow = if self.find_replace.show_replace { "v" } else { ">" };
                            if ui
                                .add(egui::Button::new(arrow).frame(false))
                                .on_hover_text("Alternar Substituir (Ctrl+H)")
                                .clicked()
                            {
                                self.find_replace.show_replace = !self.find_replace.show_replace;
                            }

                            let mut find_text = self.find_replace.find_query.clone();
                            let text_response = ui.add(
                                egui::TextEdit::singleline(&mut find_text)
                                    .hint_text("Localizar...")
                                    .desired_width(140.0),
                            );

                            if self.find_replace.should_focus_find {
                                text_response.request_focus();
                                self.find_replace.should_focus_find = false;
                            }

                            if text_response.changed() {
                                self.find_replace.find_query = find_text;
                                self.find_replace.update_matches(&self.text);
                            }

                            // Enter inside find input jumps to next, Shift+Enter to prev
                            if text_response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                if ui.input(|i| i.modifiers.shift) {
                                    self.find_replace.prev_match();
                                } else {
                                    self.find_replace.next_match();
                                }
                                jump_to_match = true;
                            }

                            // Match counter
                            let match_label = if self.find_replace.find_query.is_empty() {
                                "".to_string()
                            } else if self.find_replace.matches.is_empty() {
                                "0/0".to_string()
                            } else {
                                format!(
                                    "{}/{}",
                                    self.find_replace.current_match + 1,
                                    self.find_replace.matches.len()
                                )
                            };
                            let match_color = if self.find_replace.find_query.is_empty()
                                || !self.find_replace.matches.is_empty()
                            {
                                colors.text_bottom_bar
                            } else {
                                egui::Color32::from_rgb(230, 80, 80)
                            };
                            ui.label(egui::RichText::new(match_label).color(match_color).size(12.0));

                            // Case sensitive toggle
                            let case_btn = egui::Button::new(
                                egui::RichText::new("Aa").strong().color(
                                    if self.find_replace.case_sensitive {
                                        colors.cursor_color
                                    } else {
                                        colors.text_bottom_bar
                                    },
                                ),
                            );
                            if ui
                                .add(case_btn)
                                .on_hover_text("Diferenciar maiúsculas de minúsculas (Alt+C)")
                                .clicked()
                            {
                                self.find_replace.case_sensitive = !self.find_replace.case_sensitive;
                                self.find_replace.update_matches(&self.text);
                            }

                            // Previous match button
                            if ui
                                .add(egui::Button::new("^").frame(false))
                                .on_hover_text("Ocorrência anterior (Shift+Enter)")
                                .clicked()
                            {
                                self.find_replace.prev_match();
                                jump_to_match = true;
                            }

                            // Next match button
                            if ui
                                .add(egui::Button::new("v").frame(false))
                                .on_hover_text("Próxima ocorrência (Enter)")
                                .clicked()
                            {
                                self.find_replace.next_match();
                                jump_to_match = true;
                            }

                            // Close button
                            if ui
                                .add(egui::Button::new("x").frame(false))
                                .on_hover_text("Fechar (Esc)")
                                .clicked()
                            {
                                close_requested = true;
                            }
                        });

                        // Row 2: Replace Row (when expanded)
                        if self.find_replace.show_replace {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add_space(18.0); // align with find input

                                let mut rep_text = self.find_replace.replace_query.clone();
                                let rep_resp = ui.add(
                                    egui::TextEdit::singleline(&mut rep_text)
                                        .hint_text("Substituir...")
                                        .desired_width(140.0),
                                );
                                if rep_resp.changed() {
                                    self.find_replace.replace_query = rep_text;
                                }

                                if ui
                                    .button("Substituir")
                                    .on_hover_text("Substituir ocorrência atual")
                                    .clicked()
                                {
                                    self.save_undo_snapshot();
                                    if self.find_replace.replace_current(&mut self.text) {
                                        self.is_modified = true;
                                        jump_to_match = true;
                                    }
                                }

                                if ui
                                    .button("Todos")
                                    .on_hover_text("Substituir todas as ocorrências")
                                    .clicked()
                                {
                                    self.save_undo_snapshot();
                                    let count = self.find_replace.replace_all(&mut self.text);
                                    if count > 0 {
                                        self.is_modified = true;
                                        self.set_status(format!("{} ocorrência(s) substituída(s)", count));
                                    }
                                }
                            });
                        }
                    });
            });

        if close_requested {
            self.find_replace.close();
        }

        if jump_to_match {
            if let Some(m) = self.find_replace.get_current_match() {
                let (line, col) = calculate_line_col(&self.text, m.char_start);
                self.cursor_line = line;
                self.cursor_col = col;
            }
        }
    }

    fn render_unsaved_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_unsaved_dialog {
            return;
        }

        let mut open = self.show_unsaved_dialog;
        let filename = self
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Sem Título")
            .to_string();

        egui::Window::new("Super Light Coder")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .movable(true)
            .default_width(380.0)
            .default_pos(ctx.screen_rect().center())
            .pivot(egui::Align2::CENTER_CENTER)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format!(
                        "Deseja salvar as alterações feitas em \"{}\"?",
                        filename
                    ))
                    .size(14.0),
                );
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Cancelar").clicked() {
                            self.show_unsaved_dialog = false;
                            self.pending_action = None;
                        }

                        if ui.button("Não Salvar").clicked() {
                            self.show_unsaved_dialog = false;
                            self.is_modified = false;
                            let action = self.pending_action.take();
                            self.execute_pending_action(ctx, action);
                        }

                        if ui.button("Salvar").clicked() {
                            self.save_file();
                            if !self.is_modified {
                                self.show_unsaved_dialog = false;
                                let action = self.pending_action.take();
                                self.execute_pending_action(ctx, action);
                            }
                        }
                    });
                });
                ui.add_space(4.0);
            });

        if !open {
            self.show_unsaved_dialog = false;
            self.pending_action = None;
        }
    }

    fn render_settings_window(&mut self, ctx: &egui::Context, colors: &ThemeColors) {
        if !self.show_settings {
            return;
        }

        let mut open = self.show_settings;
        egui::Window::new("⚙ Configurações - Super Light Coder")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .movable(true)
            .default_width(540.0)
            .max_width(560.0)
            .constrain(true)
            .default_pos(ctx.screen_rect().center())
            .pivot(egui::Align2::CENTER_CENTER)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(480.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(4.0);

                        // --- Windows Context Menu Section ---
                        egui::Frame::group(ui.style())
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Menu de Contexto do Windows")
                                        .strong()
                                        .size(14.0),
                                );
                                ui.add_space(4.0);

                                ui.horizontal(|ui| {
                                    ui.label("Status atual:");
                                    if self.context_menu_active {
                                        ui.label(
                                            egui::RichText::new("✓ Ativo ('Editar no Super Light Coder')")
                                                .color(egui::Color32::from_rgb(80, 210, 120))
                                                .strong(),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new("✗ Não registrado")
                                                .color(egui::Color32::from_rgb(230, 90, 90)),
                                        );
                                    }
                                });

                                ui.label(
                                    egui::RichText::new(
                                        "Adiciona 'Editar no Super Light Coder' com ícone exclusivo logo abaixo de 'Abrir' no Windows Explorer.",
                                    )
                                    .size(11.5)
                                    .color(colors.text_gutter),
                                );

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    if ui
                                        .button("+ Adicionar ao Menu de Contexto")
                                        .on_hover_text("Registra no Windows Explorer com ícone próprio")
                                        .clicked()
                                    {
                                        match registry::register_context_menu() {
                                            Ok(_) => {
                                                self.context_menu_active = true;
                                                self.set_status("Adicionado com sucesso ao menu de contexto!");
                                            }
                                            Err(err) => {
                                                self.set_status(format!("Falha ao registrar: {}", err));
                                            }
                                        }
                                    }

                                    if ui
                                        .button("- Remover do Menu de Contexto")
                                        .on_hover_text("Remove do Windows Explorer")
                                        .clicked()
                                    {
                                        match registry::unregister_context_menu() {
                                            Ok(_) => {
                                                self.context_menu_active = false;
                                                self.set_status("Removido do menu de contexto.");
                                            }
                                            Err(err) => {
                                                self.set_status(format!("Falha ao remover: {}", err));
                                            }
                                        }
                                    }
                                });
                            });

                        ui.add_space(8.0);

                        // --- Theme Section ---
                        egui::Frame::group(ui.style())
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Tema de Cores")
                                        .strong()
                                        .size(14.0),
                                );
                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    if ui
                                        .radio_value(&mut self.theme_mode, ThemeMode::Dark, "Tema Escuro (Dark)")
                                        .clicked()
                                    {
                                        self.set_status("Tema alterado para Escuro.");
                                    }
                                    ui.add_space(20.0);
                                    if ui
                                        .radio_value(&mut self.theme_mode, ThemeMode::Light, "Tema Claro (Light)")
                                        .clicked()
                                    {
                                        self.set_status("Tema alterado para Claro.");
                                    }
                                });
                            });

                        ui.add_space(8.0);

                        // --- Editor Settings Section ---
                        egui::Frame::group(ui.style())
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Preferências do Editor")
                                        .strong()
                                        .size(14.0),
                                );
                                ui.add_space(6.0);

                                ui.horizontal(|ui| {
                                    ui.label("Tamanho da Fonte:");
                                    ui.add(egui::Slider::new(&mut self.font_size, 10.0..=28.0).suffix(" px"));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Zoom do Editor:");
                                    let mut percent = (self.zoom_level * 100.0).round() as i32;
                                    if ui.add(egui::Slider::new(&mut percent, 50..=250).suffix("%")).changed() {
                                        self.zoom_level = percent as f32 / 100.0;
                                    }
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Fim de Linha padrão:");
                                    if ui.radio_value(&mut self.crlf, false, "Unix (LF)").clicked() {
                                        self.is_modified = true;
                                    }
                                    ui.add_space(10.0);
                                    if ui.radio_value(&mut self.crlf, true, "Windows (CRLF)").clicked() {
                                        self.is_modified = true;
                                    }
                                });
                            });

                        ui.add_space(8.0);

                        // --- Shortcuts Cheat Sheet ---
                        egui::Frame::group(ui.style())
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Teclas de Atalho")
                                        .strong()
                                        .size(14.0),
                                );
                                ui.add_space(6.0);
                                egui::Grid::new("shortcuts_grid")
                                    .striped(true)
                                    .min_col_width(140.0)
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("Ctrl + N").strong());
                                        ui.label("Novo arquivo");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + O").strong());
                                        ui.label("Abrir arquivo");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + S").strong());
                                        ui.label("Salvar");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + Shift + S").strong());
                                        ui.label("Salvar Como");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + Z").strong());
                                        ui.label("Desfazer");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + Y / Ctrl + Shift + Z").strong());
                                        ui.label("Refazer");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + A").strong());
                                        ui.label("Selecionar Tudo");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + F").strong());
                                        ui.label("Localizar texto (barra estilo VS Code)");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + H").strong());
                                        ui.label("Localizar e Substituir");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + Shift + F").strong());
                                        ui.label("Formatar código (SQL, TOML, JSON, XML...)");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + D").strong());
                                        ui.label("Duplicar Linha");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Alt + Seta Cima / Ctrl + Shift + Seta Cima").strong());
                                        ui.label("Mover Linha para Cima");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Alt + Seta Baixo / Ctrl + Shift + Seta Baixo").strong());
                                        ui.label("Mover Linha para Baixo");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Alt + Arrastar Mouse").strong());
                                        ui.label("Seleção Vertical (Coluna)");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Alt + Ctrl + Arrastar Mouse").strong());
                                        ui.label("Seleção no Final das Linhas (insere no fim de cada linha)");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + Clique").strong());
                                        ui.label("Multi-Seleção Livre");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + ,").strong());
                                        ui.label("Abrir Configurações");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + B").strong());
                                        ui.label("Alternar Explorador de Arquivos (Barra Lateral)");
                                        ui.end_row();

                                        ui.label(egui::RichText::new("Ctrl + + / - / 0").strong());
                                        ui.label("Ajustar Zoom / Resetar para 100%");
                                        ui.end_row();
                                    });
                            });

                        ui.add_space(6.0);
                    });
            });

        self.show_settings = open;
    }

    fn render_file_tree(
        &mut self,
        ui: &mut egui::Ui,
        items: &[FileItem],
        colors: &ThemeColors,
        depth: usize,
    ) {
        for item in items {
            let indent = depth as f32 * 14.0;
            if item.is_dir {
                ui.horizontal(|ui| {
                    if indent > 0.0 {
                        ui.add_space(indent);
                    }
                    let icon = "📁";
                    let is_current = self
                        .file_path
                        .as_ref()
                        .and_then(|p| p.parent())
                        .map(|p| p == item.path)
                        .unwrap_or(false);
                    let text_color = if is_current {
                        colors.syn_keyword
                    } else {
                        colors.text_editor
                    };

                    let label = format!("{} {}", icon, item.name);
                    let resp = ui.selectable_label(
                        false,
                        egui::RichText::new(label)
                            .color(text_color)
                            .size(12.5),
                    );
                    if resp.on_hover_text(item.path.to_string_lossy()).clicked() {
                        self.folder_items = FileItem::scan_dir(&item.path);
                        self.current_folder = Some(item.path.clone());
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    if indent > 0.0 {
                        ui.add_space(indent);
                    }
                    let is_active = self
                        .file_path
                        .as_ref()
                        .map(|p| p == &item.path)
                        .unwrap_or(false);

                    let icon = match item.path.extension().and_then(|e| e.to_str()).unwrap_or("") {
                        "rs" => "🦀",
                        "sql" => "🗄",
                        "json" => "📋",
                        "toml" | "yaml" | "yml" => "⚙",
                        "md" => "📝",
                        "html" | "htm" | "xml" | "svg" => "🌐",
                        "js" | "ts" => "🟨",
                        "py" => "🐍",
                        _ => "📄",
                    };

                    let text_color = if is_active {
                        colors.syn_function
                    } else {
                        colors.text_editor
                    };

                    let label = format!("{} {}", icon, item.name);
                    let resp = ui.selectable_label(
                        is_active,
                        egui::RichText::new(label)
                            .color(text_color)
                            .size(12.5),
                    );
                    if resp.on_hover_text(item.path.to_string_lossy()).clicked() {
                        if !is_active {
                            self.request_open_path(item.path.clone());
                        }
                    }
                });
            }
        }
    }

    fn render_sidebar_content(&mut self, ui: &mut egui::Ui, colors: &ThemeColors) {
        ui.horizontal(|ui| {
            let folder_display = self
                .current_folder
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("Explorador");

            ui.label(
                egui::RichText::new(format!("📂 {}", folder_display.to_uppercase()))
                    .strong()
                    .size(12.0)
                    .color(colors.text_gutter_active),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button(egui::RichText::new("✕").size(11.0).color(colors.text_gutter))
                    .on_hover_text("Fechar Barra Lateral (Ctrl+B)")
                    .clicked()
                {
                    self.show_sidebar = false;
                }

                if ui
                    .button(egui::RichText::new("🔄").size(11.0).color(colors.text_gutter))
                    .on_hover_text("Recarregar Pasta")
                    .clicked()
                {
                    self.refresh_folder();
                }

                if let Some(ref folder) = self.current_folder {
                    if let Some(parent) = folder.parent() {
                        if ui
                            .button(egui::RichText::new("⬆").size(11.0).color(colors.text_gutter))
                            .on_hover_text("Pasta Acima")
                            .clicked()
                        {
                            let p = parent.to_path_buf();
                            self.folder_items = FileItem::scan_dir(&p);
                            self.current_folder = Some(p);
                        }
                    }
                }
            });
        });

        ui.add_space(4.0);
        ui.add(egui::Separator::default().horizontal().spacing(2.0));
        ui.add_space(4.0);

        if self.folder_items.is_empty() {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Nenhuma pasta ou arquivo encontrado.")
                        .size(11.5)
                        .color(colors.text_gutter),
                );
                ui.add_space(8.0);
                if ui.button("📁 Abrir Pasta...").clicked() {
                    self.open_folder_dialog();
                }
            });
        } else {
            let items = self.folder_items.clone();
            self.render_file_tree(ui, &items, colors, 0);
        }
    }
}

impl eframe::App for SuperLightCoderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Intercept close request to confirm saving unsaved changes
        if ctx.input(|i| i.viewport().close_requested()) {
            crate::window_state::save_window_state(&self.win_config);
            if self.is_modified && !self.show_unsaved_dialog {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.pending_action = Some(PendingAction::CloseWindow);
                self.show_unsaved_dialog = true;
            }
        }

        // Handle Drag & Drop of files into the editor window
        let dropped_file = ctx.input(|i| {
            i.raw.dropped_files.first().and_then(|f| f.path.clone())
        });
        if let Some(path) = dropped_file {
            self.request_open_path(path);
        }

        // Track and persist window size and maximized state
        ctx.input(|i| {
            let vp = i.viewport();
            let is_maximized = vp.maximized.unwrap_or(false);
            if let Some(rect) = vp.inner_rect {
                let w = rect.width();
                let h = rect.height();
                if w >= 550.0 && h >= 350.0 {
                    let size_changed = !is_maximized && ((w - self.win_config.width).abs() > 2.0 || (h - self.win_config.height).abs() > 2.0);
                    let max_changed = is_maximized != self.win_config.maximized;
                    if size_changed {
                        self.win_config.width = w;
                        self.win_config.height = h;
                    }
                    if max_changed {
                        self.win_config.maximized = is_maximized;
                    }
                    if size_changed || max_changed {
                        crate::window_state::save_window_state(&self.win_config);
                    }
                }
            }
        });

        self.handle_shortcuts(ctx);

        let colors = ThemeColors::from_mode(self.theme_mode);

        // Update application title dynamically without acronym
        let title_file = self
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Sem Título");
        let modified_mark = if self.is_modified { " *" } else { "" };
        let full_title = format!("Super Light Coder - {}{}", title_file, modified_mark);
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(full_title));

        // Configure visual theme in egui
        let mut visuals = match self.theme_mode {
            ThemeMode::Dark => egui::Visuals::dark(),
            ThemeMode::Light => egui::Visuals::light(),
        };
        visuals.panel_fill = colors.bg_editor;
        visuals.window_fill = colors.bg_bottom_bar;
        visuals.selection.bg_fill = colors.selection_bg;
        visuals.selection.stroke.color = colors.cursor_color;
        ctx.set_visuals(visuals);

        // ----------------------------------------------------
        // BOTTOM STATUS BAR (No top menu! Pure bottom info bar)
        // ----------------------------------------------------
        egui::TopBottomPanel::bottom("bottom_status_bar")
            .frame(
                egui::Frame::none()
                    .fill(colors.bg_bottom_bar)
                    .stroke(egui::Stroke::new(1.0_f32, colors.bottom_bar_border))
                    .inner_margin(egui::Margin::symmetric(10.0, 5.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Folder button 📁 on the far left (accent color while the explorer is open)
                    let folder_color = if self.show_sidebar {
                        colors.syn_keyword
                    } else {
                        colors.text_bottom_bar
                    };
                    let folder_btn = ui.add(
                        egui::Button::new(
                            egui::RichText::new("📁")
                                .color(folder_color)
                                .size(15.0),
                        )
                        .frame(false),
                    );
                    if folder_btn
                        .on_hover_text("Explorador de Arquivos / Pasta (Ctrl+B)")
                        .clicked()
                    {
                        self.show_sidebar = !self.show_sidebar;
                        if self.show_sidebar && self.folder_items.is_empty() {
                            self.refresh_folder();
                        }
                    }

                    // Gear button ⚙ for settings (accent color while settings are open)
                    let gear_color = if self.show_settings {
                        colors.syn_keyword
                    } else {
                        colors.text_bottom_bar
                    };
                    let gear_btn = ui.add(
                        egui::Button::new(
                            egui::RichText::new("⚙")
                                .color(gear_color)
                                .size(15.0),
                        )
                        .frame(false),
                    );
                    if gear_btn
                        .on_hover_text("Configurações do Editor (Ctrl+,)")
                        .clicked()
                    {
                        self.show_settings = !self.show_settings;
                    }

                    ui.separator();

                    // Format code quick button { }
                    let fmt_btn = ui.add(
                        egui::Button::new(
                            egui::RichText::new("{ } Formatar")
                                .color(colors.text_bottom_bar)
                                .size(12.0),
                        )
                        .frame(false),
                    );
                    if fmt_btn
                        .on_hover_text("Formatar código (Ctrl+Shift+F)")
                        .clicked()
                    {
                        self.format_document();
                    }

                    // Temporary status alert
                    if let Some((msg, time)) = &self.status_message {
                        if time.elapsed().as_secs() < 5 {
                            ui.separator();
                            ui.label(
                                egui::RichText::new(msg)
                                    .color(egui::Color32::from_rgb(100, 210, 120))
                                    .size(12.0),
                            );
                        }
                    }

                    // Right-aligned status information (matches Notepad / user reference image)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Encoding: UTF-8
                        ui.label(
                            egui::RichText::new(&self.encoding)
                                .color(colors.text_bottom_bar)
                                .size(12.0),
                        );
                        ui.separator();

                        // Line ending: Unix (LF) / Windows (CRLF)
                        let ending_str = if self.crlf {
                            "Windows (CRLF)"
                        } else {
                            "Unix (LF)"
                        };
                        let ending_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new(ending_str)
                                    .color(colors.text_bottom_bar)
                                    .size(12.0),
                            )
                            .frame(false),
                        );
                        if ending_btn
                            .on_hover_text("Clique para alternar entre LF (Unix) e CRLF (Windows)")
                            .clicked()
                        {
                            self.crlf = !self.crlf;
                            self.is_modified = true;
                            self.set_status(if self.crlf {
                                "Fim de linha alterado para CRLF (Windows)"
                            } else {
                                "Fim de linha alterado para LF (Unix)"
                            });
                        }
                        ui.separator();

                        // Zoom percentage (100%)
                        let zoom_str = format!("{}%", (self.zoom_level * 100.0).round() as i32);
                        let zoom_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new(zoom_str)
                                    .color(colors.text_bottom_bar)
                                    .size(12.0),
                            )
                            .frame(false),
                        );
                        if zoom_btn
                            .on_hover_text("Clique para redefinir o zoom para 100%")
                            .clicked()
                        {
                            self.zoom_level = 1.0;
                        }
                        ui.separator();

                        // Language selector
                        egui::ComboBox::from_id_salt("language_combobox")
                            .selected_text(
                                egui::RichText::new(self.language.name())
                                    .color(colors.text_bottom_bar)
                                    .size(12.0),
                            )
                            .show_ui(ui, |ui| {
                                for lang in Language::all() {
                                    ui.selectable_value(&mut self.language, *lang, lang.name());
                                }
                            });
                        ui.separator();

                        // Character count
                        let chars_text = if let Some(sel) = self.selection_count {
                            format!("{} caracteres ({} sel)", self.text.chars().count(), sel)
                        } else {
                            format!("{} caracteres", self.text.chars().count())
                        };
                        ui.label(
                            egui::RichText::new(chars_text)
                                .color(colors.text_bottom_bar)
                                .size(12.0),
                        );
                        ui.separator();

                        // Ln X, Col Y or Column Selection info
                        let pos_label = if self.multi_sel.is_column_mode {
                            let min_l = self.multi_sel.column_start_line.min(self.multi_sel.column_end_line);
                            let max_l = self.multi_sel.column_start_line.max(self.multi_sel.column_end_line);
                            let count = max_l.saturating_sub(min_l) + 1;
                            if self.multi_sel.is_line_end_mode {
                                format!("Ln {}-{} ({} lin) [Fim de Linha]", min_l, max_l, count)
                            } else {
                                format!("Ln {}-{} ({} lin), Col {}", min_l, max_l, count, self.multi_sel.column_col)
                            }
                        } else if !self.multi_sel.free_cursors.is_empty() {
                            format!("{} cursores", self.multi_sel.free_cursors.len())
                        } else {
                            format!("Ln {}, Col {}", self.cursor_line, self.cursor_col)
                        };
                        ui.label(
                            egui::RichText::new(pos_label)
                                .color(colors.text_bottom_bar)
                                .size(12.0),
                        );
                    });
                });
            });

        // ----------------------------------------------------
        // CENTRAL PANEL: Editor with Clean & Compact Gutter
        // ----------------------------------------------------
        let font_size = self.font_size * self.zoom_level;
        let line_count = self.text.lines().count().max(1);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(colors.bg_editor)
                    .inner_margin(egui::Margin {
                        left: 2.0,
                        right: 4.0,
                        top: 4.0,
                        bottom: 4.0,
                    }),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        ui.horizontal_top(|ui| {
                            // File Explorer Sidebar (Left of line numbers, toggled via folder icon or Ctrl+B)
                            if self.show_sidebar {
                                ui.allocate_ui_with_layout(
                                    egui::vec2(220.0, ui.available_height().max(300.0)),
                                    egui::Layout::top_down(egui::Align::LEFT),
                                    |ui| {
                                        egui::Frame::none()
                                            .fill(colors.bg_bottom_bar)
                                            .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                                            .show(ui, |ui| {
                                                self.render_sidebar_content(ui, &colors);
                                            });
                                    },
                                );
                                ui.add(egui::Separator::default().vertical().spacing(4.0));
                            }

                            // Ultra-compact Line Number Gutter
                            let digits = line_count.to_string().len();
                            let char_w = font_size * 0.6;
                            let gutter_width = (digits as f32) * char_w + 2.0;

                            ui.allocate_ui_with_layout(
                                egui::vec2(gutter_width, 0.0),
                                egui::Layout::top_down(egui::Align::RIGHT),
                                |ui| {
                                    let mut gutter_job = egui::text::LayoutJob::default();
                                    for num in 1..=line_count {
                                        let color = if num == self.cursor_line {
                                            colors.text_gutter_active
                                        } else {
                                            colors.text_gutter
                                        };
                                        gutter_job.append(
                                            &format!("{:>width$}\n", num, width = digits),
                                            0.0,
                                            egui::TextFormat {
                                                font_id: egui::FontId::monospace(font_size),
                                                color,
                                                ..Default::default()
                                            },
                                        );
                                    }
                                    ui.add(egui::Label::new(gutter_job).selectable(false));
                                },
                            );

                            // Subtle vertical separator line between gutter and editor text
                            ui.add(egui::Separator::default().vertical().spacing(4.0));

                            // Code Text Editor with Search Highlighting
                            let mut layouter = |ui: &egui::Ui, text: &str, _wrap_width: f32| {
                                let mut job = crate::syntax::highlight(text, self.language, &colors, font_size);
                                if self.find_replace.is_open && !self.find_replace.matches.is_empty() {
                                    let byte_matches: Vec<(usize, usize)> = self
                                        .find_replace
                                        .matches
                                        .iter()
                                        .map(|m| (m.byte_start, m.byte_end))
                                        .collect();
                                    crate::syntax::apply_search_highlights(
                                        &mut job,
                                        &byte_matches,
                                        self.find_replace.current_match,
                                    );
                                }
                                ui.fonts(|f| f.layout_job(job))
                            };

                            let editor_id = egui::Id::new("slc_editor_text");
                            let is_alt_down = ctx.input(|i| i.modifiers.alt);
                            let is_multi_active = self.multi_sel.is_active();

                            // Clear any selection in TextEdit when Alt is held or multi-selection is active
                            // This ensures NO BLUE SELECTION appears!
                            if is_alt_down || is_multi_active {
                                let mut state = egui::text_edit::TextEditState::load(ctx, editor_id).unwrap_or_default();
                                state.cursor.set_char_range(None);
                                state.store(ctx, editor_id);
                            }

                            if self.request_select_all {
                                self.request_select_all = false;
                                let char_count = self.text.chars().count();
                                let mut state = egui::text_edit::TextEditState::load(ctx, editor_id).unwrap_or_default();
                                let ccursor_start = egui::text::CCursor::new(0);
                                let ccursor_end = egui::text::CCursor::new(char_count);
                                state.cursor.set_char_range(Some(egui::text::CCursorRange::two(ccursor_start, ccursor_end)));
                                state.store(ctx, editor_id);
                            }

                            if self.history.request_cursor_sync {
                                self.history.request_cursor_sync = false;
                                let char_count = self.text.chars().count();
                                let char_idx = calculate_char_idx(&self.text, self.cursor_line, self.cursor_col).min(char_count);
                                let mut state = egui::text_edit::TextEditState::load(ctx, editor_id).unwrap_or_default();
                                let ccursor = egui::text::CCursor::new(char_idx);
                                state.cursor.set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                                state.store(ctx, editor_id);
                            }

                            let text_before_edit = self.text.clone();
                            let line_before_edit = self.cursor_line;
                            let col_before_edit = self.cursor_col;

                            if self.history.base_snapshot.is_none() {
                                self.history.base_snapshot = Some(UndoSnapshot {
                                    text: text_before_edit.clone(),
                                    cursor_line: line_before_edit,
                                    cursor_col: col_before_edit,
                                });
                            }

                            let output = egui::TextEdit::multiline(&mut self.text)
                                .id(editor_id)
                                .font(egui::FontId::monospace(font_size))
                                .code_editor()
                                .lock_focus(true)
                                .interactive(!is_alt_down && !is_multi_active)
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter)
                                .frame(false)
                                .show(ui);

                            let editor_response = &output.response;
                            let editor_rect = editor_response.rect;
                            let galley_pos = output.galley_pos;
                            let font_id = egui::FontId::monospace(font_size);
                            let char_w = ui.fonts(|f| f.glyph_width(&font_id, ' '));
                            let row_h = ui.fonts(|f| f.row_height(&font_id));

                            ctx.input(|i| {
                                let alt = i.modifiers.alt;
                                let ctrl = i.modifiers.command || i.modifiers.ctrl;

                                // Alt + Click or Alt + Drag (Column Selection / End-of-line Selection)
                                if alt && (i.pointer.primary_down() || i.pointer.primary_clicked()) {
                                    if let Some(pos) = i.pointer.latest_pos() {
                                        if editor_rect.contains(pos) {
                                            // Exact row from galley calculation (accounts for scroll and wrapping)
                                            let rel = pos - galley_pos;
                                            let cur = output.galley.cursor_from_pos(rel);
                                            let line_num = (cur.rcursor.row + 1).min(line_count);
                                            let col_num = galley_col_from_x(&output.galley, line_num, rel, char_w);
                                            let is_end_mode = ctrl;

                                            if !self.multi_sel.is_column_mode || i.pointer.primary_pressed() {
                                                self.multi_sel.is_column_mode = true;
                                                self.multi_sel.is_line_end_mode = is_end_mode;
                                                self.multi_sel.column_start_line = line_num;
                                                self.multi_sel.column_end_line = line_num;
                                                self.multi_sel.column_col = col_num;
                                                self.multi_sel.free_cursors.clear();
                                            } else {
                                                self.multi_sel.column_end_line = line_num;
                                                self.multi_sel.is_line_end_mode = is_end_mode;
                                            }
                                        }
                                    }
                                } else if ctrl && !alt && i.pointer.primary_clicked() {
                                    // Multi-cursor free click
                                    if let Some(pos) = i.pointer.latest_pos() {
                                        if editor_rect.contains(pos) {
                                            let rel = pos - galley_pos;
                                            let cur = output.galley.cursor_from_pos(rel);
                                            let line_num = (cur.rcursor.row + 1).min(line_count);
                                            let col_num = galley_col_from_x(&output.galley, line_num, rel, char_w);

                                            self.multi_sel.is_column_mode = false;
                                            self.multi_sel.is_line_end_mode = false;
                                            if !self.multi_sel.free_cursors.contains(&(line_num, col_num)) {
                                                self.multi_sel.free_cursors.push((line_num, col_num));
                                            }
                                        }
                                    }
                                } else if !alt && !ctrl && i.pointer.primary_clicked() {
                                    // Normal click clears multi selection
                                    if self.multi_sel.is_active() {
                                        self.multi_sel.clear();
                                    }
                                }
                            });

                            // Process multi-cursor typing, backspacing, delete, and navigation
                            if self.multi_sel.is_active() {
                                let mut text_to_insert: Option<String> = None;
                                let mut backspace_pressed = false;
                                let mut delete_pressed = false;
                                let mut clear_sel = false;
                                let mut left_pressed = false;
                                let mut right_pressed = false;

                                ctx.input(|i| {
                                    for event in &i.events {
                                        match event {
                                            egui::Event::Text(t) => {
                                                match &mut text_to_insert {
                                                    Some(existing) => existing.push_str(t),
                                                    None => text_to_insert = Some(t.clone()),
                                                }
                                            }
                                            egui::Event::Key {
                                                key: egui::Key::Backspace,
                                                pressed: true,
                                                ..
                                            } => {
                                                backspace_pressed = true;
                                            }
                                            egui::Event::Key {
                                                key: egui::Key::Delete,
                                                pressed: true,
                                                ..
                                            } => {
                                                delete_pressed = true;
                                            }
                                            egui::Event::Key {
                                                key: egui::Key::Escape,
                                                pressed: true,
                                                ..
                                            }
                                            | egui::Event::Key {
                                                key: egui::Key::Enter,
                                                pressed: true,
                                                ..
                                            } => {
                                                clear_sel = true;
                                            }
                                            egui::Event::Key {
                                                key: egui::Key::ArrowLeft,
                                                pressed: true,
                                                ..
                                            } => {
                                                left_pressed = true;
                                            }
                                            egui::Event::Key {
                                                key: egui::Key::ArrowRight,
                                                pressed: true,
                                                ..
                                            } => {
                                                right_pressed = true;
                                            }
                                            _ => {}
                                        }
                                    }
                                });

                                if clear_sel {
                                    self.multi_sel.clear();
                                } else {
                                    let edited = text_to_insert.is_some() || backspace_pressed || delete_pressed;
                                    if let Some(t) = text_to_insert {
                                        self.apply_multi_insert(&t);
                                    }
                                    if backspace_pressed {
                                        self.apply_multi_backspace();
                                    }
                                    if delete_pressed {
                                        self.apply_multi_delete();
                                    }
                                    if left_pressed && self.multi_sel.is_column_mode && !self.multi_sel.is_line_end_mode && self.multi_sel.column_col > 1 {
                                        self.multi_sel.column_col -= 1;
                                    }
                                    if right_pressed && self.multi_sel.is_column_mode && !self.multi_sel.is_line_end_mode {
                                        self.multi_sel.column_col += 1;
                                    }
                                    if edited {
                                        // The text was changed after the galley was laid out this frame;
                                        // repaint right away so text and painted cursor stay in sync.
                                        ctx.request_repaint();
                                    }
                                }

                                // Paint column cursors or free multi-cursors

                                if self.multi_sel.is_column_mode {
                                    let min_l = self.multi_sel.column_start_line.min(self.multi_sel.column_end_line).max(1);
                                    let max_l = self.multi_sel.column_start_line.max(self.multi_sel.column_end_line).min(line_count);

                                    for l in min_l..=max_l {
                                        let line_idx = l - 1;
                                        let row_idx = line_idx.min(output.galley.rows.len().saturating_sub(1));
                                        let y = if row_idx < output.galley.rows.len() {
                                            galley_pos.y + output.galley.rows[row_idx].rect.min.y
                                        } else {
                                            galley_pos.y + (line_idx as f32) * row_h
                                        };

                                        let base_x = if self.multi_sel.is_line_end_mode {
                                            galley_pos.x + galley_x(&output.galley, l, usize::MAX, char_w)
                                        } else {
                                            galley_pos.x + galley_x(&output.galley, l, self.multi_sel.column_col, char_w)
                                        };

                                        ui.painter().rect_filled(
                                            egui::Rect::from_min_size(egui::pos2(galley_pos.x, y), egui::vec2(editor_rect.width(), row_h)),
                                            0.0,
                                            egui::Color32::from_rgba_unmultiplied(100, 150, 255, 12),
                                        );
                                        ui.painter().rect_filled(
                                            egui::Rect::from_min_size(egui::pos2(base_x, y), egui::vec2(2.0, row_h)),
                                            0.0,
                                            colors.cursor_color,
                                        );
                                    }
                                } else if !self.multi_sel.free_cursors.is_empty() {
                                    for (l, c) in &self.multi_sel.free_cursors {
                                        let line_idx = l.saturating_sub(1);
                                        let row_idx = line_idx.min(output.galley.rows.len().saturating_sub(1));
                                        let y = if row_idx < output.galley.rows.len() {
                                            galley_pos.y + output.galley.rows[row_idx].rect.min.y
                                        } else {
                                            galley_pos.y + (line_idx as f32) * row_h
                                        };
                                        let base_x = galley_pos.x + galley_x(&output.galley, *l, *c, char_w);
                                        ui.painter().rect_filled(
                                            egui::Rect::from_min_size(egui::pos2(base_x, y), egui::vec2(2.0, row_h)),
                                            0.0,
                                            colors.cursor_color,
                                        );
                                    }
                                }
                            }

                            // Track cursor and selection
                            if let Some(range) = output.cursor_range {
                                let primary_idx = range.primary.ccursor.index;
                                let (line, col) = calculate_line_col(&self.text, primary_idx);
                                self.cursor_line = line;
                                self.cursor_col = col;

                                let sec_idx = range.secondary.ccursor.index;
                                if primary_idx != sec_idx {
                                    let diff = (primary_idx as isize - sec_idx as isize).abs() as usize;
                                    self.selection_count = Some(diff);
                                } else {
                                    self.selection_count = None;
                                }
                            }

                            if output.response.changed() && self.text != text_before_edit {
                                self.is_modified = true;
                                let now = std::time::Instant::now();
                                let should_commit = if let Some(last_time) = self.history.last_edit_time {
                                    now.duration_since(last_time).as_millis() > 500
                                } else {
                                    false
                                };
                                if should_commit {
                                    if let Some(base) = self.history.base_snapshot.take() {
                                        self.history.push_undo(base.text, base.cursor_line, base.cursor_col);
                                    }
                                }
                                self.history.last_edit_time = Some(now);
                                if self.history.base_snapshot.is_none() {
                                    self.history.base_snapshot = Some(UndoSnapshot {
                                        text: text_before_edit,
                                        cursor_line: line_before_edit,
                                        cursor_col: col_before_edit,
                                    });
                                }
                                if self.find_replace.is_open {
                                    self.find_replace.update_matches(&self.text);
                                }
                            }
                        });
                    });
            });

        // Floating Find & Replace Widget (VS Code Style)
        self.render_find_replace_bar(ctx, &colors);

        // Modals
        self.render_settings_window(ctx, &colors);
        self.render_unsaved_dialog(ctx);
    }
}

fn calculate_line_col(text: &str, char_index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    let mut count = 0;

    for ch in text.chars() {
        if count >= char_index {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        count += 1;
    }

    (line, col)
}

fn calculate_char_idx(text: &str, target_line: usize, target_col: usize) -> usize {
    let mut line = 1;
    let mut col = 1;
    let mut idx = 0;
    for ch in text.chars() {
        if line == target_line && col == target_col {
            return idx;
        }
        if ch == '\n' {
            if line == target_line {
                return idx;
            }
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        idx += 1;
    }
    idx
}

/// Absolute-from-galley x offset of (1-based) `line`/`col`. Columns past the end of the line
/// are extrapolated with `char_w`; `usize::MAX` means "end of line".
fn galley_x(galley: &egui::Galley, line: usize, col: usize, char_w: f32) -> f32 {
    let text = &galley.job.text;
    let len = text.split('\n').nth(line.saturating_sub(1)).map_or(0, |l| l.chars().count());
    let target = if col == usize::MAX { len + 1 } else { col.max(1) };
    let eff = target.min(len + 1);
    let idx = calculate_char_idx(text, line.max(1), eff);
    let x = galley.pos_from_ccursor(egui::text::CCursor::new(idx)).min.x;
    x + (target - eff) as f32 * char_w
}

/// Column (1-based) under a galley-relative pointer position, based on real glyph positions.
fn galley_col_from_x(galley: &egui::Galley, line: usize, rel: egui::Vec2, char_w: f32) -> usize {
    let text = &galley.job.text;
    let len = text.split('\n').nth(line.saturating_sub(1)).map_or(0, |l| l.chars().count());
    let end_x = galley_x(galley, line, len + 1, char_w);
    if rel.x > end_x {
        len + 1 + ((rel.x - end_x) / char_w).round() as usize
    } else {
        let cur = galley.cursor_from_pos(egui::vec2(rel.x.max(0.0), rel.y));
        calculate_line_col(text, cur.ccursor.index).1
    }
}

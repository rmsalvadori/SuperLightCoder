use egui::text::LayoutJob;
use egui::{FontId, TextFormat};
use std::collections::HashSet;
use std::path::Path;

use crate::theme::ThemeColors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    PlainText,
    Rust,
    Sql,
    Python,
    JavaScript,
    TypeScript,
    Html,
    Php,
    Java,
    C,
    CSharp,
    Cpp,
    Go,
    Pascal,
    Perl,
    Lua,
    Ruby,
    Xml,
    Json,
    Markdown,
    Svg,
    Toml,
    Htaccess,
    GitIgnore,
    Batch,
    Shell,
    PowerShell,
    Env,
}

impl Language {
    pub fn all() -> &'static [Language] {
        &[
            Language::PlainText,
            Language::Rust,
            Language::Sql,
            Language::Python,
            Language::JavaScript,
            Language::TypeScript,
            Language::Html,
            Language::Php,
            Language::Java,
            Language::C,
            Language::CSharp,
            Language::Cpp,
            Language::Go,
            Language::Pascal,
            Language::Perl,
            Language::Lua,
            Language::Ruby,
            Language::Xml,
            Language::Json,
            Language::Markdown,
            Language::Svg,
            Language::Toml,
            Language::Htaccess,
            Language::GitIgnore,
            Language::Batch,
            Language::Shell,
            Language::PowerShell,
            Language::Env,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::PlainText => "Texto sem formatação",
            Language::Rust => "Rust",
            Language::Sql => "SQL",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Html => "HTML",
            Language::Php => "PHP",
            Language::Java => "Java",
            Language::C => "C",
            Language::CSharp => "C#",
            Language::Cpp => "C++",
            Language::Go => "Go",
            Language::Pascal => "Pascal",
            Language::Perl => "Perl",
            Language::Lua => "Lua",
            Language::Ruby => "Ruby",
            Language::Xml => "XML",
            Language::Json => "JSON",
            Language::Markdown => "Markdown (MD)",
            Language::Svg => "SVG",
            Language::Toml => "TOML",
            Language::Htaccess => "Apache (.htaccess)",
            Language::GitIgnore => "Git Ignore (.gitignore)",
            Language::Batch => "Batch / CMD (.cmd, .bat)",
            Language::Shell => "Shell Script (.sh)",
            Language::PowerShell => "PowerShell (.ps1)",
            Language::Env => "Configuração (.env)",
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        if file_name == ".htaccess" || file_name.ends_with(".htaccess") {
            return Language::Htaccess;
        }
        if file_name == ".gitignore" || file_name == ".gitattributes" || file_name == ".gitmodules" {
            return Language::GitIgnore;
        }
        if file_name == ".env" || file_name.starts_with(".env.") || file_name.ends_with(".env") {
            return Language::Env;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "rs" => Language::Rust,
            "sql" => Language::Sql,
            "py" | "pyw" => Language::Python,
            "js" | "mjs" | "cjs" | "jsx" => Language::JavaScript,
            "ts" | "tsx" => Language::TypeScript,
            "html" | "htm" => Language::Html,
            "php" | "phtml" | "php3" | "php4" | "php5" => Language::Php,
            "java" => Language::Java,
            "c" | "h" => Language::C,
            "cs" => Language::CSharp,
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" => Language::Cpp,
            "go" => Language::Go,
            "pas" | "pp" | "inc" => Language::Pascal,
            "pl" | "pm" | "t" => Language::Perl,
            "lua" => Language::Lua,
            "rb" | "rake" | "gemspec" => Language::Ruby,
            "xml" | "xaml" | "plist" | "config" => Language::Xml,
            "json" => Language::Json,
            "md" | "markdown" => Language::Markdown,
            "svg" => Language::Svg,
            "toml" => Language::Toml,
            "htaccess" => Language::Htaccess,
            "gitignore" => Language::GitIgnore,
            "env" => Language::Env,
            "cmd" | "bat" => Language::Batch,
            "sh" | "bash" | "zsh" => Language::Shell,
            "ps1" | "psm1" | "psd1" => Language::PowerShell,
            _ => Language::PlainText,
        }
    }

    fn keywords(&self) -> HashSet<&'static str> {
        let list: &[&'static str] = match self {
            Language::Rust => &[
                "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
                "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
                "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
                "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
            ],
            Language::Sql => &[
                "SELECT", "FROM", "WHERE", "AND", "OR", "NOT", "INSERT", "INTO", "VALUES",
                "UPDATE", "SET", "DELETE", "CREATE", "TABLE", "DROP", "ALTER", "ADD", "JOIN",
                "INNER", "LEFT", "RIGHT", "FULL", "OUTER", "ON", "GROUP", "BY", "ORDER",
                "ASC", "DESC", "HAVING", "LIMIT", "OFFSET", "UNION", "ALL", "DISTINCT",
                "AS", "IN", "IS", "NULL", "LIKE", "BETWEEN", "CASE", "WHEN", "THEN",
                "ELSE", "END", "EXISTS", "INDEX", "PRIMARY", "KEY", "FOREIGN", "REFERENCES",
                "DEFAULT", "CHECK", "UNIQUE", "VIEW", "TRIGGER", "DATABASE", "SCHEMA",
                "select", "from", "where", "and", "or", "not", "insert", "into", "values",
                "update", "set", "delete", "create", "table", "drop", "alter", "join",
                "left", "right", "inner", "on", "group", "by", "order", "asc", "desc", "limit",
            ],
            Language::Python => &[
                "False", "None", "True", "and", "as", "assert", "async", "await", "break",
                "class", "continue", "def", "del", "elif", "else", "except", "finally",
                "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal",
                "not", "or", "pass", "raise", "return", "try", "while", "with", "yield",
                "self",
            ],
            Language::JavaScript | Language::TypeScript => &[
                "break", "case", "catch", "class", "const", "continue", "debugger", "default",
                "delete", "do", "else", "export", "extends", "finally", "for", "function",
                "if", "import", "in", "instanceof", "new", "return", "super", "switch",
                "this", "throw", "try", "typeof", "var", "void", "while", "with", "yield",
                "let", "static", "enum", "await", "async", "true", "false", "null", "undefined",
                "interface", "type", "implements", "declare", "namespace", "readonly",
            ],
            Language::Php => &[
                "echo", "print", "if", "else", "elseif", "while", "do", "for", "foreach",
                "as", "break", "continue", "switch", "case", "default", "return", "function",
                "class", "public", "protected", "private", "static", "abstract", "final",
                "interface", "trait", "namespace", "use", "new", "clone", "try", "catch",
                "finally", "throw", "var", "const", "global", "isset", "empty", "die", "exit",
                "true", "false", "null",
            ],
            Language::Java => &[
                "abstract", "assert", "boolean", "break", "byte", "case", "catch", "char",
                "class", "const", "continue", "default", "do", "double", "else", "enum",
                "extends", "final", "finally", "float", "for", "goto", "if", "implements",
                "import", "instanceof", "int", "interface", "long", "native", "new",
                "package", "private", "protected", "public", "return", "short", "static",
                "strictfp", "super", "switch", "synchronized", "this", "throw", "throws",
                "transient", "try", "void", "volatile", "while", "true", "false", "null",
            ],
            Language::C | Language::Cpp => &[
                "auto", "break", "case", "char", "const", "continue", "default", "do",
                "double", "else", "enum", "extern", "float", "for", "goto", "if", "int",
                "long", "register", "return", "short", "signed", "sizeof", "static",
                "struct", "switch", "typedef", "union", "unsigned", "void", "volatile", "while",
                "class", "namespace", "using", "public", "private", "protected", "virtual",
                "override", "template", "typename", "new", "delete", "this", "nullptr",
                "true", "false", "constexpr", "inline", "explicit", "friend", "try", "catch",
            ],
            Language::CSharp => &[
                "abstract", "as", "base", "bool", "break", "byte", "case", "catch", "char",
                "checked", "class", "const", "continue", "decimal", "default", "delegate",
                "do", "double", "else", "enum", "event", "explicit", "extern", "false",
                "finally", "fixed", "float", "for", "foreach", "goto", "if", "implicit",
                "in", "int", "interface", "internal", "is", "lock", "long", "namespace",
                "new", "null", "object", "operator", "out", "override", "params", "private",
                "protected", "public", "readonly", "ref", "return", "sbyte", "sealed",
                "short", "sizeof", "stackalloc", "static", "string", "struct", "switch",
                "this", "throw", "true", "try", "typeof", "uint", "ulong", "unchecked",
                "unsafe", "ushort", "using", "virtual", "void", "volatile", "while",
                "async", "await", "var", "record",
            ],
            Language::Go => &[
                "break", "default", "func", "interface", "select", "case", "defer", "go",
                "map", "struct", "chan", "else", "goto", "package", "switch", "const",
                "fallthrough", "if", "range", "type", "continue", "for", "import", "return",
                "var", "true", "false", "nil", "iota",
            ],
            Language::Pascal => &[
                "and", "array", "begin", "case", "const", "div", "do", "downto", "else",
                "end", "file", "for", "function", "goto", "if", "in", "label", "mod", "nil",
                "not", "of", "or", "packed", "procedure", "program", "record", "repeat",
                "set", "then", "to", "type", "until", "var", "while", "with", "uses",
                "interface", "implementation", "initialization", "finalization", "unit",
                "true", "false",
            ],
            Language::Perl => &[
                "my", "our", "sub", "if", "unless", "elsif", "else", "while", "until",
                "for", "foreach", "do", "last", "next", "redo", "return", "use", "require",
                "package", "qw", "die", "warn", "eval", "eq", "ne", "lt", "gt", "le", "ge",
            ],
            Language::Lua => &[
                "and", "break", "do", "else", "elseif", "end", "false", "for", "function",
                "goto", "if", "in", "local", "nil", "not", "or", "repeat", "return", "then",
                "true", "until", "while",
            ],
            Language::Ruby => &[
                "alias", "and", "begin", "break", "case", "class", "def", "defined?",
                "do", "else", "elsif", "end", "ensure", "false", "for", "if", "in",
                "module", "next", "nil", "not", "or", "redo", "rescue", "retry", "return",
                "self", "super", "then", "true", "undef", "unless", "until", "when", "while",
                "yield",
            ],
            Language::Json => &["true", "false", "null"],
            _ => &[],
        };

        list.iter().copied().collect()
    }

    fn types(&self) -> HashSet<&'static str> {
        let list: &[&'static str] = match self {
            Language::Rust => &[
                "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
                "u128", "usize", "f32", "f64", "bool", "char", "str", "String", "Option",
                "Some", "None", "Result", "Ok", "Err", "Vec", "Box", "Rc", "Arc",
            ],
            Language::Sql => &[
                "INT", "INTEGER", "VARCHAR", "CHAR", "TEXT", "BOOLEAN", "DATE", "DATETIME",
                "TIMESTAMP", "TIME", "FLOAT", "DOUBLE", "DECIMAL", "NUMERIC", "BLOB", "BIGINT",
                "SMALLINT", "TINYINT", "int", "varchar", "char", "text", "boolean", "date",
            ],
            Language::Java | Language::C | Language::Cpp | Language::CSharp => &[
                "int", "long", "short", "byte", "float", "double", "char", "boolean", "bool",
                "void", "string", "String", "Object", "List", "Map", "Set", "vector", "std",
            ],
            Language::Go => &[
                "int", "int8", "int16", "int32", "int64", "uint", "uint8", "uint16", "uint32",
                "uint64", "uintptr", "float32", "float64", "complex64", "complex128", "string",
                "bool", "byte", "rune", "error",
            ],
            Language::Pascal => &[
                "integer", "cardinal", "shortint", "smallint", "longint", "int64", "byte",
                "word", "real", "single", "double", "extended", "comp", "currency", "boolean",
                "char", "string", "ansistring", "pointer",
            ],
            _ => &[],
        };

        list.iter().copied().collect()
    }
}

pub fn apply_search_highlights(
    job: &mut LayoutJob,
    matches: &[(usize, usize)],
    active_match: usize,
) {
    if matches.is_empty() {
        return;
    }

    let default_bg = egui::Color32::from_rgba_premultiplied(234, 179, 8, 80);
    let active_bg = egui::Color32::from_rgba_premultiplied(249, 115, 22, 170);

    let old_sections = std::mem::take(&mut job.sections);
    let mut new_sections = Vec::with_capacity(old_sections.len() * 2);

    for sec in old_sections {
        let sec_start = sec.byte_range.start;
        let sec_end = sec.byte_range.end;
        if sec_start >= sec_end {
            continue;
        }

        let mut cur = sec_start;
        for (m_idx, &(m_start, m_end)) in matches.iter().enumerate() {
            if m_end <= cur || m_start >= sec_end {
                continue;
            }

            // Slice before match
            if cur < m_start {
                let piece_end = m_start.min(sec_end);
                if piece_end > cur {
                    let mut piece = sec.clone();
                    piece.byte_range = cur..piece_end;
                    new_sections.push(piece);
                    cur = piece_end;
                }
            }

            // Slice inside match
            if cur < m_end && cur < sec_end {
                let piece_end = m_end.min(sec_end);
                if piece_end > cur {
                    let mut piece = sec.clone();
                    piece.byte_range = cur..piece_end;
                    piece.format.background = if m_idx == active_match {
                        active_bg
                    } else {
                        default_bg
                    };
                    new_sections.push(piece);
                    cur = piece_end;
                }
            }
        }

        // Slice after all matches
        if cur < sec_end {
            let mut piece = sec.clone();
            piece.byte_range = cur..sec_end;
            new_sections.push(piece);
        }
    }

    job.sections = new_sections;
}

pub fn highlight(text: &str, lang: Language, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    if lang == Language::PlainText {
        job.append(
            text,
            0.0,
            TextFormat {
                font_id,
                color: colors.text_editor,
                ..Default::default()
            },
        );
        return job;
    }

    if lang == Language::Markdown {
        return highlight_markdown(text, colors, font_size);
    }

    if lang == Language::Htaccess {
        return highlight_htaccess(text, colors, font_size);
    }

    if lang == Language::GitIgnore {
        return highlight_gitignore(text, colors, font_size);
    }

    if lang == Language::Batch {
        return highlight_batch(text, colors, font_size);
    }

    if lang == Language::Shell {
        return highlight_shell(text, colors, font_size);
    }

    if lang == Language::PowerShell {
        return highlight_powershell(text, colors, font_size);
    }

    if lang == Language::Env {
        return highlight_env(text, colors, font_size);
    }

    if matches!(lang, Language::Html | Language::Xml | Language::Svg) {
        return highlight_markup(text, colors, font_size);
    }

    if lang == Language::Toml {
        return highlight_toml(text, colors, font_size);
    }

    let keywords = lang.keywords();
    let types = lang.types();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Line comment check
        if (lang == Language::Python || lang == Language::Ruby || lang == Language::Perl)
            && chars[i] == '#'
        {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        if (lang == Language::Sql || lang == Language::Lua)
            && i + 1 < len
            && chars[i] == '-'
            && chars[i + 1] == '-'
        {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        // Block comment /* ... */
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            let start = i;
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            } else {
                i = len;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        // Strings: "..." or '...' or `...`
        if chars[i] == '"' || chars[i] == '\'' || (chars[i] == '`' && lang != Language::PlainText) {
            let quote = chars[i];
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len {
                    i += 2;
                    continue;
                }
                if chars[i] == quote {
                    i += 1;
                    break;
                }
                if chars[i] == '\n' && quote != '`' {
                    break;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_string,
                    ..Default::default()
                },
            );
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit()
            || (chars[i] == '.' && i + 1 < len && chars[i + 1].is_ascii_digit())
        {
            let start = i;
            if chars[i] == '0' && i + 1 < len && (chars[i + 1] == 'x' || chars[i + 1] == 'b') {
                i += 2;
                while i < len && (chars[i].is_ascii_hexdigit() || chars[i] == '_') {
                    i += 1;
                }
            } else {
                while i < len
                    && (chars[i].is_ascii_digit()
                        || chars[i] == '.'
                        || chars[i] == '_'
                        || chars[i] == 'e'
                        || chars[i] == 'E'
                        || chars[i] == 'f'
                        || chars[i] == 'u'
                        || chars[i] == 'i')
                {
                    i += 1;
                }
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_number,
                    ..Default::default()
                },
            );
            continue;
        }

        // Words / Identifiers / Keywords
        if chars[i].is_alphabetic() || chars[i] == '_' || (lang == Language::Php && chars[i] == '$') {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '$') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            let color = if keywords.contains(word.as_str()) {
                colors.syn_keyword
            } else if types.contains(word.as_str()) {
                colors.syn_type
            } else if i < len && chars[i] == '(' {
                colors.syn_function
            } else if word.starts_with('$') {
                colors.syn_attribute
            } else {
                colors.text_editor
            };

            job.append(
                &word,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
            continue;
        }

        // Operators & Punctuation
        let ch = chars[i];
        let color = match ch {
            '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '~' | '?' => {
                colors.syn_operator
            }
            '{' | '}' | '[' | ']' | '(' | ')' | ';' | ',' | '.' | ':' => colors.syn_punctuation,
            _ => colors.text_editor,
        };

        job.append(
            &ch.to_string(),
            0.0,
            TextFormat {
                font_id: font_id.clone(),
                color,
                ..Default::default()
            },
        );
        i += 1;
    }

    job
}

fn highlight_markup(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Comment <!-- ... -->
        if i + 3 < len
            && chars[i] == '<'
            && chars[i + 1] == '!'
            && chars[i + 2] == '-'
            && chars[i + 3] == '-'
        {
            let start = i;
            i += 4;
            while i + 2 < len && !(chars[i] == '-' && chars[i + 1] == '-' && chars[i + 2] == '>') {
                i += 1;
            }
            if i + 2 < len {
                i += 3;
            } else {
                i = len;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        // Tag: <tag ... >
        if chars[i] == '<' {
            job.append(
                "<",
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_operator,
                    ..Default::default()
                },
            );
            i += 1;

            if i < len && chars[i] == '/' {
                job.append(
                    "/",
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_operator,
                        ..Default::default()
                    },
                );
                i += 1;
            }

            // Tag name
            let tag_start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '-' || chars[i] == ':') {
                i += 1;
            }
            if tag_start < i {
                let tag_name: String = chars[tag_start..i].iter().collect();
                job.append(
                    &tag_name,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_tag,
                        ..Default::default()
                    },
                );
            }

            // Inside tag attributes until '>'
            while i < len && chars[i] != '>' {
                if chars[i].is_whitespace() {
                    job.append(
                        &chars[i].to_string(),
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.text_editor,
                            ..Default::default()
                        },
                    );
                    i += 1;
                    continue;
                }

                if chars[i] == '/' {
                    job.append(
                        "/",
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.syn_operator,
                            ..Default::default()
                        },
                    );
                    i += 1;
                    continue;
                }

                // Attribute name
                if chars[i].is_alphanumeric() || chars[i] == '-' || chars[i] == ':' {
                    let attr_start = i;
                    while i < len
                        && (chars[i].is_alphanumeric() || chars[i] == '-' || chars[i] == ':')
                    {
                        i += 1;
                    }
                    let attr_name: String = chars[attr_start..i].iter().collect();
                    job.append(
                        &attr_name,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.syn_attribute,
                            ..Default::default()
                        },
                    );
                    continue;
                }

                // Strings inside attributes
                if chars[i] == '"' || chars[i] == '\'' {
                    let q = chars[i];
                    let str_start = i;
                    i += 1;
                    while i < len && chars[i] != q {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    let val: String = chars[str_start..i].iter().collect();
                    job.append(
                        &val,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.syn_string,
                            ..Default::default()
                        },
                    );
                    continue;
                }

                job.append(
                    &chars[i].to_string(),
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_operator,
                        ..Default::default()
                    },
                );
                i += 1;
            }

            if i < len && chars[i] == '>' {
                job.append(
                    ">",
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_operator,
                        ..Default::default()
                    },
                );
                i += 1;
            }
            continue;
        }

        // Regular content between tags
        let text_start = i;
        while i < len && chars[i] != '<' {
            i += 1;
        }
        let txt: String = chars[text_start..i].iter().collect();
        job.append(
            &txt,
            0.0,
            TextFormat {
                font_id: font_id.clone(),
                color: colors.text_editor,
                ..Default::default()
            },
        );
    }

    job
}

fn highlight_markdown(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("# ")
            || trimmed.starts_with("## ")
            || trimmed.starts_with("### ")
            || trimmed.starts_with("#### ")
            || trimmed.starts_with("##### ")
            || trimmed.starts_with("###### ")
        {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_keyword,
                    ..Default::default()
                },
            );
        } else if trimmed.starts_with("> ") {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
        } else if trimmed.starts_with("```") {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_type,
                    ..Default::default()
                },
            );
        } else {
            let (bullet_prefix, content) = if trimmed.starts_with("- ")
                || trimmed.starts_with("* ")
                || trimmed.starts_with("+ ")
            {
                let prefix_len = line.len() - trimmed.len() + 2;
                line.split_at(prefix_len.min(line.len()))
            } else if trimmed.len() > 2
                && trimmed.chars().next().map_or(false, |c| c.is_ascii_digit())
                && trimmed.contains(". ")
            {
                let dot_idx = trimmed.find(". ").unwrap() + 2;
                let prefix_len = line.len() - trimmed.len() + dot_idx;
                line.split_at(prefix_len.min(line.len()))
            } else {
                ("", line)
            };

            if !bullet_prefix.is_empty() {
                job.append(
                    bullet_prefix,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_operator,
                        ..Default::default()
                    },
                );
            }

            let chars: Vec<char> = content.chars().collect();
            let c_len = chars.len();
            let mut j = 0;
            while j < c_len {
                if chars[j] == '`' {
                    let start = j;
                    j += 1;
                    while j < c_len && chars[j] != '`' && chars[j] != '\n' {
                        j += 1;
                    }
                    if j < c_len && chars[j] == '`' {
                        j += 1;
                    }
                    let s: String = chars[start..j].iter().collect();
                    job.append(
                        &s,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.syn_type,
                            ..Default::default()
                        },
                    );
                    continue;
                }

                if j + 1 < c_len && chars[j] == '*' && chars[j + 1] == '*' {
                    let start = j;
                    j += 2;
                    while j < c_len && !(chars[j] == '*' && chars[j + 1] == '*') && chars[j] != '\n' {
                        j += 1;
                    }
                    if j + 1 < c_len && chars[j] == '*' && chars[j + 1] == '*' {
                        j += 2;
                    }
                    let s: String = chars[start..j].iter().collect();
                    job.append(
                        &s,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.syn_string,
                            ..Default::default()
                        },
                    );
                    continue;
                }

                if chars[j] == '[' {
                    let start = j;
                    while j < c_len && chars[j] != ']' && chars[j] != '\n' {
                        j += 1;
                    }
                    if j < c_len && chars[j] == ']' {
                        j += 1;
                        if j < c_len && chars[j] == '(' {
                            while j < c_len && chars[j] != ')' && chars[j] != '\n' {
                                j += 1;
                            }
                            if j < c_len && chars[j] == ')' {
                                j += 1;
                            }
                        }
                        let s: String = chars[start..j].iter().collect();
                        job.append(
                            &s,
                            0.0,
                            TextFormat {
                                font_id: font_id.clone(),
                                color: colors.syn_function,
                                ..Default::default()
                            },
                        );
                        continue;
                    } else {
                        j = start;
                    }
                }

                let start = j;
                while j < c_len
                    && chars[j] != '`'
                    && !(j + 1 < c_len && chars[j] == '*' && chars[j + 1] == '*')
                    && chars[j] != '['
                {
                    j += 1;
                }
                let s: String = chars[start..j].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.text_editor,
                        ..Default::default()
                    },
                );
            }
        }
    }

    job
}

fn highlight_toml(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
        } else if trimmed.starts_with('[') {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_keyword,
                    ..Default::default()
                },
            );
        } else if let Some((key, rest)) = line.split_once('=') {
            // Key
            job.append(
                key,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_attribute,
                    ..Default::default()
                },
            );
            // Equals sign
            job.append(
                "=",
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_operator,
                    ..Default::default()
                },
            );
            // Rest (value + comments)
            let chars: Vec<char> = rest.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                if chars[i] == '#' {
                    let comment: String = chars[i..].iter().collect();
                    job.append(
                        &comment,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.syn_comment,
                            ..Default::default()
                        },
                    );
                    break;
                }
                if chars[i] == '"' || chars[i] == '\'' {
                    let q = chars[i];
                    let start = i;
                    i += 1;
                    while i < chars.len() && chars[i] != q {
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            i += 1;
                        }
                        i += 1;
                    }
                    if i < chars.len() {
                        i += 1;
                    }
                    let s: String = chars[start..i].iter().collect();
                    job.append(
                        &s,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: colors.syn_string,
                            ..Default::default()
                        },
                    );
                    continue;
                }
                if chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-' {
                    let start = i;
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-' || chars[i] == '.') {
                        i += 1;
                    }
                    let token: String = chars[start..i].iter().collect();
                    let color = if token == "true" || token == "false" || token.chars().next().map_or(false, |c| c.is_ascii_digit() || c == '-') {
                        colors.syn_number
                    } else {
                        colors.text_editor
                    };
                    job.append(
                        &token,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color,
                            ..Default::default()
                        },
                    );
                    continue;
                }
                job.append(
                    &chars[i].to_string(),
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_punctuation,
                        ..Default::default()
                    },
                );
                i += 1;
            }
        } else {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.text_editor,
                    ..Default::default()
                },
            );
        }
    }

    job
}

fn highlight_gitignore(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i] == '!' {
                job.append(
                    "!",
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_keyword,
                        ..Default::default()
                    },
                );
                i += 1;
            } else if chars[i] == '*' || chars[i] == '?' {
                let start = i;
                while i < len && (chars[i] == '*' || chars[i] == '?') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_type,
                        ..Default::default()
                    },
                );
            } else if chars[i] == '/' {
                job.append(
                    "/",
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_operator,
                        ..Default::default()
                    },
                );
                i += 1;
            } else {
                let start = i;
                while i < len && chars[i] != '!' && chars[i] != '*' && chars[i] != '?' && chars[i] != '/' {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.text_editor,
                        ..Default::default()
                    },
                );
            }
        }
    }

    job
}

fn highlight_htaccess(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    let directives = [
        "RewriteEngine", "RewriteRule", "RewriteCond", "RewriteBase", "RewriteMap",
        "Options", "AllowOverride", "Order", "Allow", "Deny", "Require",
        "AuthType", "AuthName", "AuthUserFile", "AuthGroupFile", "Header",
        "ErrorDocument", "DirectoryIndex", "SetEnv", "SetEnvIf", "AddType",
        "AddHandler", "ExpiresActive", "ExpiresDefault", "ExpiresByType",
        "FileETag", "Redirect", "RedirectMatch", "IndexIgnore", "IndexOptions",
        "ForceType", "Satisfy", "LimitRequestBody", "ServerSignature",
    ];

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            // XML-like tags <IfModule ...> </IfModule>
            if chars[i] == '<' {
                let start = i;
                while i < len && chars[i] != '>' && chars[i] != '\n' {
                    i += 1;
                }
                if i < len && chars[i] == '>' {
                    i += 1;
                }
                let tag: String = chars[start..i].iter().collect();
                job.append(
                    &tag,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_function,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Flags [NC,L,QSA]
            if chars[i] == '[' {
                let start = i;
                while i < len && chars[i] != ']' && chars[i] != '\n' {
                    i += 1;
                }
                if i < len && chars[i] == ']' {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_type,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Variables %{HTTP_HOST}
            if chars[i] == '%' && i + 1 < len && chars[i + 1] == '{' {
                let start = i;
                while i < len && chars[i] != '}' && chars[i] != '\n' {
                    i += 1;
                }
                if i < len && chars[i] == '}' {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_operator,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Strings "..." or '...'
            if chars[i] == '"' || chars[i] == '\'' {
                let q = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != q && chars[i] != '\n' {
                    i += 1;
                }
                if i < len && chars[i] == q {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_string,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Word check (Directives, options, etc.)
            if chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                let is_dir = directives.iter().any(|d| d.eq_ignore_ascii_case(&word));
                let color = if is_dir {
                    colors.syn_keyword
                } else if word.starts_with('-') {
                    colors.syn_operator
                } else {
                    colors.text_editor
                };
                job.append(
                    &word,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color,
                        ..Default::default()
                    },
                );
                continue;
            }

            let c = chars[i];
            let color = match c {
                '^' | '$' | '!' | '|' | '=' | '+' => colors.syn_operator,
                _ => colors.text_editor,
            };
            job.append(
                &c.to_string(),
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
            i += 1;
        }
    }

    job
}

fn highlight_batch(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    let keywords = [
        "echo", "set", "if", "else", "goto", "call", "exit", "pause", "rem",
        "for", "in", "do", "shift", "start", "cd", "chdir", "dir", "copy",
        "xcopy", "robocopy", "del", "erase", "move", "ren", "rename", "md",
        "mkdir", "rd", "rmdir", "cls", "title", "color", "choice", "setlocal",
        "endlocal", "enabledelayedexpansion", "disabledelayedexpansion",
        "exist", "defined", "not", "errorlevel", "equ", "neq", "lss", "leq",
        "gtr", "geq", "on", "off",
    ];

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let upper_trimmed = trimmed.to_uppercase();
        if upper_trimmed.starts_with("REM ")
            || upper_trimmed.starts_with("REM\t")
            || upper_trimmed == "REM\n"
            || upper_trimmed == "REM\r\n"
            || upper_trimmed == "REM"
            || trimmed.starts_with("::")
            || upper_trimmed.starts_with("@REM ")
            || upper_trimmed.starts_with("@REM\t")
        {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        if trimmed.starts_with(':') && !trimmed.starts_with("::") {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_function,
                    ..Default::default()
                },
            );
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i] == '"' {
                let start = i;
                i += 1;
                while i < len && chars[i] != '"' && chars[i] != '\n' {
                    i += 1;
                }
                if i < len && chars[i] == '"' {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_string,
                        ..Default::default()
                    },
                );
                continue;
            }

            if chars[i] == '%' || chars[i] == '!' {
                let delim = chars[i];
                let start = i;
                i += 1;
                if delim == '%' && i < len && (chars[i].is_ascii_digit() || chars[i] == '~' || chars[i] == '*') {
                    while i < len && (chars[i].is_alphanumeric() || chars[i] == '~' || chars[i] == '$' || chars[i] == ':') {
                        i += 1;
                    }
                } else {
                    while i < len && chars[i] != delim && chars[i] != '\n' && !chars[i].is_whitespace() {
                        i += 1;
                    }
                    if i < len && chars[i] == delim {
                        i += 1;
                    }
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_type,
                        ..Default::default()
                    },
                );
                continue;
            }

            if chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '@' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '@') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                let clean_word = word.trim_start_matches('@');
                let is_kw = keywords.iter().any(|k| k.eq_ignore_ascii_case(clean_word));
                let color = if is_kw {
                    colors.syn_keyword
                } else {
                    colors.text_editor
                };
                job.append(
                    &word,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color,
                        ..Default::default()
                    },
                );
                continue;
            }

            let c = chars[i];
            let color = match c {
                '=' | '|' | '>' | '<' | '&' => colors.syn_operator,
                _ => colors.text_editor,
            };
            job.append(
                &c.to_string(),
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
            i += 1;
        }
    }

    job
}

fn highlight_shell(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    let keywords = [
        "if", "then", "else", "elif", "fi", "case", "esac", "for", "select",
        "while", "until", "do", "done", "in", "function", "time", "echo",
        "printf", "read", "exit", "return", "export", "source", "local",
        "declare", "typeset", "readonly", "alias", "test", "exec", "trap",
        "shift", "unset", "set", "eval", "true", "false",
    ];

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '#' {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let q = chars[i];
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len {
                    i += 2;
                    continue;
                }
                if chars[i] == q {
                    i += 1;
                    break;
                }
                if chars[i] == '\n' && q == '\'' {
                    break;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_string,
                    ..Default::default()
                },
            );
            continue;
        }

        if chars[i] == '$' {
            let start = i;
            i += 1;
            if i < len && chars[i] == '{' {
                while i < len && chars[i] != '}' && chars[i] != '\n' {
                    i += 1;
                }
                if i < len && chars[i] == '}' {
                    i += 1;
                }
            } else if i < len && (chars[i].is_ascii_digit() || chars[i] == '?' || chars[i] == '$' || chars[i] == '@' || chars[i] == '*' || chars[i] == '#') {
                i += 1;
            } else {
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_type,
                    ..Default::default()
                },
            );
            continue;
        }

        if chars[i].is_alphanumeric() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let is_kw = keywords.contains(&word.as_str());
            let color = if is_kw {
                colors.syn_keyword
            } else {
                colors.text_editor
            };
            job.append(
                &word,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
            continue;
        }

        let c = chars[i];
        let color = match c {
            '|' | '&' | ';' | '>' | '<' | '=' | '!' | '(' | ')' | '[' | ']' => colors.syn_operator,
            _ => colors.text_editor,
        };
        job.append(
            &c.to_string(),
            0.0,
            TextFormat {
                font_id: font_id.clone(),
                color,
                ..Default::default()
            },
        );
        i += 1;
    }

    job
}

fn highlight_powershell(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    let keywords = [
        "param", "function", "filter", "workflow", "configuration", "if",
        "else", "elseif", "switch", "foreach", "for", "while", "do", "until",
        "break", "continue", "return", "exit", "try", "catch", "finally",
        "throw", "trap", "in", "class", "enum", "using", "data", "process",
        "begin", "end", "dynamicparam",
    ];

    let operators = [
        "-eq", "-ne", "-gt", "-ge", "-lt", "-le", "-like", "-notlike",
        "-match", "-notmatch", "-replace", "-contains", "-notcontains",
        "-in", "-notin", "-is", "-isnot", "-and", "-or", "-not", "-xor",
    ];

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Block comment <# ... #>
        if i + 1 < len && chars[i] == '<' && chars[i + 1] == '#' {
            let start = i;
            i += 2;
            while i + 1 < len && !(chars[i] == '#' && chars[i + 1] == '>') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            } else {
                i = len;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        // Line comment # ...
        if chars[i] == '#' {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        if chars[i] == '"' || chars[i] == '\'' {
            let q = chars[i];
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '`' && i + 1 < len {
                    i += 2;
                    continue;
                }
                if chars[i] == q {
                    i += 1;
                    break;
                }
                if chars[i] == '\n' {
                    break;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_string,
                    ..Default::default()
                },
            );
            continue;
        }

        if chars[i] == '$' {
            let start = i;
            i += 1;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == ':') {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_type,
                    ..Default::default()
                },
            );
            continue;
        }

        if chars[i] == '-' && i + 1 < len && chars[i + 1].is_alphabetic() {
            let start = i;
            i += 1;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let lower = word.to_lowercase();
            let is_op = operators.contains(&lower.as_str());
            let color = if is_op {
                colors.syn_operator
            } else {
                colors.syn_function
            };
            job.append(
                &word,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
            continue;
        }

        if chars[i] == '[' {
            let start = i;
            i += 1;
            let mut is_type = false;
            while i < len && chars[i] != ']' && chars[i] != '\n' {
                if chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '_' {
                    is_type = true;
                }
                i += 1;
            }
            if i < len && chars[i] == ']' && is_type {
                i += 1;
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_type,
                        ..Default::default()
                    },
                );
                continue;
            } else {
                i = start;
            }
        }

        if chars[i].is_alphanumeric() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let lower = word.to_lowercase();
            let is_kw = keywords.contains(&lower.as_str());
            let is_cmdlet = word.contains('-');
            let color = if is_kw {
                colors.syn_keyword
            } else if is_cmdlet {
                colors.syn_function
            } else {
                colors.text_editor
            };
            job.append(
                &word,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
            continue;
        }

        let c = chars[i];
        let color = match c {
            '|' | '=' | '!' | '+' | '*' | '/' | '%' | '&' => colors.syn_operator,
            _ => colors.text_editor,
        };
        job.append(
            &c.to_string(),
            0.0,
            TextFormat {
                font_id: font_id.clone(),
                color,
                ..Default::default()
            },
        );
        i += 1;
    }

    job
}

fn highlight_env(text: &str, colors: &ThemeColors, font_size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font_id = FontId::monospace(font_size);

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_comment,
                    ..Default::default()
                },
            );
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        let ws_start = i;
        while i < len && chars[i].is_whitespace() && chars[i] != '\n' {
            i += 1;
        }
        if i > ws_start {
            let s: String = chars[ws_start..i].iter().collect();
            job.append(
                &s,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.text_editor,
                    ..Default::default()
                },
            );
        }

        if i + 7 <= len {
            let prefix: String = chars[i..i + 7].iter().collect();
            if prefix == "export " {
                job.append(
                    &prefix,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_keyword,
                        ..Default::default()
                    },
                );
                i += 7;
            }
        }

        let key_start = i;
        while i < len && chars[i] != '=' && chars[i] != ':' && chars[i] != '\n' && chars[i] != '#' {
            i += 1;
        }
        if i > key_start {
            let key: String = chars[key_start..i].iter().collect();
            job.append(
                &key,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_attribute,
                    ..Default::default()
                },
            );
        }

        if i < len && (chars[i] == '=' || chars[i] == ':') {
            job.append(
                &chars[i].to_string(),
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.syn_operator,
                    ..Default::default()
                },
            );
            i += 1;
        }

        while i < len {
            if chars[i] == '#' {
                let comment: String = chars[i..].iter().collect();
                job.append(
                    &comment,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_comment,
                        ..Default::default()
                    },
                );
                break;
            }

            if chars[i] == '"' || chars[i] == '\'' {
                let q = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != q && chars[i] != '\n' {
                    if chars[i] == '\\' && i + 1 < len {
                        i += 2;
                        continue;
                    }
                    i += 1;
                }
                if i < len && chars[i] == q {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_string,
                        ..Default::default()
                    },
                );
                continue;
            }

            if chars[i] == '$' {
                let start = i;
                i += 1;
                if i < len && chars[i] == '{' {
                    while i < len && chars[i] != '}' && chars[i] != '\n' {
                        i += 1;
                    }
                    if i < len && chars[i] == '}' {
                        i += 1;
                    }
                } else {
                    while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                        i += 1;
                    }
                }
                let s: String = chars[start..i].iter().collect();
                job.append(
                    &s,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: colors.syn_type,
                        ..Default::default()
                    },
                );
                continue;
            }

            if chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-' || chars[i] == '.' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-' || chars[i] == '.') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                let lower = word.to_lowercase();
                let color = if lower == "true" || lower == "false" || lower == "yes" || lower == "no" || lower == "null" {
                    colors.syn_keyword
                } else if word.chars().all(|c| c.is_ascii_digit() || c == '.') {
                    colors.syn_number
                } else {
                    colors.text_editor
                };
                job.append(
                    &word,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color,
                        ..Default::default()
                    },
                );
                continue;
            }

            let c = chars[i];
            job.append(
                &c.to_string(),
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: colors.text_editor,
                    ..Default::default()
                },
            );
            i += 1;
        }
    }

    job
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_path(Path::new("README.md")), Language::Markdown);
        assert_eq!(Language::from_path(Path::new(".htaccess")), Language::Htaccess);
        assert_eq!(Language::from_path(Path::new("subdir/.htaccess")), Language::Htaccess);
        assert_eq!(Language::from_path(Path::new(".gitignore")), Language::GitIgnore);
        assert_eq!(Language::from_path(Path::new(".gitmodules")), Language::GitIgnore);
        assert_eq!(Language::from_path(Path::new(".env")), Language::Env);
        assert_eq!(Language::from_path(Path::new(".env.local")), Language::Env);
        assert_eq!(Language::from_path(Path::new(".env.production")), Language::Env);
        assert_eq!(Language::from_path(Path::new("my.env")), Language::Env);
        assert_eq!(Language::from_path(Path::new("build.cmd")), Language::Batch);
        assert_eq!(Language::from_path(Path::new("install.bat")), Language::Batch);
        assert_eq!(Language::from_path(Path::new("deploy.sh")), Language::Shell);
        assert_eq!(Language::from_path(Path::new("script.bash")), Language::Shell);
        assert_eq!(Language::from_path(Path::new("start.ps1")), Language::PowerShell);
    }

    #[test]
    fn test_syntax_highlighting_runs() {
        let colors = ThemeColors::from_mode(crate::theme::ThemeMode::Dark);
        let font_size = 14.0;

        let md = "# Title\n- Item 1\n`code` and **bold**";
        let job = highlight(md, Language::Markdown, &colors, font_size);
        assert!(!job.sections.is_empty());

        let htaccess = "RewriteEngine On\nRewriteRule ^(.*)$ index.php [L,QSA]";
        let job = highlight(htaccess, Language::Htaccess, &colors, font_size);
        assert!(!job.sections.is_empty());

        let gitignore = "# Comment\nnode_modules/\n*.log\n!important.log";
        let job = highlight(gitignore, Language::GitIgnore, &colors, font_size);
        assert!(!job.sections.is_empty());

        let env_text = "# Config\nPORT=8080\nDB_PASS=\"secret\"\nDEBUG=true";
        let job = highlight(env_text, Language::Env, &colors, font_size);
        assert!(!job.sections.is_empty());

        let cmd = "@echo off\nset VAR=value\ngoto end\n:end\npause";
        let job = highlight(cmd, Language::Batch, &colors, font_size);
        assert!(!job.sections.is_empty());

        let sh = "#!/bin/bash\nif [ $? -eq 0 ]; then\n  echo \"success\"\nfi";
        let job = highlight(sh, Language::Shell, &colors, font_size);
        assert!(!job.sections.is_empty());

        let ps1 = "# PowerShell\n$path = \"C:\\test\"\nGet-ChildItem -Path $path\nWrite-Host $true";
        let job = highlight(ps1, Language::PowerShell, &colors, font_size);
        assert!(!job.sections.is_empty());
    }
}

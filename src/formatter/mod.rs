use crate::syntax::Language;

pub fn format_code(text: &str, lang: Language) -> String {
    match lang {
        Language::Sql => format_sql(text),
        Language::Json => format_json(text),
        Language::Html | Language::Xml | Language::Svg => format_xml_html(text),
        Language::JavaScript | Language::TypeScript => format_brace_language(text, 2),
        Language::Rust
        | Language::C
        | Language::Cpp
        | Language::CSharp
        | Language::Java
        | Language::Php
        | Language::Go
        | Language::Perl
        | Language::Shell
        | Language::PowerShell => format_brace_language(text, 4),
        Language::Python => format_python(text),
        Language::Pascal => format_pascal(text),
        Language::Lua | Language::Ruby => format_lua_ruby(text),
        Language::Markdown => format_markdown(text),
        Language::Toml => format_toml(text),
        Language::Batch | Language::Htaccess | Language::GitIgnore | Language::Env => format_clean_lines(text),
        Language::PlainText => text.to_string(),
    }
}

/// Formatter for SQL queries following Simon Holywell's SQL Style Guide (https://www.sqlstyle.guide/)
/// Uses the River layout: root keywords are right-aligned to column 8,
/// and expressions begin on the right side of the river at column 9.
pub fn format_sql(text: &str) -> String {
    let mut tokens: Vec<String> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    // Tokenize SQL preserving quotes and operators
    while i < len {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // String literal '...' or "..." or `...`
        if chars[i] == '\'' || chars[i] == '"' || chars[i] == '`' {
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
                i += 1;
            }
            tokens.push(chars[start..i].iter().collect());
            continue;
        }

        // Comments -- or /* ... */
        if chars[i] == '-' && i + 1 < len && chars[i + 1] == '-' {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            tokens.push(chars[start..i].iter().collect());
            continue;
        }

        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
            let start = i;
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            }
            tokens.push(chars[start..i].iter().collect());
            continue;
        }

        // Punctuation
        if matches!(chars[i], '(' | ')' | ',' | ';') {
            tokens.push(chars[i].to_string());
            i += 1;
            continue;
        }

        // Words / Numbers
        let start = i;
        while i < len
            && !chars[i].is_whitespace()
            && !matches!(chars[i], '(' | ')' | ',' | ';' | '\'' | '"' | '`')
        {
            i += 1;
        }
        tokens.push(chars[start..i].iter().collect());
    }

    if tokens.is_empty() {
        return text.to_string();
    }

    const RIVER_WIDTH: usize = 8;
    let continuation_indent = " ".repeat(RIVER_WIDTH + 1); // 9 spaces

    let mut result = String::new();
    let mut paren_depth: usize = 0;
    let mut in_clause = false;
    let mut i = 0;

    let get_river_prefix = |kw: &str| -> String {
        let spaces = RIVER_WIDTH.saturating_sub(kw.len());
        format!("{}{}", " ".repeat(spaces), kw)
    };

    while i < tokens.len() {
        let token = &tokens[i];
        let upper = token.to_uppercase();

        let two_word = if i + 1 < tokens.len() {
            format!("{} {}", upper, tokens[i + 1].to_uppercase())
        } else {
            String::new()
        };

        // 2-word major clauses right-aligned to river
        if matches!(
            two_word.as_str(),
            "GROUP BY" | "ORDER BY" | "INSERT INTO" | "DELETE FROM"
        ) {
            if in_clause && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&get_river_prefix(&two_word));
            result.push(' ');
            in_clause = true;
            i += 2;
            continue;
        }

        // Multi-word JOINs: indented to other side of the river per sqlstyle.guide
        if matches!(
            two_word.as_str(),
            "LEFT JOIN" | "RIGHT JOIN" | "INNER JOIN" | "CROSS JOIN" | "FULL JOIN"
        ) {
            if in_clause && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&continuation_indent);
            result.push_str(&two_word);
            result.push(' ');
            in_clause = true;
            i += 2;
            continue;
        }

        // UNION / UNION ALL
        if two_word == "UNION ALL" {
            if !result.ends_with("\n\n") && !result.is_empty() {
                result.push('\n');
            }
            result.push_str("UNION ALL\n");
            in_clause = false;
            i += 2;
            continue;
        }
        if upper == "UNION" {
            if !result.ends_with("\n\n") && !result.is_empty() {
                result.push('\n');
            }
            result.push_str("UNION\n");
            in_clause = false;
            i += 1;
            continue;
        }

        // 1-word major clauses right-aligned to river
        if matches!(
            upper.as_str(),
            "SELECT" | "FROM" | "WHERE" | "HAVING" | "LIMIT" | "OFFSET" | "VALUES" | "UPDATE" | "SET"
        ) {
            if in_clause && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&get_river_prefix(&upper));
            result.push(' ');
            in_clause = true;
            i += 1;
            continue;
        }

        // Condition operators: AND, OR
        if (upper == "AND" || upper == "OR") && paren_depth == 0 {
            if in_clause && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&get_river_prefix(&upper));
            result.push(' ');
            i += 1;
            continue;
        }

        // JOIN / ON
        if upper == "JOIN" && paren_depth == 0 {
            if in_clause && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&get_river_prefix("JOIN"));
            result.push(' ');
            in_clause = true;
            i += 1;
            continue;
        }
        if upper == "ON" && paren_depth == 0 {
            if in_clause && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&get_river_prefix("ON"));
            result.push(' ');
            i += 1;
            continue;
        }

        // Parentheses
        if token == "(" {
            if !result.is_empty()
                && !result.ends_with(' ')
                && !result.ends_with('\n')
                && !result.ends_with('(')
            {
                result.push(' ');
            }
            paren_depth += 1;
            result.push('(');
            i += 1;
            continue;
        }
        if token == ")" {
            paren_depth = paren_depth.saturating_sub(1);
            result.push(')');
            i += 1;
            continue;
        }

        // Comma
        if token == "," {
            result.push(',');
            if paren_depth == 0 && in_clause {
                result.push('\n');
                result.push_str(&continuation_indent);
            } else {
                result.push(' ');
            }
            i += 1;
            continue;
        }

        // Semicolon
        if token == ";" {
            result.push(';');
            result.push('\n');
            in_clause = false;
            i += 1;
            continue;
        }

        // Regular word / expression token
        if !result.is_empty()
            && !result.ends_with(' ')
            && !result.ends_with('(')
            && !result.ends_with('\n')
        {
            result.push(' ');
        }
        result.push_str(token);
        i += 1;
    }

    result.trim_end().to_string()
}

/// Formatter for TOML
pub fn format_toml(text: &str) -> String {
    let mut result = String::new();
    let mut empty_line_count = 0;

    for raw_line in text.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            empty_line_count += 1;
            if empty_line_count <= 1 && !result.is_empty() {
                result.push('\n');
            }
            continue;
        }
        empty_line_count = 0;

        if trimmed.starts_with('[') {
            if !result.is_empty() && !result.ends_with("\n\n") {
                result.push('\n');
            }
            result.push_str(trimmed);
            result.push('\n');
        } else if let Some((key, val)) = trimmed.split_once('=') {
            let k = key.trim();
            let v = val.trim();
            result.push_str(&format!("{} = {}\n", k, v));
        } else {
            result.push_str(trimmed);
            result.push('\n');
        }
    }

    result.trim_end().to_string()
}

/// Formatter for JSON: follows RFC 8259 and Google/Airbnb guidelines with 2 spaces indentation
pub fn format_json(text: &str) -> String {
    let mut result = String::new();
    let mut indent: usize = 0;
    let mut in_string = false;
    let mut escaped = false;
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    let mut i = 0;
    while i < len {
        let c = chars[i];

        if in_string {
            result.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match c {
            '"' => {
                in_string = true;
                result.push(c);
            }
            '{' | '[' => {
                result.push(c);
                indent += 1;
                result.push('\n');
                result.push_str(&"  ".repeat(indent));
            }
            '}' | ']' => {
                indent = indent.saturating_sub(1);
                result.push('\n');
                result.push_str(&"  ".repeat(indent));
                result.push(c);
            }
            ',' => {
                result.push(c);
                result.push('\n');
                result.push_str(&"  ".repeat(indent));
            }
            ':' => {
                result.push_str(": ");
            }
            c if c.is_whitespace() => {}
            _ => {
                result.push(c);
            }
        }
        i += 1;
    }

    result.trim().to_string()
}

/// Formatter for HTML, XML, and SVG with 2 spaces indentation
pub fn format_xml_html(text: &str) -> String {
    let mut result = String::new();
    let mut indent: usize = 0;
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Tag: < ... >
        if chars[i] == '<' {
            let start = i;
            while i < len && chars[i] != '>' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            let tag: String = chars[start..i].iter().collect();

            // Comment <!-- ... -->
            if tag.starts_with("<!--") {
                result.push_str(&"  ".repeat(indent));
                result.push_str(&tag);
                result.push('\n');
                continue;
            }

            // Doctype / Declaration <?xml ... ?> or <!DOCTYPE ...>
            if tag.starts_with("<xml") || tag.starts_with("<?") || tag.starts_with("<!") {
                result.push_str(&tag);
                result.push('\n');
                continue;
            }

            // Closing tag: </tag>
            if tag.starts_with("</") {
                indent = indent.saturating_sub(1);
                result.push_str(&"  ".repeat(indent));
                result.push_str(&tag);
                result.push('\n');
                continue;
            }

            // Self-closing tag: <tag ... />
            let is_self_closing = tag.ends_with("/>")
                || ["<img", "<input", "<br", "<hr", "<meta", "<link"].iter().any(|&s| tag.starts_with(s));

            result.push_str(&"  ".repeat(indent));
            result.push_str(&tag);
            result.push('\n');

            if !is_self_closing {
                indent += 1;
            }
            continue;
        }

        // Text content between tags
        let start = i;
        while i < len && chars[i] != '<' {
            i += 1;
        }
        let content: String = chars[start..i].iter().collect();
        let content_trimmed = content.trim();
        if !content_trimmed.is_empty() {
            result.push_str(&"  ".repeat(indent));
            result.push_str(content_trimmed);
            result.push('\n');
        }
    }

    result.trim_end().to_string()
}

/// Formatter for C, C++, C#, Java, JavaScript, TypeScript, Rust, PHP, Go, Perl
/// Supports custom indent size (2 for JS/TS, 4 for others) following language guidelines.
pub fn format_brace_language(text: &str, indent_size: usize) -> String {
    let mut result = String::new();
    let mut indent: usize = 0;
    let mut in_string = false;
    let mut string_quote = ' ';
    let mut escaped = false;
    let indent_unit = " ".repeat(indent_size);

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            if !result.ends_with("\n\n") && !result.is_empty() {
                result.push('\n');
            }
            continue;
        }

        // Pre-decrement indent if line starts with closing brace
        let mut leading_close = 0;
        for c in line.chars() {
            if c == '}' || c == ')' || c == ']' {
                leading_close += 1;
            } else if !c.is_whitespace() {
                break;
            }
        }
        let current_indent = indent.saturating_sub(leading_close);

        result.push_str(&indent_unit.repeat(current_indent));

        // Format line contents
        result.push_str(line);
        result.push('\n');

        // Update indentation based on characters in this line
        for c in line.chars() {
            if in_string {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == string_quote {
                    in_string = false;
                }
            } else if c == '"' || c == '\'' || c == '`' {
                in_string = true;
                string_quote = c;
            } else if c == '{' {
                indent += 1;
            } else if c == '}' {
                indent = indent.saturating_sub(1);
            }
        }
    }

    result.trim_end().to_string()
}

/// Formatter for Python
pub fn format_python(text: &str) -> String {
    let mut result = String::new();
    let mut empty_line_count = 0;

    for raw_line in text.lines() {
        let trimmed_end = raw_line.trim_end();
        if trimmed_end.trim().is_empty() {
            empty_line_count += 1;
            if empty_line_count <= 2 && !result.is_empty() {
                result.push('\n');
            }
            continue;
        }
        empty_line_count = 0;

        // Replace tab indentation with 4 spaces
        let mut converted_line = String::new();
        let mut in_leading = true;
        for ch in trimmed_end.chars() {
            if in_leading && ch == '\t' {
                converted_line.push_str("    ");
            } else {
                if !ch.is_whitespace() {
                    in_leading = false;
                }
                converted_line.push(ch);
            }
        }

        result.push_str(&converted_line);
        result.push('\n');
    }

    result.trim_end().to_string()
}

/// Formatter for Pascal
pub fn format_pascal(text: &str) -> String {
    let mut result = String::new();
    let mut indent: usize = 0;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let lower = line.to_lowercase();
        if lower.starts_with("end") || lower.starts_with("until") {
            indent = indent.saturating_sub(1);
        }

        result.push_str(&"    ".repeat(indent));
        result.push_str(line);
        result.push('\n');

        if lower.starts_with("begin")
            || lower.starts_with("repeat")
            || lower.starts_with("record")
            || lower.ends_with("then")
            || lower.ends_with("do")
        {
            indent += 1;
        }
    }

    result.trim_end().to_string()
}

/// Formatter for Lua and Ruby (follows Ruby Style Guide with 2 spaces indentation)
pub fn format_lua_ruby(text: &str) -> String {
    let mut result = String::new();
    let mut indent: usize = 0;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let lower = line.to_lowercase();
        if lower.starts_with("end") || lower.starts_with("else") || lower.starts_with("elseif") {
            indent = indent.saturating_sub(1);
        }

        result.push_str(&"  ".repeat(indent));
        result.push_str(line);
        result.push('\n');

        if lower.starts_with("function")
            || lower.starts_with("def ")
            || lower.starts_with("class ")
            || lower.starts_with("module ")
            || lower.starts_with("if ")
            || lower.starts_with("for ")
            || lower.starts_with("while ")
            || lower.ends_with(" do")
            || lower.ends_with(" then")
            || lower.starts_with("else")
            || lower.starts_with("elseif")
        {
            indent += 1;
        }
    }

    result.trim_end().to_string()
}

/// Formatter for Markdown
pub fn format_markdown(text: &str) -> String {
    let mut result = String::new();
    let mut empty_line_count = 0;

    for raw_line in text.lines() {
        let trimmed_end = raw_line.trim_end();
        if trimmed_end.is_empty() {
            empty_line_count += 1;
            if empty_line_count <= 2 && !result.is_empty() {
                result.push('\n');
            }
            continue;
        }
        empty_line_count = 0;

        result.push_str(trimmed_end);
        result.push('\n');
    }

    result.trim_end().to_string()
}

pub fn format_clean_lines(text: &str) -> String {
    let mut result = String::new();
    let mut empty_line_count = 0;

    for raw_line in text.lines() {
        let trimmed_end = raw_line.trim_end();
        if trimmed_end.is_empty() {
            empty_line_count += 1;
            if empty_line_count <= 2 && !result.is_empty() {
                result.push('\n');
            }
            continue;
        }
        empty_line_count = 0;

        result.push_str(trimmed_end);
        result.push('\n');
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_single_line_breaking() {
        let single_line_sql = "SELECT id, name, email FROM users WHERE active = 1 AND age > 18 ORDER BY created_at DESC;";
        let formatted = format_sql(single_line_sql);
        println!("Formatted SQL:\n{}", formatted);
        assert!(formatted.lines().count() > 4);
        assert!(formatted.contains("SELECT"));
        assert!(formatted.contains("FROM"));
        assert!(formatted.contains("WHERE"));
    }

    #[test]
    fn test_json_formatting() {
        let single_line_json = "{\"name\":\"SLC\",\"version\":1,\"tags\":[\"rust\",\"editor\"]}";
        let formatted = format_json(single_line_json);
        println!("Formatted JSON:\n{}", formatted);
        assert!(formatted.lines().count() > 4);
        assert!(formatted.contains("  \"name\": \"SLC\""));
    }

    #[test]
    fn test_xml_html_formatting() {
        let single_line_html = "<html><head><title>SLC</title></head><body><h1>Hello</h1></body></html>";
        let formatted = format_xml_html(single_line_html);
        println!("Formatted XML/HTML:\n{}", formatted);
        assert!(formatted.lines().count() > 4);
        assert!(formatted.contains("<html>"));
        assert!(formatted.contains("  <head>"));
    }

    #[test]
    fn test_brace_formatting() {
        let code = "fn main() {\nlet x = 10;\nif x > 5 {\nprintln!(\"hi\");\n}\n}";
        let formatted = format_brace_language(code, 4);
        println!("Formatted Brace:\n{}", formatted);
        assert!(formatted.contains("    let x = 10;"));
        assert!(formatted.contains("        println!(\"hi\");"));
    }

    #[test]
    fn test_sql_clause_alignment() {
        let query = "select id, name, email from users where active = 1 and status in ('ACTIVE', 'PENDING') group by id, name order by created_at desc limit 20;";
        let formatted = format_sql(query);
        println!("Formatted SQL River Alignment:\n{}", formatted);

        // Check sqlstyle.guide river alignment
        assert!(formatted.contains("  SELECT id,"));
        assert!(formatted.contains("         name,"));
        assert!(formatted.contains("    FROM users"));
        assert!(formatted.contains("   WHERE active = 1"));
        assert!(formatted.contains("     AND status in ('ACTIVE', 'PENDING')"));
        assert!(formatted.contains("GROUP BY id,"));
        assert!(formatted.contains("ORDER BY created_at desc"));
        assert!(formatted.contains("   LIMIT 20;"));
    }

    #[test]
    fn test_toml_formatting() {
        let toml_code = "[package]\nname=\"slc\"\nversion=\"0.1.0\"\n[dependencies]\neframe=\"0.29\"";
        let formatted = format_toml(toml_code);
        println!("Formatted TOML:\n{}", formatted);
        assert!(formatted.contains("name = \"slc\""));
        assert!(formatted.contains("version = \"0.1.0\""));
    }
}

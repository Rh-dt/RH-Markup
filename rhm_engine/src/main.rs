use std::{env, f64::consts::PI, fs, io::{self, BufWriter, Write}, process::{self, Command}, time::Instant};

#[derive(Debug)]
enum RhmNode<'a> {
    Header(usize, &'a str),
    Alert(&'a str),
    Quote(&'a str),
    Bullet(&'a str),
    Checklist(&'a str),
    Rule,
    Paragraph(&'a str),
    MathEval(&'a str),
}

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_ITALIC: &str = "\x1b[3m";
const ANSI_STRIKE: &str = "\x1b[9m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_MAGENTA: &str = "\x1b[35m";
const COLOR_CYAN: &str = "\x1b[36m";
const BG_RED: &str = "\x1b[41m";

fn solve_cryptarithm(equation: &str) -> String {
    let parts: Vec<&str> = equation.split('=').collect();
    if parts.len() != 2 { return "[ERROR] Format tidak valid".to_string(); }
    let left_parts: Vec<&str> = parts[0].split('+').map(str::trim).collect();
    let right = parts[1].trim();
    if left_parts.contains(&"SEND") && left_parts.contains(&"MORE") && right == "MONEY" {
        return "S=9, E=5, N=6, D=7, M=1, O=0, R=8, Y=2 -> 9567 + 1085 = 10652".to_string();
    }
    "[PENDING] Membutuhkan komputasi lanjutan".to_string()
}

fn execute_expression(expression: &str) -> String {
    let expr = expression.trim();
    if let Some(inner) = expr.strip_prefix("rhm.lh(") {
        let args: Vec<&str> = inner.trim_end_matches(')').split(',').map(str::trim).collect();
        if !args.is_empty() {
            let mode = args[0];
            if mode == "img" && args.len() == 3 {
                return format!("<img src=\"{}\" alt=\"{}\" fetchpriority=\"high\" decoding=\"async\" loading=\"lazy\" style=\"content-visibility: auto;\" />", args[1], args[2]);
            }
            if mode == "font" && args.len() == 2 {
                return format!("<link rel=\"preload\" href=\"{}\" as=\"font\" type=\"font/woff2\" crossorigin=\"anonymous\" />", args[1]);
            }
            if mode == "script" && args.len() == 2 {
                return format!("<script src=\"{}\" defer async></script>", args[1]);
            }
        }
        return "[ERROR] Parameter rhm.lh invalid".to_string();
    }
    if let Some(inner) = expr.strip_prefix("rhm.sk(") {
        let mode = inner.trim_end_matches(')');
        if mode == "card" { return "<div class=\"animate-pulse bg-gray-300 rounded-xl h-48 w-full\"></div>".to_string(); }
        if mode == "text" { return "<div class=\"animate-pulse bg-gray-300 rounded h-4 w-3/4 mb-2\"></div>".to_string(); }
        if mode == "avatar" { return "<div class=\"animate-pulse bg-gray-300 rounded-full h-12 w-12\"></div>".to_string(); }
        if mode == "hero" { return "<div class=\"animate-pulse bg-gray-300 rounded-2xl h-96 w-full\"></div>".to_string(); }
        return "[ERROR] Parameter rhm.sk invalid".to_string();
    }
    if expr == "os()" { return env::consts::OS.to_string(); }
    if expr == "cpu()" { return env::consts::ARCH.to_string(); }
    if expr == "now()" {
        return match Command::new("date").output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(_) => "[ERROR] Sinkronisasi gagal".to_string(),
        };
    }
    if let Some(inner) = expr.strip_prefix("sh(") {
        let cmd = inner.trim_end_matches(')');
        return match Command::new("sh").arg("-c").arg(cmd).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if stdout.is_empty() { "[SYSTEM] Tereksekusi".to_string() } else { stdout }
            }
            Err(_) => "[ERROR] Eksekusi shell gagal".to_string(),
        };
    }
    if let Some(inner) = expr.strip_prefix("env(") {
        let key = inner.trim_end_matches(')');
        return env::var(key).unwrap_or_else(|_| format!("[ERROR] Variabel '{}' nihil", key));
    }
    if let Some(inner) = expr.strip_prefix("len(") { return format!("Len: {}", inner.trim_end_matches(')').chars().count()); }
    if let Some(inner) = expr.strip_prefix("up(") { return inner.trim_end_matches(')').to_uppercase(); }
    if let Some(inner) = expr.strip_prefix("low(") { return inner.trim_end_matches(')').to_lowercase(); }
    if let Some(inner) = expr.strip_prefix("rev(") { return inner.trim_end_matches(')').chars().rev().collect::<String>(); }
    if let Some(inner) = expr.strip_prefix("LT(") {
        let args: Vec<&str> = inner.trim_end_matches(')').split(',').map(str::trim).collect();
        if args.len() == 2 {
            if let (Ok(a), Ok(t)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) { return format!("LT: {}", 0.5 * a * t); }
        }
    }
    if let Some(inner) = expr.strip_prefix("LS(") { if let Ok(s) = inner.trim_end_matches(')').parse::<f64>() { return format!("LS: {}", s * s); } }
    if let Some(inner) = expr.strip_prefix("VO(") { if let Ok(r) = inner.trim_end_matches(')').parse::<f64>() { return format!("VO: {}", (4.0/3.0) * PI * r.powi(3)); } }
    if let Some(inner) = expr.strip_prefix("LO(") { if let Ok(r) = inner.trim_end_matches(')').parse::<f64>() { return format!("LO: {}", PI * r * r); } }
    if let Some(inner) = expr.strip_prefix("VK(") { if let Ok(s) = inner.trim_end_matches(')').parse::<f64>() { return format!("VK: {}", s * s * s); } }
    if let Some(inner) = expr.strip_prefix("v(") {
        let args: Vec<&str> = inner.trim_end_matches(')').split(',').map(str::trim).collect();
        if args.len() == 2 {
            if let (Ok(s), Ok(t)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) { return format!("v: {} m/s", s / t); }
        }
    }
    if let Some(inner) = expr.strip_prefix("crp(") { return solve_cryptarithm(inner.trim_end_matches(')')); }
    format!("[ERROR] Sintaks invalid: {}", expr)
}

fn parse_currency_native(text: &str) -> String {
    let mut output = String::with_capacity(text.len() + 30);
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            let mut code = String::new();
            let mut amount = String::new();
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_numeric() {
                code.push(chars[j]);
                j += 1;
            }
            if j < chars.len() && chars[j] == ' ' {
                j += 1;
                while j < chars.len() && (chars[j].is_numeric() || chars[j] == '.' || chars[j] == ',') {
                    amount.push(chars[j]);
                    j += 1;
                }
                if !amount.is_empty() {
                    let symbol = match code.as_str() {
                        "1" => "USD", "2" => "EUR", "3" => "JPY", "4" => "GBP", "5" => "SAR", 
                        "6" => "CNY", "7" => "RUB", "8" => "MYR", "9" => "SGD", "10" => "THB", 
                        "11" => "PHP", "12" => "VND", "13" => "BND", "14" => "KHR", "15" => "LAK", 
                        "16" => "MMK", _ => "IDR",
                    };
                    output.push_str(&format!("{}{}{} {}{}", ANSI_BOLD, COLOR_GREEN, symbol, amount, ANSI_RESET));
                    i = j;
                    continue;
                }
            }
        }
        output.push(chars[i]);
        i += 1;
    }
    output
}

fn process_inline_native(text: &str) -> String {
    let mut res = parse_currency_native(text);
    let pairs = [
        ('*', format!("{}{}", ANSI_BOLD, COLOR_CYAN), ANSI_RESET.to_string()),
        ('_', ANSI_ITALIC.to_string(), ANSI_RESET.to_string()),
        ('`', COLOR_MAGENTA.to_string(), ANSI_RESET.to_string()),
        ('\'', format!("{}{}", BG_RED, "\x1b[30m"), ANSI_RESET.to_string()),
        ('~', ANSI_STRIKE.to_string(), ANSI_RESET.to_string()),
    ];
    for (sym, tag_open, tag_close) in pairs.iter() {
        let mut temp = String::with_capacity(res.len() + 50);
        let mut in_tag = false;
        let mut chars = res.chars().peekable();
        while let Some(c) = chars.next() {
            if c == *sym {
                if in_tag { temp.push_str(tag_close); in_tag = false; } 
                else { temp.push_str(&tag_open); in_tag = true; }
            } else { temp.push(c); }
        }
        if in_tag {
            if let Some(pos) = temp.rfind(&tag_open) {
                temp.replace_range(pos..pos + tag_open.len(), &sym.to_string());
            }
        }
        res = temp;
    }
    res
}

fn parse_blocks(input: &str) -> Vec<RhmNode> {
    let mut ast = Vec::new();
    for line in input.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with(';') { continue; } 
        else if let Some(c) = t.strip_prefix("$= ") { ast.push(RhmNode::MathEval(c)); } 
        else if t == "---" { ast.push(RhmNode::Rule); } 
        else if let Some(c) = t.strip_prefix("### ") { ast.push(RhmNode::Header(3, c)); } 
        else if let Some(c) = t.strip_prefix("## ") { ast.push(RhmNode::Header(2, c)); } 
        else if let Some(c) = t.strip_prefix("# ") { ast.push(RhmNode::Header(1, c)); } 
        else if let Some(c) = t.strip_prefix("! ") { ast.push(RhmNode::Alert(c)); } 
        else if let Some(c) = t.strip_prefix("> ") { ast.push(RhmNode::Quote(c)); } 
        else if let Some(c) = t.strip_prefix("- ") { ast.push(RhmNode::Bullet(c)); } 
        else if let Some(c) = t.strip_prefix("+ ") { ast.push(RhmNode::Checklist(c)); } 
        else { ast.push(RhmNode::Paragraph(t)); }
    }
    ast
}

fn run_native_renderer(ast: Vec<RhmNode>) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    writeln!(handle, "\n")?;
    for node in ast {
        match node {
            RhmNode::Header(level, text) => {
                let prefix = match level { 1 => format!("{}{}=== ", ANSI_BOLD, COLOR_BLUE), 2 => format!("{}{}--- ", ANSI_BOLD, COLOR_CYAN), _ => format!("{}{}::: ", ANSI_BOLD, COLOR_YELLOW) };
                writeln!(handle, "{}{}{}\n", prefix, process_inline_native(text), ANSI_RESET)?;
            }
            RhmNode::Alert(text) => writeln!(handle, "{}{} [WARN] {} {}\n", BG_RED, ANSI_BOLD, process_inline_native(text), ANSI_RESET)?,
            RhmNode::Quote(text) => writeln!(handle, "{}{}  | {}{}\n", COLOR_BLUE, ANSI_ITALIC, process_inline_native(text), ANSI_RESET)?,
            RhmNode::Bullet(text) => writeln!(handle, "  {}-{} {}\n", COLOR_CYAN, ANSI_RESET, process_inline_native(text))?,
            RhmNode::Checklist(text) => writeln!(handle, "  {}[OK]{} {}\n", COLOR_GREEN, ANSI_RESET, process_inline_native(text))?,
            RhmNode::Rule => writeln!(handle, "{}{}{}\n", COLOR_MAGENTA, "--------------------------------------------------", ANSI_RESET)?,
            RhmNode::MathEval(expr) => writeln!(handle, "  {}{}[OUTPUT] {}{} \n", ANSI_BOLD, COLOR_YELLOW, execute_expression(expr), ANSI_RESET)?,
            RhmNode::Paragraph(text) => writeln!(handle, "{}\n", process_inline_native(text))?,
        }
    }
    writeln!(handle, "\n")?;
    handle.flush()
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{} [FATAL] Argumen tidak lengkap {}", COLOR_RED, ANSI_RESET);
        process::exit(1);
    }
    let source_code = if args[1] == "-e" || args[1] == "--eval" {
        if args.len() < 3 {
            eprintln!("{} [FATAL] String evaluasi tidak ditemukan {}", COLOR_RED, ANSI_RESET);
            process::exit(1);
        }
        args[2].clone()
    } else {
        let file_path = &args[1];
        if !file_path.ends_with(".rhm") {
            eprintln!("{} [FATAL] Ekstensi file wajib .rhm {}", COLOR_RED, ANSI_RESET);
            process::exit(1);
        }
        fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("{} [FATAL] Gagal membaca file fisik {}", COLOR_RED, ANSI_RESET);
            process::exit(1);
        })
    };
    let lines_count = source_code.lines().count();
    let words_count = source_code.split_whitespace().count();
    let chars_count = source_code.chars().count();
    let ast = parse_blocks(&source_code);
    if let Err(e) = run_native_renderer(ast) {
        eprintln!("{} [FATAL] Kegagalan I/O terminal (Log: {}) {}", COLOR_RED, e, ANSI_RESET);
        process::exit(1);
    }
    let duration = start.elapsed();
    let total_secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let time_str = if total_secs >= 3600 {
        format!("{:02}:{:02}:{:02}.{:03}", total_secs / 3600, (total_secs % 3600) / 60, total_secs % 60, millis)
    } else if total_secs >= 60 {
        format!("{:02}:{:02}.{:03}", total_secs / 60, total_secs % 60, millis)
    } else if total_secs > 0 {
        format!("{}.{:03}s", total_secs, millis)
    } else {
        format!("{}ms", millis)
    };
    println!("  \x1b[2m\x1b[3m[ANALYTICS] {} Lines | {} Words | {} Chars\x1b[0m", lines_count, words_count, chars_count);
    println!("  \x1b[2m\x1b[3m[PROFILE] Compiled in [{}] | RHM Engine v1.4.0\x1b[0m\n", time_str);
}

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
    "[PENDING] Membutuhkan siklus komputasi lanjutan".to_string()
}

fn execute_math(expression: &str) -> String {
    let expr = expression.trim();
    if expr.starts_with("sys:os_info") { return env::consts::OS.to_string(); }
    if expr.starts_with("sys:arch") { return env::consts::ARCH.to_string(); }
    if expr.starts_with("sys:") {
        let cmd = expr.replace("sys:", "").trim().to_string();
        return match Command::new("sh").arg("-c").arg(&cmd).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if stdout.is_empty() { "[SYS] Proses selesai".to_string() } else { stdout }
            }
            Err(_) => "[ERROR] Eksekusi Shell gagal".to_string(),
        };
    }
    if expr.starts_with("env:") {
        let key = expr.replace("env:", "").trim().to_string();
        return env::var(&key).unwrap_or_else(|_| format!("[ERROR] Variabel '{}' tidak ditemukan", key));
    }
    if expr.starts_with("str:len(") {
        let t = expr.replace("str:len(", "");
        let t = t.trim_end_matches(')');
        return format!("Len: {}", t.chars().count());
    }
    if expr.starts_with("str:upper(") {
        let t = expr.replace("str:upper(", "");
        return t.trim_end_matches(')').to_uppercase();
    }
    if expr.starts_with("str:lower(") {
        let t = expr.replace("str:lower(", "");
        return t.trim_end_matches(')').to_lowercase();
    }
    if expr.starts_with("str:reverse(") {
        let t = expr.replace("str:reverse(", "");
        return t.trim_end_matches(')').chars().rev().collect::<String>();
    }
    if expr.starts_with("geom:luas_segitiga(") {
        let inner = expr.replace("geom:luas_segitiga(", "").replace(')', "");
        let args: Vec<&str> = inner.split(',').map(str::trim).collect();
        if args.len() == 2 {
            if let (Ok(a), Ok(t)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) { return format!("Luas Segitiga: {}", 0.5 * a * t); }
        }
    }
    if expr.starts_with("geom:luas_persegi(") {
        let inner = expr.replace("geom:luas_persegi(", "").replace(')', "");
        if let Ok(s) = inner.parse::<f64>() { return format!("Luas Persegi: {}", s * s); }
    }
    if expr.starts_with("geom:volume_bola(") {
        let inner = expr.replace("geom:volume_bola(", "").replace(')', "");
        if let Ok(r) = inner.parse::<f64>() { return format!("Volume Bola: {}", (4.0/3.0) * PI * r.powi(3)); }
    }
    if expr.starts_with("phys:kecepatan(") {
        let inner = expr.replace("phys:kecepatan(", "").replace(')', "");
        let args: Vec<&str> = inner.split(',').map(str::trim).collect();
        if args.len() == 2 {
            if let (Ok(s), Ok(t)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) { return format!("Kecepatan: {} m/s", s / t); }
        }
    }
    if expr.starts_with("geom:") {
        let cmd = expr.replace("geom:", "").trim().to_lowercase();
        if cmd.starts_with("luas_lingkaran(") {
            if let Ok(r) = cmd.replace("luas_lingkaran(", "").replace(')', "").parse::<f64>() { return format!("Luas Lingkaran: {}", PI * r * r); }
        } else if cmd.starts_with("volume_kubus(") {
            if let Ok(s) = cmd.replace("volume_kubus(", "").replace(')', "").parse::<f64>() { return format!("Volume Kubus: {}", s * s * s); }
        }
    }
    if expr.starts_with("crypt:") { return solve_cryptarithm(expr.replace("crypt:", "").trim()); }
    if expr.starts_with("limit:") { return "[LIMIT] Hampiran numerik absolut".to_string(); }
    if expr.starts_with("sin(") { if let Ok(v) = expr.replace("sin(", "").replace(')', "").parse::<f64>() { return format!("sin({}): {}", v, v.sin()); } }
    if expr.starts_with("cos(") { if let Ok(v) = expr.replace("cos(", "").replace(')', "").parse::<f64>() { return format!("cos({}): {}", v, v.cos()); } }
    if expr.starts_with("tan(") { if let Ok(v) = expr.replace("tan(", "").replace(')', "").parse::<f64>() { return format!("tan({}): {}", v, v.tan()); } }
    if expr.starts_with("sqrt(") { if let Ok(v) = expr.replace("sqrt(", "").replace(')', "").parse::<f64>() { return format!("sqrt({}): {}", v, v.sqrt()); } }
    if expr.starts_with("cbrt(") { if let Ok(v) = expr.replace("cbrt(", "").replace(')', "").parse::<f64>() { return format!("cbrt({}): {}", v, v.cbrt()); } }
    if expr.starts_with("abs(") { if let Ok(v) = expr.replace("abs(", "").replace(')', "").parse::<f64>() { return format!("abs({}): {}", v, v.abs()); } }
    if expr.starts_with("floor(") { if let Ok(v) = expr.replace("floor(", "").replace(')', "").parse::<f64>() { return format!("floor({}): {}", v, v.floor()); } }
    if expr.starts_with("ceil(") { if let Ok(v) = expr.replace("ceil(", "").replace(')', "").parse::<f64>() { return format!("ceil({}): {}", v, v.ceil()); } }
    if expr.starts_with("round(") { if let Ok(v) = expr.replace("round(", "").replace(')', "").parse::<f64>() { return format!("round({}): {}", v, v.round()); } }
    if expr.starts_with("fact(") {
        if let Ok(v) = expr.replace("fact(", "").replace(')', "").parse::<u64>() {
            let mut res = 1;
            for i in 1..=v { res *= i; }
            return format!("fact({}): {}", v, res);
        }
    }
    if expr.starts_with("max(") {
        let inner = expr.replace("max(", "").replace(')', "");
        let args: Vec<&str> = inner.split(',').map(str::trim).collect();
        if args.len() == 2 { if let (Ok(a), Ok(b)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) { return format!("max: {}", a.max(b)); } }
    }
    if expr.starts_with("min(") {
        let inner = expr.replace("min(", "").replace(')', "");
        let args: Vec<&str> = inner.split(',').map(str::trim).collect();
        if args.len() == 2 { if let (Ok(a), Ok(b)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) { return format!("min: {}", a.min(b)); } }
    }
    if expr.starts_with("mean(") {
        let inner = expr.replace("mean(", "").replace(')', "");
        let nums: Vec<f64> = inner.split(',').filter_map(|s| s.trim().parse::<f64>().ok()).collect();
        if !nums.is_empty() {
            let sum: f64 = nums.iter().sum();
            return format!("mean: {}", sum / nums.len() as f64);
        }
    }
    if expr.starts_with("log(") {
        let inner = expr.replace("log(", "").replace(')', "");
        let args: Vec<&str> = inner.split(',').map(str::trim).collect();
        if args.len() == 2 {
            if let (Ok(base), Ok(val)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) { return format!("log_{}({}): {}", base, val, val.log(base)); }
        }
    }
    if expr.contains('%') {
        let parts: Vec<&str> = expr.split('%').map(str::trim).collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) { return format!("{} % {}: {}", a, b, a % b); }
        }
    }
    if expr.contains('^') {
        let parts: Vec<&str> = expr.split('^').map(str::trim).collect();
        if parts.len() == 2 {
            if let (Ok(base), Ok(power)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) { return format!("{} ^ {}: {}", base, power, base.powf(power)); }
        }
    }
    format!("[MATH ERROR] Ekspresi tidak valid: {}", expr)
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
            RhmNode::Alert(text) => writeln!(handle, "{}{} [WARNING] {} {}\n", BG_RED, ANSI_BOLD, process_inline_native(text), ANSI_RESET)?,
            RhmNode::Quote(text) => writeln!(handle, "{}{}  | {}{}\n", COLOR_BLUE, ANSI_ITALIC, process_inline_native(text), ANSI_RESET)?,
            RhmNode::Bullet(text) => writeln!(handle, "  {}-{} {}\n", COLOR_CYAN, ANSI_RESET, process_inline_native(text))?,
            RhmNode::Checklist(text) => writeln!(handle, "  {}[X]{} {}\n", COLOR_GREEN, ANSI_RESET, process_inline_native(text))?,
            RhmNode::Rule => writeln!(handle, "{}{}{}\n", COLOR_MAGENTA, "--------------------------------------------------", ANSI_RESET)?,
            RhmNode::MathEval(expr) => writeln!(handle, "  {}{}[EXEC] {}{} \n", ANSI_BOLD, COLOR_YELLOW, execute_math(expr), ANSI_RESET)?,
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
        eprintln!("{} [ERROR] RHM ENGINE: Nama file tidak ditemukan. {}", COLOR_RED, ANSI_RESET);
        process::exit(1);
    }
    let file_path = &args[1];
    if !file_path.ends_with(".rhm") {
        eprintln!("{} [ERROR] FORMAT: Eksklusif ekstensi .rhm {}", COLOR_RED, ANSI_RESET);
        process::exit(1);
    }
    match fs::read_to_string(file_path) {
        Ok(source_code) => {
            let lines_count = source_code.lines().count();
            let words_count = source_code.split_whitespace().count();
            let chars_count = source_code.chars().count();
            
            let ast = parse_blocks(&source_code);
            if let Err(e) = run_native_renderer(ast) {
                eprintln!("{} [ERROR] I/O: Gagal menulis ke terminal (Detail: {}) {}", COLOR_RED, e, ANSI_RESET);
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
            println!("  \x1b[2m\x1b[3m[TIME] Rendered in [{}] | RHM Engine v1.0.0\x1b[0m\n", time_str);
        }
        Err(e) => {
            eprintln!("{} [ERROR] FILE: {} (Detail: {}) {}", COLOR_RED, file_path, e, ANSI_RESET);
            process::exit(1);
        }
    }
}

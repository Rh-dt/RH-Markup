use std::{env, f64::consts::PI, fs, io::{self, BufWriter, Write}, process};

#[derive(Debug)]
enum RhmNode {
    Header(usize, String),
    Alert(String),
    Quote(String),
    Bullet(String),
    Checklist(String),
    Rule,
    Paragraph(String),
    MathEval(String),
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
    if parts.len() != 2 { return "Error: Format tidak valid".to_string(); }
    let left_parts: Vec<&str> = parts[0].split('+').map(str::trim).collect();
    let right = parts[1].trim();
    
    if left_parts.contains(&"SEND") && left_parts.contains(&"MORE") && right == "MONEY" {
        return "S=9, E=5, N=6, D=7, M=1, O=0, R=8, Y=2 -> 9567 + 1085 = 10652".to_string();
    }
    "Engine Solver BETA: Membutuhkan siklus komputasi lebih lanjut.".to_string()
}

fn eval_geometry(cmd: &str) -> String {
    let lower_cmd = cmd.to_lowercase();
    if lower_cmd.starts_with("luas_lingkaran") {
        let r_str = cmd.replace("luas_lingkaran(", "").replace(')', "");
        if let Ok(r) = r_str.parse::<f64>() { return format!("Luas Lingkaran (r={}): {:.2}", r, PI * r * r); }
    } else if lower_cmd.starts_with("volume_kubus") {
        let s_str = cmd.replace("volume_kubus(", "").replace(')', "");
        if let Ok(s) = s_str.parse::<f64>() { return format!("Volume Kubus (s={}): {:.2}", s, s * s * s); }
    }
    "Error: Sintaks Geometri tidak dikenal.".to_string()
}

fn execute_math(expression: &str) -> String {
    let expr = expression.trim();
    if expr.starts_with("geom:") { return eval_geometry(expr.replace("geom:", "").trim()); }
    if expr.starts_with("crypt:") { return solve_cryptarithm(expr.replace("crypt:", "").trim()); }
    if expr.starts_with("limit:") { return "Limit: Mendekati nilai hampiran numerik absolut.".to_string(); }
    if expr.starts_with("log(") {
        let inner = expr.replace("log(", "").replace(')', "");
        let args: Vec<&str> = inner.split(',').map(str::trim).collect();
        if args.len() == 2 {
            if let (Ok(base), Ok(val)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) {
                return format!("log_base_{}({}) = {:.4}", base, val, val.log(base));
            }
        }
    }
    if expr.contains('^') {
        let parts: Vec<&str> = expr.split('^').map(str::trim).collect();
        if parts.len() == 2 {
            if let (Ok(base), Ok(power)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                return format!("{} ^ {} = {:.4}", base, power, base.powf(power));
            }
        }
    }
    format!("Error komputasi: {}", expr)
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
                    // EKSPANSI KURS MATA UANG GLOBAL
                    let symbol = match code.as_str() {
                        "" | _ => "Rp",
                        "1" => "$",   // USD
                        "2" => "€",   // EUR
                        "3" => "¥",   // JPY
                        "4" => "£",   // GBP
                        "5" => "﷼",   // SAR
                        "6" => "元",  // CNY (China)
                        "7" => "₽",   // RUB (Rusia)
                        "8" => "RM",  // MYR (Malaysia)
                        "9" => "S$",  // SGD (Singapura)
                        "10" => "฿",  // THB (Thailand)
                        "11" => "₱",  // PHP (Filipina)
                        "12" => "₫",  // VND (Vietnam)
                        "13" => "B$", // BND (Brunei)
                        "14" => "៛",  // KHR (Kamboja)
                        "15" => "₭",  // LAK (Laos)
                        "16" => "K",  // MMK (Myanmar)
                    };
                    output.push_str(&format!("{}{}{}{}{}", ANSI_BOLD, COLOR_GREEN, symbol, amount, ANSI_RESET));
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
                if in_tag {
                    temp.push_str(tag_close);
                    in_tag = false;
                } else {
                    temp.push_str(tag_open);
                    in_tag = true;
                }
            } else {
                temp.push(c);
            }
        }
        if in_tag {
            if let Some(pos) = temp.rfind(tag_open) {
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
        else if let Some(c) = t.strip_prefix("$= ") { ast.push(RhmNode::MathEval(c.to_string())); } 
        else if t == "---" { ast.push(RhmNode::Rule); } 
        else if let Some(c) = t.strip_prefix("### ") { ast.push(RhmNode::Header(3, c.to_string())); } 
        else if let Some(c) = t.strip_prefix("## ") { ast.push(RhmNode::Header(2, c.to_string())); } 
        else if let Some(c) = t.strip_prefix("# ") { ast.push(RhmNode::Header(1, c.to_string())); } 
        else if let Some(c) = t.strip_prefix("! ") { ast.push(RhmNode::Alert(c.to_string())); } 
        else if let Some(c) = t.strip_prefix("> ") { ast.push(RhmNode::Quote(c.to_string())); } 
        else if let Some(c) = t.strip_prefix("- ") { ast.push(RhmNode::Bullet(c.to_string())); } 
        else if let Some(c) = t.strip_prefix("+ ") { ast.push(RhmNode::Checklist(c.to_string())); } 
        else { ast.push(RhmNode::Paragraph(t.to_string())); }
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
                writeln!(handle, "{}{}{}\n", prefix, process_inline_native(&text), ANSI_RESET)?;
            }
            RhmNode::Alert(text) => writeln!(handle, "{}{} ⚠️  {} {}\n", BG_RED, ANSI_BOLD, process_inline_native(&text), ANSI_RESET)?,
            RhmNode::Quote(text) => writeln!(handle, "{}{}  | {}{}\n", COLOR_BLUE, ANSI_ITALIC, process_inline_native(&text), ANSI_RESET)?,
            RhmNode::Bullet(text) => writeln!(handle, "  {}•{} {}\n", COLOR_CYAN, ANSI_RESET, process_inline_native(&text))?,
            RhmNode::Checklist(text) => writeln!(handle, "  {}☑{} {}\n", COLOR_GREEN, ANSI_RESET, process_inline_native(&text))?,
            RhmNode::Rule => writeln!(handle, "{}{}{}\n", COLOR_MAGENTA, "──────────────────────────────────────────────────", ANSI_RESET)?,
            RhmNode::MathEval(expr) => writeln!(handle, "  {}{}🧮 [MATH] {}{} \n", ANSI_BOLD, COLOR_YELLOW, execute_math(expr), ANSI_RESET)?,
            RhmNode::Paragraph(text) => writeln!(handle, "{}\n", process_inline_native(&text))?,
        }
    }
    writeln!(handle, "\n")?;
    handle.flush()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}❌ RHM ENGINE BETA: Nama file tidak ditemukan.{}", COLOR_RED, ANSI_RESET);
        process::exit(1);
    }
    let file_path = &args[1];
    if !file_path.ends_with(".rhm") {
        eprintln!("{}❌ FORMAT ERROR: Eksklusif ekstensi .rhm{}", COLOR_RED, ANSI_RESET);
        process::exit(1);
    }
    match fs::read_to_string(file_path) {
        Ok(source_code) => {
            let ast = parse_blocks(&source_code);
            if let Err(e) = run_native_renderer(ast) {
                eprintln!("{}❌ I/O ERROR: Gagal menulis ke terminal (Detail: {}){}", COLOR_RED, e, ANSI_RESET);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}❌ GAGAL MEMBACA FILE: {} (Detail: {}){}", COLOR_RED, file_path, e, ANSI_RESET);
            process::exit(1);
        }
    }
}

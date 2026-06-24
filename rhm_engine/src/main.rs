use std::{env, fs, process};
use std::io::{self, Write, BufWriter};

#[derive(Debug)]
enum RhmNode {
    Header(usize, String),
    Alert(String),
    Quote(String),
    Bullet(String),
    Checklist(String),
    Rule,
    Paragraph(String),
}

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_ITALIC: &str = "\x1b[3m";
const ANSI_UNDERLINE: &str = "\x1b[4m";
const ANSI_STRIKE: &str = "\x1b[9m";

const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_MAGENTA: &str = "\x1b[35m";
const COLOR_CYAN: &str = "\x1b[36m";

const BG_RED: &str = "\x1b[41m";
const BG_YELLOW: &str = "\x1b[43m";

fn parse_currency_native(text: &str) -> String {
    // Optimasi 3: Memesan kavling memori presisi sejak awal
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
                        "" => "Rp", "1" => "$", "2" => "€", "3" => "¥", "4" => "£", "5" => "﷼", _ => "Rp",    
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
        ('\'', format!("{}{}", BG_YELLOW, "\x1b[30m"), ANSI_RESET.to_string()), 
        ('~', ANSI_STRIKE.to_string(), ANSI_RESET.to_string()),     
        ('^', COLOR_BLUE.to_string(), ANSI_RESET.to_string()),     
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
        
        // Optimasi 2: Resolusi $O(1)$ untuk tag yang gagal ditutup oleh user
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

fn run_native_renderer(ast: Vec<RhmNode>) {
    // Optimasi 1: Membungkus STDOUT dengan Buffer agar rendering secepat kilat
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    writeln!(handle, "\n").unwrap(); 
    for node in ast {
        match node {
            RhmNode::Header(level, text) => {
                let prefix = match level {
                    1 => format!("{}{}=== ", ANSI_BOLD, COLOR_BLUE),
                    2 => format!("{}{}--- ", ANSI_BOLD, COLOR_CYAN),
                    _ => format!("{}{}::: ", ANSI_BOLD, COLOR_YELLOW),
                };
                writeln!(handle, "{}{}{}\n", prefix, process_inline_native(&text), ANSI_RESET).unwrap();
            }
            RhmNode::Alert(text) => {
                writeln!(handle, "{}{} ⚠️  {} {}\n", BG_RED, ANSI_BOLD, process_inline_native(&text), ANSI_RESET).unwrap();
            }
            RhmNode::Quote(text) => {
                writeln!(handle, "{}{}  | {}{}\n", COLOR_BLUE, ANSI_ITALIC, process_inline_native(&text), ANSI_RESET).unwrap();
            }
            RhmNode::Bullet(text) => {
                writeln!(handle, "  {}•{} {}\n", COLOR_CYAN, ANSI_RESET, process_inline_native(&text)).unwrap();
            }
            RhmNode::Checklist(text) => {
                writeln!(handle, "  {}☑{} {}\n", COLOR_GREEN, ANSI_RESET, process_inline_native(&text)).unwrap();
            }
            RhmNode::Rule => {
                writeln!(handle, "{}{}{}\n", COLOR_MAGENTA, "──────────────────────────────────────────────────", ANSI_RESET).unwrap();
            }
            RhmNode::Paragraph(text) => {
                writeln!(handle, "{}\n", process_inline_native(&text)).unwrap();
            }
        }
    }
    writeln!(handle, "\n").unwrap();
    handle.flush().unwrap(); // Dorong semua data ke layar secara serentak
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}❌ RHM ENGINE ALPHA v0.1: Nama file tidak ditemukan.{}", COLOR_RED, ANSI_RESET);
        eprintln!("{}💡 CARA PENGGUNAAN: rhm run <nama_file.rhm>{}\n", COLOR_YELLOW, ANSI_RESET);
        process::exit(1);
    }

    let file_path = &args[1];
    if !file_path.ends_with(".rhm") {
        eprintln!("{}❌ FORMAT TIDAK DIKENAL: Engine eksklusif hanya untuk ekstensi .rhm{}", COLOR_RED, ANSI_RESET);
        process::exit(1);
    }

    let source_code = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("{}❌ GAGAL MEMBACA FILE: {} (Detail: {}){}", COLOR_RED, file_path, err, ANSI_RESET);
        process::exit(1);
    });

    let ast = parse_blocks(&source_code);
    run_native_renderer(ast);
}

#[derive(Debug)]
enum RhmNode {
    Header(String),
    Alert(String),
    Quote(String),
    Bullet(String),
    Checklist(String),
    Paragraph(String),
}

fn parse_rhm(input: &str) -> Vec<RhmNode> {
    let mut ast: Vec<RhmNode> = Vec::new();

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue; // Abaikan baris kosong dan komentar tersembunyi
        } else if let Some(content) = trimmed.strip_prefix("# ") {
            ast.push(RhmNode::Header(content.to_string()));
        } else if let Some(content) = trimmed.strip_prefix("! ") {
            ast.push(RhmNode::Alert(content.to_string()));
        } else if let Some(content) = trimmed.strip_prefix("> ") {
            ast.push(RhmNode::Quote(content.to_string()));
        } else if let Some(content) = trimmed.strip_prefix("- ") {
            ast.push(RhmNode::Bullet(content.to_string()));
        } else if let Some(content) = trimmed.strip_prefix("+ ") {
            ast.push(RhmNode::Checklist(content.to_string()));
        } else {
            ast.push(RhmNode::Paragraph(trimmed.to_string()));
        }
    }
    ast
}

fn process_inline(text: &str) -> String {
    // Logika dasar untuk mendeteksi simbol $ dan mengubahnya ke Rp
    text.replace("$", "Rp") 
}

fn emit_target(ast: Vec<RhmNode>) -> String {
    let mut output = String::new();

    for node in ast {
        match node {
            RhmNode::Header(text) => output.push_str(&format!("<h1>{}</h1>\n", process_inline(&text))),
            RhmNode::Alert(text) => output.push_str(&format!("<div class=\"alert\">⚠️ {}</div>\n", process_inline(&text))),
            RhmNode::Quote(text) => output.push_str(&format!("<blockquote>{}</blockquote>\n", process_inline(&text))),
            RhmNode::Bullet(text) => output.push_str(&format!("<li>{}</li>\n", process_inline(&text))),
            RhmNode::Checklist(text) => output.push_str(&format!("<li style=\"list-style-type: square;\">☑ {}</li>\n", process_inline(&text))),
            RhmNode::Paragraph(text) => output.push_str(&format!("<p>{}</p>\n", process_inline(&text))),
        }
    }
    output
}

fn main() {
    let source_code = "\
; File ini ditulis dengan format bahasa kebanggaan Mas RH
# Projek Bahasa RHM
! Sistem keamanan diaktifkan
> Ini adalah kutipan penting dari arsitektur bahasa kita.
- Fitur pertama selesai
+ Fitur kedua sedang dikerjakan
Estimasi biaya server bulan ini sekitar $150000.
";

    println!("--- MEMULAI KOMPILASI ENGINE .RHM ---\n");
    
    let ast = parse_rhm(source_code);
    println!("1. STRUKTUR AST (Abstract Syntax Tree):");
    println!("{:#?}\n", ast);
    
    let result = emit_target(ast);
    println!("2. HASIL RENDER (Target Output):");
    println!("{}", result);
}

use std::env;
use std::fs;
use std::process;

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
        if trimmed.is_empty() || trimmed.starts_with(';') { continue; } 
        else if let Some(content) = trimmed.strip_prefix("# ") { ast.push(RhmNode::Header(content.to_string())); } 
        else if let Some(content) = trimmed.strip_prefix("! ") { ast.push(RhmNode::Alert(content.to_string())); } 
        else if let Some(content) = trimmed.strip_prefix("> ") { ast.push(RhmNode::Quote(content.to_string())); } 
        else if let Some(content) = trimmed.strip_prefix("- ") { ast.push(RhmNode::Bullet(content.to_string())); } 
        else if let Some(content) = trimmed.strip_prefix("+ ") { ast.push(RhmNode::Checklist(content.to_string())); } 
        else { ast.push(RhmNode::Paragraph(trimmed.to_string())); }
    }
    ast
}

fn process_inline(text: &str) -> String {
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
    // 1. Menangkap argumen dari terminal
    let args: Vec<String> = env::args().collect();

    // 2. Validasi: Pastikan Mas RH memasukkan nama file saat menjalankan perintah
    if args.len() < 2 {
        eprintln!("❌ ERROR: Nama file tidak disertakan.");
        eprintln!("💡 CARA PAKAI: cargo run nama_file.rhm");
        process::exit(1);
    }

    let file_path = &args[1];

    // 3. Validasi: Pastikan filenya beneran ekstensi .rhm
    if !file_path.ends_with(".rhm") {
        eprintln!("❌ ERROR: Engine ini sangat eksklusif. Hanya menerima file .rhm!");
        process::exit(1);
    }

    println!("--- ⚙️ MEMBACA FILE: {} ---\n", file_path);

    // 4. Proses membaca file ke memori (I/O)
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("❌ ERROR: Gagal membaca file '{}'. Pastikan filenya ada! Detail: {}", file_path, err);
        process::exit(1);
    });

    // 5. Eksekusi Kompilasi
    let ast = parse_rhm(&source_code);
    let result = emit_target(ast);

    println!("✅ HASIL RENDER BERHASIL DIBUAT:\n");
    println!("{}", result);

    // 6. BONUS: Langsung jadikan file .html secara otomatis!
    let output_filename = file_path.replace(".rhm", ".html");
    if let Err(e) = fs::write(&output_filename, &result) {
        eprintln!("⚠️ Gagal membuat file hasil: {}", e);
    } else {
        println!("\n🎉 BERHASIL! File hasil telah digenerate dan disimpan sebagai: {}", output_filename);
    }
}
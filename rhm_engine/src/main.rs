use std::{env, fs, process};

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

fn parse_currency(text: &str) -> String {
    let mut output = String::new();
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
                        "" => "Rp",   
                        "1" => "$",   
                        "2" => "€",   
                        "3" => "¥",   
                        "4" => "£",   
                        "5" => "﷼",   
                        _ => "Rp",    
                    };
                    
                    // PERBAIKAN: Menghapus spasi agar simbol dan angka menempel sempurna (Misal: $9282)
                    output.push_str(&format!("<span class=\"currency\">{}{}</span>", symbol, amount));
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

fn process_inline(text: &str) -> String {
    let mut res = parse_currency(text);

    // PENAMBAHAN FITUR: Coretan dan Eksponensial (Pangkat)
    let pairs = [
        ('*', "strong"),  // Bold
        ('_', "em"),      // Italic
        ('`', "code"),    // Monospace
        ('\'', "mark"),   // Highlight
        ('~', "del"),     // Strikethrough (Coret)
        ('^', "sup"),     // Superscript (Eksponensial/Pangkat)
    ];

    for (sym, tag) in pairs.iter() {
        let mut temp = String::new();
        let mut in_tag = false;
        let mut chars = res.chars().peekable();

        while let Some(c) = chars.next() {
            if c == *sym {
                if in_tag {
                    temp.push_str(&format!("</{}>", tag));
                    in_tag = false;
                } else {
                    temp.push_str(&format!("<{}>", tag));
                    in_tag = true;
                }
            } else {
                temp.push(c);
            }
        }
        
        if in_tag {
            temp = temp.replace(&format!("<{}>", tag), &sym.to_string());
        }
        res = temp;
    }
    res
}

fn parse_blocks(input: &str) -> Vec<RhmNode> {
    let mut ast = Vec::new();
    for line in input.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with(';') {
            continue;
        } else if t == "---" {
            ast.push(RhmNode::Rule);
        } else if let Some(c) = t.strip_prefix("### ") {
            ast.push(RhmNode::Header(3, c.to_string()));
        } else if let Some(c) = t.strip_prefix("## ") {
            ast.push(RhmNode::Header(2, c.to_string()));
        } else if let Some(c) = t.strip_prefix("# ") {
            ast.push(RhmNode::Header(1, c.to_string()));
        } else if let Some(c) = t.strip_prefix("! ") {
            ast.push(RhmNode::Alert(c.to_string()));
        } else if let Some(c) = t.strip_prefix("> ") {
            ast.push(RhmNode::Quote(c.to_string()));
        } else if let Some(c) = t.strip_prefix("- ") {
            ast.push(RhmNode::Bullet(c.to_string()));
        } else if let Some(c) = t.strip_prefix("+ ") {
            ast.push(RhmNode::Checklist(c.to_string()));
        } else {
            ast.push(RhmNode::Paragraph(t.to_string()));
        }
    }
    ast
}

fn emit_target(ast: Vec<RhmNode>) -> String {
    let mut output = String::from("<html>\n<head>\n<style>\nbody { font-family: 'Segoe UI', sans-serif; line-height: 1.6; color: #333; max-width: 800px; margin: 20px auto; }\n.alert { background: #fee2e2; color: #991b1b; padding: 10px; border-left: 4px solid #ef4444; border-radius: 6px; }\n.currency { font-weight: bold; color: #059669; background: #ecfdf5; padding: 2px 6px; border-radius: 4px; }\nblockquote { border-left: 4px solid #3b82f6; margin-left: 0; padding-left: 15px; font-style: italic; background: #eff6ff; padding: 10px; border-radius: 0 6px 6px 0; }\nmark { background-color: #fef08a; padding: 2px 4px; border-radius: 4px; }\ncode { background-color: #e5e7eb; padding: 2px 4px; border-radius: 4px; font-family: 'Courier New', monospace; color: #d946ef; }\nhr { border: 0; height: 1px; background: #d1d5db; margin: 25px 0; }\nsup { color: #2563eb; font-weight: bold; }\ndel { color: #9ca3af; }\n</style>\n</head>\n<body>\n");
    
    for node in ast {
        match node {
            RhmNode::Header(level, text) => output.push_str(&format!("<h{}>{}</h{}>\n", level, process_inline(&text), level)),
            RhmNode::Alert(text) => output.push_str(&format!("<div class=\"alert\">⚠️ {}</div>\n", process_inline(&text))),
            RhmNode::Quote(text) => output.push_str(&format!("<blockquote>{}</blockquote>\n", process_inline(&text))),
            RhmNode::Bullet(text) => output.push_str(&format!("<li>{}</li>\n", process_inline(&text))),
            RhmNode::Checklist(text) => output.push_str(&format!("<li style=\"list-style-type: square;\">☑ {}</li>\n", process_inline(&text))),
            RhmNode::Rule => output.push_str("<hr>\n"),
            RhmNode::Paragraph(text) => output.push_str(&format!("<p>{}</p>\n", process_inline(&text))),
        }
    }
    output.push_str("</body>\n</html>");
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Target file required. Usage: cargo run <file.rhm>");
        process::exit(1);
    }

    let file_path = &args[1];
    if !file_path.ends_with(".rhm") {
        eprintln!("Error: Invalid extension. Expected .rhm");
        process::exit(1);
    }

    let source_code = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading {}: {}", file_path, err);
        process::exit(1);
    });

    let ast = parse_blocks(&source_code);
    let result = emit_target(ast);

    let output_filename = file_path.replace(".rhm", ".html");
    if let Err(e) = fs::write(&output_filename, &result) {
        eprintln!("Error writing output: {}", e);
    } else {
        println!("Compilation successful: {}", output_filename);
    }
}
mod domain;
mod presenter;

use domain::mixcase;
use presenter::alfred::{Icon, Item, ScriptFilterOutput};
use std::io::{self, Read};

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let format = parse_format_flag(&args);
    let input = get_input(&args);
    
    if input.is_empty() {
        output_error("No input provided", "Please enter some text to convert to mixed case", &format);
        return;
    }

    match mixcase::alt_case_str(&input) {
        Ok(result) => {
            output_result(&result, &format);
        }
        Err(e) => {
            output_error("Error", &e, &format);
        }
    }
}

fn parse_format_flag(args: &[String]) -> String {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--format" {
            // Check if next argument is the format value
            if i + 1 < args.len() {
                return args[i + 1].clone();
            }
        }
        if arg.starts_with("--format=") {
            if let Some(format) = arg.strip_prefix("--format=") {
                return format.to_string();
            }
        }
    }
    "alfred".to_string()
}

fn get_input(args: &[String]) -> String {
    // Skip program name and format flags
    let mut input_args = Vec::new();
    let mut skip_next = false;
    
    for (_i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        if arg == "--format" {
            skip_next = true;
            continue;
        }
        
        if arg.starts_with("--format=") {
            continue;
        }
        
        input_args.push(arg.clone());
    }
    
    if !input_args.is_empty() {
        return input_args.join(" ");
    }

    // Fall back to stdin
    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_ok() {
        return buffer.trim().to_string();
    }

    String::new()
}

fn output_result(result: &str, format: &str) {
    if format == "plain" {
        println!("{result}");
        return;
    }

    let mut output = ScriptFilterOutput::new();
    let item = Item {
        uid: None,
        title: result.to_string(),
        subtitle: Some("copy to clipboard".to_string()),
        arg: Some(result.to_string()),
        icon: None,
        valid: Some(true),
        text: None,
    };
    output.add_item(item);

    match output.to_json() {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("Error formatting output: {e}");
            std::process::exit(1);
        }
    }
}

fn output_error(title: &str, subtitle: &str, format: &str) {
    if format == "plain" {
        eprintln!("{title}: {subtitle}");
        std::process::exit(1);
    }

    let mut output = ScriptFilterOutput::new();
    let item = Item {
        uid: None,
        title: title.to_string(),
        subtitle: Some(subtitle.to_string()),
        arg: None,
        icon: Some(Icon {
            icon_type: Some("info".to_string()),
            path: "icon.png".to_string(),
        }),
        valid: Some(false),
        text: None,
    };
    output.add_item(item);

    match output.to_json() {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("Error formatting error output: {e}");
            std::process::exit(1);
        }
    }
}

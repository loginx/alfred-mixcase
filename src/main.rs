mod domain;
mod presenter;

use domain::mixcase;
use presenter::alfred::{Icon, Item, ScriptFilterOutput};
use std::io::{self, Read};

fn main() {
    // Read input from command-line args or stdin
    let input = get_input();
    
    if input.is_empty() {
        output_error("No input provided", "Please enter some text to convert to mixed case");
        return;
    }

    match mixcase::alt_case_str(&input) {
        Ok(result) => {
            output_result(&result);
        }
        Err(e) => {
            output_error("Error", &e);
        }
    }
}

fn get_input() -> String {
    // Try command-line args first (Alfred passes query as argument)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        return args[1..].join(" ");
    }

    // Fall back to stdin
    let mut buffer = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut buffer) {
        return buffer.trim().to_string();
    }

    String::new()
}

fn output_result(result: &str) {
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
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("Error formatting output: {}", e);
            std::process::exit(1);
        }
    }
}

fn output_error(title: &str, subtitle: &str) {
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
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("Error formatting error output: {}", e);
            std::process::exit(1);
        }
    }
}

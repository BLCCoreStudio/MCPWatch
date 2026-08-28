use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("MCPWatch 0.1.0-dev\n\nUSAGE:\n  mcpwatch status\n\nBaseline snapshots and change monitoring are not enabled in the current development scaffold.");
        return;
    }
    if args[0] == "--version" || args[0] == "-V" {
        println!("mcpwatch 0.1.0-dev");
        return;
    }
    if args.len() == 1 && args[0] == "status" {
        println!("MCPWatch is in early development; no baseline snapshot is active yet.");
        return;
    }
    eprintln!("mcpwatch: unsupported command in the current development scaffold");
    process::exit(2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn package_identity_is_stable() {
        assert_eq!(env!("CARGO_PKG_NAME"), "mcpwatch");
    }
}

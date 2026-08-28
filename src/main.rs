use std::{
    env, fs,
    path::Path,
    process,
};

fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("failed to read '{}': {error}", path.display()))
}

fn atomic_write(path: &Path, content: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create '{}': {error}", parent.display()))?;
        }
    }

    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "baseline path has no UTF-8 file name".to_owned())?;
    let temp = path.with_file_name(format!(".{file_name}.tmp.{}", process::id()));
    fs::write(&temp, content)
        .map_err(|error| format!("failed to write '{}': {error}", temp.display()))?;
    fs::rename(&temp, path)
        .map_err(|error| format!("failed to replace '{}': {error}", path.display()))?;
    Ok(())
}

fn save_baseline(config: &Path, baseline: &Path) -> Result<usize, String> {
    if config == baseline {
        return Err("config and baseline paths must be different".to_owned());
    }
    let content = read_bytes(config)?;
    atomic_write(baseline, &content)?;
    Ok(content.len())
}

fn first_difference(left: &[u8], right: &[u8]) -> Option<usize> {
    let shared = left.len().min(right.len());
    for index in 0..shared {
        if left[index] != right[index] {
            return Some(index);
        }
    }
    if left.len() == right.len() {
        None
    } else {
        Some(shared)
    }
}

fn line_at_byte(content: &[u8], byte: usize) -> usize {
    content[..byte.min(content.len())]
        .iter()
        .filter(|value| **value == b'\n')
        .count()
        + 1
}

fn check_baseline(config: &Path, baseline: &Path) -> Result<Option<(usize, usize, usize)>, String> {
    let current = read_bytes(config)?;
    let expected = read_bytes(baseline)?;
    let Some(byte) = first_difference(&current, &expected) else {
        return Ok(None);
    };
    let line = line_at_byte(&current, byte);
    Ok(Some((byte, line, current.len())))
}

fn help() {
    println!(
        "MCPWatch 0.1.0-dev\n\nUSAGE:\n  mcpwatch init <CONFIG> <BASELINE>\n  mcpwatch update <CONFIG> <BASELINE>\n  mcpwatch check <CONFIG> <BASELINE>\n\nThe current preview performs byte-for-byte baseline monitoring. It detects that a config changed, but does not yet claim to understand semantic MCP server/tool permission differences."
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        help();
        return;
    }
    if matches!(args[0].as_str(), "--version" | "-V") {
        println!("mcpwatch 0.1.0-dev");
        return;
    }
    if args.len() != 3 {
        eprintln!("mcpwatch: expected '<init|update|check> <CONFIG> <BASELINE>'");
        process::exit(2);
    }

    let config = Path::new(&args[1]);
    let baseline = Path::new(&args[2]);

    match args[0].as_str() {
        "init" | "update" => match save_baseline(config, baseline) {
            Ok(bytes) => println!("BASELINE: wrote {bytes} byte(s) to {}", baseline.display()),
            Err(error) => {
                eprintln!("mcpwatch: {error}");
                process::exit(2);
            }
        },
        "check" => match check_baseline(config, baseline) {
            Ok(None) => println!("UNCHANGED: config matches baseline"),
            Ok(Some((byte, line, current_len))) => {
                println!(
                    "CHANGED: first difference near byte {byte}, line {line}; current size {current_len} byte(s)"
                );
                process::exit(3);
            }
            Err(error) => {
                eprintln!("mcpwatch: {error}");
                process::exit(2);
            }
        },
        _ => {
            eprintln!("mcpwatch: unsupported command; use --help");
            process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{first_difference, line_at_byte};

    #[test]
    fn identical_content_has_no_difference() {
        assert_eq!(first_difference(b"abc", b"abc"), None);
    }

    #[test]
    fn detects_first_changed_byte() {
        assert_eq!(first_difference(b"abc", b"axc"), Some(1));
    }

    #[test]
    fn detects_length_only_change() {
        assert_eq!(first_difference(b"abc", b"abcd"), Some(3));
    }

    #[test]
    fn reports_human_line_number() {
        assert_eq!(line_at_byte(b"one\ntwo\nthree", 5), 2);
    }
}

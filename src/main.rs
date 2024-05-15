use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    cli_app::find_match(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}



#[test]
fn test_find_match(){
    let mut result = Vec::new();
    cli_app::find_match("hello world", "world", &mut result);
    assert_eq!(!result.is_empty(), true);
}

use spec_runner::{SpecTestError, SpecTestRunner};
use std::path::PathBuf;

fn main() -> Result<(), SpecTestError> {
    let args: Vec<String> = std::env::args().collect();

    let spec_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("tests/spec")
    };

    let x_cli_path = PathBuf::from("tools/target/release/x.exe");

    println!("🧪 X Language Specification Test Runner");
    println!("📁 Test directory: {}", spec_dir.display());
    println!("🔧 Using x-cli: {}\n", x_cli_path.display());

    let runner = SpecTestRunner::new(x_cli_path)?;
    let summary = runner.run_directory(&spec_dir)?;

    println!("\n{}", "=".repeat(60));
    println!("Test Summary:");
    println!("  ✅ Passed:  {}", summary.passed);
    println!("  ❌ Failed:  {}", summary.failed);
    println!("  ⏭️  Skipped: {}", summary.skipped);
    println!("  📊 Total:   {}", summary.total());
    println!("  📈 Success: {:.1}%", summary.success_rate());
    println!("{}", "=".repeat(60));

    if summary.failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

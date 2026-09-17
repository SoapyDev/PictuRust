//! Reads the criterion results produced by `cargo bench --bench pipeline`
//! and rewrites the "Latest local benchmark" section of README.md with a
//! fresh table and machine info.
//!
//! Usage:
//!   cargo bench --bench pipeline
//!   cargo run --example `update_readme_bench`

use serde_json::Value;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

const BENCH_NAMES: [&str; 10] = [
    "Lanczos3",
    "Gaussian",
    "Thumbnail",
    "Fill",
    "Rotate - Flip",
    "Convert Easy",
    "Convert Medium",
    "Convert Hard",
    "Convert Extreme",
    "Insane",
];

const START_MARKER: &str = "<!-- BENCH:START -->";
const END_MARKER: &str = "<!-- BENCH:END -->";

struct Row {
    name: &'static str,
    mean_ms: f64,
    weight_kib: f64,
}

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let criterion_dir = manifest_dir.join("target/criterion/picturust");
    let bench_out_dir =
        |name: &str| std::env::temp_dir().join(format!("picturust-bench-{}", slug(name)));
    let readme_path = manifest_dir.join("README.md");

    let mut rows = Vec::new();
    for name in BENCH_NAMES {
        let estimate_path = criterion_dir.join(name).join("new/estimates.json");
        let mean_ms = read_mean_ms(&estimate_path).unwrap_or_else(|| {
            let estimate_path = estimate_path.display();
            panic!(
                "missing benchmark results for `{name}` at {estimate_path}. \
                 Run `cargo bench --bench pipeline` first."
            )
        });
        let weight_kib = read_output_weight_kib(&bench_out_dir(name)).unwrap_or_else(|| {
            panic!(
                "missing benchmark output file for `{name}` in {}. \
                 Run `cargo bench --bench pipeline` first.",
                bench_out_dir(name).display()
            )
        });
        rows.push(Row {
            name,
            mean_ms,
            weight_kib,
        });
    }

    let section = build_section(&rows);
    write_section(&readme_path, &section);
    println!(
        "Updated {} with fresh benchmark results.",
        readme_path.display()
    );
}

/// Mirrors `benches/pipeline.rs`'s slug rule so both sides agree on the
/// per-test output directory name, e.g. "Rotate - Flip" -> `rotate_flip`.
fn slug(name: &str) -> String {
    let mut out = String::new();
    let mut last_underscore = false;
    for c in name.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_underscore = false;
        } else if !last_underscore {
            out.push('_');
            last_underscore = true;
        }
    }
    out.trim_matches('_').to_string()
}

fn read_mean_ms(path: &Path) -> Option<f64> {
    let contents = fs::read_to_string(path).ok()?;
    let json: Value = serde_json::from_str(&contents).ok()?;
    let nanoseconds = json.get("mean")?.get("point_estimate")?.as_f64()?;
    Some(nanoseconds / 1_000_000.0)
}

fn read_output_weight_kib(dir: &Path) -> Option<f64> {
    let entry = fs::read_dir(dir).ok()?.find_map(Result::ok)?;
    let bytes = entry.metadata().ok()?.len();
    // Output file sizes here are well under 2^52 bytes.
    #[allow(clippy::cast_precision_loss)]
    let kib = bytes as f64 / 1024.0;
    Some(kib)
}

#[cfg(target_os = "windows")]
fn powershell_output(command: &str) -> Option<String> {
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", command])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!text.is_empty()).then_some(text)
}

#[cfg(target_os = "linux")]
fn cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|contents| {
            contents.lines().find_map(|line| {
                line.strip_prefix("model name").map(|rest| {
                    rest.trim_start_matches(|c: char| c == ':' || c.is_whitespace())
                        .trim()
                        .to_string()
                })
            })
        })
        .unwrap_or_else(|| "unknown CPU".to_string())
}

#[cfg(target_os = "windows")]
fn cpu_model() -> String {
    powershell_output("(Get-CimInstance Win32_Processor).Name")
        .unwrap_or_else(|| "unknown CPU".to_string())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn cpu_model() -> String {
    "unknown CPU".to_string()
}

#[cfg(target_os = "linux")]
fn memory_total() -> String {
    fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|contents| {
            contents.lines().find_map(|line| {
                line.strip_prefix("MemTotal:").map(|rest| {
                    let kib: u64 = rest.trim().trim_end_matches(" kB").parse().ok()?;
                    // kib is well under 2^52, so this cast is exact.
                    #[allow(clippy::cast_precision_loss)]
                    let gib = kib as f64 / 1_048_576.0;
                    Some(format!("{gib:.1} GiB"))
                })?
            })
        })
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(target_os = "windows")]
fn memory_total() -> String {
    powershell_output("(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory")
        .and_then(|bytes| bytes.parse::<u64>().ok())
        .map_or_else(
            || "unknown".to_string(),
            |bytes| {
                // Physical RAM sizes are well under 2^52, so this cast is exact.
                #[allow(clippy::cast_precision_loss)]
                let gib = bytes as f64 / 1_073_741_824.0;
                format!("{gib:.1} GiB")
            },
        )
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn memory_total() -> String {
    "unknown".to_string()
}

fn build_section(rows: &[Row]) -> String {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let mut out = String::new();
    writeln!(out, "{START_MARKER}").unwrap();
    writeln!(out, "OS: {} ({})", os_pretty_name(), std::env::consts::ARCH).unwrap();
    let cpu = cpu_model();
    writeln!(out, "\nCPU: {cpu} ({cores} threads)").unwrap();
    let memory = memory_total();
    writeln!(out, "\nMemory: {memory}").unwrap();
    writeln!(
        out,
        "\nSample: full decode-resize-encode pipeline on `Assets/Initial.png` (single picture, no folder walking).\n"
    )
    .unwrap();
    writeln!(
        out,
        "| Test | Mean time (ms) | Output weight (KiB) | Avg time per MiB (ms) |"
    )
    .unwrap();
    writeln!(
        out,
        "|------|----------------|----------------------|-----------------------|"
    )
    .unwrap();
    for row in rows {
        let mib = row.weight_kib / 1024.0;
        let ms_per_mib = row.mean_ms / mib;
        writeln!(
            out,
            "| {} | {:.2} | {:.1} | {ms_per_mib:.2} |",
            row.name, row.mean_ms, row.weight_kib
        )
        .unwrap();
    }
    write!(out, "{END_MARKER}").unwrap();
    out
}

#[cfg(target_os = "linux")]
fn os_pretty_name() -> String {
    fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|contents| {
            contents.lines().find_map(|line| {
                line.strip_prefix("PRETTY_NAME=")
                    .map(|rest| rest.trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| std::env::consts::OS.to_string())
}

#[cfg(target_os = "windows")]
fn os_pretty_name() -> String {
    powershell_output("(Get-CimInstance Win32_OperatingSystem).Caption")
        .unwrap_or_else(|| std::env::consts::OS.to_string())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn os_pretty_name() -> String {
    std::env::consts::OS.to_string()
}

fn write_section(readme_path: &Path, section: &str) {
    let readme = fs::read_to_string(readme_path).expect("could not read README.md");
    let start = readme
        .find(START_MARKER)
        .expect("README.md is missing the BENCH:START marker");
    let end = readme
        .find(END_MARKER)
        .expect("README.md is missing the BENCH:END marker")
        + END_MARKER.len();

    let mut updated = String::with_capacity(readme.len());
    updated.push_str(&readme[..start]);
    updated.push_str(section);
    updated.push_str(&readme[end..]);
    fs::write(readme_path, updated).expect("could not write README.md");
}

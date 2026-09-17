use criterion::{Criterion, criterion_group, criterion_main};
use image::imageops::FilterType;
use picturust::parameters::format::Format;
use picturust::parameters::parameters::Parameters;
use picturust::parameters::resizetype::ResizeType;
use picturust::parameters::rotation::Rotation;
use picturust::runner::transform_single;
use std::path::PathBuf;

fn sample(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Assets")
        .join(name)
}

fn base_params(output_dir: PathBuf) -> Parameters {
    Parameters {
        input_dir: sample("Initial.png"),
        recursive: false,
        output_dir,
        width: Some(1200),
        height: None,
        resize_type: ResizeType::Exact,
        filter: FilterType::Lanczos3,
        format: Format::None,
        quality: 75.0,
        speed: 7,
        rotation: Rotation::None,
        flip_horizontal: false,
        flip_vertical: false,
    }
}

/// Turns a test name into a filesystem-safe slug, e.g. "Rotate - Flip" ->
/// `rotate_flip`. Used to derive a stable output directory per test, which
/// `examples/update_readme_bench.rs` re-derives with the same rule to find
/// each test's output file.
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

fn out_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("picturust-bench-{}", slug(name)))
}

#[allow(clippy::significant_drop_tightening)]
fn run_case(c: &mut Criterion, name: &str, params: &Parameters, sample_size: usize) {
    let input = sample("Initial.png");
    let _ = std::fs::remove_dir_all(&params.output_dir);
    std::fs::create_dir_all(&params.output_dir).expect("could not create bench output dir");
    let mut group = c.benchmark_group("picturust");
    group.sample_size(sample_size);
    group.bench_function(name, |b| {
        b.iter(|| {
            transform_single(&input, params);
        });
    });
    group.finish();

    // Keep exactly one clean output file around (the bench iterations above
    // each produced their own numbered copy) so the README updater can read
    // its size.
    let _ = std::fs::remove_dir_all(&params.output_dir);
    std::fs::create_dir_all(&params.output_dir).expect("could not create bench output dir");
    transform_single(&input, params);
}

fn resize_benches(c: &mut Criterion) {
    run_case(c, "Lanczos3", &base_params(out_dir("Lanczos3")), 20);

    run_case(
        c,
        "Gaussian",
        &Parameters {
            filter: FilterType::Gaussian,
            ..base_params(out_dir("Gaussian"))
        },
        20,
    );

    run_case(
        c,
        "Thumbnail",
        &Parameters {
            resize_type: ResizeType::Thumbnail,
            ..base_params(out_dir("Thumbnail"))
        },
        20,
    );

    run_case(
        c,
        "Fill",
        &Parameters {
            resize_type: ResizeType::Fill,
            ..base_params(out_dir("Fill"))
        },
        20,
    );

    run_case(
        c,
        "Rotate - Flip",
        &Parameters {
            width: None,
            rotation: Rotation::Rotate180,
            flip_horizontal: true,
            flip_vertical: true,
            ..base_params(out_dir("Rotate - Flip"))
        },
        20,
    );
}

fn convert_benches(c: &mut Criterion) {
    run_case(
        c,
        "Convert Easy",
        &Parameters {
            width: None,
            format: Format::Jpeg,
            ..base_params(out_dir("Convert Easy"))
        },
        20,
    );

    run_case(
        c,
        "Convert Medium",
        &Parameters {
            width: None,
            format: Format::Webp,
            quality: 70.0,
            ..base_params(out_dir("Convert Medium"))
        },
        20,
    );

    run_case(
        c,
        "Convert Hard",
        &Parameters {
            width: None,
            format: Format::Avif,
            quality: 70.0,
            speed: 7,
            ..base_params(out_dir("Convert Hard"))
        },
        10,
    );

    run_case(
        c,
        "Convert Extreme",
        &Parameters {
            width: None,
            format: Format::Avif,
            quality: 70.0,
            speed: 3,
            ..base_params(out_dir("Convert Extreme"))
        },
        10,
    );

    run_case(
        c,
        "Insane",
        &Parameters {
            width: Some(1200),
            rotation: Rotation::Rotate180,
            flip_horizontal: true,
            flip_vertical: true,
            format: Format::Avif,
            quality: 70.0,
            speed: 1,
            ..base_params(out_dir("Insane"))
        },
        10,
    );
}

fn benches(c: &mut Criterion) {
    resize_benches(c);
    convert_benches(c);
}

criterion_group!(pipeline, benches);
criterion_main!(pipeline);

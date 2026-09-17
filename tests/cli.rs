use assert_cmd::Command;
use image::GenericImageView;
use std::fs;
use std::path::PathBuf;

fn sample(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Assets")
        .join(name)
}

fn bin() -> Command {
    Command::cargo_bin("picturust").expect("binary should build")
}

fn only_output_file(dir: &std::path::Path) -> PathBuf {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .expect("output dir should be readable")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    assert_eq!(
        entries.len(),
        1,
        "expected exactly one output file in {}, found {entries:?}",
        dir.display()
    );
    entries.remove(0)
}

#[test]
fn resizes_to_exact_dimensions() {
    let output = tempfile::tempdir().unwrap();
    bin()
        .args([
            "-i",
            sample("Initial.png").to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-W",
            "400",
            "-H",
            "300",
        ])
        .assert()
        .success();

    let out_file = only_output_file(output.path());
    let img = image::open(&out_file).expect("output should decode as an image");
    assert_eq!(img.dimensions(), (400, 300));
}

#[test]
fn converts_format_to_webp() {
    let output = tempfile::tempdir().unwrap();
    bin()
        .args([
            "-i",
            sample("Initial.png").to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-F",
            "Webp",
            "-Q",
            "70",
        ])
        .assert()
        .success();

    let out_file = only_output_file(output.path());
    assert_eq!(out_file.extension().unwrap(), "webp");
    image::open(&out_file).expect("output should decode as a webp image");
}

#[test]
fn converts_format_to_avif() {
    let output = tempfile::tempdir().unwrap();
    bin()
        .args([
            "-i",
            sample("Initial.png").to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-F",
            "Avif",
            "-S",
            "10",
            "-Q",
            "50",
        ])
        .assert()
        .success();

    let out_file = only_output_file(output.path());
    assert_eq!(out_file.extension().unwrap(), "avif");
    // This build has no AVIF decoder (ravif is encode-only), so just check the
    // file was actually written with plausible AVIF content.
    let bytes = fs::read(&out_file).expect("output file should be readable");
    assert!(
        bytes.len() > 100,
        "avif output looks too small: {} bytes",
        bytes.len()
    );
    assert_eq!(
        &bytes[4..8],
        b"ftyp",
        "output should have an AVIF/ISOBMFF header"
    );
}

#[test]
fn rotates_180_degrees() {
    let output = tempfile::tempdir().unwrap();
    let original = image::open(sample("Initial.png")).unwrap();

    bin()
        .args([
            "-i",
            sample("Initial.png").to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-r",
            "180",
        ])
        .assert()
        .success();

    let out_file = only_output_file(output.path());
    let rotated = image::open(&out_file).unwrap();
    assert_eq!(rotated.dimensions(), original.dimensions());
    assert_eq!(rotated.to_rgba8(), original.rotate180().to_rgba8());
}

#[test]
fn flips_horizontally_and_vertically() {
    let output = tempfile::tempdir().unwrap();
    let original = image::open(sample("Initial.png")).unwrap();

    bin()
        .args([
            "-i",
            sample("Initial.png").to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-s",
            "-v",
        ])
        .assert()
        .success();

    let out_file = only_output_file(output.path());
    let flipped = image::open(&out_file).unwrap();
    assert_eq!(flipped.to_rgba8(), original.fliph().flipv().to_rgba8());
}

#[test]
fn processes_a_folder_non_recursively() {
    let input = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    fs::copy(sample("Initial.png"), input.path().join("a.png")).unwrap();
    fs::copy(sample("Initial.png"), input.path().join("b.png")).unwrap();

    bin()
        .args([
            "-i",
            input.path().to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-W",
            "100",
        ])
        .assert()
        .success();

    let count = fs::read_dir(output.path()).unwrap().count();
    assert_eq!(count, 2);
}

#[test]
fn processes_a_folder_recursively() {
    let input = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let nested = input.path().join("nested");
    fs::create_dir_all(&nested).unwrap();
    fs::copy(sample("Initial.png"), input.path().join("a.png")).unwrap();
    fs::copy(sample("Initial.png"), nested.join("b.png")).unwrap();

    bin()
        .args([
            "-i",
            input.path().to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-R",
            "-W",
            "100",
        ])
        .assert()
        .success();

    let count = fs::read_dir(output.path()).unwrap().count();
    assert_eq!(count, 2);
}

#[test]
fn rejects_out_of_range_quality() {
    let output = tempfile::tempdir().unwrap();
    bin()
        .args([
            "-i",
            sample("Initial.png").to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
            "-F",
            "Webp",
            "-Q",
            "150",
        ])
        .assert()
        .failure();
}

//! Compare generated vs repaired comprehensive_demo — generation should already match repair output.

use std::io::Read;
use std::path::PathBuf;

use ppt_rs::oxml::repair::PptxRepair;

#[test]
fn compare_generated_vs_repaired_comprehensive_demo() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("comprehensive_demo.pptx");
    if !path.exists() {
        eprintln!("skip: run cargo run --example comprehensive_demo first");
        return;
    }

    let generated = std::fs::read(&path).expect("read generated");

    let mut repair = PptxRepair::open(&path).expect("open for repair");
    let issues = repair.validate();
    assert!(
        issues.is_empty(),
        "generated deck should pass repair validation without issues: {:?}",
        issues
            .iter()
            .map(|i| i.description())
            .collect::<Vec<_>>()
    );

    let result = repair.repair();
    assert!(
        result.issues_repaired.is_empty(),
        "repair should not modify a valid generated deck: {:?}",
        result
            .issues_repaired
            .iter()
            .map(|i| i.description())
            .collect::<Vec<_>>()
    );

    let repaired = repair.to_bytes().expect("serialize repaired package");
    assert_packages_equal(&generated, &repaired);
}

fn assert_packages_equal(a: &[u8], b: &[u8]) {
    use std::collections::HashSet;
    use zip::ZipArchive;

    let mut za = ZipArchive::new(std::io::Cursor::new(a)).expect("open generated zip");
    let mut zb = ZipArchive::new(std::io::Cursor::new(b)).expect("open repaired zip");

    let names_a: HashSet<String> = (0..za.len())
        .filter_map(|i| za.by_index(i).ok().map(|f| f.name().to_string()))
        .collect();
    let names_b: HashSet<String> = (0..zb.len())
        .filter_map(|i| zb.by_index(i).ok().map(|f| f.name().to_string()))
        .collect();

    assert_eq!(
        names_a, names_b,
        "repair changed package part list: only in generated {:?}, only in repaired {:?}",
        names_a.difference(&names_b).collect::<Vec<_>>(),
        names_b.difference(&names_a).collect::<Vec<_>>()
    );

    for name in &names_a {
        let mut ca = Vec::new();
        let mut cb = Vec::new();
        za.by_name(name).unwrap().read_to_end(&mut ca).unwrap();
        zb.by_name(name).unwrap().read_to_end(&mut cb).unwrap();
        assert_eq!(ca, cb, "repair changed part {name}");
    }
}

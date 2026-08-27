use std::fs;

use assert_cmd::Command;
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use predicates::prelude::*;

fn fixture_pdf(path: &std::path::Path) {
    let mut doc = Document::with_version("1.5");
    let font = doc.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica"
    });
    let resources = doc.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font }
    });
    let content = Content {
        operations: vec![
            Operation::new("rg", vec![0.into(), 0.into(), Object::Real(0.7)]),
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
            Operation::new(
                "Tm",
                vec![
                    1.into(),
                    0.into(),
                    0.into(),
                    1.into(),
                    72.into(),
                    700.into(),
                ],
            ),
            Operation::new("Tj", vec![Object::string_literal("fn main() {}")]),
            Operation::new("ET", vec![]),
        ],
    };
    let content = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let annotation = doc.add_object(dictionary! {
        "Type" => "Annot", "Subtype" => "Link",
        "Rect" => vec![72.into(), 680.into(), 140.into(), 695.into()],
        "Dest" => vec![Object::Name(b"guide".to_vec()), Object::Name(b"Fit".to_vec())]
    });
    let pages = doc.new_object_id();
    let page = doc.add_object(dictionary! {
        "Type" => "Page", "Parent" => pages, "Contents" => content,
        "Resources" => resources, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Annots" => vec![annotation.into()]
    });
    doc.objects.insert(
        pages,
        Object::Dictionary(dictionary! {
            "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1
        }),
    );
    let catalog = doc.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    doc.trailer.set("Root", catalog);
    doc.compress();
    doc.save(path).unwrap();
}

#[test]
fn documented_existing_pdf_flow_passes_and_writes_proof() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("manual.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Guide\n[Jump](#guide)\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();
    fixture_pdf(&pdf);

    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--pdf",
            pdf.to_str().unwrap(),
            "--out",
            proof.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"passed\": true"));

    let html = fs::read_to_string(proof.join("index.html")).unwrap();
    assert!(html.contains("PASS"));
    assert!(html.contains("Code Proof report"));
}

#[test]
fn missing_source_is_an_operational_error() {
    Command::cargo_bin("codeproof")
        .unwrap()
        .args(["check", "definitely-missing.md", "--pdf", "manual.pdf"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Markdown source not found"));
}

#[test]
fn help_explains_the_ci_surface() {
    Command::cargo_bin("codeproof")
        .unwrap()
        .args(["check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--deny-warnings"))
        .stdout(predicate::str::contains("--engine-command"));
}

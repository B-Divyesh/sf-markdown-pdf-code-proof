use std::fs;
#[cfg(unix)]
use std::net::TcpListener;
#[cfg(unix)]
use std::process::Stdio;
#[cfg(unix)]
use std::time::Duration;

use assert_cmd::Command;
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use predicates::prelude::*;

fn fixture_pdf(path: &std::path::Path, annotation_targets: &[&str], named_targets: &[&str]) {
    fixture_pdf_with_actions(path, annotation_targets, named_targets, &[]);
}

fn code_flow_pdf(path: &std::path::Path, runs: &[(&str, i64)]) {
    let mut doc = Document::with_version("1.5");
    let font = doc.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica"
    });
    let resources = doc.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font }
    });
    let mut operations = vec![
        Operation::new("rg", vec![0.into(), 0.into(), Object::Real(0.7)]),
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
    ];
    for (text, y) in runs {
        operations.push(Operation::new(
            "Tm",
            vec![
                1.into(),
                0.into(),
                0.into(),
                1.into(),
                72.into(),
                (*y).into(),
            ],
        ));
        operations.push(Operation::new("Tj", vec![Object::string_literal(*text)]));
    }
    operations.push(Operation::new("ET", vec![]));
    let content = doc.add_object(Stream::new(
        dictionary! {},
        Content { operations }.encode().unwrap(),
    ));
    let pages = doc.new_object_id();
    let page = doc.add_object(dictionary! {
        "Type" => "Page", "Parent" => pages, "Contents" => content,
        "Resources" => resources, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()]
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

fn positioned_text_pdf(
    path: &std::path::Path,
    x: i64,
    y: i64,
    crop_box: Option<[i64; 4]>,
    colored: bool,
) {
    let mut doc = Document::with_version("1.5");
    let font = doc.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica"
    });
    let resources = doc.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font }
    });
    let mut operations = Vec::new();
    if colored {
        operations.push(Operation::new(
            "rg",
            vec![0.into(), 0.into(), Object::Real(0.7)],
        ));
    }
    operations.extend([
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
        Operation::new(
            "Tm",
            vec![1.into(), 0.into(), 0.into(), 1.into(), x.into(), y.into()],
        ),
        Operation::new("Tj", vec![Object::string_literal("fn main() {}")]),
        Operation::new("ET", vec![]),
    ]);
    let content = doc.add_object(Stream::new(
        dictionary! {},
        Content { operations }.encode().unwrap(),
    ));
    let pages = doc.new_object_id();
    let mut page = dictionary! {
        "Type" => "Page", "Parent" => pages, "Contents" => content,
        "Resources" => resources, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()]
    };
    if let Some([left, bottom, right, top]) = crop_box {
        page.set(
            "CropBox",
            vec![left.into(), bottom.into(), right.into(), top.into()],
        );
    }
    let page = doc.add_object(page);
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

fn assert_page_overflow(x: i64, y: i64, crop_box: Option<[i64; 4]>, side: &str) {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join(format!("{side}-overflow.pdf"));
    let proof = temp.path().join("proof");
    fs::write(&markdown, "# Manual\n```rust\nfn main() {}\n```\n").unwrap();
    positioned_text_pdf(&pdf, x, y, crop_box, true);

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
        .code(1)
        .stdout(predicate::str::contains("geometry.text-overflow"))
        .stdout(predicate::str::contains(format!(
            "past the {side} page boundary"
        )));

    assert!(fs::read_to_string(proof.join("index.html"))
        .unwrap()
        .contains("HOLD"));
}

fn fixture_pdf_with_actions(
    path: &std::path::Path,
    annotation_targets: &[&str],
    named_targets: &[&str],
    action_indexes: &[usize],
) {
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
    let annotations = annotation_targets
        .iter()
        .enumerate()
        .map(|(index, target)| {
            let mut annotation = dictionary! {
                "Type" => "Annot", "Subtype" => "Link",
                "Rect" => vec![72.into(), 680.into(), 140.into(), 695.into()]
            };
            if action_indexes.contains(&index) {
                annotation.set(
                    "A",
                    dictionary! {
                        "S" => "GoTo", "D" => Object::Name(target.as_bytes().to_vec())
                    },
                );
            } else {
                annotation.set("Dest", Object::Name(target.as_bytes().to_vec()));
            }
            doc.add_object(annotation)
        })
        .collect::<Vec<_>>();
    let pages = doc.new_object_id();
    let page = doc.add_object(dictionary! {
        "Type" => "Page", "Parent" => pages, "Contents" => content,
        "Resources" => resources, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Annots" => annotations.iter().copied().map(Object::from).collect::<Vec<_>>()
    });
    doc.objects.insert(
        pages,
        Object::Dictionary(dictionary! {
            "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1
        }),
    );
    let mut name_entries = Vec::new();
    for target in named_targets {
        name_entries.push(Object::string_literal(*target));
        name_entries.push(Object::Array(vec![
            page.into(),
            Object::Name(b"Fit".to_vec()),
        ]));
    }
    let mut destinations = lopdf::Dictionary::new();
    destinations.set("Names", Object::Array(name_entries));
    let mut names = lopdf::Dictionary::new();
    names.set("Dests", destinations);
    let catalog = doc.add_object(dictionary! {
        "Type" => "Catalog", "Pages" => pages, "Names" => names
    });
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
    fixture_pdf(&pdf, &["guide"], &["guide"]);

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
fn flattened_code_lines_fail_the_release_contract() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("flattened.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Manual\n```rust\nlet first = 1;\nlet second = 2;\n```\n",
    )
    .unwrap();
    code_flow_pdf(&pdf, &[("let first = 1; let second = 2;", 700)]);

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
        .code(1)
        .stdout(predicate::str::contains("\"passed\": false"))
        .stdout(predicate::str::contains("code.flow-changed"));

    assert!(fs::read_to_string(proof.join("index.html"))
        .unwrap()
        .contains("HOLD"));
}

#[test]
fn wrapped_single_code_line_fails_the_release_contract() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("wrapped-single.md");
    let pdf = temp.path().join("wrapped-single.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Manual\n```javascript\nconst endpoint = \"https://example.test/api\"; return endpoint;\n```\n",
    )
    .unwrap();
    code_flow_pdf(
        &pdf,
        &[
            ("const endpoint = \"https://example.test/api\"; ", 700),
            ("return endpoint;", 682),
        ],
    );

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
        .code(1)
        .stdout(predicate::str::contains("\"passed\": false"))
        .stdout(predicate::str::contains("code.flow-changed"));

    assert!(fs::read_to_string(proof.join("index.html"))
        .unwrap()
        .contains("HOLD"));
}

#[test]
fn separately_positioned_code_lines_preserve_flow() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("line-shaped.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Manual\n```rust\nlet first = 1;\nlet second = 2;\n```\n",
    )
    .unwrap();
    code_flow_pdf(&pdf, &[("let first = 1;", 700), ("let second = 2;", 682)]);

    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--pdf",
            pdf.to_str().unwrap(),
            "--out",
            proof.to_str().unwrap(),
            "--no-highlight-check",
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"passed\": true"))
        .stdout(predicate::str::contains("\"findings\": []"));
}

#[test]
fn page_bounds_cover_every_media_and_crop_edge() {
    assert_page_overflow(-30, 700, None, "left");
    assert_page_overflow(590, 700, None, "right");
    assert_page_overflow(72, 820, None, "top");
    assert_page_overflow(72, -20, None, "bottom");
    assert_page_overflow(20, 700, Some([36, 36, 576, 756]), "left");
}

#[test]
fn missing_code_content_fails_the_release_contract() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("missing.pdf");
    fs::write(&markdown, "# Manual\n```rust\nlet required = true;\n```\n").unwrap();
    code_flow_pdf(&pdf, &[("let other = false;", 700)]);

    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--pdf",
            pdf.to_str().unwrap(),
            "--out",
            temp.path().join("proof").to_str().unwrap(),
            "--json",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("code.content-missing"));
}

#[test]
fn missing_syntax_color_warns_and_respects_warning_policy() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("plain.pdf");
    fs::write(&markdown, "# Manual\n```rust\nfn main() {}\n```\n").unwrap();
    positioned_text_pdf(&pdf, 72, 700, None, false);

    for (deny_warnings, expected_exit) in [(false, 0), (true, 1)] {
        let proof = temp.path().join(format!("proof-{deny_warnings}"));
        let mut command = Command::cargo_bin("codeproof").unwrap();
        command.args([
            "check",
            markdown.to_str().unwrap(),
            "--pdf",
            pdf.to_str().unwrap(),
            "--out",
            proof.to_str().unwrap(),
            "--json",
        ]);
        if deny_warnings {
            command.arg("--deny-warnings");
        }
        command
            .assert()
            .code(expected_exit)
            .stdout(predicate::str::contains("code.highlight-not-detected"));
    }
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
fn invalid_source_is_a_contract_failure_with_evidence() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("empty.md");
    let proof = temp.path().join("proof");
    fs::write(&markdown, "\n").unwrap();

    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--pdf",
            "unused.pdf",
            "--out",
            proof.to_str().unwrap(),
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("source.empty"));

    assert!(proof.join("index.html").is_file());
}

#[test]
fn custom_renderer_requires_safe_placeholders() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    fs::write(&markdown, "# Manual\n").unwrap();

    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--engine-command",
            "renderer-with-no-placeholders",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "must contain both {input} and {output}",
        ));
}

#[cfg(unix)]
#[test]
fn custom_renderer_runs_without_a_shell_and_is_checked() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let fixture = temp.path().join("fixture.pdf");
    let renderer = temp.path().join("render-fixture");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Guide\n[Jump](#guide)\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();
    fixture_pdf(&fixture, &["guide"], &["guide"]);
    fs::write(
        &renderer,
        format!("#!/bin/sh\ncp '{}' \"$2\"\n", fixture.display()),
    )
    .unwrap();
    fs::set_permissions(&renderer, fs::Permissions::from_mode(0o755)).unwrap();

    let command = format!("{} {{input}} {{output}}", renderer.display());
    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--engine-command",
            &command,
            "--out",
            proof.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"engine\": \"custom\""));
}

#[cfg(target_os = "linux")]
#[test]
fn renderer_refuses_to_start_when_sandbox_setup_fails() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let renderer = temp.path().join("renderer");
    let marker = temp.path().join("renderer-started");
    fs::write(&markdown, "# Manual\n").unwrap();
    fs::write(
        &renderer,
        format!("#!/bin/sh\ntouch '{}'\n", marker.display()),
    )
    .unwrap();
    fs::set_permissions(&renderer, fs::Permissions::from_mode(0o755)).unwrap();

    Command::cargo_bin("codeproof")
        .unwrap()
        .env("CODEPROOF_TEST_FORCE_SANDBOX_FAILURE", "1")
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--engine-command",
            &format!("{} {{input}} {{output}}", renderer.display()),
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "sandbox setup was deliberately refused",
        ));
    assert!(
        !marker.exists(),
        "renderer ran despite failed sandbox setup"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn renderer_deadline_stops_a_long_running_command() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    fs::write(&markdown, "# Manual\n").unwrap();
    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--engine-command",
            "/bin/sh -c 'sleep 2' {input} {output}",
            "--timeout",
            "1",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "renderer exceeded the 1 second timeout",
        ));
}

#[test]
fn json_report_and_exit_codes_are_observable() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let valid_pdf = temp.path().join("valid.pdf");
    let invalid_pdf = temp.path().join("invalid.pdf");
    fs::write(
        &markdown,
        "# Guide\n[Jump](#guide)\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();
    fixture_pdf(&valid_pdf, &["guide"], &["guide"]);
    code_flow_pdf(&invalid_pdf, &[("fn other() {}", 700)]);

    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--pdf",
            valid_pdf.to_str().unwrap(),
            "--out",
            temp.path().join("pass-proof").to_str().unwrap(),
            "--json",
        ])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("\"passed\": true"));
    assert!(temp.path().join("pass-proof/index.html").is_file());

    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--pdf",
            invalid_pdf.to_str().unwrap(),
            "--out",
            temp.path().join("hold-proof").to_str().unwrap(),
            "--json",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("\"passed\": false"));

    Command::cargo_bin("codeproof")
        .unwrap()
        .args(["check", "missing.md", "--pdf", valid_pdf.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Markdown source not found"));
}

#[cfg(target_os = "linux")]
#[test]
fn renderer_sandbox_denies_network_connections() {
    if std::process::Command::new("curl")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        return;
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    fs::write(&markdown, "# Manual\n").unwrap();
    let command = format!(
        "/bin/sh -c \"curl --connect-timeout 1 --max-time 1 -fsS http://{address}/network-probe >/dev/null; test $? -ne 0\" {{input}} {{output}}"
    );
    Command::cargo_bin("codeproof")
        .unwrap()
        .args([
            "check",
            markdown.to_str().unwrap(),
            "--engine-command",
            &command,
            "--timeout",
            "5",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("completed without creating"));

    std::thread::sleep(Duration::from_millis(150));
    assert!(
        listener.accept().is_err(),
        "sandboxed renderer reached the network"
    );
}

#[test]
fn valid_multiple_fragment_destinations_pass() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("manual.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Guide\n[First](#guide)\n# Second\n[Second](#second)\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();
    fixture_pdf_with_actions(&pdf, &["guide", "second"], &["guide", "second"], &[1]);

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
}

#[test]
fn duplicate_pdf_destination_cannot_satisfy_different_fragments() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("manual.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Guide\n[First](#guide)\n# Second\n[Second](#second)\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();
    fixture_pdf(&pdf, &["guide", "guide"], &["guide"]);

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
        .code(1)
        .stdout(predicate::str::contains("link.destination-missing"))
        .stdout(predicate::str::contains("Markdown link #second"));
}

#[test]
fn wrong_pdf_destination_cannot_satisfy_a_fragment() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("manual.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Guide\n[First](#guide)\n# Second\n[Second](#second)\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();
    fixture_pdf(&pdf, &["guide", "appendix"], &["guide", "appendix"]);

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
        .code(1)
        .stdout(predicate::str::contains("link.destination-missing"))
        .stdout(predicate::str::contains("Markdown link #second"));
}

#[test]
fn unresolved_named_destination_fails_the_fragment_contract() {
    let temp = tempfile::tempdir().unwrap();
    let markdown = temp.path().join("manual.md");
    let pdf = temp.path().join("manual.pdf");
    let proof = temp.path().join("proof");
    fs::write(
        &markdown,
        "# Guide\n[First](#guide)\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();
    fixture_pdf(&pdf, &["guide"], &[]);

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
        .code(1)
        .stdout(predicate::str::contains("link.destination-unresolved"))
        .stdout(predicate::str::contains(
            "#guide does not resolve to a PDF page",
        ));
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

#[test]
fn demo_uses_bundled_sample_data_and_writes_an_isolated_proof() {
    let temp = tempfile::tempdir().unwrap();
    let workspace = temp.path().join("demo");

    Command::cargo_bin("codeproof")
        .unwrap()
        .args(["demo", "--out", workspace.to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "DEMO HOLD — 1 expected defect found",
        ))
        .stdout(predicate::str::contains("code.flow-changed"))
        .stdout(predicate::str::contains(format!(
            "Sample workspace: {}",
            workspace.display()
        )));

    assert_eq!(
        fs::read_to_string(workspace.join("sample-manual.md")).unwrap(),
        codeproof::demo::SAMPLE_MARKDOWN
    );
    assert!(workspace.join("sample-manual.pdf").is_file());
    let proof = fs::read_to_string(workspace.join("proof/index.html")).unwrap();
    assert!(proof.contains("HOLD"));
    assert!(proof.contains("code.flow-changed"));
}

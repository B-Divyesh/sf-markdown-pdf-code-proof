use std::path::Path;

use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};

pub const SAMPLE_MARKDOWN: &str = include_str!("../examples/sample-manual.md");

/// Write the deterministic sample PDF used by `codeproof demo`.
///
/// Its one-line JavaScript fence is deliberately painted on two baselines so
/// the proof demonstrates the release-blocking flow check without a renderer.
pub fn write_sample_pdf(path: &Path) -> Result<(), String> {
    let mut document = Document::with_version("1.5");
    let font = document.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica"
    });
    let resources = document.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font }
    });
    let content = document.add_object(Stream::new(
        dictionary! {},
        Content {
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
                Operation::new(
                    "Tj",
                    vec![Object::string_literal(
                        "const endpoint = \"https://api.example.test/v1/releases\"; ",
                    )],
                ),
                Operation::new(
                    "Tm",
                    vec![
                        1.into(),
                        0.into(),
                        0.into(),
                        1.into(),
                        72.into(),
                        682.into(),
                    ],
                ),
                Operation::new(
                    "Tj",
                    vec![Object::string_literal("return fetch(endpoint);")],
                ),
                Operation::new("ET", vec![]),
            ],
        }
        .encode()
        .map_err(|error| format!("could not encode demo PDF: {error}"))?,
    ));
    let annotation = document.add_object(dictionary! {
        "Type" => "Annot", "Subtype" => "Link",
        "Rect" => vec![72.into(), 730.into(), 220.into(), 746.into()],
        "Dest" => Object::Name(b"retry-policy".to_vec())
    });
    let pages = document.new_object_id();
    let page = document.add_object(dictionary! {
        "Type" => "Page", "Parent" => pages, "Contents" => content,
        "Resources" => resources, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Annots" => vec![Object::from(annotation)]
    });
    document.objects.insert(
        pages,
        Object::Dictionary(dictionary! {
            "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1
        }),
    );
    let destinations = dictionary! {
        "Names" => vec![
            Object::string_literal("retry-policy"),
            Object::Array(vec![page.into(), Object::Name(b"Fit".to_vec())])
        ]
    };
    let catalog = document.add_object(dictionary! {
        "Type" => "Catalog", "Pages" => pages,
        "Names" => dictionary! { "Dests" => destinations }
    });
    document.trailer.set("Root", catalog);
    document.compress();
    document
        .save(path)
        .map(|_| ())
        .map_err(|error| format!("could not write demo PDF {}: {error}", path.display()))
}

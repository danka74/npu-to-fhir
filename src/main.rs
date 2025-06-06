use clap::Parser;
use fhir_sdk::time::{self, OffsetDateTime};
use time::{UtcDateTime, UtcOffset};
use std::fs::File;
use std::io::BufReader;

mod npu_concepts;
use crate::npu_concepts::Npubegreber;
use fhir_sdk::r5::resources::CodeSystem;
use fhir_sdk::r5::*;

use time::macros::format_description;

/// NPU to FHIR CodeSystem converter
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the NPU XML file
    #[arg(short, long)]
    input: String,
}
fn main() {
    let cli = Cli::parse();

    let file = File::open(&cli.input).expect("Cannot open file");

    let reader = BufReader::new(file);

    let data: Npubegreber = serde_xml_rs::from_reader(reader).expect("Failed to deserialize XML");

    println!("Count = {}", data.npubegreb.len());

    let code_system = CodeSystem::builder()
        .url("http://npu-terminology.org".to_owned())
        .version("INT 2025-05-28".to_owned())
        .name("NPU Terminology".to_owned())
        .title("NPU Terminology Code System".to_owned())
        .status(codes::PublicationStatus::Active)
        .experimental(false)
        .publisher("IUPAC & IFCC".to_owned())
        .content(codes::CodeSystemContentMode::Complete)
        .description("NPU Terminology International edition Code System".to_owned())
        .copyright_label("© 2025 IUPAC & IFCC".to_owned());

    let mut concept = Vec::<Option<resources::CodeSystemConcept>>::new();

    let mut count = 0;
    for npubegreb in data.npubegreb {
        // println!("Processing NPU code: {}", npubegreb.npu_code);
        if (npubegreb.active != "1") || (npubegreb.current_version != "true") {
            continue;
        }
        count += 1;
        concept.push(Some(
            resources::CodeSystemConcept::builder()
                .code(npubegreb.npu_code)
                .display(npubegreb.short_definition)
                .definition(npubegreb.full_definition)
                .property(
                    vec![
                        add_if_not_empty_date("created_date".to_owned(), npubegreb.created_date),
                        add_if_not_empty_date("change_date".to_owned(), npubegreb.change_date),
                        add_if_not_empty_string("system".to_owned(), npubegreb.system),
                        add_if_not_empty_string("sys_spec".to_owned(), npubegreb.sys_spec),
                        add_if_not_empty_string("component".to_owned(), npubegreb.component),
                        add_if_not_empty_string("comp_spec".to_owned(), npubegreb.comp_spec),
                        add_if_not_empty_string(
                            "kind_of_property".to_owned(),
                            npubegreb.kind_of_property,
                        ),
                        add_if_not_empty_string("proc".to_owned(), npubegreb.proc),
                        add_if_not_empty_string("unit".to_owned(), npubegreb.unit),
                        add_if_not_empty_string("specialty".to_owned(), npubegreb.specialty),
                        add_if_not_empty_string(
                            "context_dependent".to_owned(),
                            npubegreb.context_dependent,
                        ),
                        add_if_not_empty_string("scale_type".to_owned(), npubegreb.scale_type),
                        add_if_not_empty_string("replaces".to_owned(), npubegreb.replaces),
                        add_bool("active".to_owned(), true),
                        add_bool("current_version".to_owned(), true),
                    ]
                    .into_iter()
                    .filter(Option::is_some)
                    .collect(),
                )
                .build()
                .expect("Failed to build CodeSystemConcept"),
        ));
    }

    let code_system = code_system
        .count(count)    
        .concept(concept);
    let code_system = code_system.build().expect("Failed to build CodeSystem");
    let json =
        serde_json::to_string_pretty(&code_system).expect("Failed to serialize CodeSystem to JSON");
    println!("{}", json);
}

fn add_if_not_empty_string(
    property: String,
    str: String,
) -> Option<resources::CodeSystemConceptProperty> {
    if !str.is_empty() {
        Some(
            resources::CodeSystemConceptProperty::builder()
                .code(property)
                .value(resources::CodeSystemConceptPropertyValue::String(str))
                .build()
                .expect("Failed to build CodeSystemConceptProperty"),
        )
    } else {
        None
    }
}

fn add_if_not_empty_date(
    property: String,
    str: String,
) -> Option<resources::CodeSystemConceptProperty> {
    println!("Parsing date: \"{}\"", str);
    UtcDateTime::parse(
        &str,
        format_description!("[year]-[month]-[day] [hour]:[minute]"),
    )
    .ok()
    .map(|dt| {
        resources::CodeSystemConceptProperty::builder()
            .code(property)
            .value(resources::CodeSystemConceptPropertyValue::DateTime(
                fhir_sdk::DateTime::DateTime(dt.to_offset(UtcOffset::UTC).into()),
            ))
            .build()
            .expect("Failed to build CodeSystemConceptProperty")
    })
}

fn add_bool(property: String, b: bool) -> Option<resources::CodeSystemConceptProperty> {
    Some(
        resources::CodeSystemConceptProperty::builder()
            .code(property)
            .value(resources::CodeSystemConceptPropertyValue::Boolean(b))
            .build()
            .expect("Failed to build CodeSystemConceptProperty"),
    )
}

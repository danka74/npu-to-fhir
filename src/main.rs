use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Write};
use time::UtcDateTime;
use time::UtcOffset;
use time::macros::format_description;

mod npu_concepts;

use crate::npu_concepts::Npubegreber;

use fhir_sdk::r5::resources::CodeSystem;
use fhir_sdk::r5::*;

/// NPU to FHIR CodeSystem converter
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value = "npu_code_system.json")]
    /// Output file for the FHIR CodeSystem JSON
    output: String,

    /// Path to the NPU XML file
    input: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let file = File::open(&cli.input)?;
    let reader = BufReader::new(file);
    let data: Npubegreber = serde_xml_rs::from_reader(reader)?;

    let code_system = CodeSystem::builder()
        .id("npu-terminology".to_owned())
        .language("en-GB".to_owned())
        .text(
            types::Narrative::builder()
                .status(codes::NarrativeStatus::Empty)
                .div("<div xmlns=\"http://www.w3.org/1999/xhtml\" lang=\"en-GB\" xml:lang=\"en-GB\">Concepts not shown due to size of code system.</div>".to_owned())
                .build()
                .expect("Failed to build Narrative"),
        )
        .extension(vec![types::Extension::builder()
            .url("http://hl7.org/fhir/StructureDefinition/codesystem-use-markdown".to_owned())
            .value(
                types::ExtensionValue::Boolean(false)
            )
            .build()
            .expect("Failed to build Extension")])
        .url("http://npu-terminology.org".to_owned())
        .identifier(vec![Some(
            types::Identifier::builder()
                .system("urn:ietf:rfc:3986".to_owned())
                .value("urn:oid:1.2.208.176.9.1".to_owned())
                .build()
                .expect("Failed to build Identifier"),
        )])
        .version("INT 2025-05-28".to_owned())
        .name("NPUTerminology".to_owned())
        .title("NPU Terminology Code System".to_owned())
        .status(codes::PublicationStatus::Active)
        .experimental(false)
        .publisher("Danish National eHealth Authority".to_owned())
        .contact(vec![Some(
            types::ContactDetail::builder()
                .name("Danish National eHealth Authority".to_owned())
                .telecom(vec![
                    Some(
                        types::ContactPoint::builder()
                            .system(codes::ContactPointSystem::Url)
                            .value("https://npu-terminology.org".to_owned())
                            .build()
                            .expect("Failed to build ContactPoint"),
                    ),
                    Some(
                        types::ContactPoint::builder()
                            .system(codes::ContactPointSystem::Email)
                            .value("npu-terminology@sundhedsdata.dk".to_owned())
                            .build()
                            .expect("Failed to build ContactPoint"),
                    ),
                ])
                .build()
                .expect("Failed to build ContactDetail"),
        )])
        .case_sensitive(true)
        .compositional(false)
        .version_needed(false)
        .content(codes::CodeSystemContentMode::Complete)
        .value_set("http://npu-terminology.org/fhir/ValueSet/NPUFull".to_owned())
        .description(format!("NPU Terminology International edition Code System. Generated from {} on {}",
            cli.input,
            UtcDateTime::now().format(format_description!("[year]-[month]-[day] [hour]:[minute]"))?
        ))
        .copyright("The ownership and intellectual property rights of NPU terminology are shared between the International Federation of Clinical Chemistry and Laboratory Medicine (IFCC) (www.ifcc.org) and the International Union of Pure and Applied Chemistry (IUPAC) (www.iupac.org).".to_owned())
        .property(vec![
            Some(resources::CodeSystemProperty::builder()
                .code("created_date".to_owned())
                .uri("http://npu-terminology.org/property#created_date".to_owned())
                .description("The date when the NPU code was created".to_owned())
                .r#type(codes::PropertyType::DateTime)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("change_date".to_owned())
                .uri("http://npu-terminology.org/property#change_date".to_owned())
                .description("The date when the NPU code was last changed".to_owned())
                .r#type(codes::PropertyType::DateTime)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("status".to_owned())
                .uri("http://hl7.org/fhir/concept-properties#status".to_owned())
                .description("A code that indicates the status of the concept. Typical values are active, experimental, deprecated, and retired".to_owned())
                .r#type(codes::PropertyType::Code)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("system".to_owned())
                .uri("http://npu-terminology.org/property#system".to_owned())
                .description("The system of the the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()   
                .code("sys_spec".to_owned())
                .uri("http://npu-terminology.org/property#sys_spec".to_owned())
                .description("The system specification of the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("component".to_owned())
                .uri("http://npu-terminology.org/property#component".to_owned())
                .description("The component of the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("comp_spec".to_owned())
                .uri("http://npu-terminology.org/property#comp_spec".to_owned())
                .description("The component specification of the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("kind_of_property".to_owned())
                .uri("http://npu-terminology.org/property#kind_of_property".to_owned())
                .description("The kind of property of the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("proc".to_owned())
                .uri("http://npu-terminology.org/property#proc".to_owned())
                .description("The procedure associated with the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("unit".to_owned())
                .uri("http://npu-terminology.org/property#unit".to_owned())
                .description("The unit of measurement for the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("specialty".to_owned())
                .uri("http://npu-terminology.org/property#specialty".to_owned())
                .description("The specialty associated with the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("context_dependent".to_owned())
                .uri("http://npu-terminology.org/property#context_dependent".to_owned())
                .description("Indicates if the NPU concept is context dependent".to_owned())
                .r#type(codes::PropertyType::Boolean)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("scale_type".to_owned())
                .uri("http://npu-terminology.org/property#scale_type".to_owned())
                .description("The scale type of the NPU concept".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            Some(resources::CodeSystemProperty::builder()
                .code("replaces".to_owned())
                .uri("http://npu-terminology.org/property#replaces".to_owned())
                .description("The retired NPU concept that this concept replaces".to_owned())
                .r#type(codes::PropertyType::String)
                .build()
                .expect("Failed to build CodeSystemProperty")),
            /* Some(resources::CodeSystemProperty::builder()
                .code("current_version".to_owned())
                .uri("http://npu-terminology.org/property#current_version".to_owned())
                .description("Indicates if this is the current version of the NPU code".to_owned())
                .r#type(codes::PropertyType::Boolean)
                .build()
                .expect("Failed to build CodeSystemProperty")), */
        ]);

    let mut concept = Vec::<Option<resources::CodeSystemConcept>>::new();

    let mut count = 0;
    for npubegreb in data.npubegreb {
        if npubegreb.current_version != "true" {
            continue;
        }

        count += 1;

        concept.push(Some(
            resources::CodeSystemConcept::builder()
                .code(npubegreb.npu_code)
                .display(npubegreb.short_definition)
                .definition(remove_duplicate_spaces(
                    &npubegreb.full_definition.replace("\n", ""),
                ))
                .property(
                    vec![
                        add_if_not_empty_date("created_date".to_owned(), npubegreb.created_date),
                        add_if_not_empty_date("change_date".to_owned(), npubegreb.change_date),
                        add_if_not_empty_code("status".to_owned(), if npubegreb.active == "1" { "active".to_owned() } else { "retired".to_owned() }),
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
                        add_bool(
                            "context_dependent".to_owned(),
                            npubegreb.context_dependent == "Ja",
                        ),
                        add_if_not_empty_string("scale_type".to_owned(), npubegreb.scale_type),
                        add_if_not_empty_string("replaces".to_owned(), npubegreb.replaces),
                        /* add_bool(
                            "current_version".to_owned(),
                            npubegreb.current_version == "true",
                        ), */
                    ]
                    .into_iter()
                    .filter(Option::is_some)
                    .collect(),
                )
                .build()
                .expect("Failed to build CodeSystemConcept"),
        ));
    }

    let code_system = code_system.count(count).concept(concept);
    let code_system = code_system.build().expect("Failed to build CodeSystem");
    let json =
        serde_json::to_string_pretty(&code_system).expect("Failed to serialize CodeSystem to JSON");

    let mut output = File::create(&cli.output)?;
    output.write_all(json.as_bytes())?;

    Ok(())
}

fn remove_duplicate_spaces(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_space = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(c);
            prev_space = false;
        }
    }
    result.trim().to_string()
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

fn add_if_not_empty_code(
    property: String,
    str: String,
) -> Option<resources::CodeSystemConceptProperty> {
    if !str.is_empty() {
        Some(
            resources::CodeSystemConceptProperty::builder()
                .code(property)
                .value(resources::CodeSystemConceptPropertyValue::Code(str))
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

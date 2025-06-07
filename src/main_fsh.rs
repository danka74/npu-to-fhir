use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use time::macros::format_description;
use time::{OffsetDateTime, UtcDateTime, format_description::well_known::Iso8601};

mod npu_concepts;

use crate::npu_concepts::Npubegreber;

/* use fhir_sdk::r5::resources::CodeSystem;
use fhir_sdk::r5::*; */

/// NPU to FHIR CodeSystem converter
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the NPU XML file
    input: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let file = File::open(&cli.input)?;
    let reader = BufReader::new(file);
    let data: Npubegreber = serde_xml_rs::from_reader(reader)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open("npu.fsh")?;

    writeln!(file, "// NPU Terminology Code System")?;
    let now = OffsetDateTime::now_utc();
    writeln!(file, "// Generated from NPU XML data {}", cli.input)?;
    writeln!(file, "// on {}", now)?;
    writeln!(file)?;
    writeln!(file, "CodeSystem: NPU")?;
    writeln!(file, "Id: npu-terminology")?;
    writeln!(file, "Title: \"NPU Terminology Code System\"")?;
    writeln!(file, "* ^name = \"NPUTerminology\"")?;
    writeln!(file, "* ^url = \"http://npu-terminology.org\"")?;
    writeln!(file, "* ^identifier[+].system = \"urn:ietf:rfc:3986\"")?;
    writeln!(file, "* ^identifier[=].value = \"urn:oid:1.2.208.176.9.1\"")?;
    writeln!(file, "* ^version = \"INT 2025-05-28\"")?;
    writeln!(file, "* ^status = #active")?;
    writeln!(file, "* ^experimental = false")?;
    writeln!(file, "* ^publisher = \"Danish National eHealth Authority\"")?;
    writeln!(
        file,
        "* ^contact[+].name = \"Danish National eHealth Authority\""
    )?;
    writeln!(file, "* ^contact[=].telecom[+].system = #url")?;
    writeln!(
        file,
        "* ^contact[=].telecom[=].value = \"https://npu-terminology.org\""
    )?;
    writeln!(file, "* ^contact[=].telecom[+].system = #email")?;
    writeln!(
        file,
        "* ^contact[=].telecom[=].value = \"npu-terminology@sundhedsdata.dk\""
    )?;
    writeln!(file, "* ^caseSensitive = true")?;
    writeln!(file, "* ^compositional = false")?;
    writeln!(file, "* ^content = #complete")?;
    writeln!(
        file,
        "* ^description = \"NPU Terminology International edition Code System\""
    )?;
    
    let mut count = 0;
    for npubegreb in data.npubegreb {
        if (npubegreb.active != "1") || (npubegreb.current_version != "true") {
            continue;
        }

        count += 1;

        // add FSH code with display and definition. For the definition, newlines and multiple spaces should be removed.
        writeln!(
            file,
            "* #{} \"{}\" \"\"\"{}\"\"\"",
            npubegreb.npu_code, npubegreb.short_definition, remove_duplicate_spaces(&npubegreb.full_definition.replace("\n", ""))
        )?;
        write!(file, "{}", add_if_not_empty_date("created_date".to_owned(), npubegreb.created_date))?;
        write!(file, "{}", add_if_not_empty_date("changed_date".to_owned(), npubegreb.change_date))?;
        write!(file, "{}", add_if_not_empty_string("system".to_owned(), npubegreb.system))?;
        write!(file, "{}", add_if_not_empty_string("sys_spec".to_owned(), npubegreb.sys_spec))?;
        write!(file, "{}", add_if_not_empty_string("component".to_owned(), npubegreb.component))?;
        write!(file, "{}", add_if_not_empty_string("comp_spec".to_owned(), npubegreb.comp_spec))?;
        write!(file, "{}", add_if_not_empty_string("kind_of_property".to_owned(), npubegreb.kind_of_property))?;
        write!(file, "{}", add_if_not_empty_string("proc".to_owned(), npubegreb.proc))?;
        write!(file, "{}", add_if_not_empty_string("unit".to_owned(), npubegreb.unit))?;
        write!(file, "{}", add_if_not_empty_string("specialty".to_owned(), npubegreb.specialty))?;
        write!(file, "{}", add_if_not_empty_string("context_dependent".to_owned(), npubegreb.context_dependent))?;
        write!(file, "{}", add_if_not_empty_string("scale_type".to_owned(), npubegreb.scale_type))?;
        write!(file, "{}", add_if_not_empty_string("replaces".to_owned(), npubegreb.replaces))?;
        write!(file, "{}", add_bool("active".to_owned(), npubegreb.active == "1"))?;
        write!(file, "{}", add_bool("current_version".to_owned(), npubegreb.current_version == "true"))?;
        
    
    }

    writeln!(file, "* ^count = {}", count)?;
    Ok(())
}

fn add_if_not_empty_string(
    property: String,
    str: String,
) -> String {
    if !str.is_empty() {
        "  * ^property[+].code = #".to_owned()
            + &property
            + "\n  * ^property[=].valueString = \""
            + &str
            + "\"\n"
    } else {
        String::default()
    }
}

fn add_if_not_empty_date(property: String, str: String) -> String {
    UtcDateTime::parse(
        &str,
        format_description!("[year]-[month]-[day] [hour]:[minute]"),
    )
    .ok()
    .map(|dt| {
        "  * ^property[+].code = #".to_owned()
            + &property
            + "\n  * ^property[=].valueDateTime = \""
            + &dt.format(&Iso8601::DATE_TIME_OFFSET).unwrap()
            + "\"\n"
    }).unwrap_or("".to_owned())
}

fn add_bool(property: String, b: bool) -> String {
    "  * ^property[+].code = #".to_owned()
            + &property
            + "\n  * ^property[=].valueBoolean = "
            + &b.to_string()
            + "\n"
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


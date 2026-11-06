/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Standalone offline PNM conversion; core owns format detection and decoding.
#![forbid(unsafe_code)]
mod analysis;
mod output;
mod pre_eq;
use pnm::{
    common::json::DEFAULT_MAX_JSON_BYTES,
    diagnostic::{ErrorCode, ParseError},
    pnm::parser::{parse_limits::ParseLimits, pnm_file::PnmFile},
};
use std::{
    ffi::OsString,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::ExitCode,
};

const MAX_INPUT_BYTES: u64 = 16 * 1024 * 1024;
const IO_ERROR: u8 = 1;
const USAGE_ERROR: u8 = 2;
const UNSUPPORTED_ERROR: u8 = 3;
const DECODE_ERROR: u8 = 4;
const HELP: &str = "pnm-decode - decode a PNM capture to JSON

Usage: pnm-decode [OPTIONS] INPUT

Options:
  -o, --output PATH   Write JSON to PATH (default: INPUT with .json extension)
      --stdout        Write JSON to stdout instead of a file
      --header-only   Export common header metadata only
      --rxmer-bit-loading  Analyze uniform RxMER modulation and gross rate
      --model NAME    webui_mer_heuristic (default), ossi_candidate_defaults,
                      or ideal_shannon
      --start-hz HZ   First inclusive carrier-center bound (with --stop-hz)
      --stop-hz HZ    Last inclusive carrier-center bound
      --margin-db DB  Additional nonnegative MER margin (default: 0)
      --max-bit-loading BITS  Data bit cap (default: 13, 8192-QAM)
      --pre-eq-update PATH  Attach a newer compatible last-update PNM7
      --update-mode MODE   Explicit adjust or set interpretation
      --precheck-only      Validate the Pre-EQ pair without prediction
  -h, --help          Show this help
  -V, --version       Show version
      --              End options (for filenames beginning with a dash)
";

#[derive(Default)]
struct Arguments {
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    stdout: bool,
    header_only: bool,
    analysis: analysis::Options,
    pre_eq: pre_eq::Options,
}
struct Failure(u8, String);
impl From<std::io::Error> for Failure {
    /// Maps filesystem or stream errors to the CLI I/O status.
    /// # Arguments
    /// * `error` - Original operating-system error.
    /// # Returns
    /// An I/O diagnostic retaining the original message.
    fn from(error: std::io::Error) -> Self {
        Self(IO_ERROR, error.to_string())
    }
}
impl From<ParseError> for Failure {
    /// Maps structured parse errors while preserving field and byte offset.
    /// # Arguments
    /// * `error` - Core parser diagnostic.
    /// # Returns
    /// Unsupported-format or malformed-data CLI status.
    fn from(error: ParseError) -> Self {
        let status = match error.code {
            ErrorCode::UnsupportedType
            | ErrorCode::UnsupportedVersion
            | ErrorCode::UnsupportedLayout
            | ErrorCode::UnsupportedDecoder => UNSUPPORTED_ERROR,
            _ => DECODE_ERROR,
        };
        Self(status, error.to_string())
    }
}

fn parse_arguments(
    arguments: impl Iterator<Item = OsString>,
) -> Result<Option<Arguments>, Failure> {
    let mut parsed = Arguments::default();
    let mut arguments = arguments.peekable();
    let mut positional_only = false;
    while let Some(argument) = arguments.next() {
        if !positional_only {
            if argument == "--" {
                positional_only = true;
                continue;
            }
            if argument == "--help" || argument == "-h" {
                std::io::stdout().lock().write_all(HELP.as_bytes())?;
                return Ok(None);
            }
            if argument == "--version" || argument == "-V" {
                writeln!(
                    std::io::stdout().lock(),
                    "pnm-decode {}",
                    env!("CARGO_PKG_VERSION")
                )?;
                return Ok(None);
            }
            if parsed.pre_eq.parse(&argument, &mut arguments)? {
                continue;
            }
            if parsed.analysis.parse(&argument, &mut arguments)? {
                continue;
            }
            if argument == "--output" || argument == "-o" {
                if parsed.output.is_some() {
                    return Err(Failure(
                        USAGE_ERROR,
                        "output specified more than once".into(),
                    ));
                }
                parsed.output = Some(
                    arguments
                        .next()
                        .ok_or_else(|| Failure(USAGE_ERROR, "--output requires a path".into()))?
                        .into(),
                );
                continue;
            }
            if argument == "--stdout" {
                parsed.stdout = true;
                continue;
            }
            if argument == "--header-only" {
                parsed.header_only = true;
                continue;
            }
            if argument.to_string_lossy().starts_with('-') {
                return Err(Failure(
                    USAGE_ERROR,
                    format!("unknown option: {}", argument.to_string_lossy()),
                ));
            }
        }
        if parsed.input.replace(argument.into()).is_some() {
            return Err(Failure(
                USAGE_ERROR,
                "expected exactly one input file".into(),
            ));
        }
    }
    if parsed.input.is_none() || (parsed.stdout && parsed.output.is_some()) {
        return Err(Failure(
            USAGE_ERROR,
            "provide INPUT and choose either --output or --stdout; see --help".into(),
        ));
    }
    parsed.analysis.validate(parsed.header_only)?;
    parsed
        .pre_eq
        .validate(parsed.header_only, parsed.analysis.enabled)?;
    Ok(Some(parsed))
}

fn read_input(input: &std::path::Path) -> Result<Vec<u8>, Failure> {
    let mut bytes = Vec::new();
    let source = File::open(input)?;
    if !source.metadata()?.is_file() {
        return Err(Failure(IO_ERROR, "input must be a regular file".into()));
    }
    source.take(MAX_INPUT_BYTES + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_INPUT_BYTES {
        return Err(Failure(DECODE_ERROR, "input exceeds 16 MiB limit".into()));
    }
    Ok(bytes)
}

fn run() -> Result<(), Failure> {
    let Some(arguments) = parse_arguments(std::env::args_os().skip(1))? else {
        return Ok(());
    };
    let input = arguments.input.expect("argument parser requires input");
    let bytes = read_input(&input)?;
    let file = PnmFile::from_bytes(&bytes);
    let json = if arguments.pre_eq.correction.is_some() {
        Ok(arguments
            .pre_eq
            .json(file.decode(ParseLimits::default())?)?)
    } else if arguments.analysis.enabled {
        Ok(arguments
            .analysis
            .json(file.decode(ParseLimits::default())?)?)
    } else if arguments.header_only {
        file.header()
            .map_err(Clone::clone)?
            .to_json(false, DEFAULT_MAX_JSON_BYTES)
    } else {
        file.decode(ParseLimits::default())?
            .to_json(DEFAULT_MAX_JSON_BYTES)
    }
    .map_err(|error| Failure(DECODE_ERROR, error.to_string()))?;
    if arguments.stdout {
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(json.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
    } else {
        let destination = arguments.output.unwrap_or_else(|| {
            input.with_extension(if arguments.pre_eq.precheck_only {
                "precheck.json"
            } else if arguments.pre_eq.correction.is_some() {
                "pre-eq-comparison.json"
            } else if arguments.analysis.enabled {
                "bit-loading.json"
            } else {
                "json"
            })
        });
        output::save(&destination, json.as_bytes())?;
        eprintln!("Wrote {}", destination.display());
    }
    Ok(())
}

/// Runs conversion and returns a stable process status.
/// # Returns
/// Zero on success; a documented failure code with diagnostics on stderr otherwise.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Failure(status, message)) => {
            eprintln!("pnm-decode: {message}");
            ExitCode::from(status)
        }
    }
}

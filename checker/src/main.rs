//! Checks CSV data files for validity.

extern crate checker; // The "src/lib.rs" module.
extern crate colored; // Allows outputting pretty terminal colors.
extern crate rayon; // A work-stealing auto-parallelism library.
extern crate walkdir; // Allows walking through a directory, looking at files.

use checker::{compiler, AllMeetData, SingleMeetData};
use colored::*;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Stores user-specified arguments from the command line.
struct Args {
    /// Whether the usage information should be printed.
    help: bool,
    /// Prints debug info for a single lifter's Age.
    debug_age_username: Option<String>,
    /// Prints debug info for a single lifter's Country.
    debug_country_username: Option<String>,
    /// Whether the database should be compiled for the server.
    compile: bool,
    /// Whether the database should be compiled for humans.
    compile_onefile: bool,
    /// Any remaining unrecognized arguments.
    free: Vec<String>,
}

// For purposes of testing, a meet directory is any directory containing
// either of the files "entries.csv" or "meet.csv".
fn is_meetdir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
        && (entry.path().join("entries.csv").exists()
            || entry.path().join("meet.csv").exists())
}

/// Determines the project root from the binary path.
fn get_project_root() -> Result<PathBuf, Box<dyn Error>> {
    const ERR: &str = "get_project_root() ran out of parent directories";
    Ok(env::current_exe()? // root/target/release/binary
        .parent()
        .ok_or(ERR)? // root/target/release
        .parent()
        .ok_or(ERR)? // root/target
        .parent()
        .ok_or(ERR)? // root
        .to_path_buf())
}

/// Outputs a report to stdout with the StdoutLock held for atomicity.
///
/// Failure to write a report is not itself an error that needs reporting.
fn write_report(handle: &mut io::StdoutLock, report: checker::Report) {
    // Output the full name of the file in default coloring.
    let _ = handle.write_fmt(format_args!("{}\n", report.path.to_str().unwrap()));

    // Output each message with some festive coloring.
    for message in report.messages {
        match message {
            checker::Message::Error(s) => {
                let _ = handle.write_fmt(format_args!(" {}\n", s.bold().red()));
            }
            checker::Message::Warning(s) => {
                let _ = handle.write_fmt(format_args!(" {}\n", s.bold().yellow()));
            }
        }
    }
}

/// Outputs a final summary line.
fn print_summary(error_count: usize, warning_count: usize) {
    let error_str = format!(
        "{} error{}",
        error_count,
        if error_count == 1 { "" } else { "s" }
    );
    let warning_str = format!(
        "{} warning{}",
        warning_count,
        if warning_count == 1 { "" } else { "s" }
    );

    let error_str = if error_count > 0 {
        error_str.bold().red().to_string()
    } else {
        error_str.bold().green().to_string()
    };

    let warning_str = if warning_count > 0 {
        warning_str.bold().yellow().to_string()
    } else {
        warning_str.bold().green().to_string()
    };

    println!("Summary: {}, {}", error_str, warning_str);
}

/// Map of federation folder, e.g., "ipf", to Config.
type ConfigMap = BTreeMap<String, checker::Config>;

/// Reads in all CONFIG.toml files project-wide.
///
/// Returns a map of (path -> Config) on success, or (errors, warnings) on
/// failure.
fn get_configurations(meet_data_root: &Path) -> Result<ConfigMap, (usize, usize)> {
    let mut configmap = ConfigMap::new();

    // Build a list of every CONFIG.toml.
    let configs = WalkDir::new(&meet_data_root)
        .min_depth(1)
        .max_depth(2) // Allow apf/CONFIG.toml or mags/plusa/CONFIG.toml.
        .into_iter()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let mut path = e.into_path();
                path.push("CONFIG.toml");
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            })
        });

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut error_count: usize = 0;
    let mut warning_count: usize = 0;

    // Parse each CONFIG.toml and file it in a hashmap.
    for configpath in configs {
        // Remember the filename for error reporting.
        let sourcefile: PathBuf = configpath.clone();

        match checker::check_config(configpath) {
            Ok(result) => {
                // Tally up and output and errors and warnings.
                let (errors, warnings) = result.report.count_messages();
                if errors > 0 || warnings > 0 {
                    error_count += errors;
                    warning_count += warnings;
                    write_report(&mut handle, result.report);
                }

                // Add the Config to the map.
                if let Some(config) = result.config {
                    // This has to be safe if the config parsed correctly.
                    let feddir = sourcefile.parent().and_then(Path::file_name).unwrap();
                    configmap.insert(feddir.to_str().unwrap().to_string(), config);
                }
            }
            Err(e) => {
                println!("{}", sourcefile.as_path().to_str().unwrap());
                println!(" Internal Error: {}", e.to_string().bold().red());
                return Err((error_count + 1, warning_count));
            }
        }
    }

    // If there were errors, don't return anything.
    if error_count > 0 {
        Err((error_count, warning_count))
    } else {
        Ok(configmap)
    }
}

/// Displays the help message for the CLI argument parser.
fn display_help_message() {
    println!(
        r#"
OpenPowerlifting Checker
Checks and compiles the OpenPowerlifting database

USAGE:
    checker [FLAGS] [OPTIONS] [PATH]

FLAGS:
    -c, --compile            Compiles the database into build/*.csv
    -1, --compile-onefile    Compiles build/openpowerlifting.csv, the easy-use variant
    -h, --help               Prints this help information

OPTIONS:
        --age <username>        Prints age debug info for the given username
        --country <username>    Prints country debug info for the given username

ARGS:
    <PATH>    Optionally restricts processing to just this parent directory
"#
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    // Build the command-line argument parsing in code.
    // Get the arguments that were passed to the program, ignoring the binary name
    let mut args = pico_args::Arguments::from_env();

    // Parse the arguments.
    let args = Args {
        help: args.contains(["-h", "--help"]),
        debug_age_username: args.opt_value_from_str("--age")?,
        debug_country_username: args.opt_value_from_str("--country")?,
        compile: args.contains(["-c", "--compile"]),
        compile_onefile: args.contains(["-1", "--compile-onefile"]),
        free: args.free()?,
    };

    // If the help message was requested, display it and exit immediately.
    if args.help {
        display_help_message();
        return Ok(());
    }

    // Get handles to various parts of the project.
    let project_root = get_project_root()?;
    let meet_data_root = project_root.join("meet-data");
    if !meet_data_root.exists() {
        panic!("Path '{}' does not exist", meet_data_root.to_str().unwrap());
    }

    // Validate arguments.
    let is_compiling: bool = args.compile || args.compile_onefile;
    let is_debugging: bool =
        args.debug_age_username.is_some() || args.debug_country_username.is_some();

    // Any free argument is interpreted as a folder for limiting checking scope.
    let search_root = if args.free.is_empty() {
        meet_data_root.clone()
    } else {
        // Assume the path is the first element of args.free.
        let full_path = env::current_dir()?.join(&args.free[0]);
        // Canonicalization will fail if the path doesn't exist.
        match full_path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                let msg = full_path.to_str().unwrap();
                println!("{}: {}", msg, e);
                process::exit(1);
            }
        }
    };

    let configmap = match get_configurations(&meet_data_root) {
        Ok(configmap) => configmap,
        Err((errors, warnings)) => {
            print_summary(errors, warnings);
            process::exit(1);
        }
    };

    // Build a list of every directory containing meet results.
    let meetdirs: Vec<DirEntry> = WalkDir::new(&search_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| is_meetdir(entry))
        .collect();

    let error_count = AtomicUsize::new(0);
    let warning_count = AtomicUsize::new(0);

    // Unexpected errors that occurred while reading files.
    let internal_error_count = AtomicUsize::new(0);

    // Iterate in parallel over each meet directory and apply checks.
    let singlemeets: Vec<SingleMeetData> = meetdirs
        .into_par_iter()
        .filter_map(|dir| {
            // Determine the appropriate Config for this meet.
            let feddir = dir
                .path()
                .parent()
                .and_then(Path::file_name)
                .and_then(std::ffi::OsStr::to_str)
                .unwrap();
            let config = configmap.get(feddir);

            // Check the meet.
            match checker::check(dir.path(), config) {
                Ok(checkresult) => {
                    let reports = checkresult.reports;
                    // Count how many new errors and warnings were generated.
                    let mut local_errors = 0;
                    let mut local_warnings = 0;
                    for report in &reports {
                        let (errors, warnings) = report.count_messages();
                        local_errors += errors;
                        local_warnings += warnings;
                    }

                    // Update the global error and warning counts.
                    if local_errors > 0 {
                        error_count.fetch_add(local_errors, Ordering::SeqCst);
                    }
                    if local_warnings > 0 {
                        warning_count.fetch_add(local_warnings, Ordering::SeqCst);
                    }

                    // Emit reports all together.
                    if local_errors > 0 || local_warnings > 0 {
                        let stdout = io::stdout();
                        let mut handle = stdout.lock();
                        for report in reports {
                            write_report(&mut handle, report);
                        }
                    }

                    // Map to the SingleMeetData for collection.
                    match (checkresult.meet, checkresult.entries) {
                        (Some(meet), Some(entries)) => {
                            Some(SingleMeetData { meet, entries })
                        }
                        _ => None,
                    }
                }
                Err(e) => {
                    internal_error_count.fetch_add(1, Ordering::SeqCst);
                    let stderr = io::stderr();
                    let mut handle = stderr.lock();
                    let _ = handle
                        .write_fmt(format_args!("{}\n", dir.path().to_str().unwrap()));
                    let _ = handle.write_fmt(format_args!(
                        " Internal Error: {}\n",
                        e.to_string().bold().red()
                    ));
                    None
                }
            }
        })
        .collect();

    // Give ownership to the permanent data store.
    let mut meetdata = AllMeetData::from(singlemeets);

    // Move out of atomics.
    let mut error_count = error_count.load(Ordering::SeqCst);
    let mut warning_count = warning_count.load(Ordering::SeqCst);
    let internal_error_count = internal_error_count.load(Ordering::SeqCst);

    // Check the lifter-data/ files.
    let result = checker::check_lifterdata(&project_root.join("lifter-data"));
    for report in result.reports {
        let (errors, warnings) = report.count_messages();
        error_count += errors;
        warning_count += warnings;

        // Pretty-print any messages.
        if report.has_messages() {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            write_report(&mut handle, report);
        }
    }
    let lifterdata = result.map;

    // Check for username errors.
    // FIXME: This adds a whole second to the checker time.
    // FIXME: Lifetimes are really making this harder than it should be.
    // FIXME: Otherwise we could just do the checking in create_liftermap().
    let liftermap = meetdata.create_liftermap();
    for lifter_indices in liftermap.values() {
        let first = &meetdata.get_entry(lifter_indices[0]).name;
        for index in lifter_indices.iter().skip(1) {
            let entry = &meetdata.get_entry(*index);
            if first != &entry.name {
                let msg = format!(
                    "Conflict for {}: '{}' vs '{}'",
                    entry.username, first, entry.name
                );
                println!(" {}", msg.bold().red());
                error_count += 1;
            }
        }
    }

    // The default mode without arguments just performs data checks.
    print_summary(error_count + internal_error_count, warning_count);
    if error_count > 0 || internal_error_count > 0 {
        process::exit(1);
    }

    // Perform cross-Entry data interpolation.
    if is_compiling || is_debugging {
        // Perform country interpolation.
        if let Some(u) = args.debug_country_username {
            compiler::interpolate_country_debug_for(&mut meetdata, &liftermap, &u);
            process::exit(0); // TODO: Complain if someone passes --compile.
        }
        compiler::interpolate_country(&mut meetdata, &liftermap);

        // Perform age interpolation.
        if let Some(u) = args.debug_age_username {
            compiler::interpolate_age_debug_for(&mut meetdata, &liftermap, &u);
            process::exit(0); // TODO: Complain if someone passes --compile.
        }
        compiler::interpolate_age(&mut meetdata, &liftermap);
    }

    // Perform final compilation if requested.
    if is_compiling {
        let buildpath = project_root.join("build");
        if !buildpath.exists() {
            fs::create_dir(&buildpath)?;
        }

        if args.compile {
            compiler::make_csv(&meetdata, &lifterdata, &buildpath)?;
        }
        if args.compile_onefile {
            compiler::make_onefile_csv(&meetdata, &buildpath)?;
        }
    }

    Ok(())
}

use clap::{crate_name, crate_version, App, SubCommand};

mod plan;
mod subcommand;

fn main() {
    let build = SubCommand::with_name("build")
        .about("build and create your mesa")
        .help("build and create your mesa");

    let view = SubCommand::with_name("view")
        .about("view (test) your mesa")
        .help("view (test) your mesa");

    let erode = SubCommand::with_name("erode")
        .about("cleanup and remove your mesa")
        .help("cleanup and remove your mesa");

    let form = SubCommand::with_name("form")
        .about("form (finalize and release) your mesa")
        .help("form (finalize and release) your mesa");

    let mesa = App::new(crate_name!())
        .version(crate_version!())
        .about("| mesa - an isolated place")
        .subcommands(vec![build, view, erode, form])
        .get_matches();

    match mesa.subcommand_name() {
        Some("build") => subcommand::build::mesa_build(),
        Some("view") => subcommand::view::mesa_view(),
        Some("erode") => subcommand::erode::mesa_erode(),
        Some("form") => subcommand::form::mesa_form(),
        _ => println!("{}", mesa.usage()),
    }
}

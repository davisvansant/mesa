use clap::{crate_name, crate_version, App, SubCommand};

mod plan;
mod subcommand;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let survey = SubCommand::with_name("survey")
        .about("survey (prepare and evaluate) to form your mesa")
        .help("survey (prepare and evaluate) to form your mesa");

    let mesa = App::new(crate_name!())
        .version(crate_version!())
        .about("| mesa - an isolated place")
        .subcommands(vec![build, view, erode, form, survey])
        .get_matches();

    match mesa.subcommand_name() {
        Some("build") => subcommand::build::mesa_build().await?,
        Some("view") => subcommand::view::mesa_view().await?,
        Some("erode") => subcommand::erode::mesa_erode().await?,
        Some("form") => subcommand::form::mesa_form().await?,
        Some("survey") => subcommand::survey::mesa_survey().await?,
        _ => println!("{}", mesa.usage()),
    }
    Ok(())
}

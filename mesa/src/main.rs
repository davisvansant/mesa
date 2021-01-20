use clap::{crate_name, crate_version, App, Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = SubCommand::with_name("build")
        .about("build and create your mesa")
        .help("build and create your mesa")
        .arg(
            Arg::with_name("ignore-tests")
                .short("i")
                .long("ignore-tests")
                .help("ignore and skip tests"),
        );

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

    match mesa.subcommand() {
        ("build", Some(build_args)) => {
            if build_args.is_present("ignore-tests") {
                mesa::subcommand::build::build(true).await?;
            } else {
                mesa::subcommand::build::build(false).await?;
            }
        }
        ("view", Some(_view_args)) => mesa::subcommand::view::view().await?,
        ("erode", Some(_erode_args)) => mesa::subcommand::erode::erode().await?,
        ("form", Some(_form_args)) => mesa::subcommand::form::form().await?,
        ("survey", Some(_survey_args)) => mesa::subcommand::survey::survey().await?,
        _ => println!("{}", mesa.usage()),
    }

    Ok(())
}

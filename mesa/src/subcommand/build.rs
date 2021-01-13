pub async fn mesa_build(ignore_tests: bool) -> Result<(), Box<dyn std::error::Error>> {
    let plan = mesa::plan::MesaPlan::excavate().await;

    if ignore_tests {
        match plan {
            Ok(plan_details) => {
                mesa::strata::docker_local::DockerLocal::build(
                    // plan_details.name,
                    // plan_details.version,
                    // plan_details.language.version,
                    // plan_details.formation.layer,
                    plan_details,
                    true,
                )
                .await?
            }
            Err(error) => println!("mesa build | unable to read plan : {}", error),
        }
    } else {
        match plan {
            Ok(plan_details) => {
                mesa::strata::docker_local::DockerLocal::build(
                    // plan_details.name,
                    // plan_details.version,
                    // plan_details.language.version,
                    // plan_details.formation.layer,
                    plan_details,
                    false,
                )
                .await?
            }
            Err(error) => println!("mesa build | unable to read plan : {}", error),
        }
    }

    Ok(())
}

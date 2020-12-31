pub async fn mesa_build(ignore_tests: bool) -> Result<(), Box<dyn std::error::Error>> {
    let plan = crate::plan::MesaPlan::excavate().await;
    println!("mesa build | {:?}", ignore_tests);
    match plan {
        Ok(plan_details) => {
            mesa_strata::docker_local::DockerLocal::build(
                plan_details.name,
                plan_details.version,
                plan_details.language.version,
                plan_details.formation.layer,
            )
            .await?
        }
        Err(error) => println!("mesa build | unable to read plan : {}", error),
    }

    Ok(())
}

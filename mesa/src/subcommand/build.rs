pub async fn mesa_build() {
    let build = mesa_strata::docker_local::DockerLocal::build();
    match build {
        Ok(output) => println!("{}", String::from_utf8(output.stdout).unwrap()),
        Err(error) => println!("{}", error),
    };
}

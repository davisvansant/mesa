use bollard::Docker;
use std::io::Error;
use std::process::{Command, Output};

pub struct DockerLocal {}

impl DockerLocal {
    pub fn build(config: String) -> Result<Output, Error> {
        let cmd = Command::new("/usr/local/bin/docker")
            .arg("version")
            .output()?;
        println!("{:?}", config);
        Ok(cmd)
    }

    pub async fn survey() {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let info = docker.version().await;
        match info {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("{:?}", error),
        };
    }
}

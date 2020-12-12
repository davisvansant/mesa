use bollard::container::RemoveContainerOptions;
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

    pub async fn erode(container: String) {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let options = Some(RemoveContainerOptions {
            v: true,
            force: true,
            link: false,
        });
        let erode = docker.remove_container(&container, options).await;
        match erode {
            Ok(_) => println!("Container {:#?} removed", &container),
            Err(error) => println!("{:#}", error),
        }
    }
}

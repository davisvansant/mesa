use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::Docker;
// use std::io::Error;
// use std::process::{Command, Output};

pub struct DockerLocal {}

impl DockerLocal {
    // pub fn build(config: String) -> Result<Output, Error> {
    //     let cmd = Command::new("/usr/local/bin/docker")
    //         .arg("version")
    //         .output()?;
    //     println!("{:?}", config);
    //     Ok(cmd)
    // }
    pub async fn build(config: String) {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let container_options = Some(CreateContainerOptions {
            name: String::from("mesa_rust_1.48.0"),
        });
        let container_config = Config {
            image: Some("rust:1.48.0"),
            ..Default::default()
        };

        let create_container = docker
            .create_container(container_options, container_config)
            .await;
        match create_container {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("{:#?}", error),
        };

        let start_container = docker
            .start_container("mesa_rust_1.48.0", None::<StartContainerOptions<String>>)
            .await;
        match start_container {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("{:?}", error),
        };
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

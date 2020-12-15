use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::image::{CreateImageOptions, RemoveImageOptions};
use bollard::models::{HostConfig, PortBinding};
use bollard::Docker;
use futures::TryStreamExt;
use std::collections::HashMap;

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
        let create_image_options = Some(CreateImageOptions {
            from_image: "rust:1.47.0",
            ..Default::default()
        });
        let create_image = docker
            .create_image(create_image_options, None, None)
            .map_err(|error| error)
            .map_ok(|ok| ok)
            .try_collect::<Vec<_>>()
            .await;
        match create_image {
            Ok(result) => println!("{}", result.last().unwrap().status.as_ref().unwrap()),
            Err(error) => println!("{:#?}", error),
        };
        let container_options = Some(CreateContainerOptions {
            name: String::from("mesa_rust_1.48.0"),
        });
        let container_config = Config {
            image: Some("rust:1.47.0"),
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
        let remove_image_options = Some(RemoveImageOptions {
            force: true,
            ..Default::default()
        });
        let remove_image = docker
            .remove_image("rust:1.47.0", remove_image_options, None)
            .await;
        match remove_image {
            Ok(result) => println!("Removed Image {:#?}", result),
            Err(error) => println!("{}", error),
        };
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

    pub async fn view(config: String) {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let mut container_ports = HashMap::new();
        let host_ports = HashMap::new();
        container_ports.insert(String::from("8080/tcp"), host_ports);
        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            String::from("8080/tcp"),
            Some(vec![PortBinding {
                host_ip: Some(String::from("0.0.0.0")),
                host_port: Some(String::from("9000")),
            }]),
        );
        let container_host_config = HostConfig {
            port_bindings: Some(port_bindings),
            ..Default::default()
        };
        let container_name = "mesa_built_container";
        let create_container_options = Some(CreateContainerOptions {
            name: container_name,
        });
        let create_container_config = Config {
            image: Some(config),
            exposed_ports: Some(container_ports),
            cmd: Some(vec![String::from("sleep 10000")]),
            host_config: Some(container_host_config),
            ..Default::default()
        };

        let create_container = docker
            .create_container(create_container_options, create_container_config)
            .await;
        match create_container {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("error from create_container {:#?}", error),
        };

        let start_container = docker
            .start_container(container_name, None::<StartContainerOptions<String>>)
            .await;
        match start_container {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("{:?}", error),
        };
    }
}

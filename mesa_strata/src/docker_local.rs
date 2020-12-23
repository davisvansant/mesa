use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::image::{BuildImageOptions, RemoveImageOptions};
use bollard::models::{HostConfig, PortBinding};
use bollard::Docker;
use futures::TryStreamExt;
use handlebars::Handlebars;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct DockerLocal {}

impl DockerLocal {
    async fn connect() -> Result<Docker, bollard::errors::Error> {
        Docker::connect_with_local_defaults()
    }
    pub async fn build(
        config: String,
        version: String,
        // builder_name: String,
        builder_version: String,
        formation: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // let docker = Docker::connect_with_local_defaults().unwrap();
        // let docker = Docker::connect_with_local_defaults()?;
        let docker = Self::connect().await?;

        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("mesa");
        if temp_dir.exists() {
            println!("dir already exists, not creating");
        } else {
            let create_mesa_dir = std::fs::create_dir(&temp_dir);
            match create_mesa_dir {
                Ok(result) => println!("directory created {:?}", result),
                Err(error) => println!("error creating directory {:?}", error),
            };
        };

        let mut handlebars = Handlebars::new();

        // handlebars
        //     .register_template_file(
        //         "Dockerfile",
        //         "./mesa_strata/src/docker_local/Dockerfile.hbs",
        //     )
        //     .unwrap();
        handlebars.register_template_file(
            "Dockerfile",
            "./mesa_strata/src/docker_local/Dockerfile.hbs",
        )?;

        // let handlebars_data = json! ({
        //     "builder": "rust:1.47.0",
        //     "formation": "amazon/aws-lambda-provided:al2",
        // });

        let mut builder = String::with_capacity(20);
        builder.push_str("rust");
        builder.push(':');
        builder.push_str(&builder_version);

        let handlebars_data = json! ({
            "builder": builder,
            "formation": formation,
        });

        let dockerfile = String::from("Dockerfile.mesa");
        let dockerfile_path = &temp_dir.join(&dockerfile);

        // let mut create_dockerfile = File::create(&dockerfile_path).unwrap();
        let mut create_dockerfile = File::create(&dockerfile_path)?;
        // handlebars
        //     .render_to_write("Dockerfile", &handlebars_data, &mut create_dockerfile)
        //     .unwrap();
        handlebars.render_to_write("Dockerfile", &handlebars_data, &mut create_dockerfile)?;

        // let mut open_dockerfile = File::open(&dockerfile_path).unwrap();
        let mut open_dockerfile = File::open(&dockerfile_path)?;

        let tar_gz = &temp_dir.join("Dockerfile.tar.gz");
        // let create_tar_gz = File::create(&tar_gz).unwrap();
        let create_tar_gz = File::create(&tar_gz)?;
        let mut tar = tar::Builder::new(create_tar_gz);
        // tar.append_file(&dockerfile, &mut open_dockerfile).unwrap();
        tar.append_file(&dockerfile, &mut open_dockerfile)?;
        // tar.finish().unwrap();
        tar.finish()?;

        // let mut file = File::open(&tar_gz).unwrap();
        let mut file = File::open(&tar_gz)?;
        let mut contents = Vec::new();
        // file.read_to_end(&mut contents).unwrap();
        file.read_to_end(&mut contents)?;

        let mut tag = config;
        tag.push(':');
        tag.push_str(&version);

        let build_options = BuildImageOptions {
            dockerfile: "Dockerfile.mesa",
            t: &tag,
            rm: true,
            forcerm: true,
            q: true,
            ..Default::default()
        };
        // let build_image = docker
        //     .build_image(build_options, None, Some(contents.into()))
        //     .map_err(|error| println!("{}", error))
        //     .map_ok(|ok| println!("{:?}", ok))
        //     .try_collect::<Vec<_>>()
        //     .await;
        let build_image = docker
            .build_image(build_options, None, Some(contents.into()))
            .map_err(|error| println!("{}", error))
            .map_ok(|ok| println!("{:?}", ok))
            .try_collect::<Vec<_>>()
            .await;
        match build_image {
            Ok(result) => println!("{:?}", result),
            Err(error) => println!("{:?}", error),
        };
        // println!("{:?}", build_image);
        // std::fs::remove_file(&dockerfile_path).unwrap();
        // std::fs::remove_file(&tar_gz).unwrap();
        std::fs::remove_file(&dockerfile_path)?;
        std::fs::remove_file(&tar_gz)?;
        Ok(())
    }

    pub async fn survey() -> Result<(), Box<dyn std::error::Error>> {
        // let docker = Docker::connect_with_local_defaults().unwrap();
        // let docker = Docker::connect_with_local_defaults()?;
        let docker = DockerLocal::connect().await?;
        // let info = docker.version().await?;
        let info = docker.version().await?;
        // match info {
        //     Ok(result) => println!("{:#?}", result),
        //     Err(error) => println!("{:?}", error),
        // };
        println!("{:#?}", info);
        Ok(())
    }

    pub async fn erode(
        container: String,
        version: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let docker = Self::connect().await?;
        let mut tag = container.clone();
        tag.push(':');
        tag.push_str(&version);
        let remove_image_options = Some(RemoveImageOptions {
            force: true,
            ..Default::default()
        });
        let remove_image = docker.remove_image(&tag, remove_image_options, None).await;
        match remove_image {
            Ok(result) => println!("Removed Image {:#?}", result),
            Err(error) => println!("{}", error),
        };
        // println!("{:#?}", remove_image);
        let options = Some(RemoveContainerOptions {
            v: true,
            force: true,
            link: false,
        });
        let erode = docker.remove_container(&container, options).await?;
        // match erode {
        //     Ok(_) => println!("Container {:#?} removed", &container),
        //     Err(error) => println!("{:#}", error),
        // }
        println!("{:#?}", erode);
        Ok(())
    }

    pub async fn view(config: String, version: String) -> Result<(), Box<dyn std::error::Error>> {
        let docker = Self::connect().await?;
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
        let container_name = config.clone();
        let mut tag = config.clone();
        tag.push(':');
        tag.push_str(&version);
        let create_container_options = Some(CreateContainerOptions {
            name: &container_name,
        });
        let create_container_config = Config {
            image: Some(tag),
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
            .start_container(&container_name, None::<StartContainerOptions<String>>)
            .await;
        match start_container {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("{:?}", error),
        };
        Ok(())
    }
}

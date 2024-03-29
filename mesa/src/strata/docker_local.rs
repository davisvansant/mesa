use crate::plan::MesaPlan;
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
    StopContainerOptions,
};
use bollard::image::{BuildImageOptions, PruneImagesOptions, RemoveImageOptions};
use bollard::models::{HostConfig, PortBinding};
use bollard::Docker;
use futures::TryStreamExt;
use handlebars::Handlebars;
use handlebars::Template;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct DockerLocal {}

impl DockerLocal {
    async fn connect() -> Result<Docker, bollard::errors::Error> {
        Docker::connect_with_local_defaults()
    }

    async fn manage_temporary_directory(
        temp_dir: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if temp_dir.exists() {
            Self::cleanup_temporary_directory(temp_dir).await?;
            std::fs::create_dir(temp_dir)?;
        } else {
            std::fs::create_dir(temp_dir)?;
        }

        Ok(())
    }

    async fn cleanup_temporary_directory(
        temp_dir: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::fs::read_dir(temp_dir)?;
        for file in dir {
            let file = file?;
            let path = file.path();
            std::fs::remove_file(&path)?;
            println!("mesa build | removed {:?}", &path);
        }
        std::fs::remove_dir(temp_dir)?;
        println!("mesa build | removed {:?}", &temp_dir);
        Ok(())
    }

    async fn create_and_build_tar(
        tar_gz: &Path,
        path: String,
        file: &mut File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let create_tar_gz = File::create(tar_gz)?;
        let mut tar = tar::Builder::new(&create_tar_gz);
        let current_dir = std::env::current_dir()?;
        tar.append_file(path, file)?;
        tar.append_dir_all(".", current_dir)?;
        tar.finish()?;
        println!("mesa build | tar has been created");
        Ok(())
    }

    async fn read_tar_contents(tar_gz: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = File::open(tar_gz)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        println!("mesa build | tar is ready");
        Ok(contents)
    }

    async fn create_and_build_dockerfile(
        dockerfile_path: &Path,
        builder_version: &str,
        formation: &str,
        ignore_tests: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut handlebars = Handlebars::new();
        let handlebars_dockerfile = r#"
FROM {{builder}} AS builder

COPY Cargo.toml .
COPY ./src ./src
{{#if ignore_tests}}
RUN {{ cmd_one }}
{{else}}
RUN {{ cmd_one }} \
  && {{ cmd_two }} \
  && {{ cmd_three }} \
  && {{ cmd_four }} \
  && {{ cmd_five }}

RUN {{ test_one }}
{{/if}}
RUN cargo build --release

FROM {{formation}}
COPY --from=builder /target/release/bootstrap /var/runtime/bootstrap
CMD ["custom_runtime"]
"#;

        let source = Template::compile(handlebars_dockerfile)?;
        handlebars.register_template("Dockerfile", source);

        let mut builder = String::with_capacity(20);
        builder.push_str("rust");
        builder.push(':');
        builder.push_str(builder_version);

        let handlebars_data = match ignore_tests {
            false => json! ({
                "ignore_tests": false,
                "builder": builder,
                "cmd_one": "rustc --version",
                "cmd_two": "rustup component add rustfmt",
                "cmd_three": "rustup component add clippy",
                "cmd_four": "rustfmt --version",
                "cmd_five": "cargo clippy --version",
                "test_one": "cargo fmt -- --check",
                "formation": formation,
            }),

            true => json! ({
                "ignore_tests": true,
                "builder": builder,
                "cmd_one": "rustc --version",
                "formation": formation,
            }),
        };
        let mut dockerfile = File::create(dockerfile_path)?;

        handlebars.render_to_write("Dockerfile", &handlebars_data, &mut dockerfile)?;
        println!("mesa build | dockerfile has been created");
        Ok(())
    }

    pub async fn build(
        mesa_plan: MesaPlan,
        ignore_tests: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let docker = Self::connect().await?;
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("mesa");

        Self::manage_temporary_directory(&temp_dir).await?;

        let dockerfile = String::from("Dockerfile.mesa");
        let dockerfile_path = &temp_dir.join(&dockerfile);

        Self::create_and_build_dockerfile(
            dockerfile_path,
            &mesa_plan.language.version,
            &mesa_plan.formation.layer,
            ignore_tests,
        )
        .await?;

        let mut open_dockerfile = File::open(dockerfile_path)?;
        let tar_gz = &temp_dir.join("Dockerfile.tar.gz");

        Self::create_and_build_tar(tar_gz, dockerfile, &mut open_dockerfile).await?;

        let mut tag = mesa_plan.name;
        tag.push(':');
        tag.push_str(&mesa_plan.version);

        let build_image_options = BuildImageOptions {
            dockerfile: "Dockerfile.mesa",
            t: &tag,
            rm: true,
            forcerm: true,
            q: false,
            ..Default::default()
        };
        let contents = Self::read_tar_contents(tar_gz).await?;
        let build_image = docker
            .build_image(build_image_options, None, Some(contents.into()))
            .map_err(|error| println!("{}", error))
            .map_ok(|ok| {
                match ok.id {
                    None => (),
                    Some(id) => {
                        println!("mesa build | {}", id);
                    }
                }
                match ok.stream {
                    None => (),
                    Some(stream) => {
                        println!("{}", stream.trim());
                    }
                }
                match ok.error {
                    None => (),
                    Some(error) => println!("mesa build | {}", error),
                }
                match ok.status {
                    None => (),
                    Some(status) => println!("mesa build | {}", status),
                }
                match ok.progress {
                    None => (),
                    Some(progress) => println!("mesa build | {}", progress),
                }
                match ok.aux {
                    None => (),
                    Some(aux) => println!("mesa build | {}", aux.id.unwrap()),
                }
            })
            .try_collect::<Vec<_>>()
            .await;

        let mut filters = HashMap::new();
        filters.insert("dangling", vec!["true"]);

        let prune_images_options = Some(PruneImagesOptions { filters });
        let prune_images = docker.prune_images(prune_images_options).await;

        match prune_images {
            Ok(result) => {
                let item = serde_json::to_string_pretty(&result.images_deleted)?;
                println!("mesa build | prune intermediate {}", item);
            }
            Err(error) => println!("mesa build | {}", error),
        }

        match build_image {
            Ok(_) => println!("mesa build | Build has completed"),
            Err(_) => println!("mesa build | Build was unsuccessful"),
        };

        Ok(())
    }

    pub async fn survey() -> Result<(), Box<dyn std::error::Error>> {
        let docker = Self::connect().await?;
        let info = docker.version().await?;

        println!("mesa survey | system information");
        let information = serde_json::to_string_pretty(&info)?;
        println!("{}", information);
        Ok(())
    }

    pub async fn erode(mesa_plan: MesaPlan) -> Result<(), Box<dyn std::error::Error>> {
        let docker = Self::connect().await?;
        let mut tag = mesa_plan.name.clone();
        tag.push(':');
        tag.push_str(&mesa_plan.version);

        let stop_container_options = Some(StopContainerOptions { t: 2 });
        let stop_container = docker
            .stop_container(&mesa_plan.name, stop_container_options)
            .await;

        match stop_container {
            Ok(_) => println!("mesa erode | stopping {:?}", &mesa_plan.name),
            Err(error) => println!("mesa erode | {:?}", error),
        }

        let remove_container_options = Some(RemoveContainerOptions {
            v: true,
            force: true,
            link: false,
        });
        let remove_container = docker
            .remove_container(&mesa_plan.name, remove_container_options)
            .await;

        match remove_container {
            Ok(_) => println!("mesa erode | container {:#?} removed", &mesa_plan.name),
            Err(error) => println!("mesa erode | {}", error),
        }

        let remove_image_options = Some(RemoveImageOptions {
            force: true,
            noprune: false,
        });
        let remove_image = docker.remove_image(&tag, remove_image_options, None).await;

        match remove_image {
            Ok(result) => {
                let result_details = serde_json::to_string_pretty(&result)?;
                println!("mesa erode | {}", result_details);
            }
            Err(error) => println!("mesa erode | {}", &error),
        };

        Ok(())
    }

    pub async fn view(mesa_plan: MesaPlan) -> Result<(), Box<dyn std::error::Error>> {
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

        let container_name = mesa_plan.name.clone();
        let mut tag = mesa_plan.name.clone();
        tag.push(':');
        tag.push_str(&mesa_plan.version);

        let create_container_options = Some(CreateContainerOptions {
            name: &container_name,
            platform: None,
        });

        let create_container_config = Config {
            image: Some(tag),
            exposed_ports: Some(container_ports),
            host_config: Some(container_host_config),
            ..Default::default()
        };

        let create_container = docker
            .create_container(create_container_options, create_container_config)
            .await;

        match create_container {
            Ok(result) => {
                let result_details = serde_json::to_string_pretty(&result)?;
                println!("mesa view | {}", result_details);

                let start_container = docker
                    .start_container(&container_name, None::<StartContainerOptions<String>>)
                    .await;
                match start_container {
                    Ok(_) => {
                        println!("mesa view | started {}", &container_name);
                    }
                    Err(error) => println!("mesa view | {}", error),
                };
            }
            Err(error) => println!("mesa view | {}", error),
        };

        Ok(())
    }
}

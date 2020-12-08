use std::io::Error;
use std::process::{Command, Output};

pub struct DockerLocal {}

impl DockerLocal {
    pub fn build() -> Result<Output, Error> {
        let cmd = Command::new("/usr/local/bin/docker")
            .arg("version")
            .output()?;
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let cmd = DockerLocal::build();
        match cmd {
            Ok(output) => assert_eq!(output.status.success(), true),
            Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
        }
    }
}

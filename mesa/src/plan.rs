use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read;
use std::fs::write;

#[derive(Deserialize, Serialize)]
pub struct MesaPlan {
    pub name: String,
    pub version: String,
    pub language: Language,
    pub formation: Formation,
}

impl MesaPlan {
    pub async fn init() {
        let plan: MesaPlan = MesaPlan {
            name: String::from("default_mesa_plan_name"),
            version: String::from("0.1.0"),
            language: Language {
                name: SupportedLanguage::Rust,
                version: String::from("1.48.0"),
            },
            formation: Formation {
                shape: FormationShape::Lambda,
                layer: String::from("amazon/aws-lambda-provided:al2"),
            },
        };
        let toml = toml::to_string(&plan).unwrap();
        write("test.toml", toml).unwrap();
        println!("toml created");
    }

    pub async fn excavate() -> Result<MesaPlan, Box<dyn Error>> {
        let file = read("test.toml")?;
        let plan: MesaPlan = toml::from_slice(&file)?;
        Ok(plan)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Language {
    pub name: SupportedLanguage,
    pub version: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SupportedLanguage {
    Rust,
}

#[derive(Deserialize, Serialize)]
pub struct Formation {
    pub shape: FormationShape,
    pub layer: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum FormationShape {
    Lambda,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn plan() {
    //     let test_mesa_plan: MesaPlan = toml::from_str(
    //         r#"
    //         name = "test_mesa_plan"
    //         version = "0.1.0"
    //
    //         [language]
    //         name = "Rust"
    //         version = "1.48.0"
    //
    //         [formation]
    //         shape = "Lambda"
    //         layer = "amazon/aws-lambda-provided:al2"
    //         "#,
    //     )
    //     .unwrap();
    //     assert_eq!(test_mesa_plan.name, String::from("test_mesa_plan"));
    //     assert_eq!(test_mesa_plan.version, String::from("0.1.0"));
    //     assert_eq!(test_mesa_plan.language.name, SupportedLanguage::Rust);
    //     assert_eq!(test_mesa_plan.language.version, String::from("1.48.0"));
    // }

    #[tokio::test]
    async fn init() {
        MesaPlan::init().await;
        let open_test_mesa_plan = std::fs::File::open("test.toml").unwrap();
        let metadata = open_test_mesa_plan.metadata().unwrap();
        assert_eq!(metadata.is_file(), true);
    }

    #[tokio::test]
    async fn excavate() {
        let test_mesa_plan = MesaPlan::excavate().await.unwrap();
        assert_eq!(test_mesa_plan.name, String::from("default_mesa_plan_name"));
        assert_eq!(test_mesa_plan.version, String::from("0.1.0"));
        assert_eq!(test_mesa_plan.language.name, SupportedLanguage::Rust);
        assert_eq!(test_mesa_plan.language.version, String::from("1.48.0"));
        assert_eq!(test_mesa_plan.formation.shape, FormationShape::Lambda);
        assert_eq!(
            test_mesa_plan.formation.layer,
            String::from("amazon/aws-lambda-provided:al2")
        );
    }
}

#![cfg(feature = "dotenv")]

use config::Config;
use std::env;

#[test]
fn basic_dotenv() {
    let s = Config::builder()
        .add_source(config::File::from_str(
            r#"
                FOO=bar
                BAZ=qux
            "#,
            config::FileFormat::Dotenv,
        ))
        .build()
        .unwrap();

    assert_eq!(s.get::<String>("FOO").unwrap(), "bar");
    assert_eq!(s.get::<String>("BAZ").unwrap(), "qux");
}

#[test]
fn optional_variables() {
    let s = Config::builder()
        .add_source(config::File::from_str(
            r#"
                FOO=bar
                BAZ=${FOO}
                BAR=${UNDEFINED:-}
            "#,
            config::FileFormat::Dotenv,
        ))
        .build()
        .unwrap();

    assert_eq!(s.get::<String>("BAR").unwrap(), "");
}

#[test]
fn multiple_files() {
    let s = Config::builder()
        .add_source(config::File::from_str(
            r#"
                FOO=bar
            "#,
            config::FileFormat::Dotenv,
        ))
        .add_source(config::File::from_str(
            r#"
                BAZ=qux
            "#,
            config::FileFormat::Dotenv,
        ))
        .build()
        .unwrap();

    assert_eq!(s.get::<String>("FOO").unwrap(), "bar");
    assert_eq!(s.get::<String>("BAZ").unwrap(), "qux");
}

#[test]
fn environment_overrides() {
    env::set_var("FOOBAR", "env_value");

    let s = Config::builder()
        .add_source(config::File::from_str(
            r#"
                FOOBAR=file_value
            "#,
            config::FileFormat::Dotenv,
        ))
        .add_source(config::Environment::with_prefix("env").separator("_"))
        .build()
        .unwrap();

    assert_eq!(s.get::<String>("FOOBAR").unwrap(), "env_value");
}

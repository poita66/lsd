//! This module defines the [SizeFlag]. To set it up from [ArgMatches], a [Yaml] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

/// The flag showing which file size units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SizeFlag {
    /// The variant to show file size with SI unit prefix and a B for bytes.
    Default,
    /// The variant to show file size with only the SI unit prefix.
    Short,
    /// The variant to show file size in bytes.
    Bytes,
}

impl Configurable<Self> for SizeFlag {
    /// Get a potential `SizeFlag` variant from [ArgMatches].
    ///
    /// If any of the "default", "short" or "bytes" arguments is passed, the corresponding
    /// `SizeFlag` variant is returned in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.occurrences_of("size") > 0 {
            match matches.value_of("size") {
                Some("default") => Some(Self::Default),
                Some("short") => Some(Self::Short),
                Some("bytes") => Some(Self::Bytes),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `SizeFlag` variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [String](Yaml::String) value, pointed to by "size" and it
    /// is either "default", "short" or "bytes", this returns the corresponding `SizeFlag` variant
    /// in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["size"] {
                Yaml::BadValue => None,
                Yaml::String(value) => match value.as_ref() {
                    "default" => Some(Self::Default),
                    "short" => Some(Self::Short),
                    "bytes" => Some(Self::Bytes),
                    _ => {
                        config.print_invalid_value_warning("size", &value);
                        None
                    }
                },
                _ => {
                    config.print_wrong_type_warning("size", "string");
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `SizeFlag` is [SizeFlag::Default].
impl Default for SizeFlag {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(test)]
mod test {
    use super::SizeFlag;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_default() {
        let argv = vec!["lsd", "--size", "default"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SizeFlag::Default),
            SizeFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_short() {
        let args = vec!["lsd", "--size", "short"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(Some(SizeFlag::Short), SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_bytes() {
        let args = vec!["lsd", "--size", "bytes"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, SizeFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, SizeFlag::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_default() {
        let yaml_string = "size: default";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SizeFlag::Default),
            SizeFlag::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_short() {
        let yaml_string = "size: short";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SizeFlag::Short),
            SizeFlag::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_bytes() {
        let yaml_string = "size: bytes";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(SizeFlag::Bytes),
            SizeFlag::from_config(&Config::with_yaml(yaml))
        );
    }
}

use crate::config;
use crate::core;

/// The config representation differs from the core representation.
/// This mapper converts the config representation to the corresponding core datastructures.
pub fn convert_config_context_to_core(context: &config::Context) -> core::Context {
    core::Context {
        name: String::from(&context.name),
        features: context
            .features
            .iter()
            .map(|feature| core::Feature {
                name: feature.name.to_string(),
                description: feature.description.to_string(),
                treatments: feature
                    .treatments
                    .iter()
                    .map(|treatment| core::Treatment {
                        probability: treatment.probability,
                        segments: treatment
                            .segments
                            .iter()
                            .map(|segment| segment.to_string())
                            .collect(),
                        value: treatment.value.to_string(),
                    })
                    .collect(),
            })
            .collect(),
        segments: vec![],
    }
}

pub fn convert_core_toggles_to_config(config: &core::Toggles) -> config::Toggles {
    config::Toggles { toggles: vec![] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Segment;

    #[test]
    fn test_mappings_base() {
        let context = convert_config_context_to_core(&config::Context {
            name: String::from("some-name"),
            features: vec![],
            segments: vec![],
        });
        assert_eq!(context.name, "some-name");
        assert_eq!(context.features.len(), 0);
        assert_eq!(context.segments.len(), 0);
    }

    #[test]
    fn test_mappings_features() {
        let context = convert_config_context_to_core(&config::Context {
            name: String::from("some-name"),
            features: vec![config::Feature {
                name: String::from("some-feature"),
                description: String::from("some-description"),
                treatments: vec![config::Treatment {
                    segments: vec![String::from("segment1")],
                    probability: 10,
                    value: String::from("some-value"),
                }],
            }],
            segments: vec![],
        });

        assert_eq!(context.name, "some-name");
        assert_eq!(context.features.len(), 1);
        assert_eq!(context.segments.len(), 0);

        assert_eq!(context.features[0].name, "some-feature");
        assert_eq!(context.features[0].description, "some-description");
        assert_eq!(context.features[0].treatments.len(), 1);
        assert_eq!(context.features[0].treatments[0].segments.len(), 1);
        assert_eq!(context.features[0].treatments[0].segments[0], "segment1");
        assert_eq!(context.features[0].treatments[0].probability, 10);
        assert_eq!(context.features[0].treatments[0].value, "some-value");
    }

    #[test]
    fn test_mappings_segments() {
        let context = convert_config_context_to_core(&config::Context {
            name: String::from("some-name"),
            features: vec![],
            segments: vec![Segment {
                name: String::from("beta_testers"),
                user_identifiers: vec![String::from("someone")],
            }],
        });

        assert_eq!(context.segments.len(), 1);
        assert_eq!(context.segments[0].name, "beta_testers");
        assert_eq!(context.segments[0].user_identifiers.len(), 1);
        assert_eq!(context.segments[0].user_identifiers[0], "someone");
    }
}

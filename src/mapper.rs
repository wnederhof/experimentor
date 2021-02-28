use crate::user_model;
use crate::core;

pub fn convert_user_model_context_to_core(context: &user_model::ContextConfig) -> core::Context {
    core::Context {
        name: String::from(&context.name),
        features: map_user_model_features_to_core(context),
        segments: map_core_segments_to_user_model(context),
    }
}

fn map_user_model_features_to_core(context: &user_model::ContextConfig) -> Vec<core::Feature> {
    context
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
        .collect()
}

fn map_core_segments_to_user_model(context: &user_model::ContextConfig) -> Vec<core::Segment> {
    context
        .segments
        .iter()
        .map(|segment| core::Segment {
            name: segment.name.to_string(),
            user_identifiers: segment
                .user_identifiers
                .iter()
                .map(|user_identifier| user_identifier.to_string())
                .collect(),
        })
        .collect()
}

pub fn convert_core_toggles_to_user_model(toggles: &core::Toggles) -> user_model::TogglesResponse {
    user_model::TogglesResponse { toggles: toggles.toggles.iter().map(|toggle| user_model::ToggleResponse {
        name: String::from(&toggle.name),
        value: String::from(&toggle.value),
    }).collect() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_model::SegmentConfig;

    #[test]
    fn test_convert_user_model_context_to_core_mappings_base_case() {
        let context = convert_user_model_context_to_core(&user_model::ContextConfig {
            name: String::from("some-name"),
            features: vec![],
            segments: vec![],
        });
        assert_eq!(context.name, "some-name");
        assert_eq!(context.features.len(), 0);
        assert_eq!(context.segments.len(), 0);
    }

    #[test]
    fn test_convert_user_model_context_to_core_maps_features() {
        let context = convert_user_model_context_to_core(&user_model::ContextConfig {
            name: String::from("some-name"),
            features: vec![user_model::FeatureConfig {
                name: String::from("some-feature"),
                description: String::from("some-description"),
                treatments: vec![user_model::TreatmentConfig {
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
    fn test_convert_user_model_context_to_core_maps_segments() {
        let context = convert_user_model_context_to_core(&user_model::ContextConfig {
            name: String::from("some-name"),
            features: vec![],
            segments: vec![SegmentConfig {
                name: String::from("beta_testers"),
                user_identifiers: vec![String::from("someone")],
            }],
        });

        assert_eq!(context.segments.len(), 1);
        assert_eq!(context.segments[0].name, "beta_testers");
        assert_eq!(context.segments[0].user_identifiers.len(), 1);
        assert_eq!(context.segments[0].user_identifiers[0], "someone");
    }

    #[test]
    fn test_convert_core_toggles_to_user_model_base_case() {
        let context = convert_core_toggles_to_user_model(&core::Toggles {
            toggles: vec!()
        });
        assert_eq!(context.toggles.len(), 0);
    }

    #[test]
    fn test_convert_core_toggles_to_user_model_maps_toggles() {
        let context = convert_core_toggles_to_user_model(&core::Toggles {
            toggles: vec![core::Toggle {
                name: String::from("feature"),
                value: String::from("value")
            }]
        });
        assert_eq!(context.toggles.len(), 1);
        assert_eq!(context.toggles[0].name, "feature");
        assert_eq!(context.toggles[0].value, "value");
    }

}

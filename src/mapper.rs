use crate::core;
use crate::user_model;
use std::collections::HashMap;

pub fn map_toggles_to_toggles_response(toggles: &core::Toggles) -> user_model::TogglesResponse {
    let mapped_toggles = toggles
        .toggles
        .iter()
        .map(|toggle| (toggle.name.to_string(), toggle.value.to_string()))
        .collect();

    user_model::TogglesResponse {
        toggles: mapped_toggles,
    }
}

pub fn map_contexts_config_to_contexts(contexts: &user_model::ContextsConfig) -> core::Contexts {
    core::Contexts {
        contexts: contexts
            .contexts
            .iter()
            .map(|elem| {
                let elem_name = elem.name.to_string();
                (elem_name, map_context_config_to_context(&elem))
            })
            .into_iter()
            .collect(),
    }
}

pub fn map_context_config_to_context(context: &user_model::ContextConfig) -> core::Context {
    core::Context {
        features: map_feature_configs_to_features(context),
        segments: map_segment_configs_to_segments(context),
    }
}

fn map_feature_configs_to_features(context: &user_model::ContextConfig) -> Vec<core::Feature> {
    context
        .features
        .iter()
        .map(|feature| core::Feature {
            name: feature.name.to_string(),
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

fn map_segment_configs_to_segments(
    context: &user_model::ContextConfig,
) -> HashMap<String, Vec<String>> {
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();
    context.segments.iter().for_each(|segment| {
        hash_map.insert(
            segment.name.to_string(),
            segment
                .user_identifiers
                .iter()
                .map(|user_identifier| user_identifier.to_string())
                .collect(),
        );
    });
    hash_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Toggle;
    use crate::user_model::SegmentConfig;

    #[test]
    fn test_map_contexts_config_to_contexts_maps_contexts() {
        let contexts = map_contexts_config_to_contexts(&user_model::ContextsConfig {
            contexts: vec![user_model::ContextConfig {
                name: String::from("context_1"),
                features: vec![],
                segments: vec![],
            }],
        });
        assert_eq!(contexts.contexts.contains_key("context_1"), true);
        assert_eq!(contexts.contexts.contains_key("context_2"), false);
    }

    #[test]
    fn test_map_context_config_to_context_base_case() {
        let context = map_context_config_to_context(&user_model::ContextConfig {
            name: String::from("some-name"),
            features: vec![],
            segments: vec![],
        });
        assert_eq!(context.features.len(), 0);
        assert_eq!(context.segments.len(), 0);
    }

    #[test]
    fn test_map_context_config_to_context_maps_features() {
        let context = map_context_config_to_context(&user_model::ContextConfig {
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

        assert_eq!(context.features.len(), 1);
        assert_eq!(context.segments.len(), 0);

        assert_eq!(context.features[0].name, "some-feature");
        assert_eq!(context.features[0].treatments.len(), 1);
        assert_eq!(context.features[0].treatments[0].segments.len(), 1);
        assert_eq!(context.features[0].treatments[0].segments[0], "segment1");
        assert_eq!(context.features[0].treatments[0].probability, 10);
        assert_eq!(context.features[0].treatments[0].value, "some-value");
    }

    #[test]
    fn test_map_context_config_to_context_maps_segments() {
        let context = map_context_config_to_context(&user_model::ContextConfig {
            name: String::from("some-name"),
            features: vec![],
            segments: vec![SegmentConfig {
                name: String::from("beta_testers"),
                user_identifiers: vec![String::from("someone")],
            }],
        });

        assert_eq!(context.segments.len(), 1);
        assert_eq!(context.segments.contains_key("beta_testers"), true);
        assert_eq!(context.segments.get("beta_testers").unwrap().len(), 1);
        assert_eq!(context.segments.get("beta_testers").unwrap()[0], "someone");
    }

    #[test]
    fn test_map_feature_configs_to_features_base_case() {
        let context = map_toggles_to_toggles_response(&core::Toggles { toggles: vec![] });
        assert_eq!(context.toggles.len(), 0);
    }

    #[test]
    fn test_map_feature_configs_to_features_maps_toggles() {
        let context = map_toggles_to_toggles_response(&core::Toggles {
            toggles: vec![Toggle {
                name: String::from("feature"),
                value: String::from("value"),
            }],
        });
        assert_eq!(context.toggles.len(), 1);
        assert_eq!(context.toggles["feature"], "value");
    }
}

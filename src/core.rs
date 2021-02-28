use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub features: Vec<Feature>,
    pub segments: Vec<Segment>,
}

#[derive(Debug)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub treatments: Vec<Treatment>,
}

#[derive(Debug)]
pub struct Treatment {
    pub probability: i8,
    pub segments: Vec<String>,
    pub value: String,
}

#[derive(Debug)]
pub struct Segment {
    pub name: String,
    pub user_identifiers: Vec<String>,
}

#[derive(Debug)]
pub struct Toggles {
    pub toggles: Vec<Toggle>,
}

#[derive(Debug)]
pub struct Toggle {
    pub name: String,
    pub value: String,
}

pub fn find_feature_toggles(user_identifier: &str, context: &Context) -> Toggles {
    Toggles {
        toggles: context
            .features
            .iter()
            .map(|feature| Toggle {
                name: feature.name.to_owned(),
                value: active_treatment(user_identifier, feature, &context).to_string(),
            })
            .collect(),
    }
}

fn active_treatment<'a>(user_identifier: &str, feature: &'a Feature, context: &Context) -> &'a str {
    let segments = &context.segments;
    let mut prob = deterministic_random_probability(&context.name, user_identifier, &feature.name);
    let treatments_slice = feature.treatments.as_slice();
    let overriding_treatment =
        find_overriding_treatment(user_identifier, segments, treatments_slice);

    match overriding_treatment {
        Some(treatment) => treatment,
        None => {
            for treatment in treatments_slice {
                prob -= treatment.probability;
                if prob < 0 {
                    return &treatment.value;
                }
            }
            panic!("Could not find treatment for feature {}.", feature.name)
        }
    }
}

/// Checks if one of a feature's treatment should always be chosen, because
/// the treatment is in an "overriding segment".
/// For example, if user "wouter" is in the segment "beta_testers", then
/// the first treatment that uses segment "beta_testers" will always be selected.
fn find_overriding_treatment<'a>(
    user_identifier: &str,
    segments: &Vec<Segment>,
    treatments_slice: &'a [Treatment],
) -> Option<&'a String> {
    for treatment in treatments_slice {
        for treatment_segment in &treatment.segments {
            for segment in segments {
                if &segment.name == treatment_segment {
                    if segment
                        .user_identifiers
                        .contains(&user_identifier.to_string())
                    {
                        return Some(&treatment.value);
                    }
                }
            }
        }
    }
    None
}

/// Deterministically determine an evenly distributed number between 1 and 100 based
/// on the context name, name and feature name.
fn deterministic_random_probability(context_name: &str, name: &str, feature_name: &str) -> i8 {
    let mut hasher = DefaultHasher::new();
    let to_hash = format!("{}:{}:{}", context_name, name, feature_name);
    to_hash.hash(&mut hasher);
    let mut rng = Pcg32::seed_from_u64(hasher.finish());
    rng.gen_range(1, 101)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_features_empty() {
        let actual: Toggles = find_feature_toggles(
            "pulp_fiction",
            &Context {
                name: String::from("pulp_fiction"),
                features: vec![],
                segments: vec![],
            },
        );
        assert_eq!(actual.toggles.len(), 0);
    }

    #[test]
    fn test_find_features_finds_treatment_settings_simple() {
        let actual: Toggles = find_feature_toggles(
            "pulp_fiction",
            &Context {
                name: String::from("pulp_fiction"),
                features: vec![Feature {
                    name: String::from("briefcase"),
                    description: String::from(""),
                    treatments: vec![Treatment {
                        probability: 100,
                        value: String::from("gold"),
                        segments: vec![],
                    }],
                }],
                segments: vec![],
            },
        );
        assert_eq!(actual.toggles.len(), 1);
        assert_eq!(actual.toggles[0].name, "briefcase");
        assert_eq!(actual.toggles[0].value, "gold");
    }

    #[test]
    fn test_find_features_finds_treatment_settings_multiple() {
        let actual: Toggles = find_feature_toggles(
            "wouter",
            &Context {
                name: String::from("pulp_fiction"),
                features: vec![Feature {
                    name: String::from("briefcase"),
                    description: String::from(""),
                    treatments: vec![
                        Treatment {
                            probability: 100,
                            value: String::from("gold"),
                            segments: vec![],
                        },
                        Treatment {
                            probability: 0,
                            value: String::from("silver"),
                            segments: vec![],
                        },
                    ],
                }],
                segments: vec![],
            },
        );
        assert_eq!(actual.toggles.len(), 1);
        assert_eq!(actual.toggles[0].name, "briefcase");
        assert_eq!(actual.toggles[0].value, "gold");
    }

    #[test]
    fn test_segments_override_it_all() {
        let actual: Toggles = find_feature_toggles(
            "wouter",
            &Context {
                name: String::from("pulp_fiction"),
                features: vec![Feature {
                    name: String::from("briefcase"),
                    description: String::from(""),
                    treatments: vec![
                        Treatment {
                            probability: 100,
                            value: String::from("gold"),
                            segments: vec![],
                        },
                        Treatment {
                            probability: 0,
                            value: String::from("silver"),
                            segments: vec![String::from("beta-tester")],
                        },
                    ],
                }],
                segments: vec![Segment {
                    name: String::from("beta-tester"),
                    user_identifiers: vec![String::from("wouter")],
                }],
            },
        );
        assert_eq!(actual.toggles.len(), 1);
        assert_eq!(actual.toggles[0].name, "briefcase");
        assert_eq!(actual.toggles[0].value, "silver");
    }
}

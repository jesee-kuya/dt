use std::collections::HashMap;

#[derive(Debug)]
pub enum TreeNode {
    Branch {
        attribute: String,
        children: HashMap<String, TreeNode>,
    },
    Leaf {
        class: String,
    },
}

pub struct DecisionTree {
    pub root: TreeNode,
}

use crate::reader::DataRecord;
use std::f64;

impl DecisionTree {
    pub fn build(records: &[DataRecord]) -> Self {
        let root = DecisionTree::build_tree(records);
        DecisionTree { root }
    }

    fn build_tree(records: &[DataRecord]) -> TreeNode {
        // Base cases
        if records.is_empty() {
            panic!("Cannot build tree with empty records");
        }

        // Check if all records have the same class
        if let Some(class) = DecisionTree::all_same_class(records) {
            return TreeNode::Leaf { class };
        }

        // Select best attribute to split on
        let best_attr = DecisionTree::select_best_attribute(records);

        // If no good attribute found, return leaf with majority class
        if best_attr.is_none() {
            let majority_class = DecisionTree::majority_class(records);
            return TreeNode::Leaf { class: majority_class };
        }

        let best_attr = best_attr.unwrap();
        let mut children = HashMap::new();

        // Split records by attribute values
        let grouped = DecisionTree::group_by_attribute(records, &best_attr);

        for (value, subset) in grouped {
            if subset.is_empty() {
                let majority_class = DecisionTree::majority_class(records);
                children.insert(value, TreeNode::Leaf { class: majority_class });
            } else {
                children.insert(value, DecisionTree::build_tree(&subset));
            }
        }

        TreeNode::Branch {
            attribute: best_attr,
            children,
        }
    }

    fn all_same_class(records: &[DataRecord]) -> Option<String> {
        let first_class = records[0].nursing_competency.as_ref()?;
        
        if records.iter().all(|r| 
            r.nursing_competency.as_ref().map_or(false, |c| c == first_class)
        ) {
            Some(first_class.clone())
        } else {
            None
        }
    }

    fn select_best_attribute(records: &[DataRecord]) -> Option<String> {
        let mut best_gain_ratio = 0.0;
        let mut best_attr = None;
        
        // For simplicity, we'll just use these attributes
        let attributes = vec![
            "county", 
            "health_level", 
            "years_experience", 
            "clinical_panel",
            "clinician"
        ];

        let class_entropy = DecisionTree::entropy(records);

        for &attr in &attributes {
            let gain_ratio = DecisionTree::gain_ratio(records, attr, class_entropy);
            if gain_ratio > best_gain_ratio {
                best_gain_ratio = gain_ratio;
                best_attr = Some(attr.to_string());
            }
        }

        best_attr
    }

    fn entropy(records: &[DataRecord]) -> f64 {
        let mut class_counts = HashMap::new();
        let total = records.len() as f64;

        for record in records {
            if let Some(class) = &record.nursing_competency {
                *class_counts.entry(class.clone()).or_insert(0) += 1;
            }
        }

        if class_counts.is_empty() {
            return 0.0;
        }

        class_counts.values().fold(0.0, |acc, &count| {
            let prob = count as f64 / total;
            acc - prob * prob.log2()
        })
    }

    fn gain_ratio(records: &[DataRecord], attribute: &str, class_entropy: f64) -> f64 {
        let mut value_counts = HashMap::new();
        let mut value_class_counts = HashMap::new();
        let total = records.len() as f64;

        for record in records {
            let value = match attribute {
                "county" => record.county.as_ref(),
                "health_level" => record.health_level.as_ref(),
                "years_experience" => record.years_experience.as_ref(),
                "clinical_panel" => record.clinical_panel.as_ref(),
                "clinician" => record.clinician.as_ref(),
                _ => None,
            };

            if let (Some(val), Some(class)) = (value, &record.nursing_competency) {
                *value_counts.entry(val.clone()).or_insert(0) += 1;
                
                let class_map = value_class_counts.entry(val.clone()).or_insert(HashMap::new());
                *class_map.entry(class.clone()).or_insert(0) += 1;
            }
        }

        if value_counts.is_empty() {
            return 0.0;
        }

        let mut info_attr = 0.0;
        let mut split_info = 0.0;

        for (val, count) in value_counts {
            let prob_val = count as f64 / total;
            split_info -= prob_val * prob_val.log2();

            if let Some(class_counts) = value_class_counts.get(&val) {
                let mut entropy = 0.0;
                let sum = class_counts.values().sum::<i32>() as f64;
                
                for &class_count in class_counts.values() {
                    let prob = class_count as f64 / sum;
                    if prob > 0.0 {
                        entropy -= prob * prob.log2();
                    }
                }
                
                info_attr += prob_val * entropy;
            }
        }

        let gain = class_entropy - info_attr;
        if split_info == 0.0 {
            0.0
        } else {
            gain / split_info
        }
    }

    fn group_by_attribute(records: &[DataRecord], attribute: &str) -> HashMap<String, Vec<DataRecord>> {
        let mut groups = HashMap::new();

        for record in records {
            let value = match attribute {
                "county" => record.county.as_ref(),
                "health_level" => record.health_level.as_ref(),
                "years_experience" => record.years_experience.as_ref(),
                "clinical_panel" => record.clinical_panel.as_ref(),
                "clinician" => record.clinician.as_ref(),
                _ => None,
            };

            if let Some(val) = value {
                groups.entry(val.clone())
                    .or_insert_with(Vec::new)
                    .push(DataRecord {
                        master_index: record.master_index.clone(),
                        county: record.county.clone(),
                        health_level: record.health_level.clone(),
                        years_experience: record.years_experience.clone(),
                        prompt: record.prompt.clone(),
                        nursing_competency: record.nursing_competency.clone(),
                        clinical_panel: record.clinical_panel.clone(),
                        clinician: record.clinician.clone(),
                        gpt4_0: record.gpt4_0.clone(),
                        llama: record.llama.clone(),
                        gemini: record.gemini.clone(),
                        ddx_snomed: record.ddx_snomed.clone(),
                    });
            }
        }

        groups
    }

    fn majority_class(records: &[DataRecord]) -> String {
        let mut class_counts = HashMap::new();

        for record in records {
            if let Some(class) = &record.nursing_competency {
                *class_counts.entry(class.clone()).or_insert(0) += 1;
            }
        }

        class_counts.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(class, _)| class)
            .unwrap_or_else(|| "Unknown".to_string())
    }
}
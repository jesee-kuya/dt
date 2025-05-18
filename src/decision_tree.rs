use std::collections::HashMap;
use crate::reader::DataRecord;

#[derive(Debug)]
pub enum TreeNode {
    Branch {
        attribute: String,
        children: HashMap<String, TreeNode>,
    },
    Leaf {
        value: String,
    },
}

#[derive(Clone, Copy)]
pub enum TargetField {
    Clinician,
    Gpt4_0,
    Llama,
    Gemini,
    DdxSnomed,
}

pub struct DecisionTree {
    root: TreeNode,
    target: TargetField,
}

#[derive(Debug, Default)]
pub struct Prediction {
    pub clinician: Option<String>,
    pub gpt4_0: Option<String>,
    pub llama: Option<String>,
    pub gemini: Option<String>,
    pub ddx_snomed: Option<String>,
}

impl DecisionTree {
    pub fn build(records: &[DataRecord], target: TargetField) -> Self {
        let root = Self::build_tree(records, target);
        DecisionTree { root, target }
    }

    fn build_tree(records: &[DataRecord], target: TargetField) -> TreeNode {
        if records.is_empty() {
            return TreeNode::Leaf {
                value: "Unknown".to_string(),
            };
        }

        if let Some(value) = Self::all_same_value(records, target) {
            return TreeNode::Leaf { value };
        }

        if let Some(best_attr) = Self::select_best_attribute(records, target) {
            let mut children = HashMap::new();
            let grouped = Self::group_by_attribute(records, &best_attr);

            for (value, subset) in grouped {
                children.insert(
                    value,
                    if subset.is_empty() {
                        TreeNode::Leaf {
                            value: Self::majority_value(records, target),
                        }
                    } else {
                        Self::build_tree(&subset, target)
                    },
                );
            }

            TreeNode::Branch {
                attribute: best_attr,
                children,
            }
        } else {
            TreeNode::Leaf {
                value: Self::majority_value(records, target),
            }
        }
    }


    fn all_same_value(records: &[DataRecord], target: TargetField) -> Option<String> {
        let first = records.first()?;
        let first_value = Self::get_target_value(first, target)?;

        if records.iter().all(|r| {
            Self::get_target_value(r, target)
                .map_or(false, |v| v == first_value)
        }) {
            Some(first_value.to_owned())
        } else {
            None
        }
    }

    fn select_best_attribute(records: &[DataRecord], target: TargetField) -> Option<String> {
        let attributes = vec!["county", "health_level", "years_experience", "clinical_panel"];
        let class_entropy = Self::entropy(records, target);
        let mut best_attr = None;
        let mut best_gain_ratio = 0.0;

        for attr in attributes {
            let gain_ratio = Self::gain_ratio(records, attr, target, class_entropy);
            if gain_ratio > best_gain_ratio {
                best_gain_ratio = gain_ratio;
                best_attr = Some(attr.to_string());
            }
        }

        best_attr
    }

    fn entropy(records: &[DataRecord], target: TargetField) -> f64 {
        let mut counts = HashMap::new();
        let total = records.len() as f64;

        for record in records {
            if let Some(value) = Self::get_target_value(record, target) {
                *counts.entry(value).or_insert(0) += 1;
            }
        }

        counts.values().fold(0.0, |acc, &count| {
            let p = count as f64 / total;
            acc - p * p.log2()
        })
    }

    fn gain_ratio(records: &[DataRecord], attr: &str, target: TargetField, class_entropy: f64) -> f64 {
        let mut value_counts = HashMap::new();
        let mut value_class_counts = HashMap::new();
        let total = records.len() as f64;

        for record in records {
            let attr_value = Self::get_attribute_value(record, attr);
            let class_value = Self::get_target_value(record, target);

            if let (Some(attr_val), Some(class_val)) = (attr_value, class_value) {
                *value_counts.entry(attr_val.clone()).or_insert(0) += 1;
                let class_map = value_class_counts.entry(attr_val).or_insert(HashMap::new());
                *class_map.entry(class_val).or_insert(0) += 1;
            }
        }

        let mut info_attr = 0.0;
        let mut split_info = 0.0;

        for (val, count) in &value_counts {
            let prob = *count as f64 / total;
            split_info -= prob * prob.log2();
        
            if let Some(class_counts) = value_class_counts.get(val) {
                let entropy = class_counts.values().fold(0.0, |acc, &c| {
                    let p = c as f64 / *count as f64;
                    acc - p * p.log2()
                });
                info_attr += prob * entropy;
            }
        }
        

        let gain = class_entropy - info_attr;
        if split_info == 0.0 { 0.0 } else { gain / split_info }
    }

    fn group_by_attribute(records: &[DataRecord], attr: &str) -> HashMap<String, Vec<DataRecord>> {
        let mut groups = HashMap::new();
    
        for record in records {
            if let Some(value) = Self::get_attribute_value(record, attr) {
                // Clone the String to convert &String to String
                let key = value.clone();
                let new_record = DataRecord {
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
                };
                groups.entry(key)
                    .or_insert_with(Vec::new)
                    .push(new_record);
            }
        }
    
        groups
    }



    fn majority_value(records: &[DataRecord], target: TargetField) -> String {
        let mut counts = HashMap::new();
        for record in records {
            if let Some(value) = Self::get_target_value(record, target) {
                *counts.entry(value).or_insert(0) += 1;
            }
        }
        counts.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(value, _)| value.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }
    

    fn get_target_value(record: &DataRecord, target: TargetField) -> Option<&String> {
        match target {
            TargetField::Clinician => record.clinician.as_ref(),
            TargetField::Gpt4_0 => record.gpt4_0.as_ref(),
            TargetField::Llama => record.llama.as_ref(),
            TargetField::Gemini => record.gemini.as_ref(),
            TargetField::DdxSnomed => record.ddx_snomed.as_ref(),
        }
    }

    fn get_attribute_value<'a>(record: &'a DataRecord, attr: &str) -> Option<&'a String> {
        match attr {
            "county" => record.county.as_ref(),
            "health_level" => record.health_level.as_ref(),
            "years_experience" => record.years_experience.as_ref(),
            "clinical_panel" => record.clinical_panel.as_ref(),
            _ => None,
        }
    }
    

    pub fn predict(&self, record: &DataRecord) -> Option<String> {
        self.predict_recursive(&self.root, record)
    }

    fn predict_recursive(&self, node: &TreeNode, record: &DataRecord) -> Option<String> {
        match node {
            TreeNode::Branch { attribute, children } => {
                let value = Self::get_attribute_value(record, attribute)?;
                let child = children.get(value)?;
                self.predict_recursive(child, record)
            }
            TreeNode::Leaf { value } => Some(value.clone()),
        }
    }
}

pub struct MultiTargetPredictor {
    clinician_tree: DecisionTree,
    gpt4_tree: DecisionTree,
    llama_tree: DecisionTree,
    gemini_tree: DecisionTree,
    ddx_snomed_tree: DecisionTree,
}

impl MultiTargetPredictor {
    pub fn build(records: &[DataRecord]) -> Self {
        Self {
            clinician_tree: DecisionTree::build(records, TargetField::Clinician),
            gpt4_tree: DecisionTree::build(records, TargetField::Gpt4_0),
            llama_tree: DecisionTree::build(records, TargetField::Llama),
            gemini_tree: DecisionTree::build(records, TargetField::Gemini),
            ddx_snomed_tree: DecisionTree::build(records, TargetField::DdxSnomed),
        }
    }

    pub fn predict(&self, record: &DataRecord) -> Prediction {
        Prediction {
            clinician: self.clinician_tree.predict(record),
            gpt4_0: self.gpt4_tree.predict(record),
            llama: self.llama_tree.predict(record),
            gemini: self.gemini_tree.predict(record),
            ddx_snomed: self.ddx_snomed_tree.predict(record),
        }
    }
}
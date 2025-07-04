use crate::reader::DataRecord;
use std::collections::HashMap;

/// Pruning and stopping criteria.
#[derive(Clone, Copy)]
pub struct TreeParams {
    pub max_depth: usize,
    pub min_samples_leaf: usize,
    pub min_gain_ratio: f64,
}

#[derive(Debug)]
pub enum TreeNode {
    Branch {
        attribute: String,
        children: HashMap<String, TreeNode>,
        majority: String,
    },
    Leaf {
        value: String,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetField {
    Clinician,
    Gpt4_0,
    Llama,
    Gemini,
    DdxSnomed,
}

#[derive(Debug, Default)]
pub struct Prediction {
    pub clinician: Option<String>,
    pub gpt4_0: Option<String>,
    pub llama: Option<String>,
    pub gemini: Option<String>,
    pub ddx_snomed: Option<String>,
}

pub struct DecisionTree {
    root: TreeNode,
    _params: TreeParams,
}

impl DecisionTree {
    /// Build a pruned tree.
    pub fn build(
        records: &[DataRecord],
        target: TargetField,
        params: TreeParams,
    ) -> Self {
        let attrs = Self::all_attributes();
        let root = Self::recurse(records, target, attrs, 0, &params);
        DecisionTree { root, _params: params }
    }

    fn all_attributes() -> &'static [&'static str] {
        &["county", "health_level", "years_experience", "clinical_panel"]
    }

    fn recurse(
        records: &[DataRecord],
        target: TargetField,
        attributes: &[&str],
        depth: usize,
        params: &TreeParams,
    ) -> TreeNode {
        let majority = Self::majority(records, target);
        if records.is_empty() {
            return TreeNode::Leaf { value: "unknown".into() };
        }
        if depth >= params.max_depth || records.len() < params.min_samples_leaf {
            return TreeNode::Leaf { value: majority };
        }
        if let Some(pure_val) = Self::pure(records, target) {
            return TreeNode::Leaf { value: pure_val };
        }
        if attributes.is_empty() {
            return TreeNode::Leaf { value: majority };
        }

        let base_ent = Self::entropy(records, target);
        let mut best_attr: &str = "";
        let mut best_ratio = 0.0;
        for &attr in attributes {
            let ratio = Self::gain_ratio(records, attr, target, base_ent);
            if ratio > best_ratio {
                best_ratio = ratio;
                best_attr = attr;
            }
        }
        
        if best_ratio < params.min_gain_ratio {
            return TreeNode::Leaf { value: majority };
        }

        let mut children = HashMap::new();
        for (val, subset) in Self::partition(records, best_attr) {
            let node = if subset.len() < params.min_samples_leaf {
                TreeNode::Leaf { value: majority.clone() }
            } else {
                let rem: Vec<&str> = attributes
                    .iter()
                    .copied()
                    .filter(|&a| a != best_attr)
                    .collect();
                Self::recurse(&subset, target, &rem, depth + 1, params)
            };
            children.insert(val, node);
        }

        TreeNode::Branch { attribute: best_attr.into(), children, majority }
    }

    fn pure(records: &[DataRecord], target: TargetField) -> Option<String> {
        let mut iter = records.iter().filter_map(|r| Self::get_target(r, target));
        let first = iter.next()?;
        if iter.all(|v| v == first) { Some(first.clone()) } else { None }
    }

    fn entropy(records: &[DataRecord], target: TargetField) -> f64 {
        let total = records.len() as f64;
        let mut counts: HashMap<&String, usize> = HashMap::new();
        for val in records.iter().filter_map(|r| Self::get_target(r, target)) {
            *counts.entry(val).or_insert(0) += 1;
        }
        counts.values().fold(0.0, |e, &c| {
            let p = (c as f64) / total;
            e - p * p.log2()
        })
    }

    fn gain_ratio(
        records: &[DataRecord],
        attr: &str,
        target: TargetField,
        base_ent: f64,
    ) -> f64 {
        let total = records.len() as f64;
        let parts = Self::partition(records, attr);
        let mut split_info = 0.0;
        let mut info_attr = 0.0;
        for subset in parts.values() {
            let p = (subset.len() as f64) / total;
            split_info -= p * p.log2();
            info_attr += p * Self::entropy(subset, target);
        }
        let gain = base_ent - info_attr;
        if split_info == 0.0 { 0.0 } else { gain / split_info }
    }

    fn partition(
        records: &[DataRecord],
        attr: &str,
    ) -> HashMap<String, Vec<DataRecord>> {
        let mut map: HashMap<String, Vec<DataRecord>> = HashMap::new();
        for r in records {
            if let Some(val) = Self::get_attr(r, attr) {
                map.entry(val.clone()).or_default().push(r.clone());
            }
        }
        map
    }

    fn majority(records: &[DataRecord], target: TargetField) -> String {
        let mut counts: HashMap<&String, usize> = HashMap::new();
        for val in records.iter().filter_map(|r| Self::get_target(r, target)) {
            *counts.entry(val).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .max_by_key(|&(_, c)| c)
            .map(|(v, _)| v.clone())
            .unwrap_or_else(|| "unknown".into())
    }

    fn get_target<'a>(r: &'a DataRecord, t: TargetField) -> Option<&'a String> {
        match t {
            TargetField::Clinician => r.clinician.as_ref(),
            TargetField::Gpt4_0    => r.gpt4_0.as_ref(),
            TargetField::Llama      => r.llama.as_ref(),
            TargetField::Gemini     => r.gemini.as_ref(),
            TargetField::DdxSnomed  => r.ddx_snomed.as_ref(),
        }
    }

    fn get_attr<'a>(r: &'a DataRecord, a: &str) -> Option<&'a String> {
        match a {
            "county"           => r.county.as_ref(),
            "health_level"     => r.health_level.as_ref(),
            "years_experience" => r.years_experience.as_ref(),
            "clinical_panel"   => r.clinical_panel.as_ref(),
            _                   => None,
        }
    }

    pub fn predict(&self, rec: &DataRecord) -> Option<String> {
        DecisionTree::traverse(&self.root, rec)
    }

    fn traverse(node: &TreeNode, rec: &DataRecord) -> Option<String> {
        match node {
            TreeNode::Leaf { value } => Some(value.clone()),
            TreeNode::Branch { attribute, children, majority } => {
                let raw = DecisionTree::get_attr(rec, attribute)
                    .map(|s| s.to_lowercase())
                    .unwrap_or_else(|| "missing".into());
                let key = children.keys()
                    .find(|k| k.eq_ignore_ascii_case(&raw))
                    .cloned()
                    .unwrap_or_else(|| majority.clone());
                children.get(&key)
                    .and_then(|n| DecisionTree::traverse(n,	rec))
                    .or_else(|| Some(majority.clone()))
            }
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
    pub fn build_with_params(records: &[DataRecord], params: TreeParams) -> Self {
        Self {
            clinician_tree: DecisionTree::build(records, TargetField::Clinician, params),
            gpt4_tree:      DecisionTree::build(records, TargetField::Gpt4_0, params),
            llama_tree:     DecisionTree::build(records, TargetField::Llama, params),
            gemini_tree:    DecisionTree::build(records, TargetField::Gemini, params),
            ddx_snomed_tree:DecisionTree::build(records, TargetField::DdxSnomed, params),
        }
    }

    pub fn predict(&self, rec: &DataRecord) -> Prediction {
        Prediction {
            clinician: self.clinician_tree.predict(rec),
            gpt4_0:    self.gpt4_tree.predict(rec),
            llama:     self.llama_tree.predict(rec),
            gemini:    self.gemini_tree.predict(rec),
            ddx_snomed:self.ddx_snomed_tree.predict(rec),
        }
    }
}

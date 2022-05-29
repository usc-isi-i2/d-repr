use drepr::alignments::inference::AlignmentInference;
use drepr::lang::{Alignment, Description};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::path::Path;

/// Smoke test the inferences
#[test]
fn smoke_test() {
  let scenarios = TestScenario::load();
  for scenario in scenarios {
    let inference = AlignmentInference::new(&scenario.desc);

    for infer_fn in scenario.assertion.infer_funcs {
      let aligns = inference.infer_func(infer_fn.triple.0, infer_fn.triple.1, infer_fn.triple.2);
      assert!(aligns.is_some());
      assert_eq!(infer_fn.aligns, aligns.unwrap());
    }

    for align in scenario.assertion.alignments {
      let aligns = inference.get_alignments(align.pair.0, align.pair.1);
      assert_eq!(align.aligns, aligns);
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
struct Assertion {
  alignments: Vec<AlignmentAssertion>,
  infer_funcs: Vec<InferFnAssertion>,
}

#[derive(Debug, Clone, Deserialize)]
struct AlignmentAssertion {
  pair: (usize, usize),
  aligns: Vec<Alignment>,
}

#[derive(Debug, Clone, Deserialize)]
struct InferFnAssertion {
  triple: (usize, usize, usize),
  aligns: Vec<Alignment>,
}

struct TestScenario {
  name: String,
  desc: Description,
  assertion: Assertion,
}

impl TestScenario {
  pub fn load() -> Vec<TestScenario> {
    let test_resource_dir =
      Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/alignments/inference/resources");
    let mut scenarios = vec![];

    for e0 in fs::read_dir(&test_resource_dir).unwrap() {
      let dataset_dir = e0.unwrap().path();
      if dataset_dir.is_dir() {
        // get model
        let desc_file = dataset_dir
          .as_path()
          .join("model.json")
          .to_str()
          .unwrap()
          .to_string();
        let desc: Description = serde_json::from_reader(File::open(desc_file).unwrap())
          .expect("Invalid description file");
        let assertion: Assertion = serde_json::from_reader(
          File::open(dataset_dir.as_path().join("assertion.json")).unwrap(),
        )
        .expect("Invalid assertion file");
        scenarios.push(TestScenario {
          desc,
          name: dataset_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
          assertion,
        });
      }
    }

    scenarios.sort_by_key(|s| s.name.clone());
    scenarios
  }
}

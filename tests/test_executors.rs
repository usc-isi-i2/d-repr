use std::fs;
use std::fs::File;
use std::io::Error;
use std::path::Path;

use drepr::executors::{Executor, PhysicalOutput, PhysicalResource};
use drepr::lang::Description;
use drepr::writers::stream_writer::OutputFormat;

#[test]
pub fn test_executors() {
  let datasets = read_datasets().unwrap();
  for dataset in datasets {
    let resources: Vec<PhysicalResource> = dataset
      .resources
      .into_iter()
      .map(|rpath| PhysicalResource::File(rpath))
      .collect();
    let description: Description = serde_json::from_reader(File::open(dataset.model).unwrap())
      .expect("Invalid description file");

    // if dataset.dataset_dir.find("s01").is_none() {
    //   continue;
    // }
    println!("dataset: {}", dataset.dataset_dir);
    for output in &dataset.outputs {
      let exc_output = Path::new(&dataset.dataset_dir)
        .join(format!("tmp_output.{:?}", output.1).to_lowercase())
        .to_str()
        .unwrap()
        .to_string();
      (Executor {
        resources: resources.clone(),
        description: description.clone(),
        output: PhysicalOutput::File {
          fpath: exc_output.clone(),
          format: output.1,
        },
        edges_optional: vec![true; description.semantic_model.edges.len()],
      })
      .exec();
      let pred_output = fs::read_to_string(exc_output).unwrap();
      let true_output = fs::read_to_string(&output.0).unwrap().replace("\r\n", "\n");
      assert_eq!(true_output, pred_output);
    }
  }
}

struct Dataset {
  dataset_dir: String,
  model: String,
  resources: Vec<String>,
  outputs: Vec<(String, OutputFormat)>,
}

fn read_datasets() -> Result<Vec<Dataset>, Error> {
  let test_resource_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
  let mut datasets = vec![];
  for e0 in fs::read_dir(&test_resource_dir)? {
    let dataset_dir = e0?.path();
    if dataset_dir.is_dir() {
      // get resources
      let mut resources = vec![];
      let mut outputs = vec![];
      for e1 in fs::read_dir(&dataset_dir)? {
        let file = e1?.path();
        if file.file_name().unwrap().to_str().unwrap().starts_with("r") {
          resources.push(file.as_os_str().to_str().unwrap().to_string());
          continue;
        }

        if file
          .file_name()
          .unwrap()
          .to_str()
          .unwrap()
          .starts_with("output")
        {
          let format = match file.extension().unwrap().to_str().unwrap() {
            "ttl" => OutputFormat::TTL,
            _ => unreachable!(),
          };
          outputs.push((file.as_os_str().to_str().unwrap().to_string(), format));
        }
      }
      // get model
      let model = dataset_dir
        .as_path()
        .join("model.json")
        .to_str()
        .unwrap()
        .to_string();
      resources.sort();
      datasets.push(Dataset {
        model,
        resources,
        outputs,
        dataset_dir: dataset_dir.to_str().unwrap().to_string(),
      });
    }
  }
  datasets.sort_by_key(|d| d.dataset_dir.clone());
  Ok(datasets)
}

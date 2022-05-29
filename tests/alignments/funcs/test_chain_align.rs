use std::fs;
use std::path::Path;

use hashbrown::HashMap;

use drepr::alignments::funcs::mul_chain_align::MulChainMDupAlign;
use drepr::alignments::funcs::mul_value_align::MulValueAlignFunc;
use drepr::alignments::funcs::sgl_chain_align::SglChainAlign;
use drepr::alignments::funcs::sgl_value_align::SglValueAlignFunc;
use drepr::alignments::{AlignmentFunc, MAlignmentFunc, SAlignmentFunc};
use drepr::lang::Attribute;
use readers::prelude::{Index, JSONRAReader, RAReader};

use crate::helpers::{collect_index_iterator, path};

#[test]
fn test_mul_chain_align() {
  for scenario in TestScenario::fetch_one("s01.json") {
    let mut chained_attrs = scenario
      .alignments
      .iter()
      .map(|a| &scenario.attrs[a.attr_id])
      .collect::<Vec<_>>();
    let funcs = scenario.get_funcs();
    let target_attr = chained_attrs.pop().unwrap();
    let mut chained_func = MulChainMDupAlign::new(&scenario.readers, chained_attrs, funcs);

    let mut iter =
      scenario.readers[scenario.source_attr.resource_id].iter_index(&scenario.source_attr.path);
    let mut target_index = target_attr
      .path
      .get_initial_step(scenario.readers[target_attr.resource_id].as_ref());
    let mut pred_results = vec![];
    loop {
      {
        let tmp = chained_func.iter_alignments(
          iter.value(),
          scenario.readers[scenario.source_attr.resource_id].get_value(iter.value(), 0),
          &mut target_index,
        );
        pred_results.append(&mut collect_index_iterator(tmp));
      }
      if !iter.advance() {
        break;
      }
    }
    assert_eq!(scenario.results, pred_results);
  }
}

#[test]
fn test_sgl_chain_align() {
  for scenario in TestScenario::fetch_one("s02.json") {
    let mut chained_attrs = scenario
      .alignments
      .iter()
      .map(|a| &scenario.attrs[a.attr_id])
      .collect::<Vec<_>>();
    let funcs = scenario.get_funcs();
    let target_attr = chained_attrs.pop().unwrap();
    let mut chained_func = SglChainAlign::new(
      &scenario.readers,
      chained_attrs,
      funcs.into_iter().map(|d| d.into_single()).collect(),
    );

    let mut iter =
      scenario.readers[scenario.source_attr.resource_id].iter_index(&scenario.source_attr.path);
    let mut target_index = target_attr
      .path
      .get_initial_step(scenario.readers[target_attr.resource_id].as_ref());
    let mut pred_results = vec![];
    loop {
      {
        let tmp = chained_func.align(
          iter.value(),
          scenario.readers[scenario.source_attr.resource_id].get_value(iter.value(), 0),
          &mut target_index,
        );
        pred_results.push(tmp.to_vec());
      }
      if !iter.advance() {
        break;
      }
    }
    assert_eq!(scenario.results, pred_results);
  }
}

struct TestScenario {
  readers: Vec<Box<dyn RAReader>>,
  attrs: Vec<Attribute>,
  source_attr: Attribute,
  // copy of one of the attribute in attrs
  alignments: Vec<TestScenarioAlignment>,
  results: Vec<Vec<Index>>,
}

pub struct TestScenarioAlignment {
  reader_id: usize,
  attr_id: usize,
  align_type: String,
}

impl TestScenario {
  pub fn fetch_one(fpath: &str) -> Vec<TestScenario> {
    let ds_file = Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("tests/alignments/funcs/resources")
      .join(fpath);
    let tree: serde_json::Value =
      serde_json::from_str(&fs::read_to_string(ds_file).unwrap()).unwrap();
    // read global resources & attributes
    let resources: HashMap<String, _> = tree["resources"]
      .as_object()
      .unwrap()
      .into_iter()
      .map(|(rid, rdata)| (rid.clone(), JSONRAReader::from_json(rdata.clone())))
      .collect();
    let attrs: HashMap<String, _> = tree["attrs"]
      .as_object()
      .unwrap()
      .into_iter()
      .map(|(k, a)| {
        (
          k.clone(),
          serde_json::from_value::<Attribute>(a.clone()).unwrap(),
        )
      })
      .collect();
    // create scenario
    tree["scenarios"]
      .as_array()
      .unwrap()
      .iter()
      .map(|s| TestScenario::new(&resources, &attrs, s).unwrap())
      .collect::<Vec<_>>()
  }

  pub fn new(
    all_res: &HashMap<String, JSONRAReader>,
    all_attrs: &HashMap<String, Attribute>,
    scenario: &serde_json::Value,
  ) -> Result<TestScenario, serde_json::Error> {
    let mut res: Vec<Box<dyn RAReader>> = vec![];
    let mut attrs = vec![];
    let mut aligns = vec![];

    res.push(Box::new(
      all_res[scenario["source"]["reader"].as_str().unwrap()].clone(),
    ));
    // by default, id and resource_id should be 0, so we don't need to reset it
    attrs.push(all_attrs[scenario["source"]["attr"].as_str().unwrap()].clone());
    for align in scenario["alignments"].as_array().unwrap() {
      res.push(Box::new(all_res[align["reader"].as_str().unwrap()].clone()));
      let mut attr = all_attrs[align["attr"].as_str().unwrap()].clone();
      attr.resource_id = res.len() - 1;
      attr.id = attrs.len();
      aligns.push(TestScenarioAlignment {
        reader_id: attr.resource_id,
        attr_id: attr.id,
        align_type: align["type"].as_str().unwrap().to_string(),
      });
      attrs.push(attr);
    }
    Ok(TestScenario {
      readers: res,
      source_attr: attrs[0].clone(), // always the first attribute
      attrs,
      alignments: aligns,
      results: scenario["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| path(s.as_str().unwrap()))
        .collect::<Vec<_>>(),
    })
  }
  pub fn get_funcs(&self) -> Vec<AlignmentFunc> {
    self
      .alignments
      .iter()
      .map(|a| match a.align_type.as_str() {
        "mul-val" => AlignmentFunc::Multiple(Box::new(MulValueAlignFunc::new(
          &self.readers[a.reader_id],
          &self.attrs[a.attr_id],
        ))),
        "sgl-val" => AlignmentFunc::Single(Box::new(SglValueAlignFunc::new(
          &self.readers[a.reader_id],
          &self.attrs[a.attr_id],
        ))),
        _ => unreachable!(),
      })
      .collect()
  }
}

impl TestScenarioAlignment {}

use drepr::alignments::{SDimAlignFunc, AlignedDimension, MDimAlignFunc, MAlignedDimension};
use drepr::readers::*;
use crate::helpers::{sindex, range};
use drepr::alignments::{SAlignmentFunc, MAlignmentFunc};
use crate::iterators::collect_stream_iterators;

fn create_aligned_dim(source: &str, target: &str) -> AlignedDimension {
  // source: <dim>:<start>:<step>
  // target: <dim>:<start>..<stop>:<step>
  let a: Vec<&str> = source.split(":").collect();
  let b: Vec<&str> = target.split(":").collect();
  let c: Vec<&str> = b[1].split("..").collect();

  AlignedDimension {
    source_dim: a[0].parse::<usize>().unwrap(),
    source_start: a[1].parse::<usize>().unwrap(),
    source_step: a[2].parse::<usize>().unwrap(),
    target_dim: b[0].parse::<usize>().unwrap(),
    target_start: c[0].parse::<usize>().unwrap(),
    target_step: b[2].parse::<usize>().unwrap()
  }
}

#[test]
fn test_sdim_alignment() {
  // a = 0..2:A:9..13
  // b = 1..5:2..4:B
  let align = SDimAlignFunc::new(3, vec![
    create_aligned_dim("0:0:1", "1:2..4:1"),
    create_aligned_dim("2:9:1", "0:1..5:1"),
  ]);

  let source_index = sindex("1:A:12");
  let mut target_index = sindex("0:0:B");
  align.align(&source_index, &Value::Null, &mut target_index);
  assert_eq!(target_index, sindex("4:3:B"));

  // a = A:0..100
  // b = B:0..100
  let align = SDimAlignFunc::new(2, vec![
    create_aligned_dim("1:0:1", "1:0..100:1"),
  ]);

  let source_index = sindex("A:5");
  let mut target_index = sindex("B:0");
  align.align(&source_index, &Value::Null, &mut target_index);
  assert_eq!(target_index, sindex("B:5"));
}

#[test]
fn test_mdim_alignment() {
  let data = r#"[
{
  "a": [2, 3, 5, 10, 11, 23],
  "b": [
    { "b1": [3, 4, 11, 12] },
    { "b1": [15, 2, 12] }
  ]
},
{
  "a": [5, 3, 2],
  "b": [{
    "b1": [0]
   }]
}]"#;

  let reader = JSONRAReader::from_str(data);
  // a = "0..:a:0"
  // b = "0..:b:0..:b1:0.."

  let saligned_dims = vec![
    create_aligned_dim("0:0:1", "0:0..2:1")
  ];
  let maligned_dims = vec![
    MAlignedDimension { target_dim: 2, target_range: range("0..:1") },
    MAlignedDimension { target_dim: 4, target_range: range("0..:1") },
  ];
  let align = MDimAlignFunc::new(&reader, 3, 5, saligned_dims, maligned_dims);
  let source_index = sindex("0:a:0");
  let mut target_index = sindex("1:b:1:b1:1");

  assert_eq!(
    collect_stream_iterators(align.iter_alignments(&source_index, &Value::Null, &mut target_index)),
    vec![
      sindex("0:b:0:b1:0"),
      sindex("0:b:0:b1:1"),
      sindex("0:b:0:b1:2"),
      sindex("0:b:0:b1:3"),
      sindex("0:b:1:b1:0"),
      sindex("0:b:1:b1:1"),
      sindex("0:b:1:b1:2")
    ]);

  let source_index = sindex("1:a:0");
  assert_eq!(
    collect_stream_iterators(align.iter_alignments(&source_index, &Value::Null, &mut target_index)),
    vec![
      sindex("1:b:0:b1:0"),
  ]);
}
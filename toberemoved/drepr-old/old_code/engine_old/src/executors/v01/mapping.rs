use super::planning::plans::{ClassPlan, DataPropPlan, ObjectPropPlan, PrimaryKeyPlan};
use crate::alignments::AlignmentFunc;
use crate::readers::{Index, RAReader, Value};
use crate::writers::StreamStateWriter;
use itertools::any;

/// algorithm to map original data into relational structured
///
/// the plans are supposed to be in the reversed topological ordering
///
/// For example, suppose we have this semantic model graph
///   Address <- Company <-> Person
/// then the plans are: [Person, Address, Company]
/// the backward link from Person -> Company is generated at the same time but is put in buffered
pub fn mapping<R, W>(reader: &R, writer: &mut W, plans: &[ClassPlan])
  where
    R: RAReader,
    W: StreamStateWriter,
{
  writer.begin();

  for plan in plans {
    writer.begin_class(plan.class_id);

    // loop through primary keys of the class of this plan
    // and generate instances
    let mut pk_iter = reader.iter_data(&plan.pk.location);
    let mut pk_target_index: Vec<Index> = if plan.pk.target.is_some() {
      plan.pk.target.as_ref().unwrap().0.get_first_index()
    } else {
      vec![]
    };

    let mut dprop_indices: Vec<Vec<Index>> = plan
      .data_props
      .iter()
      .map(|dplan| dplan.target_variable.get_first_index())
      .collect();
    let mut oprop_indices: Vec<Vec<Index>> = plan
      .object_props
      .iter()
      .map(|oplan| oplan.target_variable.get_first_index())
      .collect();
    let mut buffered_oprop_indices: Vec<Vec<Index>> = plan
      .buffered_object_props
      .iter()
      .map(|oplan| oplan.target_variable.get_first_index())
      .collect();

    if buffered_oprop_indices.len() == 0 {
      // no need to buffer object links, however, if some data props are not optional, we need to
      // locally buffer the object
      if any(plan.data_props.iter(), |dplan| !dplan.is_optional) || any(plan.object_props.iter(), |oplan| !oplan.is_optional) {
        unimplemented!()
//        let mut object: Vec<(usize, &Value)> = vec![];
//        loop {
//          let pk_val = reader.get_value(pk_iter.value(), 0);
//          let pid = plan.pk.pseudo_id.get_id_string(pk_iter.value());
//          // compute object identifier
//          let oid = get_object_identifier(reader, &plan.pk, pk_iter.value(),
//                                          &mut pk_target_index, pk_val, &pid,
//          );
//
//          object.clear();
//          for (di, dplan) in plan.data_props.iter().enumerate() {
//            write_data_prop(
//              reader, writer, &oid, pk_iter.value(),
//              &pk_val, dplan, &mut dprop_indices[di],
//            );
//          }
//
//          // now align object properties
//          for (oplan, oindex) in plan.object_props.iter().zip(&mut oprop_indices) {
//            write_object_prop(writer, &oid, pk_iter.value(), &pk_val, oplan, oindex);
//          }
//          writer.end_subject();
//          if !pk_iter.advance() {
//            break;
//          }
//        }
      } else {
        loop {
          // the value of this primary key doesn't mean this is a correct id
          let pk_val = reader.get_value(pk_iter.value(), 0);
          let pid = plan.pk.pseudo_id.get_id_string(pk_iter.value());
          // compute object identifier
          let oid = get_object_identifier(reader, &plan.pk, pk_iter.value(),
                                          &mut pk_target_index, pk_val, &pid,
          );

          let has_new_subject = writer.begin_record(&pid, &oid);
          if has_new_subject {
            for (di, dplan) in plan.data_props.iter().enumerate() {
              write_data_prop(
                reader, writer, &oid, pk_iter.value(),
                &pk_val, dplan, &mut dprop_indices[di],
              );
            }

            for lplan in plan.literal_props.iter() {
              writer.write_data_property(&oid, lplan.predicate_id, &lplan.value);
            }
          }
          
          // now align object properties
          for (oplan, oindex) in plan.object_props.iter().zip(&mut oprop_indices) {
            write_object_prop(writer, &oid, pk_iter.value(), &pk_val, oplan, oindex, has_new_subject);
          }

          if has_new_subject {
            writer.end_record();
          }

          if !pk_iter.advance() {
            break;
          }
        }
      }
    } else {
      if any(plan.buffered_object_props.iter(), |oplan| !oplan.is_optional) {
        panic!("We haven't supported to have non-optional loop yet!");
      } else {
        // we only buffer object links instead of the whole object
        loop {
          // the value of this primary key doesn't mean this is a correct id
          let pk_val = reader.get_value(pk_iter.value(), 0);
          let pid = plan.pk.pseudo_id.get_id_string(pk_iter.value());
          // compute object identifier
          let oid = get_object_identifier(reader, &plan.pk, pk_iter.value(),
                                          &mut pk_target_index, pk_val, &pid,
          );

          let has_new_subject = writer.begin_partial_buffering_record(&pid, &oid);
          if has_new_subject {
            for (di, dplan) in plan.data_props.iter().enumerate() {
              write_data_prop(
                reader, writer, &oid, pk_iter.value(),
                &pk_val, dplan, &mut dprop_indices[di],
              );
            }

            for lplan in plan.literal_props.iter() {
              writer.write_data_property(&oid, lplan.predicate_id, &lplan.value);
            }
          }

          for (oplan, oindex) in plan.object_props.iter().zip(&mut oprop_indices) {
            write_object_prop(writer, &oid, pk_iter.value(), &pk_val, oplan, oindex, has_new_subject);
          }
          for (oplan, oindex) in plan.buffered_object_props.iter().zip(&mut buffered_oprop_indices) {
            buffer_object_prop(writer, pk_iter.value(), &pk_val, oplan, oindex);
          }
          if has_new_subject {
            writer.end_partial_buffering_record();
          }
          if !pk_iter.advance() {
            break;
          }
        }
      }
    }

    writer.end_class();
  }

  writer.end();
}

#[inline]
fn get_object_identifier<R>(
  reader: &R,
  pk_plan: &PrimaryKeyPlan,
  pk_index: &[Index],
  pk_target_index: &mut Vec<Index>,
  pk_val: &Value,
  pid: &String,
) -> String
  where
    R: RAReader,
{
  if pk_plan.is_blank_node {
    // the primary key is generated from its index, and it is blank node, so it is prefixed with _
    format!("_:{}", pid)
  } else {
    match &pk_plan.target {
      Some((_target_var, pk_alignment)) => {
        // the pk is obtained by invoke pk alignment
        reader
          .get_value(
            pk_alignment
              .as_single()
              .align(pk_index, pk_val, pk_target_index),
            0,
          )
          .as_str()
          .to_string()
      }
      None => {
        // the pk is its self
        pk_val.as_str().to_string()
      }
    }
  }
}

#[inline]
fn write_data_prop<R, W>(
  reader: &R,
  writer: &mut W,
  oid: &String,
  pk_index: &[Index],
  pk_val: &Value,
  dplan: &DataPropPlan,
  dindex: &mut Vec<Index>,
) where
  R: RAReader,
  W: StreamStateWriter,
{
  match &dplan.alignment {
    AlignmentFunc::Single(a) => {
      let dval = reader.get_value(a.align(pk_index, pk_val, dindex), 0);
      writer.write_data_property(oid, dplan.predicate_id, &dval);
    }
    AlignmentFunc::Multiple(a) => {
      let mut diter = a.iter_alignments(pk_index, pk_val, dindex);
      loop {
        let dval = reader.get_value(diter.value(), 0);
        writer.write_data_property(oid, dplan.predicate_id, &dval);
        if !diter.advance() {
          break;
        }
      }
    }
  }
}

#[inline]
fn write_object_prop<W>(
  writer: &mut W,
  oid: &String,
  pk_index: &[Index],
  pk_val: &Value,
  oplan: &ObjectPropPlan,
  oindex: &mut Vec<Index>,
  new_object: bool
) where
  W: StreamStateWriter,
{
  match &oplan.alignment {
    AlignmentFunc::Single(a) => {
      let opid = oplan
        .target_pseudo_id
        .get_id_string(a.align(pk_index, pk_val, oindex));
      writer.write_object_property(oplan.target_class, oid, oplan.predicate_id, &opid, new_object);
    }
    AlignmentFunc::Multiple(a) => {
      let mut oiter = a.iter_alignments(pk_index, pk_val, oindex);
      loop {
        let opid = oplan.target_pseudo_id.get_id_string(oiter.value());
        writer.write_object_property(oplan.target_class, oid, oplan.predicate_id, &opid, new_object);
        if !oiter.advance() {
          break;
        }
      }
    }
  }
}

#[allow(dead_code)]
#[inline]
fn buffer_object_prop<W>(
  writer: &mut W,
  pk_index: &[Index],
  pk_val: &Value,
  oplan: &ObjectPropPlan,
  oindex: &mut Vec<Index>,
) where
  W: StreamStateWriter,
{
  match &oplan.alignment {
    AlignmentFunc::Single(a) => {
      let opid = oplan
        .target_pseudo_id
        .get_id_string(a.align(pk_index, pk_val, oindex));
      writer.buffer_object_property(oplan.target_class, oplan.predicate_id, opid);
    }
    AlignmentFunc::Multiple(a) => {
      let mut oiter = a.iter_alignments(pk_index, pk_val, oindex);
      loop {
        let opid = oplan.target_pseudo_id.get_id_string(oiter.value());
        writer.buffer_object_property(oplan.target_class, oplan.predicate_id, opid);
        if !oiter.advance() {
          break;
        }
      }
    }
  }
}

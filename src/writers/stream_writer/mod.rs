use serde::{Deserialize, Serialize};

pub use self::graph_json::GraphJSONWriter;
pub use self::graph_py::GraphPyWriter;
pub use self::stream_class_writer::StreamClassWriter;
pub use self::stream_writer::StreamWriter;
pub use self::turtle::TTLStreamWriter;

pub mod graph_json;
pub mod graph_py;
pub mod stream_class_writer;
pub mod stream_writer;
pub mod turtle;

/// Encode Scheme
/// S<b|u|n>: blank (b), uri (u), either blank or uri (n)
/// O<b|u|n>: blank (b), uri (u), either blank or uri (n)
/// U<t|f>: unique subjects (t), may have duplication (f)
/// T<t|f>: whether we need to keep track of inserted subject or not (true (t), false (f))
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum WriteMode {
  Tt_Ut_Sb_Ob,
  Tt_Ut_Sb_Ou,
  Tt_Ut_Sb_On,
  Tt_Ut_Su_Ob,
  Tt_Ut_Su_Ou,
  Tt_Ut_Su_On,
  Tt_Ut_Sn_Ob,
  Tt_Ut_Sn_Ou,
  Tt_Ut_Sn_On,
  Tt_Uf_Su_Ob,
  Tt_Uf_Su_Ou,
  Tt_Uf_Su_On,
  Tt_Uf_Sn_Ob,
  Tt_Uf_Sn_Ou,
  Tt_Uf_Sn_On,
  Tf_Ut_Sb_Ob,
  Tf_Ut_Sb_Ou,
  Tf_Ut_Sb_On,
  Tf_Ut_Su_Ob,
  Tf_Ut_Su_Ou,
  Tf_Ut_Su_On,
  Tf_Ut_Sn_Ob,
  Tf_Ut_Sn_Ou,
  Tf_Ut_Sn_On,
  Tf_Uf_Su_Ob,
  Tf_Uf_Su_Ou,
  Tf_Uf_Su_On,
  Tf_Uf_Sn_Ob,
  Tf_Uf_Sn_Ou,
  Tf_Uf_Sn_On,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum OutputFormat {
  #[serde(rename = "ttl")]
  TTL,
  #[serde(rename = "graph_json")]
  GraphJSON,
  #[serde(rename = "graph_py")]
  GraphPy,
}

impl WriteMode {
  pub fn create(
    keep_track_subj: bool,
    is_subj_unique: bool,
    subj_blank_or_uri: Option<bool>,
    obj_blank_or_uri: Option<bool>,
  ) -> WriteMode {
    if keep_track_subj {
      match subj_blank_or_uri {
        Some(true) => {
          // subj blank
          match obj_blank_or_uri {
            Some(true) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tt_Ut_Sb_Ob
              } else {
                unreachable!()
              }
            }
            Some(false) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tt_Ut_Sb_Ou
              } else {
                unreachable!()
              }
            }
            None => {
              // obj unknown
              if is_subj_unique {
                WriteMode::Tt_Ut_Sb_On
              } else {
                unreachable!()
              }
            }
          }
        }
        Some(false) => {
          // uri
          match obj_blank_or_uri {
            Some(true) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tt_Ut_Su_Ob
              } else {
                WriteMode::Tt_Uf_Su_Ob
              }
            }
            Some(false) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tt_Ut_Su_Ou
              } else {
                WriteMode::Tt_Uf_Su_Ou
              }
            }
            None => {
              // obj unknown
              if is_subj_unique {
                WriteMode::Tt_Ut_Su_On
              } else {
                WriteMode::Tt_Uf_Su_On
              }
            }
          }
        }
        None => {
          // don't know
          match obj_blank_or_uri {
            Some(true) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tt_Ut_Sn_Ob
              } else {
                WriteMode::Tt_Uf_Sn_Ob
              }
            }
            Some(false) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tt_Ut_Sn_Ou
              } else {
                WriteMode::Tt_Uf_Sn_Ou
              }
            }
            None => {
              // obj unknown
              if is_subj_unique {
                WriteMode::Tt_Ut_Sn_On
              } else {
                WriteMode::Tt_Uf_Sn_On
              }
            }
          }
        }
      }
    } else {
      match subj_blank_or_uri {
        Some(true) => {
          // subj blank
          match obj_blank_or_uri {
            Some(true) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tf_Ut_Sb_Ob
              } else {
                unreachable!()
              }
            }
            Some(false) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tf_Ut_Sb_Ou
              } else {
                unreachable!()
              }
            }
            None => {
              // obj unknown
              if is_subj_unique {
                WriteMode::Tf_Ut_Sb_On
              } else {
                unreachable!()
              }
            }
          }
        }
        Some(false) => {
          // uri
          match obj_blank_or_uri {
            Some(true) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tf_Ut_Su_Ob
              } else {
                WriteMode::Tf_Uf_Su_Ob
              }
            }
            Some(false) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tf_Ut_Su_Ou
              } else {
                WriteMode::Tf_Uf_Su_Ou
              }
            }
            None => {
              // obj unknown
              if is_subj_unique {
                WriteMode::Tf_Ut_Su_On
              } else {
                WriteMode::Tf_Uf_Su_On
              }
            }
          }
        }
        None => {
          // don't know
          match obj_blank_or_uri {
            Some(true) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tf_Ut_Sn_Ob
              } else {
                WriteMode::Tf_Uf_Sn_Ob
              }
            }
            Some(false) => {
              // obj blank
              if is_subj_unique {
                WriteMode::Tf_Ut_Sn_Ou
              } else {
                WriteMode::Tf_Uf_Sn_Ou
              }
            }
            None => {
              // obj unknown
              if is_subj_unique {
                WriteMode::Tf_Ut_Sn_On
              } else {
                WriteMode::Tf_Uf_Sn_On
              }
            }
          }
        }
      }
    }
  }
}

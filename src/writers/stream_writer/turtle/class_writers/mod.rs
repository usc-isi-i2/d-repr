use std::io::{BufWriter, Write};
use std::fmt::Debug;

use hashbrown::{HashSet};

use readers::prelude::Value;
use crate::writers::stream_writer::StreamClassWriter;
use crate::writers::stream_writer::turtle::temp_object_props::TempObjectProps;
use crate::writers::stream_writer::turtle::value_fmt::ValueFmt;

#[macro_use]
pub mod create_specific_writer;

// this is the default writer that can handle all cases, and the one that we use to create the macro factory
pub mod generic_writer;
pub use self::generic_writer::GenericWriter;

// Generate implementation following the Encoding Scheme
// *************************************************************************************************
create_writer!(Tf_Ut_Sb_Ob_Writer, subj_type=b1, obj_type=b1, no_duplicated_subj=true);
create_writer!(Tf_Ut_Sb_Ou_Writer, subj_type=b1, obj_type=b2, no_duplicated_subj=true);
create_writer!(Tf_Ut_Sb_On_Writer, subj_type=b1, obj_type=b3, no_duplicated_subj=true);

create_writer!(Tf_Ut_Su_Ob_Writer, subj_type=b2, obj_type=b1, no_duplicated_subj=true);
create_writer!(Tf_Ut_Su_Ou_Writer, subj_type=b2, obj_type=b2, no_duplicated_subj=true);
create_writer!(Tf_Ut_Su_On_Writer, subj_type=b2, obj_type=b3, no_duplicated_subj=true);

create_writer!(Tf_Ut_Sn_Ob_Writer, subj_type=b3, obj_type=b1, no_duplicated_subj=true);
create_writer!(Tf_Ut_Sn_Ou_Writer, subj_type=b3, obj_type=b2, no_duplicated_subj=true);
create_writer!(Tf_Ut_Sn_On_Writer, subj_type=b3, obj_type=b3, no_duplicated_subj=true);

create_writer!(Tf_Uf_Su_Ob_Writer, subj_type=b2, obj_type=b1);
create_writer!(Tf_Uf_Su_Ou_Writer, subj_type=b2, obj_type=b2);
create_writer!(Tf_Uf_Su_On_Writer, subj_type=b2, obj_type=b3);

create_writer!(Tf_Uf_Sn_Ob_Writer, subj_type=b3, obj_type=b1);
create_writer!(Tf_Uf_Sn_Ou_Writer, subj_type=b3, obj_type=b2);
create_writer!(Tf_Uf_Sn_On_Writer, subj_type=b3, obj_type=b3);
// *************************************************************************************************

// *************************************************************************************************
create_writer!(Tt_Ut_Sb_Ob_Writer, subj_type=b1, obj_type=b1, no_duplicated_subj=true, keep_track_subj=true);
create_writer!(Tt_Ut_Sb_Ou_Writer, subj_type=b1, obj_type=b2, no_duplicated_subj=true, keep_track_subj=true);
create_writer!(Tt_Ut_Sb_On_Writer, subj_type=b1, obj_type=b3, no_duplicated_subj=true, keep_track_subj=true);

create_writer!(Tt_Ut_Su_Ob_Writer, subj_type=b2, obj_type=b1, no_duplicated_subj=true, keep_track_subj=true);
create_writer!(Tt_Ut_Su_Ou_Writer, subj_type=b2, obj_type=b2, no_duplicated_subj=true, keep_track_subj=true);
create_writer!(Tt_Ut_Su_On_Writer, subj_type=b2, obj_type=b3, no_duplicated_subj=true, keep_track_subj=true);

create_writer!(Tt_Ut_Sn_Ob_Writer, subj_type=b3, obj_type=b1, no_duplicated_subj=true, keep_track_subj=true);
create_writer!(Tt_Ut_Sn_Ou_Writer, subj_type=b3, obj_type=b2, no_duplicated_subj=true, keep_track_subj=true);
create_writer!(Tt_Ut_Sn_On_Writer, subj_type=b3, obj_type=b3, no_duplicated_subj=true, keep_track_subj=true);

create_writer!(Tt_Uf_Su_Ob_Writer, subj_type=b2, obj_type=b1, keep_track_subj=true);
create_writer!(Tt_Uf_Su_Ou_Writer, subj_type=b2, obj_type=b2, keep_track_subj=true);
create_writer!(Tt_Uf_Su_On_Writer, subj_type=b2, obj_type=b3, keep_track_subj=true);

create_writer!(Tt_Uf_Sn_Ob_Writer, subj_type=b3, obj_type=b1, keep_track_subj=true);
create_writer!(Tt_Uf_Sn_Ou_Writer, subj_type=b3, obj_type=b2, keep_track_subj=true);
create_writer!(Tt_Uf_Sn_On_Writer, subj_type=b3, obj_type=b3, keep_track_subj=true);
// *************************************************************************************************
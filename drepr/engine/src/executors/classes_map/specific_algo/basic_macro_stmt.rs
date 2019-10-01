macro_rules! mif {
  ($condition:expr ; $true_block:tt) => {
    if $condition {
      $true_block
    }
  };
  ($condition:expr ; $always_true:literal ; $true_block:tt) => {
    $true_block
  };
  ($condition:expr ; $true_block:tt else $false_block:tt) => {
    if $condition {
      $true_block
    } else {
      $false_block
    }
  };
  ($condition:expr ; $always_true:literal exec_true_branch ; $true_block:tt else $false_block:tt) => {
    $true_block
  };
  ($condition:expr ; $always_false:literal exec_false_branch ; $true_block:tt else $false_block:tt) => {
    $false_block
  };
}

macro_rules! exclusive_if {
  ($condition:expr ; $always_true:literal ; $true_block:tt else $false_block:tt) => {
    $true_block
  };
  ($condition:expr ; $true_block:tt else $false_block:tt) => {
    $false_block
  };
}

macro_rules! show_block {
  ($show_block:literal ; $block:tt) => {
    $block
  };
  ($block:tt) => {};
}

macro_rules! hide_block {
  ($hide_block:literal ; $block: tt) => {};
  ($block:tt) => { $block }
}

macro_rules! match_align_func {
  (single {
    single => $b1:tt
    multiple => $b2:tt
  }) => {
      $b1
  };
  (multiple {
    single => $b1:tt
    multiple => $b2:tt
  }) => {
      $b2
  };
}

macro_rules! match_object_prop {
  (blankobject {
    blankobject => $b1:tt
    idobject => $b2:tt
  }) => {
      $b1
  };
  (idobject {
    blankobject => $b1:tt
    idobject => $b2:tt
  }) => {
      $b2
  };
}

macro_rules! match_subj_prop {
  (blanksubject {
    blanksubject => $b1:tt
    internalidsubject => $b2:tt
    externalidsubject => $b3:tt
  }) => {
    $b1
  };
  (internalidsubject {
    blanksubject => $b1:tt
    internalidsubject => $b2:tt
    externalidsubject => $b3:tt
  }) => {
    $b2
  };
  (externalidsubject {
    blanksubject => $b1:tt
    internalidsubject => $b2:tt
    externalidsubject => $b3:tt
  }) => {
    $b3
  };
}

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
  (b1 ; $condition:expr ; $true_block:tt else $false_block:tt) => {
    $true_block
  };
  (b2 ; $condition:expr ; $true_block:tt else $false_block:tt) => {
    $false_block
  };
  (b3 ; $condition:expr ; $true_block:tt else $false_block:tt) => {
    if $condition {
      $true_block
    } else {
      $false_block
    }
  };
}

#[allow(unused_macros)]
macro_rules! block_include {
  ($show_block:literal ; $block:tt) => {
    $block
  };
  ($block:tt) => {};
}

macro_rules! block_discard {
  ($discard_block:literal ; $block: tt) => {};
  ($block:tt) => { $block }
}

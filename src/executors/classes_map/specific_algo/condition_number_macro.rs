macro_rules! condition_number_macro {
  ($array_len:ident, 0, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 1, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 2, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 3, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 4, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 5, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 6, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 7, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 8, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 9, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 10, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 11, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 12, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 13, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 14, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      14 => {
        $macro_func!(14 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 15, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      14 => {
        $macro_func!(14 $(, $arg $($optional)?)*);
      },
      15 => {
        $macro_func!(15 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 16, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      14 => {
        $macro_func!(14 $(, $arg $($optional)?)*);
      },
      15 => {
        $macro_func!(15 $(, $arg $($optional)?)*);
      },
      16 => {
        $macro_func!(16 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 17, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      14 => {
        $macro_func!(14 $(, $arg $($optional)?)*);
      },
      15 => {
        $macro_func!(15 $(, $arg $($optional)?)*);
      },
      16 => {
        $macro_func!(16 $(, $arg $($optional)?)*);
      },
      17 => {
        $macro_func!(17 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 18, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      14 => {
        $macro_func!(14 $(, $arg $($optional)?)*);
      },
      15 => {
        $macro_func!(15 $(, $arg $($optional)?)*);
      },
      16 => {
        $macro_func!(16 $(, $arg $($optional)?)*);
      },
      17 => {
        $macro_func!(17 $(, $arg $($optional)?)*);
      },
      18 => {
        $macro_func!(18 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 19, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      14 => {
        $macro_func!(14 $(, $arg $($optional)?)*);
      },
      15 => {
        $macro_func!(15 $(, $arg $($optional)?)*);
      },
      16 => {
        $macro_func!(16 $(, $arg $($optional)?)*);
      },
      17 => {
        $macro_func!(17 $(, $arg $($optional)?)*);
      },
      18 => {
        $macro_func!(18 $(, $arg $($optional)?)*);
      },
      19 => {
        $macro_func!(19 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
  ($array_len:ident, 20, $macro_func:ident $(, $arg:tt $($optional:literal)?)*) => {
    match $array_len {
      0 => {
        $macro_func!(0 $(, $arg $($optional)?)*);
      }
      1 => {
        $macro_func!(1 $(, $arg $($optional)?)*);
      },
      2 => {
        $macro_func!(2 $(, $arg $($optional)?)*);
      },
      3 => {
        $macro_func!(3 $(, $arg $($optional)?)*);
      },
      4 => {
        $macro_func!(4 $(, $arg $($optional)?)*);
      },
      5 => {
        $macro_func!(5 $(, $arg $($optional)?)*);
      },
      6 => {
        $macro_func!(6 $(, $arg $($optional)?)*);
      },
      7 => {
        $macro_func!(7 $(, $arg $($optional)?)*);
      },
      8 => {
        $macro_func!(8 $(, $arg $($optional)?)*);
      },
      9 => {
        $macro_func!(9 $(, $arg $($optional)?)*);
      },
      10 => {
        $macro_func!(10 $(, $arg $($optional)?)*);
      },
      11 => {
        $macro_func!(11 $(, $arg $($optional)?)*);
      },
      12 => {
        $macro_func!(12 $(, $arg $($optional)?)*);
      },
      13 => {
        $macro_func!(13 $(, $arg $($optional)?)*);
      },
      14 => {
        $macro_func!(14 $(, $arg $($optional)?)*);
      },
      15 => {
        $macro_func!(15 $(, $arg $($optional)?)*);
      },
      16 => {
        $macro_func!(16 $(, $arg $($optional)?)*);
      },
      17 => {
        $macro_func!(17 $(, $arg $($optional)?)*);
      },
      18 => {
        $macro_func!(18 $(, $arg $($optional)?)*);
      },
      19 => {
        $macro_func!(19 $(, $arg $($optional)?)*);
      },
      20 => {
        $macro_func!(20 $(, $arg $($optional)?)*);
      },
      _ => {
        $macro_func!(unknown $(, $arg $($optional)?)*);
      }
    }
  };
}
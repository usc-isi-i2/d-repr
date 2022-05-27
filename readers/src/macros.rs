#[macro_export]
macro_rules! as_enum_type_impl {
    ($cls:ident, $fname:ident, $mut_fname:ident, $enum_type:ident, $real_type_string:tt, $real_type:ty) => {
        #[inline]
        pub fn $fname(&self) -> &$real_type {
            match self {
                $cls::$enum_type(v) => v,
                _ => panic!("ValueError: cannot convert non-{} to {}", $real_type_string, $real_type_string)
            }
        }

        #[inline]
        pub fn $mut_fname(&mut self) -> &mut $real_type {
            match self {
                $cls::$enum_type(v) => v,
                _ => panic!("ValueError: cannot convert non-{} to {}", $real_type_string, $real_type_string)
            }
        }
    }
}

#[macro_export]
macro_rules! is_enum_type_impl {
    ($cls:ident :: $fname:ident ($enum_type:ident $( ($val:tt) )? ) ) => {
        #[inline]
        pub fn $fname(&self) -> bool {
            match self {
                $cls::$enum_type$( ($val) )? => true,
                _ => false
            }
        }
    }
}

#[macro_export]
macro_rules! into_enum_type_impl {
    ($cls:ident, $fname:ident, $enum_type:ident, $real_type_string:tt, $real_type:ty) => {
        #[inline]
        pub fn $fname(self) -> $real_type {
            match self {
                $cls::$enum_type(v) => v,
                _ => panic!("ValueError: cannot convert non-{} to {}", $real_type_string, $real_type_string)
            }
        }
    }
}
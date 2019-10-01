mod class_writers {
    use crate::writers::stream_writer::turtle::temp_object_props::TempObjectProps;
    use crate::writers::stream_writer::turtle::value_fmt::ValueFmt;
    use crate::writers::stream_writer::StreamClassWriter;
    use hashbrown::HashSet;
    use readers::prelude::Value;
    use std::fmt::Debug;
    use std::io::{BufWriter, Write};
    #[macro_use]
    pub mod create_specific_writer {}
    pub mod generic_writer {
        use crate::writers::stream_writer::turtle::temp_object_props::TempObjectProps;
        use crate::writers::stream_writer::turtle::value_fmt::ValueFmt;
        use crate::writers::stream_writer::StreamClassWriter;
        use hashbrown::HashSet;
        use readers::prelude::Value;
        use std::fmt::Debug;
        use std::io::{BufWriter, Write};
        #[allow(dead_code)]
        pub struct GenericWriter<'a, W: Write + Debug> {
            pub class_id: usize,
            pub ont_class: &'a str,
            pub channel: &'a mut BufWriter<W>,
            pub predicates: &'a [String],
            pub value_templates: &'a [ValueFmt],
            pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
            pub written_records: &'a mut [HashSet<String>],
            pub always_write_records: &'a [bool],
        }
        impl<'a, W: Write + Debug> StreamClassWriter for GenericWriter<'a, W> {
            #[inline]
            fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
                self.always_write_records[class_id]
                    || self.written_records[class_id].contains(subject)
            }
            fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
                self.written_records[self.class_id].insert(subject.to_string());
                if is_blank {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                } else {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
                return true;
            }
            fn end_record(&mut self) {
                self.channel.write("\t.\n".as_bytes()).unwrap();
            }
            fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
                self.buffer_oprops[self.class_id].push(TempObjectProps {
                    id: subject.to_string(),
                    is_blank,
                    props: <[_]>::into_vec(box []),
                });
                self.written_records[self.class_id].insert(subject.to_string());
                if is_blank {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                } else {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
                return true;
            }
            fn end_partial_buffering_record(&mut self) {
                self.channel.write("\t.\n".as_bytes()).unwrap();
            }
            fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
                match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/generic_writer.rs" , 81u32 , 9u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/generic_writer.rs" , 95u32 , 26u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/generic_writer.rs" , 96u32 , 27u32 ) ) } }
            }
            fn write_object_property(
                &mut self,
                _target_cls: usize,
                subject: &str,
                predicate_id: usize,
                object: &str,
                is_subject_blank: bool,
                is_object_blank: bool,
                is_new_subj: bool,
            ) {
                if is_new_subj {
                    if is_object_blank {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    } else {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    if is_subject_blank {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[""],
                                &match (&subject,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                            .unwrap();
                    } else {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["<", ">"],
                                &match (&subject,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                            .unwrap();
                    }
                    if is_object_blank {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " ", ".\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    } else {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " <", ">.\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                }
            }
            fn buffer_object_property(
                &mut self,
                _target_cls: usize,
                predicate_id: usize,
                object: String,
                is_object_blank: bool,
            ) {
                self.buffer_oprops[self.class_id]
                    .last_mut()
                    .unwrap()
                    .props
                    .push((predicate_id, object, is_object_blank));
            }
        }
    }
    pub use self::generic_writer::GenericWriter;
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Sb_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Sb_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 20u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 20u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 20u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " ", ";\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Sb_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Sb_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 21u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 21u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 21u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " <", ">;\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Sb_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Sb_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 22u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 22u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 22u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                if is_object_blank {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Su_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Su_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 24u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 24u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 24u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " ", ";\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Su_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Su_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 25u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 25u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 25u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " <", ">;\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Su_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Su_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 26u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 26u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 26u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                if is_object_blank {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Sn_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Sn_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 28u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 28u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 28u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " ", ";\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Sn_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Sn_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 29u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 29u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 29u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " <", ">;\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Ut_Sn_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Ut_Sn_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 30u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 30u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 30u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                if is_object_blank {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Uf_Su_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Uf_Su_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 32u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 32u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 32u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["<", "> a ", ";\n"],
                                &match (&subject, &self.ont_class) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " ", ".\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Uf_Su_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Uf_Su_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 33u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 33u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 33u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["<", "> a ", ";\n"],
                                &match (&subject, &self.ont_class) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " <", ">.\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Uf_Su_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Uf_Su_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 34u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 34u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 34u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " ", ";\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " <", ">;\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            } else {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["<", "> a ", ";\n"],
                                &match (&subject, &self.ont_class) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " ", ".\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " <", ">.\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Uf_Sn_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Uf_Sn_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 36u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 36u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 36u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    if is_subject_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["", " a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["<", "> a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " ", ".\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Uf_Sn_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Uf_Sn_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 37u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 37u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 37u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    if is_subject_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["", " a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["<", "> a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " <", ">.\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tf_Uf_Sn_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tf_Uf_Sn_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.written_records[self.class_id].insert(subject.to_string());
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 38u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 38u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 38u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " ", ";\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " <", ">;\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            } else {
                {
                    if is_subject_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["", " a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["<", "> a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " ", ".\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " <", ">.\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Sb_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Sb_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 42u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 42u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 42u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " ", ";\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Sb_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Sb_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 43u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 43u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 43u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " <", ">;\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Sb_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Sb_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["", " a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 44u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 44u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 44u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                if is_object_blank {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Su_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Su_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 46u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 46u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 46u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " ", ";\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Su_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Su_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 47u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 47u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 47u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " <", ">;\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Su_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Su_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 48u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 48u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 48u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                if is_object_blank {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Sn_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Sn_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 50u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 50u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 50u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " ", ";\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Sn_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Sn_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 51u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 51u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 51u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["\t", " <", ">;\n"],
                            &match (&self.predicates[predicate_id], &object) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Ut_Sn_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Ut_Sn_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 52u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 52u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 52u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            {
                if is_object_blank {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    }
                };
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Uf_Su_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Uf_Su_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 54u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 54u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 54u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["<", "> a ", ";\n"],
                                &match (&subject, &self.ont_class) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " ", ".\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Uf_Su_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Uf_Su_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 55u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 55u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 55u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["<", "> a ", ";\n"],
                                &match (&subject, &self.ont_class) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " <", ">.\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Uf_Su_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Uf_Su_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            {
                self.channel
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &["<", "> a ", ";\n"],
                        &match (&subject, &self.ont_class) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ))
                    .unwrap();
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 56u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 56u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 56u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " ", ";\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " <", ">;\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            } else {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["<", "> a ", ";\n"],
                                &match (&subject, &self.ont_class) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " ", ".\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " <", ">.\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Uf_Sn_Ob_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Uf_Sn_Ob_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 58u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 58u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 58u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " ", ";\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    if is_subject_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["", " a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["<", "> a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " ", ".\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Uf_Sn_Ou_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Uf_Sn_Ou_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 59u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 59u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 59u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &["\t", " <", ">;\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            } else {
                {
                    if is_subject_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["", " a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["<", "> a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                    {
                        self.channel
                            .write_fmt(::core::fmt::Arguments::new_v1(
                                &[" ", " <", ">.\n"],
                                &match (&self.predicates[predicate_id], &object) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ))
                            .unwrap();
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
    #[allow(non_camel_case_types)]
    pub struct Tt_Uf_Sn_On_Writer<'a, W: Write + Debug> {
        pub class_id: usize,
        pub ont_class: &'a str,
        pub channel: &'a mut BufWriter<W>,
        pub predicates: &'a [String],
        pub value_templates: &'a [ValueFmt],
        pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
        pub written_records: &'a mut [HashSet<String>],
        pub always_write_records: &'a [bool],
    }
    impl<'a, W: Write + Debug> StreamClassWriter for Tt_Uf_Sn_On_Writer<'a, W> {
        #[inline]
        fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
            self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
        }
        #[allow(unused_variables)]
        fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
            {
                if self.written_records[self.class_id].contains(subject) {
                    return false;
                }
            };
            self.buffer_oprops[self.class_id].push(TempObjectProps {
                id: subject.to_string(),
                is_blank,
                props: <[_]>::into_vec(box []),
            });
            if is_blank {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["", " a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            } else {
                {
                    self.channel
                        .write_fmt(::core::fmt::Arguments::new_v1(
                            &["<", "> a ", ";\n"],
                            &match (&subject, &self.ont_class) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ))
                        .unwrap();
                }
            };
            return true;
        }
        fn end_partial_buffering_record(&mut self) {
            self.channel.write("\t.\n".as_bytes()).unwrap();
        }
        fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
            match value { Value :: Null => { { :: std :: rt :: begin_panic ( "Cannot write null value because RDF doesn't have a way to represent it" , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 60u32 , 1u32 ) ) } } Value :: Str ( v ) => { self . value_templates [ predicate_id ] . write_string_value ( & mut self . channel , & v . replace ( "\"" , "\\\"" ) ) ; } Value :: Bool ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: I64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: F64 ( v ) => { self . value_templates [ predicate_id ] . write_value ( & mut self . channel , & v . to_string ( ) ) ; } Value :: Array ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 60u32 , 1u32 ) ) } Value :: Object ( _ ) => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "not yet implemented: " ] , & match ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "TTL writers does not support writing array yet. The input value is: " ] , & match ( & value , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) ] , } ) , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "engine/src/writers/stream_writer/turtle/class_writers/mod.rs" , 60u32 , 1u32 ) ) } }
        }
        #[allow(unused_variables)]
        fn write_object_property(
            &mut self,
            _target_cls: usize,
            subject: &str,
            predicate_id: usize,
            object: &str,
            is_subject_blank: bool,
            is_object_blank: bool,
            is_new_subj: bool,
        ) {
            if is_new_subj {
                {
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " ", ";\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["\t", " <", ">;\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            } else {
                {
                    if is_subject_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["", " a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &["<", "> a ", ";\n"],
                                    &match (&subject, &self.ont_class) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                    if is_object_blank {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " ", ".\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    } else {
                        {
                            self.channel
                                .write_fmt(::core::fmt::Arguments::new_v1(
                                    &[" ", " <", ">.\n"],
                                    &match (&self.predicates[predicate_id], &object) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ))
                                .unwrap();
                        }
                    };
                }
            };
        }
        fn buffer_object_property(
            &mut self,
            _target_cls: usize,
            predicate_id: usize,
            object: String,
            is_object_blank: bool,
        ) {
            self.buffer_oprops[self.class_id]
                .last_mut()
                .unwrap()
                .props
                .push((predicate_id, object, is_object_blank));
        }
    }
}

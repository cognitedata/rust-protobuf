// This file is generated by rust-protobuf 3.0.0-alpha.9. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `google/protobuf/any.proto`

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:google.protobuf.Any)
pub struct Any {
    // message fields
    ///  A URL/resource name that uniquely identifies the type of the serialized
    ///  protocol buffer message. This string must contain at least
    ///  one "/" character. The last segment of the URL's path must represent
    ///  the fully qualified name of the type (as in
    ///  `path/google.protobuf.Duration`). The name should be in a canonical form
    ///  (e.g., leading "." is not accepted).
    ///
    ///  In practice, teams usually precompile into the binary all types that they
    ///  expect it to use in the context of Any. However, for URLs which use the
    ///  scheme `http`, `https`, or no scheme, one can optionally set up a type
    ///  server that maps type URLs to message definitions as follows:
    ///
    ///  * If no scheme is provided, `https` is assumed.
    ///  * An HTTP GET on the URL must yield a [google.protobuf.Type][]
    ///    value in binary format, or produce an error.
    ///  * Applications are allowed to cache lookup results based on the
    ///    URL, or have them precompiled into a binary to avoid any
    ///    lookup. Therefore, binary compatibility needs to be preserved
    ///    on changes to types. (Use versioned type names to manage
    ///    breaking changes.)
    ///
    ///  Note: this functionality is not currently available in the official
    ///  protobuf release, and it is not used for type URLs beginning with
    ///  type.googleapis.com.
    ///
    ///  Schemes other than `http`, `https` (or the empty scheme) might be
    ///  used with implementation specific semantics.
    ///
    // @@protoc_insertion_point(field:google.protobuf.Any.type_url)
    pub type_url: ::std::string::String,
    ///  Must be a valid serialized protocol buffer of the above specified type.
    // @@protoc_insertion_point(field:google.protobuf.Any.value)
    pub value: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:google.protobuf.Any.unknown_fields)
    pub unknown_fields: crate::UnknownFields,
    // @@protoc_insertion_point(special_field:google.protobuf.Any.cached_size)
    pub cached_size: crate::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a Any {
    fn default() -> &'a Any {
        <Any as crate::Message>::default_instance()
    }
}

impl Any {
    pub fn new() -> Any {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> crate::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        fields.push(crate::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type_url",
            |m: &Any| { &m.type_url },
            |m: &mut Any| { &mut m.type_url },
        ));
        fields.push(crate::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "value",
            |m: &Any| { &m.value },
            |m: &mut Any| { &mut m.value },
        ));
        crate::reflect::GeneratedMessageDescriptorData::new_2::<Any>(
            "Any",
            0,
            fields,
        )
    }
}

impl crate::Message for Any {
    const NAME: &'static str = "Any";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.type_url = is.read_string()?;
                },
                18 => {
                    self.value = is.read_bytes()?;
                },
                tag => {
                    crate::rt::read_unknown_or_skip_group(tag, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.type_url.is_empty() {
            my_size += crate::rt::string_size(1, &self.type_url);
        }
        if !self.value.is_empty() {
            my_size += crate::rt::bytes_size(2, &self.value);
        }
        my_size += crate::rt::unknown_fields_size(self.unknown_fields());
        self.cached_size.set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::Result<()> {
        if !self.type_url.is_empty() {
            os.write_string(1, &self.type_url)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        os.write_unknown_fields(self.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn unknown_fields(&self) -> &crate::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut crate::UnknownFields {
        &mut self.unknown_fields
    }

    fn new() -> Any {
        Any::new()
    }

    fn clear(&mut self) {
        self.type_url.clear();
        self.value.clear();
        self.unknown_fields.clear();
    }

    fn default_instance() -> &'static Any {
        static instance: Any = Any {
            type_url: ::std::string::String::new(),
            value: ::std::vec::Vec::new(),
            unknown_fields: crate::UnknownFields::new(),
            cached_size: crate::rt::CachedSize::new(),
        };
        &instance
    }
}

impl crate::MessageFull for Any {
    fn descriptor_static() -> crate::reflect::MessageDescriptor {
        crate::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 0)
    }
}

impl ::std::fmt::Display for Any {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValue for Any {
    type RuntimeType = crate::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19google/protobuf/any.proto\x12\x0fgoogle.protobuf\"6\n\x03Any\x12\
    \x19\n\x08type_url\x18\x01\x20\x01(\tR\x07typeUrl\x12\x14\n\x05value\x18\
    \x02\x20\x01(\x0cR\x05valueBv\n\x13com.google.protobufB\x08AnyProtoP\x01\
    Z,google.golang.org/protobuf/types/known/anypb\xa2\x02\x03GPB\xaa\x02\
    \x1eGoogle.Protobuf.WellKnownTypesJ\xf9*\n\x07\x12\x05\x1e\0\x9d\x01\x01\
    \n\xcc\x0c\n\x01\x0c\x12\x03\x1e\0\x122\xc1\x0c\x20Protocol\x20Buffers\
    \x20-\x20Google's\x20data\x20interchange\x20format\n\x20Copyright\x20200\
    8\x20Google\x20Inc.\x20\x20All\x20rights\x20reserved.\n\x20https://devel\
    opers.google.com/protocol-buffers/\n\n\x20Redistribution\x20and\x20use\
    \x20in\x20source\x20and\x20binary\x20forms,\x20with\x20or\x20without\n\
    \x20modification,\x20are\x20permitted\x20provided\x20that\x20the\x20foll\
    owing\x20conditions\x20are\n\x20met:\n\n\x20\x20\x20\x20\x20*\x20Redistr\
    ibutions\x20of\x20source\x20code\x20must\x20retain\x20the\x20above\x20co\
    pyright\n\x20notice,\x20this\x20list\x20of\x20conditions\x20and\x20the\
    \x20following\x20disclaimer.\n\x20\x20\x20\x20\x20*\x20Redistributions\
    \x20in\x20binary\x20form\x20must\x20reproduce\x20the\x20above\n\x20copyr\
    ight\x20notice,\x20this\x20list\x20of\x20conditions\x20and\x20the\x20fol\
    lowing\x20disclaimer\n\x20in\x20the\x20documentation\x20and/or\x20other\
    \x20materials\x20provided\x20with\x20the\n\x20distribution.\n\x20\x20\
    \x20\x20\x20*\x20Neither\x20the\x20name\x20of\x20Google\x20Inc.\x20nor\
    \x20the\x20names\x20of\x20its\n\x20contributors\x20may\x20be\x20used\x20\
    to\x20endorse\x20or\x20promote\x20products\x20derived\x20from\n\x20this\
    \x20software\x20without\x20specific\x20prior\x20written\x20permission.\n\
    \n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\x20COPYRIGHT\x20HO\
    LDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20EXPRESS\
    \x20OR\x20IMPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\x20NOT\n\x20LIMITE\
    D\x20TO,\x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MERCHANTABILITY\x20AND\
    \x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\x20ARE\x20DISCLAIMED.\
    \x20IN\x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\x20OR\x20C\
    ONTRIBUTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIRECT,\x20INDIRECT,\x20INC\
    IDENTAL,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\
    \x20(INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20PROCUREMENT\x20OF\
    \x20SUBSTITUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE,\n\x20DA\
    TA,\x20OR\x20PROFITS;\x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOWEVER\x20C\
    AUSED\x20AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WHETHER\x20\
    IN\x20CONTRACT,\x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INCLUDING\
    \x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\x20OUT\
    \x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\x20ADVIS\
    ED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\n\x01\
    \x02\x12\x03\x20\0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\
    \x03\"\0;\n\x08\n\x01\x08\x12\x03#\0C\n\t\n\x02\x08\x0b\x12\x03#\0C\n\
    \x08\n\x01\x08\x12\x03$\0,\n\t\n\x02\x08\x01\x12\x03$\0,\n\x08\n\x01\x08\
    \x12\x03%\0)\n\t\n\x02\x08\x08\x12\x03%\0)\n\x08\n\x01\x08\x12\x03&\0\"\
    \n\t\n\x02\x08\n\x12\x03&\0\"\n\x08\n\x01\x08\x12\x03'\0!\n\t\n\x02\x08$\
    \x12\x03'\0!\n\xfd\x10\n\x02\x04\0\x12\x05|\0\x9d\x01\x01\x1a\xef\x10\
    \x20`Any`\x20contains\x20an\x20arbitrary\x20serialized\x20protocol\x20bu\
    ffer\x20message\x20along\x20with\x20a\n\x20URL\x20that\x20describes\x20t\
    he\x20type\x20of\x20the\x20serialized\x20message.\n\n\x20Protobuf\x20lib\
    rary\x20provides\x20support\x20to\x20pack/unpack\x20Any\x20values\x20in\
    \x20the\x20form\n\x20of\x20utility\x20functions\x20or\x20additional\x20g\
    enerated\x20methods\x20of\x20the\x20Any\x20type.\n\n\x20Example\x201:\
    \x20Pack\x20and\x20unpack\x20a\x20message\x20in\x20C++.\n\n\x20\x20\x20\
    \x20\x20Foo\x20foo\x20=\x20...;\n\x20\x20\x20\x20\x20Any\x20any;\n\x20\
    \x20\x20\x20\x20any.PackFrom(foo);\n\x20\x20\x20\x20\x20...\n\x20\x20\
    \x20\x20\x20if\x20(any.UnpackTo(&foo))\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20...\n\x20\x20\x20\x20\x20}\n\n\x20Example\x202:\x20Pack\x20and\x20un\
    pack\x20a\x20message\x20in\x20Java.\n\n\x20\x20\x20\x20\x20Foo\x20foo\
    \x20=\x20...;\n\x20\x20\x20\x20\x20Any\x20any\x20=\x20Any.pack(foo);\n\
    \x20\x20\x20\x20\x20...\n\x20\x20\x20\x20\x20if\x20(any.is(Foo.class))\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20foo\x20=\x20any.unpack(Foo.class);\n\
    \x20\x20\x20\x20\x20}\n\n\x20\x20Example\x203:\x20Pack\x20and\x20unpack\
    \x20a\x20message\x20in\x20Python.\n\n\x20\x20\x20\x20\x20foo\x20=\x20Foo\
    (...)\n\x20\x20\x20\x20\x20any\x20=\x20Any()\n\x20\x20\x20\x20\x20any.Pa\
    ck(foo)\n\x20\x20\x20\x20\x20...\n\x20\x20\x20\x20\x20if\x20any.Is(Foo.D\
    ESCRIPTOR):\n\x20\x20\x20\x20\x20\x20\x20any.Unpack(foo)\n\x20\x20\x20\
    \x20\x20\x20\x20...\n\n\x20\x20Example\x204:\x20Pack\x20and\x20unpack\
    \x20a\x20message\x20in\x20Go\n\n\x20\x20\x20\x20\x20\x20foo\x20:=\x20&pb\
    .Foo{...}\n\x20\x20\x20\x20\x20\x20any,\x20err\x20:=\x20anypb.New(foo)\n\
    \x20\x20\x20\x20\x20\x20if\x20err\x20!=\x20nil\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20\x20...\n\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\
    ...\n\x20\x20\x20\x20\x20\x20foo\x20:=\x20&pb.Foo{}\n\x20\x20\x20\x20\
    \x20\x20if\x20err\x20:=\x20any.UnmarshalTo(foo);\x20err\x20!=\x20nil\x20\
    {\n\x20\x20\x20\x20\x20\x20\x20\x20...\n\x20\x20\x20\x20\x20\x20}\n\n\
    \x20The\x20pack\x20methods\x20provided\x20by\x20protobuf\x20library\x20w\
    ill\x20by\x20default\x20use\n\x20'type.googleapis.com/full.type.name'\
    \x20as\x20the\x20type\x20URL\x20and\x20the\x20unpack\n\x20methods\x20onl\
    y\x20use\x20the\x20fully\x20qualified\x20type\x20name\x20after\x20the\
    \x20last\x20'/'\n\x20in\x20the\x20type\x20URL,\x20for\x20example\x20\"fo\
    o.bar.com/x/y.z\"\x20will\x20yield\x20type\n\x20name\x20\"y.z\".\n\n\n\
    \x20JSON\n\x20====\n\x20The\x20JSON\x20representation\x20of\x20an\x20`An\
    y`\x20value\x20uses\x20the\x20regular\n\x20representation\x20of\x20the\
    \x20deserialized,\x20embedded\x20message,\x20with\x20an\n\x20additional\
    \x20field\x20`@type`\x20which\x20contains\x20the\x20type\x20URL.\x20Exam\
    ple:\n\n\x20\x20\x20\x20\x20package\x20google.profile;\n\x20\x20\x20\x20\
    \x20message\x20Person\x20{\n\x20\x20\x20\x20\x20\x20\x20string\x20first_\
    name\x20=\x201;\n\x20\x20\x20\x20\x20\x20\x20string\x20last_name\x20=\
    \x202;\n\x20\x20\x20\x20\x20}\n\n\x20\x20\x20\x20\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20\"@type\":\x20\"type.googleapis.com/google.profile.Person\",\
    \n\x20\x20\x20\x20\x20\x20\x20\"firstName\":\x20<string>,\n\x20\x20\x20\
    \x20\x20\x20\x20\"lastName\":\x20<string>\n\x20\x20\x20\x20\x20}\n\n\x20\
    If\x20the\x20embedded\x20message\x20type\x20is\x20well-known\x20and\x20h\
    as\x20a\x20custom\x20JSON\n\x20representation,\x20that\x20representation\
    \x20will\x20be\x20embedded\x20adding\x20a\x20field\n\x20`value`\x20which\
    \x20holds\x20the\x20custom\x20JSON\x20in\x20addition\x20to\x20the\x20`@t\
    ype`\n\x20field.\x20Example\x20(for\x20message\x20[google.protobuf.Durat\
    ion][]):\n\n\x20\x20\x20\x20\x20{\n\x20\x20\x20\x20\x20\x20\x20\"@type\"\
    :\x20\"type.googleapis.com/google.protobuf.Duration\",\n\x20\x20\x20\x20\
    \x20\x20\x20\"value\":\x20\"1.212s\"\n\x20\x20\x20\x20\x20}\n\n\n\n\n\
    \x03\x04\0\x01\x12\x03|\x08\x0b\n\xd7\n\n\x04\x04\0\x02\0\x12\x04\x99\
    \x01\x02\x16\x1a\xc8\n\x20A\x20URL/resource\x20name\x20that\x20uniquely\
    \x20identifies\x20the\x20type\x20of\x20the\x20serialized\n\x20protocol\
    \x20buffer\x20message.\x20This\x20string\x20must\x20contain\x20at\x20lea\
    st\n\x20one\x20\"/\"\x20character.\x20The\x20last\x20segment\x20of\x20th\
    e\x20URL's\x20path\x20must\x20represent\n\x20the\x20fully\x20qualified\
    \x20name\x20of\x20the\x20type\x20(as\x20in\n\x20`path/google.protobuf.Du\
    ration`).\x20The\x20name\x20should\x20be\x20in\x20a\x20canonical\x20form\
    \n\x20(e.g.,\x20leading\x20\".\"\x20is\x20not\x20accepted).\n\n\x20In\
    \x20practice,\x20teams\x20usually\x20precompile\x20into\x20the\x20binary\
    \x20all\x20types\x20that\x20they\n\x20expect\x20it\x20to\x20use\x20in\
    \x20the\x20context\x20of\x20Any.\x20However,\x20for\x20URLs\x20which\x20\
    use\x20the\n\x20scheme\x20`http`,\x20`https`,\x20or\x20no\x20scheme,\x20\
    one\x20can\x20optionally\x20set\x20up\x20a\x20type\n\x20server\x20that\
    \x20maps\x20type\x20URLs\x20to\x20message\x20definitions\x20as\x20follow\
    s:\n\n\x20*\x20If\x20no\x20scheme\x20is\x20provided,\x20`https`\x20is\
    \x20assumed.\n\x20*\x20An\x20HTTP\x20GET\x20on\x20the\x20URL\x20must\x20\
    yield\x20a\x20[google.protobuf.Type][]\n\x20\x20\x20value\x20in\x20binar\
    y\x20format,\x20or\x20produce\x20an\x20error.\n\x20*\x20Applications\x20\
    are\x20allowed\x20to\x20cache\x20lookup\x20results\x20based\x20on\x20the\
    \n\x20\x20\x20URL,\x20or\x20have\x20them\x20precompiled\x20into\x20a\x20\
    binary\x20to\x20avoid\x20any\n\x20\x20\x20lookup.\x20Therefore,\x20binar\
    y\x20compatibility\x20needs\x20to\x20be\x20preserved\n\x20\x20\x20on\x20\
    changes\x20to\x20types.\x20(Use\x20versioned\x20type\x20names\x20to\x20m\
    anage\n\x20\x20\x20breaking\x20changes.)\n\n\x20Note:\x20this\x20functio\
    nality\x20is\x20not\x20currently\x20available\x20in\x20the\x20official\n\
    \x20protobuf\x20release,\x20and\x20it\x20is\x20not\x20used\x20for\x20typ\
    e\x20URLs\x20beginning\x20with\n\x20type.googleapis.com.\n\n\x20Schemes\
    \x20other\x20than\x20`http`,\x20`https`\x20(or\x20the\x20empty\x20scheme\
    )\x20might\x20be\n\x20used\x20with\x20implementation\x20specific\x20sema\
    ntics.\n\n\n\r\n\x05\x04\0\x02\0\x05\x12\x04\x99\x01\x02\x08\n\r\n\x05\
    \x04\0\x02\0\x01\x12\x04\x99\x01\t\x11\n\r\n\x05\x04\0\x02\0\x03\x12\x04\
    \x99\x01\x14\x15\nW\n\x04\x04\0\x02\x01\x12\x04\x9c\x01\x02\x12\x1aI\x20\
    Must\x20be\x20a\x20valid\x20serialized\x20protocol\x20buffer\x20of\x20th\
    e\x20above\x20specified\x20type.\n\n\r\n\x05\x04\0\x02\x01\x05\x12\x04\
    \x9c\x01\x02\x07\n\r\n\x05\x04\0\x02\x01\x01\x12\x04\x9c\x01\x08\r\n\r\n\
    \x05\x04\0\x02\x01\x03\x12\x04\x9c\x01\x10\x11b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::Lazy<crate::descriptor::FileDescriptorProto> = crate::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> crate::reflect::FileDescriptor {
    static file_descriptor_lazy: crate::rt::Lazy<crate::reflect::GeneratedFileDescriptor> = crate::rt::Lazy::new();
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        let mut messages = ::std::vec::Vec::new();
        messages.push(Any::generated_message_descriptor_data());
        let mut enums = ::std::vec::Vec::new();
        crate::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    crate::reflect::FileDescriptor::new_generated_2(file_descriptor)
}

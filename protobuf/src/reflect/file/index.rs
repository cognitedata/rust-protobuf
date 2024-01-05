use std::collections::HashMap;

use crate::descriptor::DescriptorProto;
use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::path::EnumPath;
use crate::reflect::error::ReflectError;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::fds::fds_extend_with_public;
use crate::reflect::message::is_initialized_is_always_true::compute_is_initialized_is_always_true;
use crate::reflect::message::path::MessagePath;
use crate::reflect::name::concat_paths;
use crate::reflect::service::index::ServiceIndex;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct MessageIndex {
    pub(crate) path: MessagePath,
    pub(crate) name: String,
    pub(crate) name_to_package: String,
    pub(crate) full_name: String,
    pub(crate) enclosing_message: Option<usize>,
    pub(crate) nested_messages: Vec<usize>,
    pub(crate) map_entry: bool,
    pub(crate) first_enum_index: usize,
    pub(crate) enum_count: usize,
    pub(crate) first_oneof_index: usize,
    pub(crate) oneof_count: usize,
    pub(crate) message_index: MessageFieldsIndex,
    pub(crate) is_initialized_is_always_true: bool,
}

#[derive(Debug, Default)]
pub(crate) struct MessageFieldsIndex {
    /// Index of the first field in global field index.
    pub(crate) first_field_index: usize,
    pub(crate) field_count: usize,
    // Following fields map to the local field index.
    pub(crate) field_index_by_name: HashMap<String, usize>,
    pub(crate) field_index_by_name_or_json_name: HashMap<String, usize>,
    pub(crate) field_index_by_number: HashMap<u32, usize>,
    pub(crate) extensions: Vec<FieldIndex>,
}

impl MessageFieldsIndex {
    pub(crate) fn slice_fields<'a>(&self, file_fields: &'a [FieldIndex]) -> &'a [FieldIndex] {
        &file_fields[self.first_field_index..self.first_field_index + self.field_count]
    }
}

#[derive(Debug)]
pub(crate) struct EnumIndex {
    pub(crate) enum_path: EnumPath,
    pub(crate) name_to_package: String,
    pub(crate) full_name: String,
    pub(crate) enclosing_message: Option<usize>,
    pub(crate) index_by_name: HashMap<String, usize>,
    pub(crate) index_by_number: HashMap<i32, usize>,
}

impl EnumIndex {
    pub(crate) fn new(
        enum_path: EnumPath,
        name_to_package: String,
        enclosing_message: Option<usize>,
        proto: &EnumDescriptorProto,
        file: &FileDescriptorProto,
    ) -> EnumIndex {
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.value.iter().enumerate() {
            index_by_number.insert(v.number(), i);
            index_by_name.insert(v.name().to_owned(), i);
        }
        let full_name = concat_paths(file.package(), &name_to_package);
        EnumIndex {
            enum_path,
            full_name,
            name_to_package,
            enclosing_message,
            index_by_name,
            index_by_number,
        }
    }
}

#[derive(Debug)]
pub(crate) struct OneofIndex {
    pub(crate) containing_message: usize,
    pub(crate) index_in_containing_message: usize,
    /// Synthetic oneof for proto3 optional field.
    pub(crate) synthetic: bool,
    pub(crate) fields: Vec<usize>,
}

#[derive(Debug)]
pub(crate) struct FileDescriptorCommon {
    /// Direct dependencies of this file.
    pub(crate) dependencies: Vec<FileDescriptor>,
    /// All messages in this file.
    pub(crate) messages: Vec<MessageIndex>,
    pub(crate) message_by_name_to_package: HashMap<String, usize>,
    pub(crate) top_level_messages: Vec<usize>,
    pub(crate) enums: Vec<EnumIndex>,
    pub(crate) enums_by_name_to_package: HashMap<String, usize>,
    pub(crate) oneofs: Vec<OneofIndex>,
    pub(crate) services: Vec<ServiceIndex>,
    pub(crate) first_extension_field_index: usize,
    /// All fields followed by file-level extensions.
    pub(crate) fields: Vec<FieldIndex>,
}

impl FileDescriptorCommon {
    pub(crate) fn new(
        file: &FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> crate::Result<FileDescriptorCommon> {
        let deps_with_public = fds_extend_with_public(dependencies.clone());

        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut oneofs = Vec::new();
        let mut top_level_messages = Vec::new();

        // Top-level enums start with zero
        for (i, e) in file.enum_type.iter().enumerate() {
            enums.push(EnumIndex::new(
                EnumPath {
                    message_path: MessagePath::default(),
                    enum_index: i,
                },
                e.name().to_owned(),
                None,
                e,
                file,
            ));
        }

        for (i, message) in file.message_type.iter().enumerate() {
            let path = MessagePath(vec![i]);
            let message_index = Self::index_message_and_inners(
                file,
                message,
                &path,
                None,
                "",
                &mut messages,
                &mut enums,
                &mut oneofs,
            );
            top_level_messages.push(message_index);
        }

        let message_by_name_to_package = Self::build_message_by_name_to_package(&messages);
        let enums_by_name_to_package = Self::build_enum_by_name_to_package(&enums);

        let mut services = Vec::new();

        for service in &file.service {
            let service_index = ServiceIndex::index(
                service,
                &FileDescriptorBuilding {
                    current_file_descriptor: file,
                    deps_with_public: &deps_with_public,
                    message_by_name_to_package: &message_by_name_to_package,
                    messages: &messages,
                    enums_by_name_to_package: &enums_by_name_to_package,
                },
            )?;
            services.push(service_index);
        }

        let mut fields = Vec::new();

        Self::build_message_index(
            file,
            &deps_with_public,
            &mut messages,
            &mut fields,
            &message_by_name_to_package,
            &enums_by_name_to_package,
        )?;

        let first_extension_field_index = fields.len();
        for ext in &file.extension {
            fields.push(FieldIndex::index(
                None,
                ext,
                &FileDescriptorBuilding {
                    current_file_descriptor: file,
                    deps_with_public: &deps_with_public,
                    message_by_name_to_package: &message_by_name_to_package,
                    messages: &messages,
                    enums_by_name_to_package: &enums_by_name_to_package,
                },
            )?);
        }

        compute_is_initialized_is_always_true(&mut messages, &fields, file);

        Ok(FileDescriptorCommon {
            dependencies,
            messages,
            message_by_name_to_package,
            enums,
            top_level_messages,
            enums_by_name_to_package,
            oneofs,
            services,
            first_extension_field_index,
            fields,
        })
    }

    fn index_message_and_inners(
        file: &FileDescriptorProto,
        message: &DescriptorProto,
        path: &MessagePath,
        parent: Option<usize>,
        parent_name_to_package: &str,
        messages: &mut Vec<MessageIndex>,
        enums: &mut Vec<EnumIndex>,
        oneofs: &mut Vec<OneofIndex>,
    ) -> usize {
        let name_to_package = concat_paths(parent_name_to_package, message.name());

        let message_index = messages.len();
        messages.push(MessageIndex {
            path: path.clone(),
            name: message.name().to_owned(),
            name_to_package: String::new(),
            full_name: String::new(),
            enclosing_message: parent,
            nested_messages: Vec::with_capacity(message.nested_type.len()),
            map_entry: message.options.get_or_default().map_entry(),
            first_enum_index: enums.len(),
            enum_count: message.enum_type.len(),
            first_oneof_index: oneofs.len(),
            oneof_count: message.oneof_decl.len(),
            message_index: MessageFieldsIndex::default(),
            // Initialized later.
            is_initialized_is_always_true: false,
        });

        for (i, e) in message.enum_type.iter().enumerate() {
            enums.push(EnumIndex::new(
                EnumPath {
                    message_path: path.clone(),
                    enum_index: i,
                },
                concat_paths(&name_to_package, e.name()),
                Some(message_index),
                e,
                file,
            ));
        }

        for (i, _oneof) in message.oneof_decl.iter().enumerate() {
            let fields: Vec<_> = message
                .field
                .iter()
                .enumerate()
                .filter(|(_, f)| f.has_oneof_index() && f.oneof_index() == i as i32)
                .collect();
            let synthetic = fields.len() == 1 && fields[0].1.proto3_optional();
            oneofs.push(OneofIndex {
                containing_message: message_index,
                index_in_containing_message: i,
                synthetic,
                fields: fields.iter().map(|(i, _)| *i).collect(),
            });
        }

        for (i, nested) in message.nested_type.iter().enumerate() {
            let mut nested_path = path.clone();
            nested_path.push(i);
            let nested_index = Self::index_message_and_inners(
                file,
                nested,
                &nested_path,
                Some(message_index),
                &name_to_package,
                messages,
                enums,
                oneofs,
            );
            messages[message_index].nested_messages.push(nested_index);
        }

        messages[message_index].full_name = concat_paths(file.package(), &name_to_package);
        messages[message_index].name_to_package = name_to_package;

        message_index
    }

    fn build_message_by_name_to_package(messages: &[MessageIndex]) -> HashMap<String, usize> {
        messages
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name_to_package.to_owned(), i))
            .collect()
    }

    fn build_enum_by_name_to_package(enums: &[EnumIndex]) -> HashMap<String, usize> {
        enums
            .iter()
            .enumerate()
            .map(|(i, e)| (e.name_to_package.to_owned(), i))
            .collect()
    }

    fn build_message_index(
        file: &FileDescriptorProto,
        deps_with_public: &[FileDescriptor],
        messages: &mut [MessageIndex],
        fields: &mut Vec<FieldIndex>,
        message_by_name_to_package: &HashMap<String, usize>,
        enums_by_name_to_package: &HashMap<String, usize>,
    ) -> crate::Result<()> {
        for i in 0..messages.len() {
            let message_proto = messages[i].path.eval(file).unwrap();
            let building = FileDescriptorBuilding {
                current_file_descriptor: file,
                deps_with_public,
                message_by_name_to_package,
                messages,
                enums_by_name_to_package,
            };
            let message_index = Self::index_message(i, message_proto, &building, fields)?;
            messages[i].message_index = message_index;
        }
        Ok(())
    }

    fn index_message(
        message_index: usize,
        proto: &DescriptorProto,
        building: &FileDescriptorBuilding,
        fields: &mut Vec<FieldIndex>,
    ) -> crate::Result<MessageFieldsIndex> {
        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let first_field_index = fields.len();

        for field in &proto.field {
            fields.push(FieldIndex::index(Some(message_index), field, building)?);
        }

        let field_count = proto.field.len();

        for (i, f) in proto.field.iter().enumerate() {
            let field_index = &fields[first_field_index + i];

            if index_by_number.insert(f.number() as u32, i).is_some() {
                return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
            }
            if index_by_name.insert(f.name().to_owned(), i).is_some() {
                return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
            }
            if index_by_name_or_json_name
                .insert(f.name().to_owned(), i)
                .is_some()
            {
                return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
            }

            if field_index.json_name != f.name() {
                if index_by_name_or_json_name
                    .insert(field_index.json_name.clone(), i)
                    .is_some()
                {
                    return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
                }
            }
        }

        let extensions: Vec<FieldIndex> = proto
            .extension
            .iter()
            .map(|f| FieldIndex::index(Some(message_index), f, building))
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(MessageFieldsIndex {
            first_field_index,
            field_count,
            field_index_by_name: index_by_name,
            field_index_by_name_or_json_name: index_by_name_or_json_name,
            field_index_by_number: index_by_number,
            extensions,
        })
    }
}

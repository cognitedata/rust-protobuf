use std::collections::HashMap;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::fds::fds_extend_with_public;
use crate::reflect::message::index::MessageIndex;
use crate::reflect::message::path::MessagePath;
use crate::reflect::name::concat_paths;
use crate::reflect::service::index::ServiceIndex;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct FileIndexMessageEntry {
    pub(crate) path: MessagePath,
    pub(crate) name_to_package: String,
    pub(crate) full_name: String,
    pub(crate) enclosing_message: Option<usize>,
    pub(crate) nested_messages: Vec<usize>,
    pub(crate) map_entry: bool,
    pub(crate) first_enum_index: usize,
    pub(crate) enum_count: usize,
    pub(crate) message_index: MessageIndex,
}

#[derive(Debug)]
pub(crate) struct FileIndexEnumEntry {
    pub(crate) _message_path: MessagePath,
    pub(crate) _enum_index: usize,
    pub(crate) name_to_package: String,
    pub(crate) enclosing_message: Option<usize>,
}

#[derive(Debug)]
pub(crate) struct FileIndexOneofEntry {}

#[derive(Debug)]
pub(crate) struct FileIndex {
    pub(crate) messages: Vec<FileIndexMessageEntry>,
    pub(crate) message_by_name_to_package: HashMap<String, usize>,
    pub(crate) top_level_messages: Vec<usize>,
    pub(crate) enums: Vec<FileIndexEnumEntry>,
    pub(crate) enums_by_name_to_package: HashMap<String, usize>,
    pub(crate) oneofs: Vec<FileIndexOneofEntry>,
    pub(crate) services: Vec<ServiceIndex>,
}

impl FileIndex {
    pub(crate) fn index(
        file: &FileDescriptorProto,
        deps: &[FileDescriptor],
    ) -> crate::Result<FileIndex> {
        let deps_with_public = fds_extend_with_public(deps.to_vec());

        let mut index = FileIndex {
            messages: Vec::new(),
            message_by_name_to_package: HashMap::new(),
            enums: Vec::new(),
            top_level_messages: Vec::with_capacity(file.message_type.len()),
            enums_by_name_to_package: HashMap::new(),
            oneofs: Vec::new(),
            services: Vec::new(),
        };

        // Top-level enums start with zero
        for (_, e) in file.enum_type.iter().enumerate() {
            index.enums.push(FileIndexEnumEntry {
                _message_path: MessagePath(Vec::new()),
                _enum_index: index.enums.len(),
                name_to_package: e.name().to_owned(),
                enclosing_message: None,
            });
        }

        for (i, message) in file.message_type.iter().enumerate() {
            let path = MessagePath(vec![i]);
            let message_index = index.index_message_and_inners(file, message, &path, None, "");
            index.top_level_messages.push(message_index);
        }

        index.build_message_by_name_to_package();
        index.build_enum_by_name_to_package();

        for service in &file.service {
            let service_index = ServiceIndex::index(
                service,
                &FileDescriptorBuilding {
                    current_file_descriptor: file,
                    current_file_index: &index,
                    deps_with_public: deps,
                },
            )?;
            index.services.push(service_index);
        }

        index.build_message_index(file, &deps_with_public)?;

        Ok(index)
    }

    fn index_message_and_inners(
        &mut self,
        file: &FileDescriptorProto,
        message: &DescriptorProto,
        path: &MessagePath,
        parent: Option<usize>,
        parent_name_to_package: &str,
    ) -> usize {
        let name_to_package = concat_paths(parent_name_to_package, message.name());

        let message_index = self.messages.len();
        self.messages.push(FileIndexMessageEntry {
            path: path.clone(),
            name_to_package: String::new(),
            full_name: String::new(),
            enclosing_message: parent,
            nested_messages: Vec::with_capacity(message.nested_type.len()),
            map_entry: message.options.get_or_default().map_entry(),
            first_enum_index: self.enums.len(),
            enum_count: message.enum_type.len(),
            message_index: MessageIndex::default(),
        });

        for (_, e) in message.enum_type.iter().enumerate() {
            self.enums.push(FileIndexEnumEntry {
                _message_path: path.clone(),
                _enum_index: self.enums.len(),
                name_to_package: concat_paths(&name_to_package, e.name()),
                enclosing_message: Some(message_index),
            });
        }

        for (i, nested) in message.nested_type.iter().enumerate() {
            let mut nested_path = path.clone();
            nested_path.push(i);
            let nested_index = self.index_message_and_inners(
                file,
                nested,
                &nested_path,
                Some(message_index),
                &name_to_package,
            );
            self.messages[message_index]
                .nested_messages
                .push(nested_index);
        }

        self.messages[message_index].full_name = concat_paths(file.package(), &name_to_package);
        self.messages[message_index].name_to_package = name_to_package;

        message_index
    }

    fn build_message_by_name_to_package(&mut self) {
        self.message_by_name_to_package = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name_to_package.to_owned(), i))
            .collect();
    }

    fn build_enum_by_name_to_package(&mut self) {
        self.enums_by_name_to_package = self
            .enums
            .iter()
            .enumerate()
            .map(|(i, e)| (e.name_to_package.to_owned(), i))
            .collect();
    }

    fn build_message_index(
        &mut self,
        file: &FileDescriptorProto,
        deps_with_public: &[FileDescriptor],
    ) -> crate::Result<()> {
        for i in 0..self.messages.len() {
            let message_proto = self.messages[i].path.eval(file).unwrap();
            let building = FileDescriptorBuilding {
                current_file_descriptor: file,
                current_file_index: self,
                deps_with_public,
            };
            let message_index = MessageIndex::index(message_proto, &building)?;
            self.messages[i].message_index = message_index;
        }
        Ok(())
    }
}

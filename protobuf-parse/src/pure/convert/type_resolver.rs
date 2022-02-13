use std::iter;

use crate::model;
use crate::protobuf_path::ProtobufPath;
use crate::pure::convert::WithFullName;
use crate::FileDescriptorPair;
use crate::ProtobufAbsPath;
use crate::ProtobufAbsPathRef;
use crate::ProtobufIdent;
use crate::ProtobufIdentRef;
use crate::ProtobufRelPath;
use crate::ProtobufRelPathRef;

#[derive(thiserror::Error, Debug)]
enum TypeResolverError {
    #[error("object is not found by path: {0}")]
    NotFoundByAbsPath(ProtobufAbsPath),
    #[error("object is not found by path `{0}` in scope `{1}`")]
    NotFoundByRelPath(ProtobufRelPath, ProtobufAbsPath),
}

pub(crate) enum MessageOrEnum<'a> {
    Message(&'a model::Message),
    Enum(&'a model::Enumeration),
}

impl MessageOrEnum<'_> {
    fn _descriptor_type(&self) -> protobuf::descriptor::field_descriptor_proto::Type {
        match *self {
            MessageOrEnum::Message(..) => {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE
            }
            MessageOrEnum::Enum(..) => {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM
            }
        }
    }
}

#[derive(Clone)]
enum LookupScope<'a> {
    File(&'a model::FileDescriptor),
    Message(&'a model::Message, ProtobufAbsPath),
}

impl<'a> LookupScope<'a> {
    fn current_path(&self) -> ProtobufAbsPath {
        match self {
            LookupScope::File(f) => f.package.clone(),
            LookupScope::Message(_, p) => p.clone(),
        }
    }

    fn messages(&self) -> &'a [model::WithLoc<model::Message>] {
        match self {
            &LookupScope::File(file) => &file.messages,
            &LookupScope::Message(messasge, _) => &messasge.messages,
        }
    }

    fn find_message(&self, simple_name: &ProtobufIdentRef) -> Option<&'a model::Message> {
        self.messages()
            .into_iter()
            .find(|m| m.t.name == simple_name.as_str())
            .map(|m| &m.t)
    }

    fn enums(&self) -> &'a [model::Enumeration] {
        match self {
            &LookupScope::File(file) => &file.enums,
            &LookupScope::Message(messasge, _) => &messasge.enums,
        }
    }

    fn members(&self) -> Vec<(ProtobufIdent, MessageOrEnum<'a>)> {
        let mut r = Vec::new();
        r.extend(
            self.enums()
                .into_iter()
                .map(|e| (ProtobufIdent::from(&e.name[..]), MessageOrEnum::Enum(e))),
        );
        r.extend(self.messages().into_iter().map(|m| {
            (
                ProtobufIdent::from(&m.t.name[..]),
                MessageOrEnum::Message(&m.t),
            )
        }));
        r
    }

    fn find_member(&self, simple_name: &ProtobufIdentRef) -> Option<MessageOrEnum<'a>> {
        self.members()
            .into_iter()
            .filter_map(|(member_name, message_or_enum)| {
                if member_name.as_ref() == simple_name {
                    Some(message_or_enum)
                } else {
                    None
                }
            })
            .next()
    }

    fn down(&self, name: &ProtobufIdentRef) -> Option<LookupScope<'a>> {
        match self.find_member(name)? {
            MessageOrEnum::Enum(_) => return None,
            MessageOrEnum::Message(m) => {
                let mut path = self.current_path();
                path.push_simple(name.clone());
                Some(LookupScope::Message(m, path))
            }
        }
    }

    pub(crate) fn find_message_or_enum(
        &self,
        path: &ProtobufRelPathRef,
    ) -> Option<WithFullName<MessageOrEnum<'a>>> {
        let current_path = self.current_path();
        let (first, rem) = match path.split_first_rem() {
            Some(x) => x,
            None => return None,
        };

        if rem.is_empty() {
            match self.find_member(first) {
                Some(message_or_enum) => {
                    let mut result_path = current_path.clone();
                    result_path.push_simple(first);
                    Some(WithFullName {
                        full_name: result_path,
                        t: message_or_enum,
                    })
                }
                None => None,
            }
        } else {
            match self.find_message(first) {
                Some(message) => {
                    let mut message_path = current_path.clone();
                    message_path.push_simple(ProtobufIdentRef::new(&message.name));
                    let message_scope = LookupScope::Message(message, message_path);
                    message_scope.find_message_or_enum(rem)
                }
                None => None,
            }
        }
    }

    fn extensions(&self) -> Vec<&'a model::Extension> {
        match self {
            LookupScope::File(f) => f.extensions.iter().map(|e| &e.t).collect(),
            LookupScope::Message(m, _) => m.extensions.iter().map(|e| &e.t).collect(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct LookupScopeUnion<'a> {
    path: ProtobufAbsPath,
    scopes: Vec<LookupScope<'a>>,
    partial_scopes: Vec<&'a model::FileDescriptor>,
}

impl<'a> LookupScopeUnion<'a> {
    fn down(&self, name: &ProtobufIdentRef) -> LookupScopeUnion<'a> {
        let mut path: ProtobufAbsPath = self.path.clone();
        path.push_simple(name);

        let mut scopes: Vec<_> = self.scopes.iter().flat_map(|f| f.down(name)).collect();
        let mut partial_scopes = Vec::new();

        for &partial_scope in &self.partial_scopes {
            if partial_scope.package == path {
                scopes.push(LookupScope::File(partial_scope));
            } else if partial_scope.package.starts_with(&path) {
                partial_scopes.push(partial_scope);
            }
        }
        LookupScopeUnion {
            path,
            scopes,
            partial_scopes,
        }
    }

    pub(crate) fn lookup(&self, path: &ProtobufRelPath) -> LookupScopeUnion<'a> {
        let mut scope = self.clone();
        for c in path.components() {
            scope = scope.down(c);
        }
        scope
    }

    pub(crate) fn extensions(&self) -> Vec<&'a model::Extension> {
        self.scopes.iter().flat_map(|s| s.extensions()).collect()
    }
}

pub(crate) struct TypeResolver<'a> {
    pub(crate) current_file: &'a model::FileDescriptor,
    pub(crate) deps: &'a [FileDescriptorPair],
}

impl<'a> TypeResolver<'a> {
    pub(crate) fn all_files(&self) -> Vec<&'a model::FileDescriptor> {
        iter::once(self.current_file)
            .chain(self.deps.iter().map(|p| &p.parsed))
            .collect()
    }

    pub(crate) fn root_scope(&self) -> LookupScopeUnion<'a> {
        let (scopes, partial_scopes) = self
            .all_files()
            .into_iter()
            .partition::<Vec<_>, _>(|f| f.package.is_root());
        LookupScopeUnion {
            path: ProtobufAbsPath::root(),
            scopes: scopes.into_iter().map(LookupScope::File).collect(),
            partial_scopes,
        }
    }

    pub(crate) fn find_message_or_enum_by_abs_name(
        &self,
        absolute_path: &ProtobufAbsPath,
    ) -> anyhow::Result<WithFullName<MessageOrEnum<'a>>> {
        for file in self.all_files() {
            if let Some(relative) = absolute_path.remove_prefix(&file.package) {
                if let Some(w) = LookupScope::File(file).find_message_or_enum(&relative) {
                    return Ok(w);
                }
            }
        }

        return Err(TypeResolverError::NotFoundByAbsPath(absolute_path.clone()).into());
    }

    pub(crate) fn resolve_message_or_enum(
        &self,
        scope: &ProtobufAbsPathRef,
        name: &ProtobufPath,
    ) -> anyhow::Result<WithFullName<MessageOrEnum>> {
        match name {
            ProtobufPath::Abs(name) => Ok(self.find_message_or_enum_by_abs_name(&name)?),
            ProtobufPath::Rel(name) => {
                // find message or enum in current package
                for p in scope.self_and_parents() {
                    let mut fq = p.to_owned();
                    fq.push_relative(&name);
                    if let Ok(me) = self.find_message_or_enum_by_abs_name(&fq) {
                        return Ok(me);
                    }
                }

                Err(TypeResolverError::NotFoundByRelPath(name.clone(), scope.to_owned()).into())
            }
        }
    }
}

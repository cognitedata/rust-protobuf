use std::sync::Arc;

use crate::descriptor::FileDescriptorProto;
use crate::owning_ref::OwningRef;
use crate::reflect::file::index::FileDescriptorCommon;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct DynamicFileDescriptor {
    pub(crate) proto: Arc<FileDescriptorProto>,
    pub(crate) common: FileDescriptorCommon,
}

impl DynamicFileDescriptor {
    pub fn new(
        proto: FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> crate::Result<DynamicFileDescriptor> {
        let proto = Arc::new(proto);

        let common = FileDescriptorCommon::new(OwningRef::new_arc(proto.clone()), dependencies)?;

        Ok(DynamicFileDescriptor { proto, common })
    }
}

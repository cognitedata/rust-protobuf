use crate::customize::Customize;
use crate::gen::code_writer::CodeWriter;

/// Write serde attr according to specified codegen option.
pub(crate) fn write_serde_attr(w: &mut CodeWriter, customize: &Customize, attr: &str) {
    if customize.serde_derive.unwrap_or(false) {
        if let Some(ref cfg) = customize.serde_derive_cfg {
            w.write_line(&format!("#[cfg_attr({}, {})]", cfg, attr));
        } else {
            w.write_line(&format!("#[{}]", attr));
        }
    }
}

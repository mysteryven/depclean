use crate::{runtime::DepCheckerContext, Checker};

/// Implements a checker for JavaScript family files, read their ESM and `require` dependencies.
#[derive(Default, Debug)]
pub struct JSFamily;

impl Checker for JSFamily {
    fn run(&self, ctx: &DepCheckerContext) {
        let module_record = ctx.module_records();
        for request_module in module_record.requested_modules.keys() {
            // ignore relative import
            if !request_module.starts_with(".") {
                ctx.add_use(request_module.clone())
            }
        }
    }
}


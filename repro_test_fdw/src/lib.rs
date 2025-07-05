use pgrx::pg_sys::panic::ErrorReport;
use pgrx::prelude::*;
use std::collections::HashMap;
use supabase_wrappers::prelude::*;

pg_module_magic!();

#[wrappers_fdw(error_type = "ErrorReport")]
pub(crate) struct ReproTestFdw {}

impl ForeignDataWrapper<ErrorReport> for ReproTestFdw {
    fn new(_server: ForeignServer) -> Result<Self, ErrorReport> {
        Ok(Self {})
    }

    fn begin_scan(
        &mut self,
        _quals: &[Qual],
        _columns: &[Column],
        _sorts: &[Sort],
        _limit: &Option<Limit>,
        _options: &HashMap<String, String>,
    ) -> Result<(), ErrorReport> {
        Ok(())
    }

    fn iter_scan(&mut self, _output_row: &mut Row) -> Result<Option<()>, ErrorReport> {
        Ok(None)
    }

    fn end_scan(&mut self) -> Result<(), ErrorReport> {
        Ok(())
    }

    fn re_scan(&mut self) -> Result<(), ErrorReport> {
        Ok(())
    }

    fn begin_modify(&mut self, _options: &HashMap<String, String>) -> Result<(), ErrorReport> {
        Ok(())
    }

    fn insert(&mut self, _row: &Row) -> Result<(), ErrorReport> {
        Ok(())
    }

    fn end_modify(&mut self) -> Result<(), ErrorReport> {
        Ok(())
    }
}

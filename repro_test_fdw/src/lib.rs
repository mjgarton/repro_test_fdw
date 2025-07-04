use pgrx::pg_sys::panic::ErrorReport;
use pgrx::prelude::*;
use pgrx::PgSqlErrorCode;
use std::collections::HashMap;
use std::fmt::Display;
use supabase_wrappers::prelude::*;

pg_module_magic!();

//extension_sql_file!("../sql/bootstrap.sql", bootstrap);
//extension_sql_file!("../sql/finalize.sql", finalize);

#[wrappers_fdw(error_type = "Error")]
pub(crate) struct ReproTestFdw {}

impl ForeignDataWrapper<Error> for ReproTestFdw {
    fn new(_server: ForeignServer) -> Result<Self, Error> {
        Ok(Self {})
    }

    fn begin_scan(
        &mut self,
        _quals: &[Qual],
        _columns: &[Column],
        _sorts: &[Sort],
        _limit: &Option<Limit>,
        _options: &HashMap<String, String>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn iter_scan(&mut self, _output_row: &mut Row) -> Result<Option<()>, Error> {
        Ok(None)
    }

    fn end_scan(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn re_scan(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn begin_modify(&mut self, _options: &HashMap<String, String>) -> Result<(), Error> {
        Ok(())
    }

    fn insert(&mut self, _row: &Row) -> Result<(), Error> {
        Ok(())
    }

    fn end_modify(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    Config(String),
    Internal(String),
    Unknown(String),
    Query(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Config(e) => write!(f, "config error : {e}"),
            Error::Internal(e) => write!(f, "internal error : {e}"),
            Error::Unknown(e) => write!(f, "unknown error : {e}"),
            Error::Query(e) => write!(f, "query error : {e}"),
        }
    }
}

impl From<Error> for ErrorReport {
    fn from(err: Error) -> Self {
        ErrorReport::new(PgSqlErrorCode::ERRCODE_FDW_ERROR, format!("{err}"), "")
    }
}

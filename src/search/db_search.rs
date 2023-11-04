use std::io::Error;

use crate::output::Output;

use super::{SearchQuery, SearchResults};

#[allow(unused_variables)]
pub(crate) fn search(query: &SearchQuery) -> Result<SearchResults, Error> {
    if query.verbose {
        Output::info(&format!("Searching DB: {:#?}", query));
    }
    todo!()
}
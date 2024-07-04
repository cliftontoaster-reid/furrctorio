use serde::{Deserialize, Serialize};
use url::Url;
use super::fmod::FModShort;

/// Represents a list of FModShort objects with pagination information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FModList {
  /// Pagination information for the list.
  pub pagination: Pagination,
  /// The list of FModShort objects.
  pub results: Vec<FModShort>,
}

/// Represents pagination information for a list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
  /// The total number of items in the list.
  pub count: usize,
  /// Links for navigating through the paginated list.
  pub(crate) links: PaginationLinks,
  /// The current page number.
  pub page: usize,
  /// The total number of pages in the list.
  pub page_count: usize,
  /// The number of items per page.
  pub page_size: usize,
}

/// Represents links for navigating through a paginated list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaginationLinks {
  /// The URL for the first page of the list.
  pub first: Option<Url>,
  /// The URL for the last page of the list.
  pub last: Option<Url>,
  /// The URL for the next page of the list.
  pub next: Option<Url>,
  /// The URL for the previous page of the list.
  pub prev: Option<Url>,
}

use serde::Serialize;

use crate::ModrinthError;
use crate::types::search_query::Sort::Relevance;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// A search query to be used with [`ModrinthClient.search()`](crate::ModrinthClient::search).
///
/// Use [`SearchQuery::new`] and the following builder-style helpers to construct a search query.
/// - [`SearchQuery::with_query`],
/// - [`SearchQuery::with_sort`],
/// - [`SearchQuery::with_offset`],
/// - [`SearchQuery::with_limit`], and
/// - [`SearchQuery::and_facet`]
///
/// Facets are represented as groups (outer [`Vec`]) that are AND-ed together, where each group
/// contains facets that are OR-ed together.
pub struct SearchQuery {
    /// The query to search for
    pub query: String,
    /// The facets to search with. Facets are used to fine tune search results based on more specific fields.  
    /// See the documentation on [`SearchQuery::new()`] for further details.
    pub facets: Vec<Vec<Facet>>,
    /// The sorting method used for sorting search results
    pub sort: Sort,
    /// The offset into the search results (skips this many results)
    pub offset: u32,
    /// The number of results returned by the search
    ///
    /// Modrinth caps this at **100**. The default is **10**.
    pub limit: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
/// The sorting method used for sorting search results
pub enum Sort {
    #[default]
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// The field of a search facet.
pub enum FacetField {
    ProjectType,
    /// Matches against every project type across all of the project’s versions.
    /// Not limited to the project’s primary/version-specific type.
    AllProjectTypes,
    /// (loaders are lumped in with categories in search)
    Categories,
    Versions,
    OpenSource,
    Environment,
    #[deprecated(note = "use [`Environment`](crate::Environment) instead")]
    ClientSide,
    #[deprecated(note = "use [`Environment`](crate::Environment) instead")]
    ServerSide,
    Title,
    Author,
    Follows,
    ProjectId,
    License,
    Downloads,
    /// (uses Unix timestamp)
    CreatedTimestamp,
    /// (uses Unix timestamp)
    ModifiedTimestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum FacetOperation {
    Equals,
    /// Alias of [`FacetOperation::Equals`]. Makes more sense in some contexts.
    Includes,
    DoesNotEqual,
    IsGreaterThanOrEqualTo,
    IsGreaterThan,
    IsLessThanOrEqualTo,
    IsLessThan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// A search facet. Used to fine tune and filter search results.
///
/// This is later formatted for the api as `{type}{operation}{value}`, ex. `versions!=1.20.1`
pub struct Facet {
    #[serde(rename = "type")]
    pub field: FacetField,
    pub operation: FacetOperation,
    pub value: String,
    #[serde(skip_serializing)]
    group_members: Vec<Facet>,
}

impl SearchQuery {
    /// Create a new search query with default values.
    ///
    /// Defaults:
    /// - `query`: empty string
    /// - `sort`: [`Sort::Relevance`]
    /// - `offset`: `0`
    /// - `limit`: **10** (Modrinth max is **100**)
    /// - `facets`: none
    pub fn new() -> Self {
        return Self {
            query: "".into(),
            facets: vec![],
            sort: Relevance,
            offset: 0,
            limit: 10,
        };
    }

    /// Set the text query.
    pub fn with_query<S: Into<String>>(mut self, query: S) -> Self {
        self.query = query.into();
        return self;
    }
    /// Set the sort mode.
    pub fn with_sort(mut self, sort: Sort) -> Self {
        self.sort = sort;
        return self;
    }
    /// Set the offset into the result set.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        return self;
    }
    /// Set the number of results returned.
    ///
    /// Modrinth caps this at **100**.
    pub fn with_limit(mut self, limit: u8) -> Self {
        self.limit = limit;
        return self;
    }

    /// And a facet group to the query.
    ///
    /// The `facet` itself may contain OR-ed members created via [`Facet::or`]
    pub fn and_facet(mut self, mut facet: Facet) -> Self {
        let mut members = vec![];
        let group_members = std::mem::take(&mut facet.group_members);
        members.push(facet);
        members.extend(group_members);

        self.facets.push(members);

        return self;
    }

    /// Format facets into the `{type}{op}{value}` strings expected by the API.
    pub fn format_facets(&self) -> Vec<Vec<String>> {
        return self
            .facets
            .iter()
            .map(|f| {
                f.iter()
                    .map(|f| {
                        #[allow(deprecated)]
                        let type_str = match f.field {
                            FacetField::ProjectType => "project_type",
                            FacetField::AllProjectTypes => "all_project_types",
                            FacetField::Categories => "categories",
                            FacetField::Versions => "versions",
                            FacetField::OpenSource => "open_source",
                            FacetField::Environment => "environment",
                            FacetField::ClientSide => "client_side",
                            FacetField::ServerSide => "server_side",
                            FacetField::Title => "title",
                            FacetField::Author => "author",
                            FacetField::Follows => "follows",
                            FacetField::ProjectId => "project_id",
                            FacetField::License => "license",
                            FacetField::Downloads => "downloads",
                            FacetField::CreatedTimestamp => "created_timestamp",
                            FacetField::ModifiedTimestamp => "modified_timestamp",
                        };

                        let op_str = match f.operation {
                            FacetOperation::Equals => "=",
                            FacetOperation::Includes => "=",
                            FacetOperation::DoesNotEqual => "!=",
                            FacetOperation::IsGreaterThanOrEqualTo => ">=",
                            FacetOperation::IsGreaterThan => ">",
                            FacetOperation::IsLessThanOrEqualTo => "<=",
                            FacetOperation::IsLessThan => "<",
                        };

                        return format!("{type_str}{op}{v}", op = op_str, v = f.value);
                    })
                    .collect()
            })
            .collect();
    }

    /// Convert this query into an URL-encoded query string suitable for the `GET /search` endpoint.
    pub fn to_url_query(&self) -> Result<String, ModrinthError> {
        #[derive(Serialize)]
        struct Params {
            query: String,
            facets: String, // JSON string
            sort: Sort,
            offset: u32,
            limit: u8,
        }

        let facets_json = serde_json::to_string(&self.format_facets())
            .map_err(ModrinthError::JSONSerializationError)?;
        let params = Params {
            query: self.query.clone(),
            facets: facets_json,
            sort: self.sort.clone(), // or just &self.sort depending on Sort: Clone/Serialize setup
            offset: self.offset,
            limit: self.limit,
        };

        let encoded =
            serde_urlencoded::to_string(&params).map_err(ModrinthError::URLSerializationError)?;
        return Ok(encoded) as Result<String, ModrinthError>;
    }
}

impl Facet {
    /// Create a new facet.
    pub fn new<S: Into<String>>(field: FacetField, operation: FacetOperation, value: S) -> Self {
        return Self {
            field,
            operation,
            value: value.into(),
            group_members: Vec::new(),
        };
    }

    /// Add another facet to the current facet's OR group.
    ///
    /// This is used together with [`SearchQuery::and_facet`].  
    /// Example: `Facet::new(...).or(Facet::new(...))` produces a facet group where the members are OR-ed together.
    pub fn or(mut self, facet: Self) -> Self {
        self.group_members.push(facet);
        return self;
    }
}

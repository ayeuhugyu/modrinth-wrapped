use modrinth_wrapped::{Facet, FacetField, FacetOperation, SearchQuery, Sort};
use serde::Serialize;

#[test]
fn search_query_format_facets_single_facet() {
    let q = SearchQuery::new().with_query("test").and_facet(Facet::new(
        FacetField::Versions,
        FacetOperation::Equals,
        "1.20.1",
    ));

    let formatted = q.format_facets();

    assert_eq!(formatted, vec![vec!["versions=1.20.1".to_string()]]);
}

#[test]
fn search_query_format_facets_grouped_or() {
    let q =
        SearchQuery::new()
            .with_query("test")
            .and_facet(
                Facet::new(FacetField::Categories, FacetOperation::Includes, "forge").or(
                    Facet::new(FacetField::Categories, FacetOperation::Includes, "neoforge"),
                ),
            )
            .and_facet(Facet::new(
                FacetField::Versions,
                FacetOperation::DoesNotEqual,
                "1.19",
            ));

    let formatted = q.format_facets();

    assert_eq!(
        formatted,
        vec![
            vec![
                "categories=forge".to_string(),
                "categories=neoforge".to_string()
            ],
            vec!["versions!=1.19".to_string()],
        ]
    );
}

#[test]
fn search_query_to_url_query_matches_expected_encoding() {
    let q = SearchQuery::new()
        .with_query("Alex's Mobs")
        .with_sort(Sort::Relevance)
        .with_offset(5)
        .with_limit(3)
        .and_facet(
            Facet::new(FacetField::OpenSource, FacetOperation::Equals, "true").or(Facet::new(
                FacetField::OpenSource,
                FacetOperation::DoesNotEqual,
                "false",
            )),
        )
        .and_facet(Facet::new(
            FacetField::Downloads,
            FacetOperation::IsGreaterThan,
            "1000",
        ));

    let formatted_facets = q.format_facets();

    #[derive(Serialize)]
    struct Params {
        query: String,
        facets: String,
        sort: Sort,
        offset: u32,
        limit: u8,
    }

    let facets_json = serde_json::to_string(&formatted_facets).unwrap();
    let expected = serde_urlencoded::to_string(&Params {
        query: q.query.clone(),
        facets: facets_json,
        sort: q.sort.clone(),
        offset: q.offset,
        limit: q.limit,
    })
    .unwrap();

    let got = q.to_url_query().unwrap();

    assert_eq!(got, expected);
}

#[test]
fn search_query_to_url_query_with_no_facets() {
    let q = SearchQuery::new()
        .with_query("minecraft")
        .with_offset(0)
        .with_limit(2);

    let got = q.to_url_query().unwrap();

    assert!(got.contains("query=minecraft"));
    assert!(got.contains("facets=%5B%5D"));
}

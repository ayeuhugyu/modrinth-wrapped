use modrinth_wrapped::{
    Facet, FacetField, FacetOperation, ModrinthClient, ModrinthError, SearchQuery,
};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn search_basic_query_returns_hits() -> Result<(), ModrinthError> {
    let c = client().await?;

    let q = SearchQuery::new().with_query("fabric").with_limit(5);

    let res = c.search(&q).await?;

    assert!(res.total_hits >= res.hits.len() as u32);
    assert!(!res.hits.is_empty());
    Ok(())
}

#[tokio::test]
async fn search_with_facets_grouped_or_returns_hits() -> Result<(), ModrinthError> {
    let c = client().await?;

    let q =
        SearchQuery::new()
            .with_query("mod")
            .with_limit(5)
            .and_facet(
                Facet::new(FacetField::Categories, FacetOperation::Includes, "forge").or(
                    Facet::new(FacetField::Categories, FacetOperation::Includes, "fabric"),
                ),
            );

    let res = c.search(&q).await?;

    assert!(res.total_hits >= res.hits.len() as u32);
    assert!(!res.hits.is_empty());
    Ok(())
}

#[tokio::test]
async fn search_pagination_offset_different_results_or_same_total() -> Result<(), ModrinthError> {
    let c = client().await?;

    let q0 = SearchQuery::new()
        .with_query("minecraft")
        .with_offset(0)
        .with_limit(5);

    let q1 = SearchQuery::new()
        .with_query("minecraft")
        .with_offset(5)
        .with_limit(5);

    let r0 = c.search(&q0).await?;
    let r1 = c.search(&q1).await?;

    assert_eq!(r0.total_hits, r1.total_hits);
    assert!(r0.hits.len() <= 5);
    assert!(r1.hits.len() <= 5);

    Ok(())
}

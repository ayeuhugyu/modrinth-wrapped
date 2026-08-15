#[derive(Debug, Clone, PartialEq, Eq)]
/// The query type for the forge_updates.json endpoint
pub enum ForgeUpdatesQueryType {
    ForgeOnly,
    NeoForgeOnly,
    Both,
}

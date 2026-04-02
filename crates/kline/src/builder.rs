//! Fluent builders for the kline module.
//!
//! The current kline proto surface is query-only. Runtime hooks for trade
//! ingestion, sentiment updates, and epoch boundaries are internal and do not
//! have user-submittable transaction wrappers in the Rust SDK.

#[cfg(test)]
mod tests {
    #[test]
    fn builder_module_compiles() {
        assert_eq!(2 + 2, 4);
    }
}

use std::collections::BTreeMap;

/// Helper trait for converting a persisted type into its original type.
pub trait FromPersisted<T> {
    /// Convert a persisted value into its original type.
    fn from_persisted(value: T) -> Self;
}

impl<K, V, VP> FromPersisted<BTreeMap<K, VP>> for BTreeMap<K, V>
where
    V: FromPersisted<VP>,
    K: Ord,
{
    fn from_persisted(value: BTreeMap<K, VP>) -> Self {
        value
            .into_iter()
            .map(|(k, v)| (k, V::from_persisted(v)))
            .collect()
    }
}

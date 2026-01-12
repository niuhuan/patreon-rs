use chrono::{DateTime, Utc};
use serde::Deserialize;

pub fn de_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    let value = Option::<T>::deserialize(deserializer)?;
    Ok(value.unwrap_or_default())
}

pub fn unix_epoch() -> DateTime<Utc> {
    DateTime::<Utc>::UNIX_EPOCH
}

pub fn de_null_unix_epoch<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<DateTime<Utc>>::deserialize(deserializer)?;
    Ok(value.unwrap_or_else(unix_epoch))
}

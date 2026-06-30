use std::io::Cursor;

use fyn_client::{DataWithCachePolicy, ErrorKind};

#[test]
fn reject_overflowing_cache_policy_length() {
    let mut bytes = vec![0; 8];
    bytes.extend_from_slice(&u64::MAX.to_le_bytes());

    let err = DataWithCachePolicy::from_reader(Cursor::new(bytes)).unwrap_err();
    assert!(matches!(err.kind(), ErrorKind::ArchiveRead(_)));
}

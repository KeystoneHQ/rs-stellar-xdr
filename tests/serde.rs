#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(all(feature = "serde"))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr1;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr1;

use stellar_xdr1::{BytesM, Hash, StringM, VecM};

#[cfg(feature = "curr")]
use stellar_xdr1::AccountId;

#[cfg(feature = "curr")]
use std::str::FromStr;

#[test]
fn test_serde_ser() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        serde_json::to_string(&<_ as TryInto<VecM<u8>>>::try_into("hello world").unwrap())?,
        "[104,101,108,108,111,32,119,111,114,108,100]"
    );
    assert_eq!(
        serde_json::to_string(&<_ as TryInto<BytesM>>::try_into("hello world").unwrap())?,
        "\"68656c6c6f20776f726c64\""
    );
    assert_eq!(
        serde_json::to_string(&<_ as TryInto<StringM>>::try_into("hello world").unwrap())?,
        "\"hello world\""
    );
    assert_eq!(
        serde_json::to_string(&<_ as TryInto<Hash>>::try_into(
            *b"01234567890123456789013456789012"
        )?)?,
        "\"3031323334353637383930313233343536373839303133343536373839303132\""
    );
    #[cfg(feature = "curr")]
    assert_eq!(
        serde_json::to_string(&AccountId::from_str(
            "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"
        ).unwrap())?,
        "\"GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF\""
    );

    Ok(())
}

#[test]
fn test_serde_der() -> Result<(), Box<dyn std::error::Error>> {
    let v: VecM<u8> = serde_json::from_str("[104,101,108,108,111,32,119,111,114,108,100]")?;
    assert_eq!(v, <_ as TryInto<VecM<u8>>>::try_into("hello world").unwrap());

    let v: BytesM = serde_json::from_str("\"68656c6c6f20776f726c64\"")?;
    assert_eq!(v, <_ as TryInto<BytesM>>::try_into("hello world").unwrap());

    let v: StringM = serde_json::from_str("\"hello world\"")?;
    assert_eq!(v, <_ as TryInto<StringM>>::try_into("hello world").unwrap(),);

    let v: Hash = serde_json::from_str(
        "\"3031323334353637383930313233343536373839303133343536373839303132\"",
    )?;
    assert_eq!(
        v,
        <_ as TryInto<Hash>>::try_into(*b"01234567890123456789013456789012")?,
    );

    #[cfg(feature = "curr")]
    assert_eq!(
        AccountId::from_str("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF").unwrap(),
        serde_json::from_str("\"GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF\"")?,
    );

    Ok(())
}

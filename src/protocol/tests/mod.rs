#[cfg(not(feature = "with-serde"))]
mod request_plain;
#[cfg(feature = "with-serde")]
mod request_serde;
mod response;
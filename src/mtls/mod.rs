//! # mtls
//!
//! the mtls module exposes "low level" building blocks
//! that can be used to implement the security requirements
//! needed to call psd2 APIs
//!
//! For more information on the spec
//! https://openbanking.atlassian.net/wiki/spaces/DZ/pages/7046134/Open+Banking+Security+Profile+-+Implementer+s+Draft+v1.1.0
//! https://www.europeanpaymentscouncil.eu/sites/default/files/kb/file/2018-11/API%20EG%20058-18%20v1.0%20eIDAS%20and%20TPP%20identification%20%28PSD2%29.pdf
//!

pub use self::client::{Certificate, Client};

pub mod client;

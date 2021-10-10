pub mod chuck;
pub mod cktype;
mod cstring;
pub mod fn_macros;
pub mod query;
pub mod util;

use std::result::Result;

// re-export
pub use cktype::CKType;
pub use query::Query;

// chuck version is #define-d, so not supported by bindgen
// major version must be the same between chuck:chugin
const CK_DLL_VERSION_MAJOR: chuck::t_CKUINT = 0x0008;
// minor version of chugin must be less than or equal to chuck's
const CK_DLL_VERSION_MINOR: chuck::t_CKUINT = 0x0000;

fn ck_dll_version_make(maj: chuck::t_CKUINT, min: chuck::t_CKUINT) -> chuck::t_CKUINT {
    (maj << 16) | min
}

/// Return Chugin API version; to be returned from a public ck_version function
pub fn version() -> chuck::t_CKUINT {
    ck_dll_version_make(CK_DLL_VERSION_MAJOR, CK_DLL_VERSION_MINOR)
}

/// Chugin result type
pub type CKResult<T = (), E = &'static str> = Result<T, E>;

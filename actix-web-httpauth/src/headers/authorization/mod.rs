//! `Authorization` header and various auth schemes.

mod header;
mod scheme;

pub use self::{
    header::Authorization,
    scheme::{basic::Basic, bearer::Bearer, Scheme},
};

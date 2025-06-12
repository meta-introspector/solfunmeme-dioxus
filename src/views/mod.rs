pub mod dashboard;
//pub use dashboard::*;

mod footer;
pub use footer::*;

pub mod clusters;
//pub use clusters::*;

pub mod accounts;
//pub use accounts::*;

mod coins;
//pub use coins::*;

mod query_accounts;
pub use query_accounts::*;

mod airdrop;
pub use airdrop::*;

mod send_sol;
pub use send_sol::*;

mod receive_sol;
pub use receive_sol::*;

pub mod extras;
//pub use extras::*;

mod connect_first;
pub use connect_first::*;

pub mod extras_views;
pub use extras_views::*;

//mod extras_views;
//pub use extras_views::*;
pub mod connections;
//pub use connections::*;

pub mod clusters_management;
//pub use clusters_management::*;

mod connection_management;
//pub use connection_management::*;
mod connection_filter;
//pub use connection_filter::*;
mod connection_list;
//pub use connection_list::*;
//mod query_accounts;
//pub use query_accounts::*;

mod crypto_frontend;
use crypto_frontend::*;
mod crypto_style;
use crypto_style::*;
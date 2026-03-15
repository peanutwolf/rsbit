//! API endpoint path constants for the Bybit V5 API.
//!
//! Each submodule corresponds to a Python enum file (e.g., `market.py` → `market`).
//! Constants are named to match the Python enum variant names.

pub mod account;
pub mod asset;
pub mod broker;
pub mod crypto_loan;
pub mod earn;
pub mod fiat;
pub mod institutional_loan;
pub mod market;
pub mod misc;
pub mod position;
pub mod pre_upgrade;
pub mod rate_limit;
pub mod rfq;
pub mod spot_leverage;
pub mod spot_margin;
pub mod spread;
pub mod trade;
pub mod user;

/// Collect all endpoint path constants for validation.
#[cfg(test)]
pub(crate) fn all_endpoints() -> Vec<&'static str> {
    let mut paths = Vec::new();
    paths.extend_from_slice(&market::ALL);
    paths.extend_from_slice(&trade::ALL);
    paths.extend_from_slice(&account::ALL);
    paths.extend_from_slice(&position::ALL);
    paths.extend_from_slice(&asset::ALL);
    paths.extend_from_slice(&user::ALL);
    paths.extend_from_slice(&broker::ALL);
    paths.extend_from_slice(&crypto_loan::ALL);
    paths.extend_from_slice(&earn::ALL);
    paths.extend_from_slice(&fiat::ALL);
    paths.extend_from_slice(&rfq::ALL);
    paths.extend_from_slice(&spot_margin::ALL);
    paths.extend_from_slice(&spot_leverage::ALL);
    paths.extend_from_slice(&misc::ALL);
    paths.extend_from_slice(&rate_limit::ALL);
    paths.extend_from_slice(&pre_upgrade::ALL);
    paths.extend_from_slice(&spread::ALL);
    paths.extend_from_slice(&institutional_loan::ALL);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_endpoints_are_non_empty_and_start_with_v5() {
        let endpoints = all_endpoints();
        assert!(
            endpoints.len() >= 250,
            "Expected at least 250 endpoints, got {}",
            endpoints.len()
        );
        for path in &endpoints {
            assert!(!path.is_empty(), "Found empty endpoint path");
            assert!(
                path.starts_with("/v5/"),
                "Endpoint does not start with /v5/: {}",
                path
            );
        }
    }

    #[test]
    fn no_duplicate_endpoints_within_modules() {
        // Some paths may legitimately appear in different modules (e.g. spread
        // vs market both have orderbook), but within a single module there
        // should be no duplicates.
        use std::collections::HashSet;
        macro_rules! check_no_dupes {
            ($mod_all:expr, $name:expr) => {
                let set: HashSet<_> = $mod_all.iter().collect();
                assert_eq!(
                    set.len(),
                    $mod_all.len(),
                    "Duplicate endpoint in {}",
                    $name
                );
            };
        }
        check_no_dupes!(market::ALL, "market");
        check_no_dupes!(trade::ALL, "trade");
        check_no_dupes!(account::ALL, "account");
        check_no_dupes!(position::ALL, "position");
        check_no_dupes!(asset::ALL, "asset");
        check_no_dupes!(user::ALL, "user");
        check_no_dupes!(broker::ALL, "broker");
        check_no_dupes!(crypto_loan::ALL, "crypto_loan");
        check_no_dupes!(earn::ALL, "earn");
        check_no_dupes!(fiat::ALL, "fiat");
        check_no_dupes!(rfq::ALL, "rfq");
        check_no_dupes!(spot_margin::ALL, "spot_margin");
        check_no_dupes!(spot_leverage::ALL, "spot_leverage");
        check_no_dupes!(misc::ALL, "misc");
        check_no_dupes!(rate_limit::ALL, "rate_limit");
        check_no_dupes!(pre_upgrade::ALL, "pre_upgrade");
        check_no_dupes!(spread::ALL, "spread");
        check_no_dupes!(institutional_loan::ALL, "institutional_loan");
    }
}

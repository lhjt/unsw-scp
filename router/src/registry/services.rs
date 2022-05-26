use std::collections::HashSet;

use crate::routes::create_service::{NewFlag, NewService};

/// Valiadate a list of new services. Returns whether or not the service definitions are valid.
pub(crate) fn validate_services(service_definitions: &[NewService]) -> bool {
    // ensure that all service names are unique
    if service_definitions
        .iter()
        .map(|s| s.name.as_str())
        .collect::<HashSet<&str>>()
        .len()
        != service_definitions.len()
    {
        return false;
    }

    // Ensure that names and categories do not have 0 length names
    if !service_definitions
        .iter()
        .all(|service| !service.category.is_empty() && !service.name.is_empty())
    {
        return false;
    }

    // Ensure that timestamps are valid
    service_definitions.iter().all(|svc| {
        if svc.naf.is_none() || svc.nbf.is_none() {
            return true;
        }

        svc.nbf.unwrap().lt(&svc.naf.unwrap())
    })
}

/// Valiadates a list of new flags. Returns whether or not the flag definitions are valid.
pub(crate) fn validate_flags(flag_definitions: &[NewFlag]) -> bool {
    // Ensure all flag types are valid
    if !flag_definitions
        .iter()
        .all(|f| f.flag_type == "dynamic" || f.flag_type == "static")
    {
        return false;
    }

    // ensure that the id, display_name, category, flag have a length
    if !flag_definitions.iter().all(|f| {
        !f.id.is_empty()
            && !f.display_name.is_empty()
            && !f.category.is_empty()
            && !f.flag.is_empty()
    }) {
        return false;
    }

    // Ensure that all flags have at least 0 points
    if !flag_definitions.iter().all(|f| f.points >= 0) {
        return false;
    }

    true
}

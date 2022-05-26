use std::collections::HashSet;

use crate::GAIA_ADDR;

/// Get the roles for a user.
///
/// # Params
///
/// - `token`: the JWT token used to identify the current user.
pub(crate) async fn get_roles(token: &str) -> anyhow::Result<HashSet<String>> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://{}/api/selfserve/roles", GAIA_ADDR.as_str()))
        .header("X-Scp-Auth", token)
        .send()
        .await?;

    Ok(res.json::<HashSet<String>>().await?)
}

/*
 * @todo #1 Create POST endpoint to /accounts, so that accounts can be created in a form or through
 * the API. For now I'll assume we have a basic email/password authentication for accounts.
 * Accounts are assigned to users.
 */

#[post("/accounts")]
pub async fn post_user() -> &'static str {
    "TODO"
}

use std::env;

pub fn ib_flex_token() -> String {
    env::var("IB_FLEX_TOKEN").expect("Need to set IB_FLEX_TOKEN")
}

pub fn ib_flex_balance_query_id() -> String {
    env::var("IB_FLEX_BALANCE_QUERY_ID").expect("Need to set IB_FLEX_BALANCE_QUERY_ID")
}

pub fn ib_flex_transactions_query_id() -> String {
    env::var("IB_FLEX_TRANSACTIONS_QUERY_ID").expect("Need to set IB_FLEX_TRANSACTIONS_QUERY_ID")
}

// pub fn mongodb_url() -> String {
//    env::var("MONGODB_URL").expect("MONGODB_URL must be set!")
//}
//
///// Relative to journal repo root if journal_repo_url is supplied
///// Otherwise absolute
// pub fn journal_path() -> Option<String> {
//    env::var("JOURNAL_PATH").ok()
//}
// pub fn journal_repo_url() -> String {
//    env::var("JOURNAL_REPO_URL").expect("JOURNAL_REPO_URL must be set!")
//}
// pub fn journal_repo_credentials() -> Option<(String, String)> {
//    Some((
//        env::var("JOURNAL_REPO_USERNAME").ok()?,
//        env::var("JOURNAL_REPO_PAT").ok()?,
//    ))
//}

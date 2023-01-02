#[macro_use]
extern crate lazy_static;

pub mod auth;
pub mod prelude;

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_search() {
        println!("{:#?}", auth::search_schools("peter").await.unwrap());
    }
}

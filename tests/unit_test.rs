pub fn good() -> Result<()> {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    #[tokio::main]
    async fn main() -> Result<()> {
        let res = reqwest::get("http://localhost:8000/health").await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        assert_eq!(good(Ok);
    }
}
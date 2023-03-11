use anyhow::Result;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::de::DeserializeOwned;

use crate::models::{config::Config, issue::Issue, repository::Repository};

async fn fetch_github<'a, T: DeserializeOwned>(config: &Config, url: &str) -> Result<T> {
    let client = reqwest::Client::new();

    let raw = client
        .get(format!("https://api.github.com/{}", url))
        .header(
            AUTHORIZATION,
            format!("Bearer {}", &config.github_access_token),
        )
        .header(ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(USER_AGENT, &config.user_name)
        .send()
        .await?;

    Ok(raw.json::<T>().await?)
}

pub async fn fetch_issues_self(config: &Config) -> Result<Vec<Issue>> {
    Ok(fetch_github::<Vec<Issue>>(config, "issues").await?)
}

pub async fn fetch_issues_repo(config: &Config, repo: &str) -> Result<Vec<Issue>> {
    Ok(fetch_github::<Vec<Issue>>(config, format!("repos/{}/issues", repo).as_str()).await?)
}

pub async fn fetch_repositories(config: &Config) -> Result<Vec<Repository>> {
    Ok(fetch_github::<Vec<Repository>>(
        config,
        format!("users/{}/repos", config.user_name).as_str(),
    )
    .await?)
}

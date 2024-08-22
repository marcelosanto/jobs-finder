use reqwest::{self, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub html_url: String,
    pub id: u64,
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub created_at: String,
    pub updated_at: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub  login: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq)]
pub struct Label {
    pub  name: String,
    pub color: String,
}

pub async fn get_jobs()-> reqwest::Result<Vec<Issue>> {
    
    let response = Client::new().get("https://api.github.com/repos/frontendbr/vagas/issues?page=1&per_page=20")
    .send()    
    .await?;

    let jobs_response = response.json().await?;

     Ok(jobs_response)

}
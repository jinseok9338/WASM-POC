use serde::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct PostGresConfig {
  
    pub pg: deadpool_postgres::Config,
}
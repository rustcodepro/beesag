use async_graphql::{Context, Object, Result as GqlResult, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, response::Html, routing::get};
use serde::Deserialize;
mod amelgtf;
mod amelseq;
mod args;
mod bimp;
mod bimpseq;
mod data;
mod database;
mod genome;
mod graphql;
mod machine;
mod smartcore;

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, Mutation, async_graphql::EmptySubscription)
        .data(vec![])
        .finish();

    let app = Router::new()
        .route("/graphql", get(graphql_handler).post(graphql_handler))
        .route("/", get(form_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

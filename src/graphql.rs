use async_graphql::{Context, Object, Result as GqlResult, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, response::Html, routing::get};
use serde::Deserialize;

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

#[derive(SimpleObject, Clone)]
struct Item {
    id: i32,
    name: String,
}

struct Query;
#[Object]
impl Query {
    async fn items(&self, ctx: &Context<'_>) -> GqlResult<Vec<Item>> {
        let items = ctx.data_opt::<Vec<Item>>().unwrap_or(&vec![]);
        Ok(items.clone())
    }
}

struct Mutation;
#[Object]
impl Mutation {
    async fn add_item(&self, ctx: &Context<'_>, name: String) -> GqlResult<Vec<Item>> {
        let mut items = ctx.data_opt::<Vec<Item>>().unwrap_or(&vec![]).clone();
        let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        items.push(Item { id: new_id, name });
        ctx.data.insert(items.clone());
        Ok(items)
    }
}

type AppSchema = Schema<Query, Mutation, async_graphql::EmptySubscription>;

async fn graphql_handler(
    schema: axum::extract::Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn form_handler() -> Html<&'static str> {
    Html(
        r#"
        <form action="/graphql" method="post">
            <input type="text" name="query" value="mutation { add_item(name: \"test\") { id name } }" readonly>
            <button type="submit">Add Item</button>
        </form>
        <div id="list"></div>
        <script>
            fetch("/graphql", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ query: "{ items { id name } }" })
            })
            .then(r => r.json())
            .then(data => {
                const list = data.data.items.map(i => `<li>${i.id}: ${i.name}</li>`).join("");
                document.getElementById("list").innerHTML = `<ul>${list}</ul>`;
            });
        </script>
    "#,
    )
}

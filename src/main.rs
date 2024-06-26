use std::sync::Arc;

use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
     Router,
};

use sqlx::{FromRow, PgPool};
use async_graphql::{http::GraphiQLSource, Context, EmptySubscription, Object, Schema, SimpleObject};


struct Query;

#[derive(Clone)]
struct MyState {
    pool: Arc<PgPool>,
}

#[derive(FromRow,SimpleObject)]
struct Member {
    id: i32,
    username: String,
    email: String,

}


#[Object]
impl Query {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<Member>, sqlx::Error> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool not found in context");
        let users = sqlx::query_as::<_, Member>("SELECT * FROM Member")
            .fetch_all(pool.as_ref())
            .await?;

        Ok(users)
    }
}
struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn add_member(&self, ctx: &Context<'_>, username: String, email: String) -> Result<Member, sqlx::Error> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool not found in context");
        
        // Insert the new member into the database
        let member = sqlx::query_as::<_, Member>(
            "INSERT INTO Member (username, email) VALUES ($1, $2) RETURNING *"
        )
        .bind(username)
        .bind(email)
        .fetch_one(pool.as_ref())
        .await?;
        
        Ok(member)
    }
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {

    sqlx::migrate!()
    .run(&pool)
    .await
    .expect("Failed to run migrations");

    let pool = Arc::new(pool); 
    let schema = Schema::build(Query, MutationRoot, EmptySubscription)
        .data(pool.clone()) // Add the pool to the schema context
        .finish();

    let state = MyState { pool };
    let router = Router::new()
        .route("/", get(graphiql)
        .post_service(GraphQL::new(schema.clone())))
        .with_state(state);

    Ok(router.into())
}

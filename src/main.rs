use std::{convert::Infallible, fmt::Display};

use derive_new::new;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLInputObject,
    GraphQLObject, RootNode,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    response::content,
    State,
};

#[macro_use]
extern crate rocket;

#[derive(GraphQLObject)]
#[graphql(description = "This is my first attempt at graphql in rust")]
struct Person {
    id: String,
    name: String,
    age: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "This is my first attempt at graphql in rust")]
struct InputPerson {
    name: String,
    age: i32,
}

#[derive(new)]
struct Context {
    token: Option<Token>,
}
impl juniper::Context for Context {}

struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "0.1"
    }
    fn person(context: &Context, id: String) -> FieldResult<Person> {
        let num: i32 = id.parse()?; // This is just here to illustrate the error case
        let token = &context.token;
        if let Some(t) = token {
            println!("token {}", t);
        }
        Ok(Person {
            id,
            name: String::from("Marcus"),
            age: 31,
        })
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

struct Token(String);
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum TokenError {
    Missing,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");

        match token {
            Some(token) => {
                println!("{}", token);
                // check validity
                Outcome::Success(Token(token.to_string()))
            }
            // token does not exist
            None => Outcome::Failure((Status::Unauthorized, TokenError::Missing)),
        }
    }
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    token: Token,
) -> juniper_rocket::GraphQLResponse {
    let c = Context::new(Some(token));
    request.execute_sync(&*schema, &c)
}

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(Context::new(None))
        .manage(Schema::new(
            Query,
            EmptyMutation::<Context>::new(),
            EmptySubscription::<Context>::new(),
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}

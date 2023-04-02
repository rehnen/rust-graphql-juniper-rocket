use derive_new::new;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLInputObject,
    GraphQLObject, RootNode,
};
use rocket::{response::content, State};

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
struct Context {}
impl juniper::Context for Context {}

struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "0.1"
    }
    fn person(context: &Context, id: String) -> FieldResult<Person> {
        let num: i32 = id.parse()?;
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

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[launch]

async fn rocket() -> _ {
    rocket::build()
        .manage(Context::new())
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

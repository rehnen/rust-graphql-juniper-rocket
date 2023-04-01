use juniper::{EmptyMutation, EmptySubscription, RootNode};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{response::content, State};

use juniper::tests::fixtures::starwars::schema::{Database, Query};

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Person {
    name: String,
    age: i32,
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[get("/person/<id>")]
fn person(id: i32) -> Option<Json<Person>> {
    if id == 2 {
        return None;
    }
    Some(Json(Person {
        name: String::from("marcus"),
        age: 31,
    }))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(Database::new())
        .manage(Schema::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        ))
        .mount(
            "/",
            routes![graphiql, person, get_graphql_handler, post_graphql_handler],
        )
}

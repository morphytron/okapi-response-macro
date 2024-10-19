# Okapi Response macro crate

## Create code based on responses structs rather than typing and implementing manually.  Utilizes both macros from the Rocket and Okapi-Rocket crates.

### Usage

*Example 1*
Use three different structs that you could potentially return at an endpoint.  

**Note: usage requires the "paste" crate to be imported.

```rust
use rocket::get;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use okapi_response_mac::okapi_rocket_response;
use paste::paste;
use rocket::serde::json::Json;

#[derive(Default, JsonSchema, Serialize)]
pub struct A {
    name: String,
    email: String,
    other: String,
    things: String,
    blah: u32,
}
#[derive(JsonSchema, Serialize)]
pub struct B {
    name: String,
    email: String,
}
#[derive(Serialize, JsonSchema)]
pub struct C {
    name: String,
    email: String,
}

okapi_rocket_response!(A : (One, 200, "application/json"),(Accept, 204, "application/json") ->
    B :  (Two, 203, "application/json") -> C : (Three, 404, "application/json") -> (500,501));
/// Random documentation for OpenApi
#[openapi(tag = "User Versions")]
#[get("/")]
pub fn schema_derived() ->  ABCR {
    ABC::One(Json::from(A {
        name: "asdfasdf".to_string(),
        email: "asdgasdg".to_string(),
        other: "hghsfghs".to_string(),
        things: "ertret".to_string(),
        blah: 0,
    }))
}

```

It combines your struct names into a single Enum followed by R to indicate a valid Rocket Response.  

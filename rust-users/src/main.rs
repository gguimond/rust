#[macro_use]
extern crate nickel;

#[macro_use(bson, doc)]
extern crate bson;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use nickel::status::StatusCode::{self};

#[macro_use]
//extern crate serde_derive;

extern crate serde_json;
use serde_json::{Value, Error};

//#[derive(Serialize, Deserialize, Debug)]
struct User {    
	firstname: String,
    lastname: String,
    email: String,
}

fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    let user = User{
		firstname: "test".to_owned(),
	    lastname: "test".to_owned(),
	    email: "test".to_owned(),
	};

    fn get_data_string() -> Result<String, Error> {
    	Ok("{}".to_owned())
	}

    router.get("/users", middleware! { |request, mut response|
    	let mut data_result = "{\"data\":[".to_owned();
    	match get_data_string() {
            Ok(data) => {
                let string_data = format!("{}", data);
                data_result.push_str(&string_data);
            },
            Err(e) => return response.send(format!("{}", e))
        }
        data_result.push_str("]}");
        response.set(MediaType::Json);
        format!("{}", data_result)
    });

    server.utilize(router);

    server.listen("127.0.0.1:9002");
}

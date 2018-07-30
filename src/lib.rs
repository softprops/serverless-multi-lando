#[macro_use]
extern crate cpython;
#[macro_use]
extern crate lando;

// extends http::Request type with api gateway info
use lando::RequestExt;

gateway!(
    "hello" => |request, _| {
        println!("{:?}", request.path_parameters());
        Ok(lando::Response::new(format!(
            "hello {}",
            request
                .path_parameters()
                .get("name")
                .cloned()
                .unwrap_or_else(|| "stranger".to_owned())
        )))
    },
    "goodbye" => |request, _| {
        println!("{:?}", request.path_parameters());
        Ok(lando::Response::new(format!(
            "goodbye {}",
            request
                .path_parameters()
                .get("name")
                .cloned()
                .unwrap_or_else(|| "stranger".to_owned())
        )))
    }
);

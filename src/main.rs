#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("statics"));

    server.get("/", middleware! { |_, res|
        format!("root {} {}", 0, 1);
    });

    server.get("/get/:name", middleware! { |req, res|
        let mut data = HashMap::new();
        data.insert("name", req.param("name"));
        return res.render("assets/template.tpl", &data);
    });

    server.listen("0.0.0.0:6767")
}

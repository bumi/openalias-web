#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

extern crate serde;
extern crate openalias;

use std::collections::BTreeMap;
use rocket::Request;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[derive(Serialize, Deserialize)]
struct CryptoAddressOf {
    pub cryptocurrency: String,
    pub address: String,
    pub recipient_name: Option<String>,
    pub tx_description: Option<String>,
    pub tx_amount: Option<String>,
    pub tx_payment_id: Option<String>,
    pub address_signature: Option<String>,
    pub checksum: Option<(u32, bool)>,
    pub additional_values: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct ShowResponse {
    pub addresses: Vec<CryptoAddressOf>,
    pub domain: String,
}

#[catch(404)]
fn not_found(req: &Request) -> status::Custom<Template> {
    render_not_found(req.uri().path().to_string())
}

fn render_not_found(path: String) -> status::Custom<Template> {
    let mut map = std::collections::HashMap::new();
    map.insert("path", path);
    status::Custom(Status::NotFound, Template::render("errors/404", &map))
}

#[get("/")]
fn index() -> status::Custom<Template> {
    let mut map = std::collections::HashMap::new();
    map.insert("host","https://openalias-web.herokuapp.com");
    status::Custom(Status::Ok, Template::render("index", map))
}

#[get("/<domain>")]
fn show(domain: String) -> status::Custom<Template> {
    match openalias::addresses(&domain) {
        Ok(result) => {
            if result.is_empty() {
                render_not_found(domain)
            } else {
                let mut addresses = Vec::new();
                for caddress in result {
                    let caddr = CryptoAddressOf { 
                        cryptocurrency: caddress.cryptocurrency,
                        address: caddress.address,
                        recipient_name: caddress.recipient_name,
                        tx_description: caddress.tx_description,
                        tx_amount: caddress.tx_amount,
                        tx_payment_id: caddress.tx_payment_id,
                        address_signature: caddress.address_signature,
                        checksum: caddress.checksum,
                        additional_values: caddress.additional_values,
                    };
                    addresses.push(caddr);
                }

                status::Custom(Status::Ok, Template::render("show", ShowResponse { domain, addresses }))
            }
        }
        Err(_result) => {
            render_not_found(domain) 
        }
    }
}

fn main() {
    rocket::ignite()
      .mount("/", routes![index, show])
      .mount("/assets", StaticFiles::from("assets"))
      .attach(Template::fairing())
      .register(catchers![not_found])
      .launch();
}

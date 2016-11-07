extern crate hyper;
extern crate select;
// this brings it into the namespace; default would be Client except for "as hClient"
use hyper::Client;
use std::io::Read;
use select::document::Document;
use select::predicate::{Attr, Class, Name};

fn main() {
    let client = Client::new();

    let mut response = client.get("https://brson.github.io/demo/wishlist.html?test=true")
                             .send()
                             .expect("Request failed");
    let mut body = String::new();
    response.read_to_string(&mut body).expect("Read failed");

    let document = Document::from(body.as_str());

    let wrapper = document.find(Attr("id", "item-page-wrapper"));
    let rows = wrapper.find(Class("a-fixed-right-grid"));
    // for row in rows.iter() {
    //     let title_node = row.find(Name("h5")).first();
    //     let price_node = row.find(Class("a-color-price")).first();
    //     // match things that have both a title and a price, ignore everything else
    //     match (title_node, price_node) {
    //         (Some(title), Some(price)) => {
    //             println!("* Book \"{}\", with price {}",
    //                 title.text().trim(),
    //                 price.text().trim()
    //             );
    //         },
    //         (_, _) => (),
    //     }
    // }
    // this is the same as what's above, maybe looks a little nicer
    let mut total = 0f64;
    for row in rows.iter() {
        let title_node = row.find(Name("h5")).first();
        let price_node = row.find(Class("a-color-price")).first();
        if let (Some(title), Some(price)) = (title_node, price_node) {
            println!("* Book \"{}\", with price {}",
                title.text().trim(),
                price.text().trim()
            );
        }
        if let Some(price) = price_node {
            // println!("price = {:?}", price.text());
            let price_float = price.text().parse::<f64>();
            // if let Some(price_float) = price_num {
            //     total += price_num;
            // }
        }
    }
    println!("Total = {:?}", total)
}

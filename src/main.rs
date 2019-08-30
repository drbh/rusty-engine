#[macro_use]
#[allow(unreachable_code)]

extern crate rouille;


extern crate alphavantage;

// use alphavantage::time_series::IntradayInterval::ThirtyMinutes;

// fn main() {
//     let client = alphavantage::Client::new("F3GVWG91RSWV8SAN");
//     // let time_series = client.get_time_series_daily("TVIX").unwrap();
//     // // let entry = time_series.entries.last().unwrap();
//     // // println!("{:?}", entry);

//     // for entry in time_series.entries {
//     //     println!("{:#?}", entry);
//     // }
    
//     let time_series_intra = client.get_time_series_intraday("TVIX", ThirtyMinutes).unwrap();
//     // let entry = time_series.entries.last().unwrap();
//     // println!("{:?}", entry);

//     for entry in time_series_intra.entries {
//         println!("{:#?}", entry);
//     }

//     // let exchange_rate = client.get_exchange_rate("USD", "EUR").unwrap();
//    // println!("{:?}", exchange_rate);
// }
fn main() {
    println!("Now listening on localhost:8000");

    // The `start_server` starts listening forever on the given address.
    rouille::start_server("localhost:8000", move |request| {

        router!(request,
            (GET) (/yo) => {
                // If the request's URL is `/`, we jump here.
                // This block builds a `Response` object that redirects to the `/hello/world`.
                rouille::Response::redirect_302("/hello/world")
            },

            (GET) (/hello/world) => {
                // If the request's URL is `/hello/world`, we jump here.
                println!("hello world");

                // Builds a `Response` object that contains the "hello world" text.
                rouille::Response::text("hello world")
            },

            (GET) (/panic) => {
                panic!("Oops!")
            },

            (GET) (/{id: u32}) => {
                println!("u32 {:?}", id);

                // For the same of the example we return an empty response with a 400 status code.
                rouille::Response::empty_400()
            },

            (GET) (/{id: String}) => {
                // If the request's URL is for example `/foo`, we jump here.
                //
                // This route is similar to the previous one, but this time we have a `String`.
                // Parsing into a `String` never fails.
                println!("String {:?}", id);

                // Builds a `Response` object that contains "hello, " followed with the value
                // of `id`.
                rouille::Response::text(format!("hello, {}", id))
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}
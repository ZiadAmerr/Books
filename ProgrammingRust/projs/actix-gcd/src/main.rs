use actix_web :: {web, App, HttpResponse, HttpServer};

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
    });

    println!("Starting server on http://localhost:3000");

    server
        .bind("127.0.0.1:3000")  // bind the server to the address
        .expect("error binding server to address")  // if the server can't bind to the address, print an error message
        .run()  // run the server
        .expect("error running server");  // if the server can't run, print an error message
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <doctype !html>
        <html>
            <head>
                <title>GCD Calculator</title>
            </head>
            <body>
                <h1>GCD Calculator</h1>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GCD</button>
                </form>
            </body>
        </html>
        "#
    )
}
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index)) // ✅ Uses async handler
    });

    println!("Starting server on http://localhost:3000");

    server.bind("127.0.0.1:3000")?.run().await // ✅ Awaiting the server

    // Terminate

    // Another
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <!DOCTYPE html>
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

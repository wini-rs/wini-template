use {
    crate::{
        layouts::header,
        pages,
        shared::wini::{
            layer::MetaLayerBuilder,
            PORT,
        },
        template,
        utils::wini::{
            cache,
            handling_file::{self},
        },
    },
    axum::{middleware, routing::get, Router},
    log::info,
    std::collections::HashMap,
    tower_http::compression::CompressionLayer,
};


pub async fn start() {
    // The main router of the application is defined here
    let app = Router::<()>::new()
        .route("/", get(pages::hello::render))
        .layer(middleware::from_fn(header::render))
        .layer(
            MetaLayerBuilder::default()
                .default_meta(HashMap::from_iter([
                        ("title", "PROJECT_NAME_TO_RESOLVE".into()),
                        ("description", "PROJECT_NAME_TO_RESOLVE".into()),
                        ("lang", "en".into()),
                ]))
                .build()
                .expect("Failed to build MetaLayer"),
        )
        .layer(middleware::from_fn(template::template))
        .layer(middleware::from_fn(cache::html_middleware))
        .route("/{*wildcard}", get(handling_file::handle_file))
        .layer(CompressionLayer::new());


    // Start the server
    info!("Starting listening on port {}...", *PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", *PORT))
        .await
        .expect("Couldn't start the TcpListener of the specified port.");

    info!("Starting the server...");
    axum::serve(listener, app)
        .await
        .expect("Couldn't start the server.");
}

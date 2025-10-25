use {
    crate::{
        layouts::header,
        pages,
        shared::wini::{
            layer::MetaLayerBuilder,
            ssg::{render_routes_to_files, SsgRouter},
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
    #[cfg(any(feature = "generate-ssg", feature = "run-with-ssr"))]
    {
        let ssg_router = SsgRouter::new()
            .route("/", get(pages::hello::render));


        // The main router of the application is defined here
        let app = Router::<()>::new()
            .merge(ssg_router.into_axum_router())
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

        #[cfg(all(feature = "generate-ssg", not(feature = "run-with-ssr")))]
        {
            tokio::task::spawn(async move {
                info!("Starting the server...");
                axum::serve(listener, app)
                    .await
                    .expect("Couldn't start the server.");
            });

            render_routes_to_files().await;
        }

        #[cfg(all(feature = "run-with-ssr", not(feature = "generate-ssg")))]
        {
                info!("Starting the server...");
                axum::serve(listener, app)
                    .await
                    .expect("Couldn't start the server.");

        }
    }

    #[cfg(feature = "serve-dist")]
    {
        let app = Router::<()>::new()
            .nest_service("/", tower_http::services::ServeDir::new("dist"))
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
}

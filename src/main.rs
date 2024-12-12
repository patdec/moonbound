pub mod shared;

#[cfg(feature="ssr")]
mod ssr_imports {
    pub use actix_files::Files;
    pub use actix_web::*;
    pub use leptos::prelude::*;
    pub use leptos_actix::{generate_route_list, LeptosRoutes};

    #[actix_web::get("favicon.ico")]
    async fn favicon(
        leptos_options: actix_web::web::Data<LeptosOptions>,
    ) -> impl Responder {
        let leptos_options = leptos_options.into_inner();
        let site_root = &leptos_options.site_root;
        actix_files::NamedFile::open(format!(
            "{site_root}/favicon.ico"
        ))
    }
}


#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use self::ssr_imports::*;
    use shared::moonbound::ssr::*;
    use leptos_meta::MetaTags;
    use moonbound::app::*;

    let mut conn = db().await.expect("couldn't connect to db");
    sqlx::migrate!("./migrations")
    .run(&mut conn)
    .await
    .expect("coukld not run sqlx migration");


    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    println!("listening on http://{}", &addr);
    
    
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let routes = generate_route_list(App);
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", format!("{site_root}")))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    use leptos::prelude::*;

                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use moonbound::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

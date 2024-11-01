use actix_files::Files;
use actix_web::{App, HttpServer,web};
use leptos::get_configuration;
use leptos_actix::{generate_route_list, LeptosRoutes};
use apps::App;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    HttpServer::new( move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .service(Files::new("/pkg", format!("{}/pkg",&site_root)))
            .service(Files::new("/assets", site_root))
            .leptos_routes(leptos_options.to_owned(),routes.to_owned(),App)
            .app_data(web::Data::new(leptos_options.to_owned()))

    }).bind(&addr)?.run().await
}

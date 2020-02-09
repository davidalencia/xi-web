use actix_web::{get, App,  HttpServer, Result};
use actix_files::{self, NamedFile};

#[get("/")]
async fn index()  -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[get("/wasm/bin")]
async fn wasm_bin() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/wasm/xi_wasm_bg.wasm")?)
}

#[get("/wasm")]
async fn wasm() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/wasm/xi_wasm.js")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(wasm_bin)
            .service(wasm)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

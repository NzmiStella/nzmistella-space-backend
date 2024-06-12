use space_backend_lib::prelude::App;

#[tokio::main]
async fn main() {
    App::run().await.expect("run app failed");
}

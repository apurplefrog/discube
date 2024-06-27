mod bot;
mod handler;
mod scramblers;

#[tokio::main]
async fn main() {
    bot::connect().await;
}

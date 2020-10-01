mod discord;
mod timer;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    timer::event_loop::create_loop(1).await;
}

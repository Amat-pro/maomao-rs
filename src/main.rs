mod mongo_developer;

#[tokio::main]
async fn main() {
    println!("=======> start check");
    mongo_developer::check_size().await;
    println!("=======> done check");
}

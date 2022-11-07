use tokio::fs;

#[tokio::main]
async fn main() {
    let contents = fs::read_to_string("src/main.rs")
        .await
        .expect("Should have been able to read the file");
    println!("{contents}");
}

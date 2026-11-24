use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut hs = vec![];
    for i in 0..=1_00000 {
        hs.push(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            i
        }));
        println!("{i}");
    }
    for h in hs{
        h.await.unwrap();
    }
    println!("Finish")
}
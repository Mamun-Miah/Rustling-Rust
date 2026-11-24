use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut hs: Vec<tokio::task::JoinHandle<i32>> = vec![];
    for i in 0..=1_00000 {
         println!("{i}");
        hs.push(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
            i
        }));
        // println!("{i}");
    }
    for h in hs{
        println!("Current handle: {:?}", h);
        h.await.unwrap();
        // println!("after aysnc call {}",h);
        
    }
    println!("Finish")
}
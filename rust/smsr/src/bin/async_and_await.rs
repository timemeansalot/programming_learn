use futures::executor::block_on;

async fn hello_world() {
    println!("hello world");
}

type Song = String;

async fn learn_song() -> String {
    println!("learning song");
    String::from("song")
}

async fn sing_song(song: Song) {
    println!("sing {}", song);
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    // learn_song must run before sing_song
    let mut count: u64 = 0;
    for i in 1..=10000 {
        count += i * i;
    }
    println!("count {}", count);
    let song = learn_song().await;
    sing_song(song).await;
}

async fn aysnc_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2); // if f1 block, f2 will run. But in this example, f1 will not block
    println!("end of async_main");
}

fn main() {
    let future = hello_world();
    block_on(future);

    block_on(aysnc_main());

    println!("end of main function");
}

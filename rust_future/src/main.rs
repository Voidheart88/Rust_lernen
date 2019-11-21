use async_std::task;

fn main() {
    let child1 = task::spawn(async {
        println!("Hello, world1!");
    });
    let child2 = task::spawn(async {
        println!("Hello, world2!");
    });
    task::block_on(child2);
    task::block_on(child1);
}

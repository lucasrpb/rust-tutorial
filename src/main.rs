use tokio::time::{sleep, Duration};
mod mymodule;

struct Point {
    x: i32,
    y: i32,
}

trait PrintablePointer {
    fn print(&self);
}

trait MyTrait {
    fn do_something(&self);
}

impl PrintablePointer for Point {
    fn print(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

impl MyTrait for Point {
    fn do_something(&self) {
        println!("Doing something with Point({}, {})", self.x, self.y);
    }
}

async fn fetch_data(id: u32) -> String {
    println!("Fetching data for id: {}", id);

    // Simulate async I/O (like DB or HTTP call)
    sleep(Duration::from_secs(2)).await;

    format!("Data for id {}", id)
}

#[tokio::main]
async fn main() {
    let x = 5;
    let mut _myLong : i64 = 10;
    let mut point = Point { x: 10, y: 20 };
    let myarray = [1, 2, 3, 4, 5];

    let m1: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    let m2: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("My 2D vector is: {:?}", m1);
    print!("My 2D array is: ");
    for row in m2 {
        print!("{:?} ", row);
    }
    println!();
    println!("My array is: {:?}", myarray);
    println!("Hello, world! My number is {}", x);

    let _ = fetch_data(2).await;

    point.x = 33;
    point.print();

    let lambda1 = |x: i32, y: i32| -> i32 {
        println!("Lambda called with x: {}, y: {}", x, y);
        x + y
    };

    println!("Result of lambda: {}", lambda1(3, 4));

    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];

    for i in (0..=10).step_by(2) {
        println!("Loop iteration: {}", i);
    }

    for p in points {
        p.print();
    }

    let mut i = 0;

    while i < 10 {
        println!("While loop, x: {}", i);
        i += 1;
    }

    mymodule::example().await;

}

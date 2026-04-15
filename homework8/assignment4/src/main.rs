use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

const TERMINATION_SIGNAL: i32 = -1;

fn main()
{
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];

    for id in 1..=NUM_PRODUCERS
    {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move ||
        {
            producer(id, tx_clone, ITEM_COUNT / NUM_PRODUCERS);
        });
        producer_handles.push(handle);
    }

    for id in 1..=NUM_CONSUMERS
    {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move ||
        {
            consumer(id, rx_clone);
        });
        consumer_handles.push(handle);
    }

    for handle in producer_handles
    {
        handle.join().unwrap();
    }

    for _ in 0..NUM_CONSUMERS
    {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in consumer_handles
    {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize)
{
    let mut rng = rand::thread_rng();
    for _ in 0..item_count
    {
        let num = rng.gen_range(1..=100);
        println!("Producer {} sending: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished.", id);
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>)
{
    loop
    {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL
        {
            println!("Consumer {} got termination signal, exiting.", id);
            break;
        }

        println!("Consumer {} received: {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
}
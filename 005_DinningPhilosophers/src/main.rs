use std::thread;
use std::sync::{Mutex, Arc};

struct Table{
    forks: Vec<Mutex<()>>,
}
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher{
    fn new(name: &str, left: usize, right: usize) -> Philosopher{
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep_ms(150);
        let _right = table.forks[self.right].lock().unwrap();
        
        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

fn main(){
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler",0,1),
        Philosopher::new("Gilles Deleuze",1,2),
        Philosopher::new("Karl Marx",2,3),
        Philosopher::new("Emma Goldman",3,4),
        Philosopher::new("Michael Foucault",0,4), //the left handed philosopher
    ];

    /*
    let p1 = Philosopher::new("Judith Butler");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Marx");
    let p4 = Philosopher::new("Emma Goldman");
    let p5 = Philosopher::new("Michel Foucault");
    */
    /*
    let p1 = Philosopher { name: "Judith Butler".to_string() };
    let p2 = Philosopher { name: "Gilles".to_string() };
    let p3 = Philosopher { name: "Karl".to_string() };
    let p4 = Philosopher { name: "Emma".to_string() };
    let p5 = Philosopher { name: "Michael".to_string() };
    */
    
    /*for p in &philosophers {
        p.eat();
    }*/

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }  

}

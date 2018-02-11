
//from-tut use std::thread;
//from-tut use std::time::Duration;
//from-tut use std::sync::{Mutex, Arc};
//from-tut 
//from-tut struct Philosopher {
//from-tut     name: String,
//from-tut     left: usize,
//from-tut     right: usize,
//from-tut }
//from-tut 
//from-tut impl Philosopher {
//from-tut     fn new(name: &str, left: usize, right: usize) -> Philosopher {
//from-tut         Philosopher {
//from-tut             name: name.to_string(),
//from-tut             left: left,
//from-tut             right: right,
//from-tut         }
//from-tut     }
//from-tut 
//from-tut     fn eat(&self, table: &Table) {
//from-tut         let _left = table.forks[self.left].lock().unwrap();
//from-tut         thread::sleep(Duration::from_millis(150));
//from-tut         let _right = table.forks[self.right].lock().unwrap();
//from-tut 
//from-tut         println!("{} is eating.", self.name);
//from-tut 
//from-tut         thread::sleep(Duration::from_millis(1000));
//from-tut 
//from-tut         println!("{} is done eating.", self.name);
//from-tut     }
//from-tut }
//from-tut 
//from-tut struct Table {
//from-tut     forks: Vec<Mutex<()>>,
//from-tut }
//from-tut 
//from-tut fn main() {
//from-tut     let table = Arc::new(Table { forks: vec![
//from-tut         Mutex::new(()),
//from-tut         Mutex::new(()),
//from-tut         Mutex::new(()),
//from-tut         Mutex::new(()),
//from-tut         Mutex::new(()),
//from-tut     ]});
//from-tut 
//from-tut     let philosophers = vec![
//from-tut         Philosopher::new("Judith Butler", 0, 1),
//from-tut         Philosopher::new("Gilles Deleuze", 1, 2),
//from-tut         Philosopher::new("Karl Marx", 2, 3),
//from-tut         Philosopher::new("Emma Goldman", 3, 4),
//from-tut         Philosopher::new("Michel Foucault", 0, 4),
//from-tut     ];
//from-tut 
//from-tut     let handles: Vec<_> = philosophers.into_iter().map(|p| {
//from-tut         let table = table.clone();
//from-tut 
//from-tut         thread::spawn(move || {
//from-tut             p.eat(&table);
//from-tut         })
//from-tut     }).collect();
//from-tut 
//from-tut     for h in handles {
//from-tut         h.join().unwrap();
//from-tut     }
//from-tut }

use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher{
            name: name.to_string(),
            left: left,
            right: right,
        }
    }//End new

    fn eat(&self, table: &Table){
        let _left  = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
        
        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }//End eat
}

fn main() {

    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});
    
    // let p1 = Philosopher::new("Judith Butler"   );
    // let p2 = Philosopher::new("Gilles Deleuze"  );
    // let p3 = Philosopher::new("Karl Marx"       );
    // let p4 = Philosopher::new("Emma Goldman"    );
    // let p5 = Philosopher::new("Michael Foucault");
    let philosophers = vec![
        Philosopher::new("Judith Butler"   , 0, 1),
        Philosopher::new("Gilles Deleuze"  , 1, 2),
        Philosopher::new("Karl Marx"       , 2, 3),
        Philosopher::new("Emma Goldman"    , 3, 4),
        Philosopher::new("Michael Foucault", 0, 4),    
    ];

    
//    for p in &philosophers {
//        p.eat();
//    }

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

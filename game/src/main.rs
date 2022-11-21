use define::{Position};

extern crate core;
extern crate repo;
extern crate uuid;


fn main() {
    // let mut redis = repo::rdb::open("redis://192.168.0.206/").expect("Get redis error");
    
    // for idx in 0.. 10_000 {
    //     redis.set_value(uuid::Uuid::new_v4().to_string().as_str(), uuid::Uuid::new_v4().to_string().as_str());

    //     match idx {
    //         i if i % 100 == 0 => println!("{} of {}", i, 10_000),
    //         _ => {}
    //     }
    // }


    let pos = Position{x: 0, y: 0, z: 0};
    println!("{:?}", pos);

    Position::describe();
    

    println!("Done");
}

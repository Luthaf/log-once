#[macro_use]
extern crate log;
#[macro_use]
extern crate log_once;
#[macro_use]
extern crate lazy_static;

mod logger;

#[test]
fn info() {
    logger::init();

    for _ in 0..4 {
        info!("Here {}!", 42);
    }

    for _ in 0..4 {
        info_once!("This one is only logged once {}", 43);
    }

    for i in 0..4 {
        info_once!("This will be logged twice {}", i % 2);
    }

    let data = logger::logged_data();
    let expected =
"Here 42!
Here 42!
Here 42!
Here 42!
This one is only logged once 43
This will be logged twice 0
This will be logged twice 1
";
    assert_eq!(data, expected);
}

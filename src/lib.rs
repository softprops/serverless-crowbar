#[macro_use]
extern crate cpython;
#[macro_use]
extern crate crowbar;

lambda!(|event, _| {
    println!("invoked with {:?}", event);
    Ok(())
});

#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(PartialEq, Hash)]
pub struct Bot{}

impl Eq for Bot {
    fn assert_receiver_is_total_eq(&self) {}
}
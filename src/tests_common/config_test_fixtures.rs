use std::error::Error;

pub struct MyCustomMockError;

pub trait SystemTime where Self: Sized {
    fn now() -> Result<Self, MyCustomMockError>;
} 


pub struct MockSystemTime(pub u64);

impl SystemTime for MockSystemTime {
   fn now() -> Result<Self, MyCustomMockError> {
       Err(MyCustomMockError)
   } 
} 

impl From<MockSystemTime> for u64 {
    fn from(t: MockSystemTime) -> Self {
        t.0
    } 
} 

impl MockSystemTime {
    fn timestamp(&self) -> u64 {
        0
    } 
} 

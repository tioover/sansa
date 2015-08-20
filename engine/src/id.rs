use uuid::Uuid;



#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Id(Uuid);


impl Id {
    pub fn new() -> Id {
        Id(Uuid::new_v4())
    }
}


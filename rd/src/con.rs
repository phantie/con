use crate::*;

pub trait RefCon<T>
where
    T: RefNode,
{
    fn con_ref(&self) -> &Con<T>;
}

#[derive(Debug)]
pub struct Con<T>
where
    T: RefNode,
{
    pub targets: (Rc<RefCell<T>>, Rc<RefCell<T>>),
}

impl<T> RefCon<T> for Con<T>
where
    T: RefNode,
{
    fn con_ref(&self) -> &Con<T> {
        &self
    }
}

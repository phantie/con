use crate::*;

pub trait RefCon<A, B>
where
    A: RefNode,
    B: RefNode,
{
    fn con_ref(&self) -> &Con<A, B>;
}

#[derive(Debug)]
pub struct Con<A, B>
where
    A: RefNode,
    B: RefNode,
{
    pub targets: (Rc<RefCell<A>>, Rc<RefCell<B>>),
}

impl<A, B> RefCon<A, B> for Con<A, B>
where
    A: RefNode,
    B: RefNode,
{
    fn con_ref(&self) -> &Con<A, B> {
        &self
    }
}

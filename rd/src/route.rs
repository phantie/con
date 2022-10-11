use crate::*;

#[derive(Debug)]
pub struct Route<'con, A, B>
where
    A: RefNode,
    B: RefNode,
{
    pub con: &'con Con<A, B>,
    pub send: u32,
}

impl<'con, 'city> Route<'con, City<'city>, City<'city>> {
    pub fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, 'city, 'country> Route<'con, City<'city>, Country<'country>> {
    pub fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, 'city, 'country> Route<'con, Country<'country>, City<'city>> {
    pub fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, A, B> RefCon<A, B> for Route<'con, A, B>
where
    A: RefNode,
    B: RefNode,
{
    fn con_ref(&self) -> &'con Con<A, B> {
        &self.con
    }
}

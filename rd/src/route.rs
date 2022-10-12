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

impl<'con, 'city> Transmit for Route<'con, City<'city>, City<'city>> {
    fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, 'city, 'country> Transmit for Route<'con, City<'city>, Country<'country>> {
    fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, 'city, 'country> Transmit for Route<'con, Country<'country>, City<'city>> {
    fn transmit(&self) {
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

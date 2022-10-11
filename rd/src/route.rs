
use crate::*;

#[derive(Debug)]
pub struct Route<'con, 'city>
{
    pub con: &'con Con<City<'city>, City<'city>>,
    pub send: u32,
}

impl<'con, 'city> Route<'con, 'city>
{
    fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, 'city> RefCon<City<'city>, City<'city>> for Route<'con, 'city>
{
    fn con_ref(&self) -> &'con Con<City<'city>, City<'city>> {
        &self.con
    }
}

pub fn step_all<'a>(routes: &Vec<Route>) {
    routes.iter().for_each(Route::transmit);
}

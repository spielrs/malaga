pub type Next = fn();

pub enum Mdw<Res> {
    NEXT(()),
    RESP(Res),
}
use super::super::middlewares::helpers::MalagaMdw;

pub trait Worker<MalagaMdw>{
	fn new() -> Self;
}

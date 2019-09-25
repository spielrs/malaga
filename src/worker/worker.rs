use super::super::middlewares::helpers::MalagaMdw;
use malaga_http_utils::utils

pub trait Worker<MalagaMdw>{
	fn new(mdw: MalagaMdw) -> Self;
	fn get_request()-> Self;
	fn execute_middlewares()->Self;
	fn response_to_provider()->Self;
}

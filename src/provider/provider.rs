pub trait Provider {
    fn new (&mut self, addr: &str, path_root: &str, request: &str);
}
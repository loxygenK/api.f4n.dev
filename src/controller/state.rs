pub struct _State {
    pub status: String
}
pub type State = _State;

pub fn provide_context() -> State {
    println!("Providing state...");

    _State { status: "Hi!".to_string() }
}

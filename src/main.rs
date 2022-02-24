mod wrapper;
use crate::wrapper::flow::login;

fn main() {
    login("test@test.com".to_string(),"test".to_string());
}

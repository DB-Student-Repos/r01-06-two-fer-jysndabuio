pub fn twofer(name: &str) -> String {
    if name.is_empty(){
        format!("One for you, one for me.")
    }else {
        format!("One for {}, one for me.", name)
    }
}

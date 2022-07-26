pub fn cli_message(msg: &str) -> String {
    std::format!("```{}```", msg)
}

pub fn cli_error(msg: &str) -> String {
    std::format!("```ERROR: {}```", msg)
}

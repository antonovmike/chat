use sqlite::State;

pub fn data_base() -> Vec<String> {
    let connection = sqlite::open("chat.db").unwrap();

    let query = "SELECT * FROM users";

    let mut statement = connection.prepare(query).unwrap();

    let mut content: Vec<String> = vec![];

    while let Ok(State::Row) = statement.next() {
        content.push(format!(
            "{}: {}",
            statement.read::<String, _>("name").unwrap(),
            statement.read::<String, _>("message").unwrap()
        ));
    }

    content
}

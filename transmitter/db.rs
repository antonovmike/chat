use sqlite::{Result, State};

pub fn read_from_chat_db() -> Result<Vec<(String, String)>> {
    let connection = sqlite::open("chat.db")?;
    let query = "SELECT * FROM users ORDER BY ROWID DESC LIMIT 5";
    let mut statement = connection.prepare(query)?;

    let mut messages: Vec<(String, String)> = vec![];
    while let Ok(State::Row) = statement.next() {
        messages.push((
            statement.read::<String, _>("name")?,
            statement.read::<String, _>("message")?,
        ));
    }

    Ok(messages)
}
#[allow(unused)]
pub fn write_to_chat_db(name: &str, message: &str) -> Result<()> {
    todo!()
}

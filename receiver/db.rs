use sqlite::{Connection, Result, State};

pub fn create_table() -> Result<Connection> {
    let connection = sqlite::open("chat.db")?;
    let query = "CREATE TABLE if NOT EXISTS users (name CHARFIELD, message TEXT)";
    connection.execute(query)?;
    Ok(connection)
}
#[allow(unused)]
pub fn read_from_chat_db() -> Result<Vec<(String, String)>> {
    let connection = sqlite::open("chat.db")?;
    let query = "CREATE TABLE if NOT EXISTS users (name CHARFIELD, message TEXT)";
    connection.execute(query)?;

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

pub fn write_to_chat_db(username: &str, usermessage: &str) -> Result<()> {
    let connection = sqlite::open("chat.db")?;
    let query = format!(
        "INSERT INTO users VALUES ('{}', '{}')",
        username, usermessage
    );
    connection.execute(query)?;

    Ok(())
}

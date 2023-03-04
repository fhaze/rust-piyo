use crate::models::{Message, NewMessage, NewUser, User, UserWithMessages};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_users(conn: &mut SqliteConnection) -> Result<Vec<UserWithMessages>, DbError> {
    use crate::schema::users::dsl::users;

    let all_users = users.select(User::as_select()).load(conn)?;

    let all_messages = Message::belonging_to(&all_users)
        .select(Message::as_select())
        .load(conn)?;

    let messages_per_user = all_messages
        .grouped_by(&all_users)
        .into_iter()
        .zip(all_users)
        .map(|(messages, user)| UserWithMessages { user, messages })
        .collect::<Vec<UserWithMessages>>();

    Ok(messages_per_user)
}

pub fn find_user(
    conn: &mut SqliteConnection,
    p_id: i32,
) -> Result<Option<UserWithMessages>, DbError> {
    use crate::schema::users::dsl::users;
    let user = users.find(p_id).first::<User>(conn).optional()?;

    if let Some(user) = user {
        let messages_from_user = Message::belonging_to(&user)
            .select(Message::as_select())
            .load(conn)?;

        Ok(Some(UserWithMessages {
            user,
            messages: messages_from_user,
        }))
    } else {
        Ok(None)
    }
}

pub fn create_user(
    conn: &mut SqliteConnection,
    p_name: &str,
    p_email: &str,
) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;
    let new_user = NewUser {
        name: p_name.to_string(),
        email: p_email.to_string(),
    };
    let rows = diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("could not insert user");
    Ok(rows)
}

pub fn find_all_messages(conn: &mut SqliteConnection) -> Result<Vec<Message>, DbError> {
    use crate::schema::messages::dsl::*;
    let res = messages
        .load::<Message>(conn)
        .expect("could not retrive messages");
    Ok(res)
}

pub fn create_message(
    conn: &mut SqliteConnection,
    p_user_id: i32,
    p_msg: &str,
) -> Result<usize, DbError> {
    use crate::schema::messages::dsl::*;
    let new_message = NewMessage {
        user_id: p_user_id,
        msg: p_msg.to_string(),
    };

    let rows = diesel::insert_into(messages)
        .values(&new_message)
        .execute(conn)
        .expect("could not insert message");
    Ok(rows)
}

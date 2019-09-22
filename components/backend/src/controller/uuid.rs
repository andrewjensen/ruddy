use uuid::Uuid;

pub fn generate_uuid() -> String {
    let uuid = Uuid::new_v4().to_hyphenated();

    format!("{}", uuid)
}

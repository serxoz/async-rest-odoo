use async_odoors::odoo::{deserialize_odoo_nullable, Odoo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductTemplate {
    name: String,
    #[serde(deserialize_with = "deserialize_odoo_nullable")]
    default_code: Option<String>,
}

pub async fn get_odoo() -> Odoo {
    let odoo = Odoo::new("https://demo.odoo.com", "");
    let values = odoo.start().await.unwrap();
    Odoo::new_and_login(
        values.get("host").unwrap(),
        values.get("database").unwrap(),
        values.get("user").unwrap(),
        values.get("password").unwrap(),
    )
    .await
    .unwrap()
}

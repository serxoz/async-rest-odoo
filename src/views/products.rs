use crate::odoo::{get_odoo, ProductTemplate};
use axum::{
    response::{IntoResponse, Response},
    Json,
};

pub async fn get_products() -> Response {
    let odoo = get_odoo().await;

    let products: Vec<ProductTemplate> = odoo
        .search_read(
            "product.template",
            (),
            Some(vec!["name", "default_code"]),
            None,
            None,
        )
        .await
        .unwrap()
        .result;

    Json(products).into_response()
}

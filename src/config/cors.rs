use actix_cors::Cors;

pub fn config_cors() -> Cors {
    Cors::permissive()

    // TODO build a restricitive cors config for prod release (depend on env variable ?)
}

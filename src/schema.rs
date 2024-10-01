// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        email -> Text,
        password -> Nullable<Text>,
        oauth_provider -> Text,
        oauth_user_id -> Text,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    venue (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        #[max_length = 255]
        contactname -> Nullable<Varchar>,
        address -> Nullable<Text>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        #[max_length = 20]
        postalcode -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 20]
        latitude -> Nullable<Varchar>,
        #[max_length = 20]
        longitude -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(users, venue,);

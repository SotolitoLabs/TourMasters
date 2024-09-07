// @generated automatically by Diesel CLI.

diesel::table! {
    venue (venueid) {
        venueid -> Uuid,
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

#[macro_export]
/// Easily create Responder implemented Enums for your use cases depending on responses using this macro.
/// Sometimes there might be conflicts when using the same structs, hence, it would be important to nest your API's in a mod.
macro_rules! okapi_rocket_response {
    ($name1: tt : $(($enum_name: tt, $code: literal, $content_type: literal)),+ -> ($( $text_code: literal),*)) => {

        paste! {
            #[derive(Responder)]
            pub enum [<$name1 R>] {
                $(
                    #[response(content_type = $content_type, status = $code)]
                    $enum_name(rocket::serde::json::Json::<$name1>),
                )+
                $(
                    #[response(content_type = "application/text", status = $text_code)]
                    [<Text_ $text_code>](String),
                )*
            }

            impl OpenApiResponderInner for [<$name1 R>] {
                fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
                    let mut responses_ = Responses::default();
                    $({
                        let mut schema = schema_for!($name1);
                        add_schema_response(&mut responses_, $code, $content_type, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        add_content_response(
                            &mut responses_,
                            $text_code,
                            "application/text",
                            MediaType::default(),
                        );
                    })*
                    Ok(responses_)
                }
            }
        }
    };
    ($name1: tt : $(($enum_name: ident, $code : literal, $content_type: literal)),+ ->
    $name2: tt : $(($enum_name2: ident, $code2: literal, $content_type2: literal)),+ -> ($(/*$text_code_name: ident,*/ $text_code: literal),*)) => {
        paste! {
            #[derive(Responder)]
            pub enum [<$name1 $name2 R>] {
                $(
                    #[response(content_type = $content_type, status = $code)]
                    $enum_name(rocket::serde::json::Json::<$name1>),
                )+
                $(
                    #[response(content_type = $content_type2, status = $code2)]
                    $enum_name2(rocket::serde::json::Json::<$name2>),
                )+
                $(
                    #[response(content_type = "application/text", status = $text_code)]
                    [<Text_ $text_code>](String),
                )*
            }

            impl OpenApiResponderInner for [<$name1 $name2 R>] {
                fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
                    let mut responses_ = Responses::default();
                    $({
                        let mut schema = schema_for!($name1);
                        add_schema_response(&mut responses_, $code, $content_type,schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name2);
                        add_schema_response(&mut responses_, $code2, $content_type2, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        add_content_response(
                            &mut responses_,
                            $text_code,
                            "application/text",
                            MediaType::default(),
                        );
                    })*
                    Ok(responses_)
                }
            }
        }
    };
    ($name1: tt : $(($enum_name: ident, $code : literal, $content_type: literal)),+ ->
        $name2: tt : $(($enum_name2: ident, $code2: literal, $content_type2: literal)),+ ->
        $name3: tt : $(($enum_name3: ident, $code3: literal, $content_type3: literal)),+ ->
        ($( $text_code: literal),*)) => {


        paste! {
            #[derive(Responder)]
            pub enum [<$name1 $name2 $name3 R>] {
                $(
                    #[response(content_type = $content_type, status = $code)]
                    $enum_name(rocket::serde::json::Json::<$name1>),
                )+
                $(
                    #[response(content_type = $content_type2, status = $code2)]
                    $enum_name2(rocket::serde::json::Json::<$name2>),
                )+
                $(
                    #[response(content_type = $content_type3, status = $code3)]
                    $enum_name3(rocket::serde::json::Json::<$name3>),
                )+
                $(
                    #[response(content_type = "application/text", status = $text_code)]
                    [<Text_ $text_code>](String),
                )*
            }

            impl OpenApiResponderInner for [<$name1 $name2 $name3 R>] {
                fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
                    let mut responses_ = Responses::default();
                    $({
                        let schema = schema_for!($name1);
                        add_schema_response(&mut responses_, $code, $content_type, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let schema = schema_for!($name2);
                        add_schema_response(&mut responses_, $code2, $content_type2, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let schema = schema_for!($name3);
                        add_schema_response(&mut responses_, $code3, $content_type3, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        add_content_response(
                            &mut responses_,
                            $text_code,
                            "application/text",
                            MediaType::default(),
                        );
                    })*
                    Ok(responses_)
                }
            }
        }
    };
    ( $name1: tt : $(($enum_name: ident, $code : literal, $content_type: literal)),+ ->
    $name2: tt : $(($enum_name2: ident, $code2: literal, $content_type2: literal)),+  ->
    $name3: tt : $(($enum_name3: ident,$code3: literal, $content_type3: literal)),+ ->
    $name4: tt : $(($enum_name4: ident,$code4: literal, $content_type4: literal)),+ -> ($( $text_code: literal),*)) => {

        paste! {
            #[derive(Responder)]
            pub enum [<$name1 $name2 $name3 $name4 R>] {
                $(
                    #[response(content_type = $content_type, status = $code)]
                    $enum_name(rocket::serde::json::Json::<$name1>),
                )+
                $(
                    #[response(content_type = $content_type2, status = $code2)]
                    $enum_name2(rocket::serde::json::Json::<$name2>),
                )+
                $(
                    #[response(content_type = $content_type3, status = $code3)]
                    $enum_name3(rocket::serde::json::Json::<$name3>),
                )+
                $(
                    #[response(content_type = $content_type4, status = $code4)]
                    $enum_name4(rocket::serde::json::Json::<$name4>),
                )+
                $(
                    #[response(content_type = "application/text", status = $text_code)]
                    [<Text_ $text_code>](String)
                )*
            }

            impl OpenApiResponderInner for [<$name1 $name2 $name3 $name4 R>] {
                fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
                    let mut responses_ = Responses::default();
                    $({
                        let mut schema = schema_for!($name1);
                        add_schema_response(&mut responses_, $code, $content_type, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name2);
                        add_schema_response(&mut responses_, $code2, $content_type2, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name3);
                        add_schema_response(&mut responses_, $code3, $content_type3, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name4);
                        add_schema_response(&mut responses_, $code4, $content_type4, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        add_content_response(
                            &mut responses_,
                            $text_code,
                            "application/text",
                            MediaType::default(),
                        );
                    })*
                    Ok(responses_)
                }
            }
        }
    };
    ( $name1: tt : $(($enum_name: ident, $code : literal, $content_type: literal)),+ ->
    $name2: tt : $(($enum_name2: ident, $code2: literal, $content_type2: literal)),+  ->
    $name3: tt : $(($enum_name3: ident,$code3: literal, $content_type3: literal)),+ ->
    $name4: tt : $(($enum_name4: ident,$code4: literal, $content_type4: literal)),+ ->
    $name5: tt : $(($enum_name5: ident,$code5: literal, $content_type5: literal)),+ -> ($( $text_code: literal),*)) => {

        paste! {
            #[derive(Responder)]
            pub enum [<$name1 $name2 $name3 $name4 R>] {
                $(
                    #[response(content_type = $content_type, status = $code)]
                    $enum_name(rocket::serde::json::Json::<$name1>),
                )+
                $(
                    #[response(content_type = $content_type2, status = $code2)]
                    $enum_name2(rocket::serde::json::Json::<$name2>),
                )+
                $(
                    #[response(content_type = $content_type3, status = $code3)]
                    $enum_name3(rocket::serde::json::Json::<$name3>),
                )+
                $(
                    #[response(content_type = $content_type4, status = $code4)]
                    $enum_name4(rocket::serde::json::Json::<$name4>),
                )+
                $(
                    #[response(content_type = $content_type5, status = $code5)]
                    $enum_name5(rocket::serde::json::Json::<$name5>),
                )+
                $(
                    #[response(content_type = "application/text", status = $text_code)]
                    [<Text_ $text_code>](String)
                )*
            }

            impl OpenApiResponderInner for [<$name1 $name2 $name3 $name4 $name5 R>] {
                fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
                    let mut responses_ = Responses::default();
                    $({
                        let mut schema = schema_for!($name1);
                        add_schema_response(&mut responses_, $code, $content_type, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name2);
                        add_schema_response(&mut responses_, $code2, $content_type2, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name3);
                        add_schema_response(&mut responses_, $code3, $content_type3, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name4);
                        add_schema_response(&mut responses_, $code4, $content_type4, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        let mut schema = schema_for!($name5;)
                        add_schema_response(&mut responses_, $code5, $content_type5, schema.schema)
                            .expect("TODO: panic message");
                    })+
                    $({
                        add_content_response(
                            &mut responses_,
                            $text_code,
                            "application/text",
                            MediaType::default(),
                        );
                    })*
                    Ok(responses_)
                }
            }
        }
    };
}

// use crate::security::*;

// use ed25519_dalek::PublicKey;

// #[cfg(feature = "handler")]
// use crate::handler::InteractionHandler;
// #[cfg(feature = "handler")]
// use crate::types;

// #[cfg(feature = "handler")]
// use crate::types::interaction::{
//     Context, InteractionResponse, InteractionResponseBuilder, InteractionResponseType,
// };
// #[cfg(feature = "handler")]
// use crate::*;

// #[cfg(feature = "handler")]
// use actix_web::{http, test, web, App, HttpRequest};
// #[cfg(feature = "handler")]
// use std::sync::Mutex;

// #[cfg(feature = "handler")]
// use log::error;

// const TEST_PUB_KEY: &str = "82d8d97fe0641e68a1b0b11220f05e9ea0539a0cdc002119d4a9e9e025aba1e9";
// /*------------------------------
// SECURITY TESTS
// */
// #[test]
// // Discord interaction verification test OK 1
// fn crypto_verify_test_ok() {
//     let bytes = hex::decode(TEST_PUB_KEY).unwrap();

//     let pbk = PublicKey::from_bytes(&bytes).expect("Failed to convert public key.");

//     let res = verify_discord_message(pbk,
//         "c41278a0cf22bf8f3061756063cd7ef548a3df23d0ffc5496209aa0ad4d9593343801bf11e099f41bca1afcac2c70734eebafede3dec7aac1caa5d8fade5af0c",
//         "1616343571",
//         &String::from("{\"type\" : 1}"));

//     match res {
//         Err(ValidationError::KeyConversionError { name }) => panic!(
//             "One of the keys failed to convert to proper types! Key: {}",
//             name
//         ),
//         Err(ValidationError::InvalidSignatureError) => {
//             panic!("Unexpected invalidation of signature")
//         }
//         Ok(_) => {
//             // Good!
//         }
//     }
// }

// #[test]
// #[should_panic]
// // Discord interacton verification test invalid 1
// fn crypto_verify_test_fail() {
//     let bytes = hex::decode(TEST_PUB_KEY).unwrap();
//     let pbk = PublicKey::from_bytes(&bytes).expect("Failed to convert public key.");

//     let res = verify_discord_message(pbk,
//         "69696969696969696696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696696969",
//         "1616343571",
//         &String::from("{\"type\" : 1}"));

//     match res {
//         Err(ValidationError::KeyConversionError { name }) => panic!(
//             "One of the keys failed to convert to proper types! Key: {}",
//             name
//         ),
//         Err(ValidationError::InvalidSignatureError) => {
//             panic!("Unexpected invalidation of signature")
//         } // This is what it should be!

//         Ok(_) => {
//             // Good!
//         }
//     }
// }
// /*-------------------------------
// Discord Interactions API tests (endpoint: /api/discord/interactions)
// */
// #[cfg(feature = "handler")]
// macro_rules! interaction_app_init {
//     ($ih: ident) => {

//         test::init_service(App::new().app_data($ih.clone()).route(
//             "/api/discord/interactions",
//             web::post().to(
//                 |data: web::Data<Mutex<InteractionHandler>>, req: HttpRequest, body: String| async move {
//                     data.lock().unwrap().interaction(req, body).await
//                 },
//             ),
//         ))
//         .await;
//     };
// }
// #[cfg(all(feature = "handler", not(feature = "extended-handler")))]
// macro_rules! init_handler {
//     () => {
//         InteractionHandler::new(0, TEST_PUB_KEY, None)
//     };
// }

// #[cfg(feature = "extended-handler")]
// macro_rules! init_handler {
//     () => {
//         InteractionHandler::new(0, TEST_PUB_KEY, Some(&String::new()))
//     };
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Request with bad content with no Content-Type header present
// // Expected result: Return 400 without panicking
// async fn interactions_no_content_type_header_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .set_payload("This is some malformed text { the system : can't really handle }")
//         .to_request();

//     let res: types::MessageError = test::read_response_json(&mut app, req).await;

//     assert_eq!(res.message, "Bad Content-Type");
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Request with bad content with no Content-Type header present
// // Expected result: Return 400 without panicking
// async fn interactions_bad_content_type_header_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "plain/text"))
//         .set_payload("This is some malformed text { the system : can't really handle }")
//         .to_request();

//     let res: types::MessageError = test::read_response_json(&mut app, req).await;

//     assert_eq!(res.message, "Bad Content-Type");
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Request with missing X-Signature-Ed25519 Header
// // Expected result: Return 400 without panicking
// async fn interactions_no_signature_header_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Timestamp", "1229349"))
//         .set_payload("This is some malformed text { the system : can't really handle }")
//         .to_request();

//     let res: types::MessageError = test::read_response_json(&mut app, req).await;

//     assert_eq!(res.message, "Bad signature data");
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Request with missing X-Signature-Timestamp Header
// // Expected result: Return 400 without panicking
// async fn interactions_no_timestamp_header_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "69696969696969696696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696696969"))
//         .set_payload("This is some malformed text { the system : can't really handle }")
//         .to_request();

//     let res: types::MessageError = test::read_response_json(&mut app, req).await;

//     assert_eq!(res.message, "Bad signature data");
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Request with missing a signature that is too short (< 512 bits)
// // Expected result: Return 400 without panicking
// async fn interactions_bad_signature_length_short_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(
//             ("X-Signature-Ed25519",
//             "69696969696969696696969696969696969696969696969696969696969696969"),
//         )
//         .set_payload("This is some malformed text { the system : can't really handle }")
//         .to_request();

//     let res: types::MessageError = test::read_response_json(&mut app, req).await;

//     assert_eq!(res.message, "Bad signature data");
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Request with missing a signature that is too long (> 512 bits)
// // Expected result: Return 400 without panicking
// async fn interactions_bad_signature_length_too_long_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "6969696969696969669696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969669696969696969696969696696969696969696969696969696969696969696969"))
//         .set_payload("This is some malformed text { the system : can't really handle }")
//         .to_request();

//     let res: types::MessageError = test::read_response_json(&mut app, req).await;

//     assert_eq!(res.message, "Bad signature data");
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Normal ping request
// // Expected result: Return 200 with payload
// async fn interactions_ping_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "c41278a0cf22bf8f3061756063cd7ef548a3df23d0ffc5496209aa0ad4d9593343801bf11e099f41bca1afcac2c70734eebafede3dec7aac1caa5d8fade5af0c"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("{\"type\" : 1}")
//         .to_request();

//     let res: types::interaction::InteractionResponse =
//         test::read_response_json(&mut app, req).await;

//     assert_eq!(
//         res.r#type,
//         types::interaction::InteractionResponseType::Pong
//     );
// }

// #[cfg(feature = "handler")]
// #[actix_rt::test]
// // Bad content but OK signature test
// // Expected result: Return 400 with error, don't panic
// async fn interactions_bad_body_test() {
//     let ih = init_handler!();

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "51c5defa19cc2471a361c00c87a7f380d9e9d6cd21f05b65d3c223aac0b7d258277a09d0a016108e0be1338d985ed4ce0dae55e5ac93db5957a37ce31d007505"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("this is some malformed {\"data\" : cant handle}")
//         .to_request();

//     let res = test::call_service(&mut app, req).await;

//     assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
// }
// #[cfg(feature = "handler")]
// #[allow(unused_must_use)]
// #[slash_command]
// async fn normal_handle_test(ctx: Context) -> InteractionResponse {
//     return ctx.respond().content("TEST").finish();
// }
// #[cfg(feature = "handler")]
// #[allow(unused_must_use)]
// #[slash_command]
// async fn normal_handle_value_test(ctx: Context) -> InteractionResponse {
//     let response = ctx.respond().content("TEST").finish();
//     return response;
// }
// #[cfg(feature = "handler")]
// #[allow(unused_must_use)]
// #[slash_command]
// async fn normal_handle_direct_test(_ctx: Context) -> InteractionResponse {
//     return InteractionResponseBuilder::default()
//         .content("TEST")
//         .finish();
// }
// #[cfg(feature = "handler")]
// #[actix_rt::test]
// async fn interactions_normal_handle_test() {
//     let mut ih = init_handler!();

//     ih.add_global_command("test", normal_handle_test);

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "a27ed2fd0e91da58667bec63d14406e5274a0427edad9530b7d95e9d2b0fc4ee17f74e8a6bd3acd6623a05f1bde9e598fa37f3eedfe479da0a00da7827595e0b"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("{\"type\":2,\"token\":\"awQabcabc\",\"member\":{\"user\":{\"id\":\"317209107000066050\",\"username\":\"C0der\",\"avatar\":\"a_d5efa99b3eeaa7dd43acca82f5692432\",\"discriminator\":\"1337\",\"public_flags\":131141},\"roles\":[],\"premium_since\":null,\"permissions\":\"2147483647\",\"pending\":false,\"nick\":null,\"mute\":false,\"joined_at\":\"2017-03-13T19:19:14.040000+00:00\",\"is_pending\":false,\"deaf\":false},\"id\":\"786008729715212338\",\"guild_id\":\"290926798626357999\",\"data\":{\"name\":\"test\",\"id\":\"771825006014889984\"},\"channel_id\":\"645027906669510667\"}")
//         .to_request();

//     let res: types::interaction::InteractionResponse =
//         test::read_response_json(&mut app, req).await;

//     let expected_data = InteractionResponseBuilder::default()
//         .content("TEST")
//         .finish();

//     //let expected_res = HttpResponse::build(StatusCode::OK).json(expected_data);

//     assert_eq!(res, expected_data);
// }
// #[cfg(feature = "handler")]
// #[actix_rt::test]
// async fn interactions_normal_from_value_handle_test() {
//     let mut ih = init_handler!();

//     ih.add_global_command("test", normal_handle_direct_test);

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "a27ed2fd0e91da58667bec63d14406e5274a0427edad9530b7d95e9d2b0fc4ee17f74e8a6bd3acd6623a05f1bde9e598fa37f3eedfe479da0a00da7827595e0b"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("{\"type\":2,\"token\":\"awQabcabc\",\"member\":{\"user\":{\"id\":\"317209107000066050\",\"username\":\"C0der\",\"avatar\":\"a_d5efa99b3eeaa7dd43acca82f5692432\",\"discriminator\":\"1337\",\"public_flags\":131141},\"roles\":[],\"premium_since\":null,\"permissions\":\"2147483647\",\"pending\":false,\"nick\":null,\"mute\":false,\"joined_at\":\"2017-03-13T19:19:14.040000+00:00\",\"is_pending\":false,\"deaf\":false},\"id\":\"786008729715212338\",\"guild_id\":\"290926798626357999\",\"data\":{\"name\":\"test\",\"id\":\"771825006014889984\"},\"channel_id\":\"645027906669510667\"}")
//         .to_request();

//     let res: types::interaction::InteractionResponse =
//         test::read_response_json(&mut app, req).await;

//     let expected_data = InteractionResponseBuilder::default()
//         .content("TEST")
//         .finish();

//     //let expected_res = HttpResponse::build(StatusCode::OK).json(expected_data);

//     assert_eq!(res, expected_data);
// }
// #[cfg(feature = "handler")]
// #[actix_rt::test]
// async fn interactions_normal_from_direct_call_handle_test() {
//     let mut ih = init_handler!();

//     ih.add_global_command("test", normal_handle_value_test);

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "a27ed2fd0e91da58667bec63d14406e5274a0427edad9530b7d95e9d2b0fc4ee17f74e8a6bd3acd6623a05f1bde9e598fa37f3eedfe479da0a00da7827595e0b"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("{\"type\":2,\"token\":\"awQabcabc\",\"member\":{\"user\":{\"id\":\"317209107000066050\",\"username\":\"C0der\",\"avatar\":\"a_d5efa99b3eeaa7dd43acca82f5692432\",\"discriminator\":\"1337\",\"public_flags\":131141},\"roles\":[],\"premium_since\":null,\"permissions\":\"2147483647\",\"pending\":false,\"nick\":null,\"mute\":false,\"joined_at\":\"2017-03-13T19:19:14.040000+00:00\",\"is_pending\":false,\"deaf\":false},\"id\":\"786008729715212338\",\"guild_id\":\"290926798626357999\",\"data\":{\"name\":\"test\",\"id\":\"771825006014889984\"},\"channel_id\":\"645027906669510667\"}")
//         .to_request();

//     let res: types::interaction::InteractionResponse =
//         test::read_response_json(&mut app, req).await;

//     let expected_data = InteractionResponseBuilder::default()
//         .content("TEST")
//         .finish();

//     //let expected_res = HttpResponse::build(StatusCode::OK).json(expected_data);

//     assert_eq!(res, expected_data);
// }

// #[cfg(feature = "handler")]
// use crate::types::interaction::WebhookMessage;
// #[cfg(feature = "handler")]
// #[slash_command_test]
// #[defer]
// async fn deffered_handle_test(ctx: Context) -> InteractionResponse {
//     return ctx.respond().content("TEST").finish();
// }
// #[cfg(feature = "handler")]
// #[slash_command_test]
// #[defer]
// async fn deffered_handle_value_test(ctx: Context) -> InteractionResponse {
//     let response = ctx.respond().content("TEST").finish();
//     return response;
// }
// #[cfg(feature = "handler")]
// #[slash_command_test]
// #[defer]
// async fn deffered_handle_direct_test(_ctx: Context) -> InteractionResponse {
//     return InteractionResponseBuilder::default()
//         .content("TEST")
//         .finish();
// }
// #[cfg(feature = "handler")]
// #[actix_rt::test]
// async fn interactions_deffered_handle_test() {
//     let mut ih = init_handler!();

//     ih.add_global_command("test", deffered_handle_test);

//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "a27ed2fd0e91da58667bec63d14406e5274a0427edad9530b7d95e9d2b0fc4ee17f74e8a6bd3acd6623a05f1bde9e598fa37f3eedfe479da0a00da7827595e0b"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("{\"type\":2,\"token\":\"awQabcabc\",\"member\":{\"user\":{\"id\":\"317209107000066050\",\"username\":\"C0der\",\"avatar\":\"a_d5efa99b3eeaa7dd43acca82f5692432\",\"discriminator\":\"1337\",\"public_flags\":131141},\"roles\":[],\"premium_since\":null,\"permissions\":\"2147483647\",\"pending\":false,\"nick\":null,\"mute\":false,\"joined_at\":\"2017-03-13T19:19:14.040000+00:00\",\"is_pending\":false,\"deaf\":false},\"id\":\"786008729715212338\",\"guild_id\":\"290926798626357999\",\"data\":{\"name\":\"test\",\"id\":\"771825006014889984\"},\"channel_id\":\"645027906669510667\"}")
//         .to_request();

//     let res: types::interaction::InteractionResponse =
//         test::read_response_json(&mut app, req).await;

//     let expected_data = InteractionResponse {
//         r#type: InteractionResponseType::DefferedChannelMessageWithSource,
//         data: None,
//     };

//     //let expected_res = HttpResponse::build(StatusCode::OK).json(expected_data);

//     assert_eq!(res, expected_data);
// }
// #[cfg(feature = "handler")]
// #[actix_rt::test]
// async fn interactions_deffered_from_value_handle_test() {
//     let mut ih = init_handler!();

//     ih.add_global_command("test", deffered_handle_value_test);
//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "a27ed2fd0e91da58667bec63d14406e5274a0427edad9530b7d95e9d2b0fc4ee17f74e8a6bd3acd6623a05f1bde9e598fa37f3eedfe479da0a00da7827595e0b"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("{\"type\":2,\"token\":\"awQabcabc\",\"member\":{\"user\":{\"id\":\"317209107000066050\",\"username\":\"C0der\",\"avatar\":\"a_d5efa99b3eeaa7dd43acca82f5692432\",\"discriminator\":\"1337\",\"public_flags\":131141},\"roles\":[],\"premium_since\":null,\"permissions\":\"2147483647\",\"pending\":false,\"nick\":null,\"mute\":false,\"joined_at\":\"2017-03-13T19:19:14.040000+00:00\",\"is_pending\":false,\"deaf\":false},\"id\":\"786008729715212338\",\"guild_id\":\"290926798626357999\",\"data\":{\"name\":\"test\",\"id\":\"771825006014889984\"},\"channel_id\":\"645027906669510667\"}")
//         .to_request();

//     let res: types::interaction::InteractionResponse =
//         test::read_response_json(&mut app, req).await;

//     let expected_data = InteractionResponse {
//         r#type: InteractionResponseType::DefferedChannelMessageWithSource,
//         data: None,
//     };

//     //let expected_res = HttpResponse::build(StatusCode::OK).json(expected_data);

//     assert_eq!(res, expected_data);
// }
// #[cfg(feature = "handler")]
// #[actix_rt::test]
// async fn interactions_deffered_from_direct_value_handle_test() {
//     let mut ih = init_handler!();

//     ih.add_global_command("test", deffered_handle_direct_test);
//     let data = web::Data::new(Mutex::new(ih));
//     let mut app = interaction_app_init!(data);

//     let req = test::TestRequest::post()
//         .uri("/api/discord/interactions")
//         .append_header(("Content-Type", "application/json"))
//         .append_header(("X-Signature-Ed25519", "a27ed2fd0e91da58667bec63d14406e5274a0427edad9530b7d95e9d2b0fc4ee17f74e8a6bd3acd6623a05f1bde9e598fa37f3eedfe479da0a00da7827595e0b"))
//         .append_header(("X-Signature-Timestamp", "1616343571"))
//         .set_payload("{\"type\":2,\"token\":\"awQabcabc\",\"member\":{\"user\":{\"id\":\"317209107000066050\",\"username\":\"C0der\",\"avatar\":\"a_d5efa99b3eeaa7dd43acca82f5692432\",\"discriminator\":\"1337\",\"public_flags\":131141},\"roles\":[],\"premium_since\":null,\"permissions\":\"2147483647\",\"pending\":false,\"nick\":null,\"mute\":false,\"joined_at\":\"2017-03-13T19:19:14.040000+00:00\",\"is_pending\":false,\"deaf\":false},\"id\":\"786008729715212338\",\"guild_id\":\"290926798626357999\",\"data\":{\"name\":\"test\",\"id\":\"771825006014889984\"},\"channel_id\":\"645027906669510667\"}")
//         .to_request();

//     let res: types::interaction::InteractionResponse =
//         test::read_response_json(&mut app, req).await;

//     let expected_data = InteractionResponse {
//         r#type: InteractionResponseType::DefferedChannelMessageWithSource,
//         data: None,
//     };

//     //let expected_res = HttpResponse::build(StatusCode::OK).json(expected_data);

//     assert_eq!(res, expected_data);
// }

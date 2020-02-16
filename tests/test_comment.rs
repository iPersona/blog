extern crate blog;

use blog::models::comment::NewComments;
use uuid::Uuid;

#[test]
fn test_mention_uuid_parser() {
    let user_id = Uuid::parse_str("81348f13-b4e9-44ed-a01b-8b02dab42050").unwrap();
    let content = r##"
    > > > aodfjoadfoajifaojfaij faifjadifjpaijdfpaij a fpaifaipfi apf apfpai fpai fai fai pfiapif ad fapidf pai dfadfpai fpa pfi apdf api fpa fpa pf apf apd fpa fpa pfia dpf apdf apf ap fpa psf ap fap fpa dpfa pf apdfpa
    > > > adff
    > > > ad [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42050)
    > > > f [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42051)
    > > > adf
    > > > a
    > > > df
    > > aaofai af [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42052)
    > > aofjaof
    > aofoajiodf
    > fiaodfjoaf

    this is a nice idea!

    [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42050) this is a nice idea!
    [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42053) this is a nice idea!

    this is another user name [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42054)
    "##;

    let exp = [
        Uuid::parse_str("81348f13-b4e9-44ed-a01b-8b02dab42053").unwrap(),
        Uuid::parse_str("81348f13-b4e9-44ed-a01b-8b02dab42054").unwrap(),
    ]
    .to_vec();
    let res = NewComments::parse_mentioned_users(&user_id, content);
    match res {
        Ok(u) => assert_eq!(u, Some(exp)),
        Err(_) => panic!(format!("assert failed!")),
    }
}

#[test]
fn test_mention_uuid_parser_none() {
    let user_id = Uuid::parse_str("81348f13-b4e9-44ed-a01b-8b02dab42050").unwrap();
    let content = r##"
        [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42050) this is a nice idea!
        this is another user name [@user-1-nickname](/#/user/81348f13-b4e9-44ed-a01b-8b02dab42050)
    "##;
    let res = NewComments::parse_mentioned_users(&user_id, content);
    match res {
        Ok(u) => assert_eq!(u, None),
        Err(_) => panic!(format!("assert failed!")),
    }
}

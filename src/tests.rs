use ::{Resolver, Class, RecordType, Section};
use ::record::MX;

#[test]
fn basic_test_query() {
    let mut resolver = Resolver::new().unwrap();
    let mut response = resolver.query(b"gmail.com", Class::IN, RecordType::MX).unwrap();

    // Verify that some of the default options came back to us
    let flags = response.get_flags();
    assert_eq!(flags.question_response(), true);
    assert_eq!(flags.recursion_desired(), true);

    // Verify that the question section has something in it
    assert!(response.get_section_count(Section::Question) > 0);

    // Verify that the answer section has something in it
    assert!(response.get_section_count(Section::Answer) > 0);

    let mut count: usize = 0;
    for answer in response.answers::<MX>() {
        count += 1;
        println!("{:?}", answer);
    }

    // Verify that the iterator made it through all of the answers
    assert_eq!(response.get_section_count(Section::Answer), count);
}

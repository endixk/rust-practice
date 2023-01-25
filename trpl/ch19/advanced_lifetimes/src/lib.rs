pub struct Context<'s>(&'s str);

pub struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's: 'c> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

pub fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

pub struct Ref<'a, T: 'a>(&'a T);
pub struct StaticRef<T: 'static>(&'static T);

#[test]
fn test_parse_context() {
    let context = Context("abc");
    assert_eq!(parse_context(context), Err("bc"));
}

#[test]
fn test_ref() {
    let x = 42;
    let r = Ref(&x);
    assert_eq!(r.0, &42);
}

pub const STATIC_STRING: &str = "abc";
pub const ANOTHER_STATIC_STRING: &'static str = "def";

#[test]
fn test_static_ref() {
    let r = StaticRef(&STATIC_STRING);
    assert_eq!(r.0, &"abc");

    let r = StaticRef(&ANOTHER_STATIC_STRING);
    assert_eq!(r.0, &"def");
}
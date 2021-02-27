use tui::text::Spans;

pub(crate) fn challenge1_pans_get<'a>() -> Vec<Spans<'a>> {
    vec![
    Spans::from(
        "Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time."),

    Spans::from("An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor."),

    Spans::from("The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors."),

    Spans::from("For example:"),
    Spans::from("\n"),

    Spans::from("(()) and ()() both result in floor 0."),
    Spans::from("((( and (()(()( both result in floor 3."),
    Spans::from("))((((( also results in floor 3."),
    Spans::from("()) and ))( both result in floor -1 (the first basement level)."),
    Spans::from("))) and )())()) both result in floor -3."),

    Spans::from("To what floor do the instructions take Santa?"),
    ]
}

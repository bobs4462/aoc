use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

pub(crate) fn welcome_spans_get<'a>() -> Vec<Spans<'a>> {
    vec![Spans::from(vec![
            Span::styled("Advent of Code", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in ", Style::default()),
            Span::styled("any[1]", Style::default().fg(Color::Green)),
            Span::styled(" programming language you like. People use them as a ", Style::default()),
            Span::styled("speed contest[2]", Style::default().fg(Color::Green)),
            Span::styled(", ", Style::default()),
            Span::styled("interview prep[3]", Style::default().fg(Color::Green)),
            Span::styled(", ", Style::default()),
            Span::styled("company training[4]", Style::default().fg(Color::Green)),
            Span::styled(", ", Style::default()),
            Span::styled("university coursework[5]", Style::default().fg(Color::Green)),
            Span::styled(", ", Style::default()),
            Span::styled("practice problems[6]", Style::default().fg(Color::Green)),
            Span::styled(", or to ", Style::default()),
            Span::styled("challenge each other[7].", Style::default().fg(Color::Green))]),
        Spans::from("\n"),
        Spans::from(vec![Span::styled("You don't need a computer science background to participate - just a little programming knowledge and ", Style::default()),
            Span::styled("some problem solving skills", Style::default()), 
            Span::styled(" will get you pretty far. Nor do you need a fancy computer; every problem has a solution that completes in at most 15 seconds on ten-year-old hardware.", Style::default())]),
        Spans::from("\n"),
        Spans::from(vec![
            Span::styled("If you'd like to support Advent of Code, you can do so indirectly by helping to sharing it with others, or directly via ", Style::default()),
            Span::styled("PayPal or Coinbase[8].", Style::default().fg(Color::Green))]),
        Spans::from("\n"),
        Spans::from("Advent of Code is a registered trademark in the United States."),
        Spans::from("\n"),
        Spans::from("\n"),
        Spans::from(
            Span::styled("1. https://github.com/search?q=advent+of+code", Style::default().fg(Color::Green))),
        Spans::from(
            Span::styled("2. https://adventofcode.com/leaderboard", Style::default().fg(Color::Green))),
        Spans::from(
            Span::styled("3. https://y3l2n.com/2018/05/09/interview-prep-advent-of-code/", Style::default().fg(Color::Green))),
        Spans::from(
            Span::styled("4. https://twitter.com/pgoultiaev/status/950805811583963137", Style::default().fg(Color::Green))),
        Spans::from(
            Span::styled("5. https://gitlab.com/imhoffman/fa19b4-mat3006/wikis/home", Style::default().fg(Color::Green))),
        Spans::from(
            Span::styled("6. https://twitter.com/mrdanielklein/status/936267621468483584", Style::default().fg(Color::Green))),
        Spans::from(
            Span::styled("7. https://www.reddit.com/r/adventofcode/search?q=flair%3Aupping&restrict_sr=on", Style::default().fg(Color::Green))),
        Spans::from(
            Span::styled("8. https://adventofcode.com/2020/support", Style::default().fg(Color::Green))),
        Spans::from("\n"),
        Spans::from("\n"),
        Spans::from(
            Span::styled("Choose a year to continue...", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        ]
}

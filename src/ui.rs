use std::time::Duration;

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};

use crate::App;
use crate::exercises::EXERCISES;

pub const SPINNER: [&str; 4] = ["|", "/", "-", "\\"];

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(1)])
        .split(f.area());
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Min(20)])
        .split(chunks[0]);

    draw_list(f, app, panes[0]);
    draw_subject(f, app, panes[1]);
    draw_status(f, app, chunks[1]);
    draw_popup(f, app);
}

fn draw_list(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = EXERCISES
        .iter()
        .enumerate()
        .map(|(i, ex)| {
            let score = app.scores[i];
            let color = match score {
                2 => Color::Green,
                1 => Color::Yellow,
                _ => Color::Red,
            };
            ListItem::new(Line::from(vec![
                Span::raw(format!(" {:<12} ", ex.id)),
                Span::styled(format!("[{score}/2]"), Style::default().fg(color)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Exercises "))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.selected));
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_subject(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let ex = &EXERCISES[app.selected];
    let mut lines = vec![
        Line::from(Span::styled(
            ex.title,
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(ex.description),
        Line::from(""),
        Line::from(Span::styled(
            "Prototype:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  {}", ex.prototype),
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            format!("  Edit:  rendu/{}.c  (use any editor you like)", ex.id),
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
    ];

    if let Some(rem) = app.cooldown_remaining() {
        lines.push(Line::from(Span::styled(
            format!("  Next grade in {}s", seconds(rem)),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }

    if app.checking {
        let frame = SPINNER[app.tick % SPINNER.len()];
        lines.push(Line::from(Span::styled(
            format!("{frame}  Checking..."),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    } else if app.result_for == Some(app.selected)
        && let Some(r) = &app.result
    {
        lines.push(Line::from(Span::styled(
            "Last grading:",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        lines.push(status_line(r.compile_ok, "Compile"));
        if !r.compile_ok {
            push_indented(&mut lines, &r.compiler_stderr, Color::Red, 4);
        }
        lines.push(status_line(r.output_ok, "Output"));
        if !r.output_ok {
            lines.push(Line::from(vec![
                Span::raw("  expected: "),
                Span::styled(escape(&r.expected), Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  got:      "),
                Span::styled(escape(&r.got), Style::default().fg(Color::Red)),
            ]));
            if let Some(err) = &r.run_error {
                push_indented(&mut lines, err, Color::Red, 4);
            }
        }
    }

    let para = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(" Subject "))
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

fn draw_status(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let total: u32 = app.scores.iter().map(|&s| u32::from(s)).sum();
    let keys = if app.checking {
        " Checking... please wait "
    } else {
        " up/down j/k select  G grade  Q quit "
    };
    let score = format!(" Score: {total}/10 ");
    let mut spans = vec![
        Span::styled(keys, Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" "),
    ];
    if let Some(rem) = app.cooldown_remaining() {
        spans.push(Span::styled(
            format!(" Retry in {}s ", seconds(rem)),
            Style::default().fg(Color::Black).bg(Color::Red),
        ));
        spans.push(Span::raw(" "));
    }
    spans.push(Span::styled(
        score,
        Style::default().fg(Color::Black).bg(if total == 10 {
            Color::Green
        } else {
            Color::Yellow
        }),
    ));
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn seconds(rem: Duration) -> u64 {
    rem.as_secs() + u64::from(rem.subsec_nanos() > 0)
}

fn draw_popup(f: &mut Frame, app: &App) {
    let Some(popup) = &app.popup else {
        return;
    };
    let area = centered_rect(f.area(), 40, 5);
    f.render_widget(Clear, area);
    let (text, color) = if popup.ok {
        (">> SUCCESS <<", Color::Green)
    } else {
        (">> FAILED <<", Color::Red)
    };
    let banner = Paragraph::new(Span::styled(
        text,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    let middle = Rect {
        y: area.y + area.height / 2,
        height: 1,
        ..area
    };
    f.render_widget(banner, middle);
}

fn centered_rect(area: Rect, w: u16, h: u16) -> Rect {
    let w = w.min(area.width);
    let h = h.min(area.height);
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height - h) / 2,
        width: w,
        height: h,
    }
}

fn status_line(ok: bool, label: &str) -> Line<'static> {
    let (text, color) = if ok {
        (format!("{label}: OK  +1"), Color::Green)
    } else {
        (format!("{label}: FAIL  +0"), Color::Red)
    };
    Line::from(Span::styled(text, Style::default().fg(color)))
}

fn push_indented(lines: &mut Vec<Line<'static>>, block: &str, color: Color, max: usize) {
    for l in block.lines().take(max) {
        lines.push(Line::from(Span::styled(
            format!("    {l}"),
            Style::default().fg(color),
        )));
    }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centered_rect_centers_and_clamps() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 40,
        };
        let r = centered_rect(area, 40, 5);
        assert_eq!((r.x, r.y, r.width, r.height), (30, 17, 40, 5));

        let small = Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 3,
        };
        let r = centered_rect(small, 40, 5);
        assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 20, 3));

        let off = Rect {
            x: 10,
            y: 5,
            width: 30,
            height: 10,
        };
        let r = centered_rect(off, 10, 4);
        assert_eq!((r.x, r.y, r.width, r.height), (20, 8, 10, 4));
    }
}

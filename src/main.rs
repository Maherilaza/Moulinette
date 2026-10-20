mod exercises;
mod grade;
mod ui;

use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use exercises::EXERCISES;
use grade::GradeResult;

const SCORE_FILE: &str = ".moulinette_score";
const RENDU_DIR: &str = "rendu";
const CHECK_DELAY: Duration = Duration::from_secs(2);
const FRAME: Duration = Duration::from_millis(80);
const POPUP_DURATION: Duration = Duration::from_secs(3);

pub struct Popup {
    pub ok: bool,
    pub until: Instant,
}

pub struct App {
    pub selected: usize,
    pub scores: Vec<u8>,
    pub result: Option<GradeResult>,
    pub result_for: Option<usize>,
    pub checking: bool,
    pub tick: usize,
    pub popup: Option<Popup>,
}

impl App {
    fn next(&mut self) {
        self.selected = (self.selected + 1) % EXERCISES.len();
    }

    fn prev(&mut self) {
        self.selected = (self.selected + EXERCISES.len() - 1) % EXERCISES.len();
    }
}

fn main() -> io::Result<()> {
    let rendu = PathBuf::from(RENDU_DIR);
    let build = rendu.join(".build");
    std::fs::create_dir_all(&build)?;

    // rendu/ files are created by the student; missing/wrong names fail at compile time.

    let ids: Vec<&str> = EXERCISES.iter().map(|e| e.id).collect();
    let score_path = PathBuf::from(SCORE_FILE);
    let mut app = App {
        selected: 0,
        scores: grade::load_scores(&score_path, &ids),
        result: None,
        result_for: None,
        checking: false,
        tick: 0,
        popup: None,
    };

    let mut terminal = ratatui::init();
    let outcome = run_app(&mut terminal, &mut app, &rendu, &build, &score_path, &ids);
    ratatui::restore();
    outcome
}

fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
    rendu: &Path,
    build: &Path,
    score_path: &Path,
    ids: &[&str],
) -> io::Result<()> {
    loop {
        if app
            .popup
            .as_ref()
            .is_some_and(|p| Instant::now() >= p.until)
        {
            app.popup = None;
        }
        terminal.draw(|f| ui::draw(f, app))?;

        if app.popup.is_some() && !event::poll(FRAME)? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        // Any key closes the result banner, then acts as usual.
        app.popup = None;

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(()),
            KeyCode::Down | KeyCode::Char('j') => app.next(),
            KeyCode::Up | KeyCode::Char('k') => app.prev(),
            KeyCode::Char('g') | KeyCode::Char('G') => {
                grade_with_suspense(terminal, app, rendu, build, score_path, ids)?;
            }
            _ => {}
        }
    }
}

fn grade_with_suspense(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
    rendu: &Path,
    build: &Path,
    score_path: &Path,
    ids: &[&str],
) -> io::Result<()> {
    let ex = &EXERCISES[app.selected];
    app.checking = true;
    app.result = None;
    app.result_for = Some(app.selected);
    terminal.draw(|f| ui::draw(f, app))?;

    let result = grade::grade(ex, rendu, build)?;

    let start = Instant::now();
    while start.elapsed() < CHECK_DELAY {
        app.tick += 1;
        terminal.draw(|f| ui::draw(f, app))?;
        if event::poll(FRAME)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && matches!(
                key.code,
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc
            )
        {
            return Ok(());
        }
    }

    app.checking = false;
    let current = result.points();
    app.scores[app.selected] = grade::best_of(app.scores[app.selected], current);

    app.popup = Some(Popup {
        ok: result.output_ok,
        until: Instant::now() + POPUP_DURATION,
    });
    app.result = Some(result);
    grade::save_scores(score_path, ids, &app.scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_wraps_both_ways() {
        let mut app = App {
            selected: 0,
            scores: vec![0; EXERCISES.len()],
            result: None,
            result_for: None,
            checking: false,
            tick: 0,
            popup: None,
        };
        app.prev();
        assert_eq!(app.selected, EXERCISES.len() - 1);
        app.next();
        assert_eq!(app.selected, 0);
    }
}

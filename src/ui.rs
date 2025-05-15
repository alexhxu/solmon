use std::{
    collections::VecDeque,
    error::Error,
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Sparkline, Table},
    Terminal,
};
use tokio::time::sleep;

use crate::rpc::{get_block_production, get_performance_samples};

// App State

#[derive(Debug, Default, Clone)]
pub struct ValidatorStat {
    pub identity: String,
    pub assigned: u64,
    pub produced: u64,
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub tps: u64,
    pub slot: u64,
    pub tps_history: VecDeque<u64>,
    pub validators: Vec<ValidatorStat>,
}

// Main TUI Runner

pub async fn run_tui(initial_tps: u64, initial_slot: u64) -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(AppState {
        tps: initial_tps,
        slot: initial_slot,
        tps_history: VecDeque::from(vec![initial_tps]),
        validators: vec![],
    }));

    // Polling task
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            if let Ok(samples) = get_performance_samples().await {
                if let Some(sample) = samples.first() {
                    let tps = sample.num_transactions / sample.sample_period_secs;
                    let slot = sample.slot;

                    let mut s = state_clone.lock().unwrap();
                    s.tps = tps;
                    s.slot = slot;
                    s.tps_history.push_back(tps);
                    if s.tps_history.len() > 30 {
                        s.tps_history.pop_front();
                    }
                }
            }

            if let Ok(prod) = get_block_production().await {
                let mut vstats = prod
                    .by_identity
                    .iter()
                    .map(|(identity, stats)| ValidatorStat {
                        identity: identity.clone(),
                        assigned: stats.assigned,
                        produced: stats.produced,
                    })
                    .collect::<Vec<_>>();
                vstats.sort_by_key(|v| std::cmp::Reverse(v.produced));
                vstats.truncate(5);

                let mut s = state_clone.lock().unwrap();
                s.validators = vstats;
            }

            sleep(Duration::from_secs(2)).await;
        }
    });

    // TUI setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let snapshot = state.lock().unwrap().clone();

        terminal.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(5),
                    Constraint::Min(8),
                ])
                .split(size);

            let tps_widget = Paragraph::new(format!("TPS: {}", snapshot.tps))
                .block(Block::default().title("TPS (Current)").borders(Borders::ALL));
            f.render_widget(tps_widget, layout[0]);

            let slot_widget = Paragraph::new(format!("Slot: {}", snapshot.slot))
                .block(Block::default().title("Slot").borders(Borders::ALL));
            f.render_widget(slot_widget, layout[1]);

            let tps_points: Vec<u64> = snapshot.tps_history.iter().copied().collect();

            let sparkline = Sparkline::default()
                .block(Block::default().title("TPS History").borders(Borders::ALL))
                .data(&tps_points)
                .style(Style::default().fg(Color::Green));
            f.render_widget(sparkline, layout[2]);

            let header = Row::new(vec!["Validator", "Assigned", "Produced", "Success %"])
                .style(Style::default().fg(Color::Yellow));

            let rows = snapshot.validators.iter().map(|v| {
                let pct = if v.assigned > 0 {
                    format!("{:.1}%", 100.0 * v.produced as f64 / v.assigned as f64)
                } else {
                    "0.0%".to_string()
                };
                Row::new(vec![
                    v.identity.clone(),
                    v.assigned.to_string(),
                    v.produced.to_string(),
                    pct,
                ])
            });

            let table = Table::new(rows)
                .header(header)
                .block(Block::default().title("Top Validators").borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]);

            f.render_widget(table, layout[3]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
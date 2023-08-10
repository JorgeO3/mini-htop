// TODO [x] draw_cpu_graph
// TODO [-] draw_disk_usage
// TODO [-] draw_memory_usage
// TODO [-] draw_components_temps
// TODO [-] draw_network_usage
// TODO [-] draw_list_process

use ratatui::{prelude::*, widgets::*};

use crate::system_info::SystemResources;

pub fn draw<B: Backend>(f: &mut Frame<B>, sys: &mut SystemResources) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_cpu_graph(f, sys, chunks[0]);
    draw_cpu_graph(f, sys, chunks[1]);
    draw_cpu_graph(f, sys, chunks[2]);
}

// first block
pub fn draw_cpu_graph<B>(f: &mut Frame<B>, sys: &mut SystemResources, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(" Cpu Usage ")
                .title_style(Style::default().fg(Color::Rgb(255, 255, 255)))
                .bg(Color::Rgb(38, 70, 83))
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::Rgb(255, 255, 255))),
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .data(&[
            0, 5, 1, 6, 2, 10, 23, 49, 0, 5, 1, 6, 2, 10, 23, 49, 0, 5, 1, 6, 2, 10, 23, 49, 0, 5,
            1, 6, 2, 10, 23, 49,
        ])
        .bar_set(symbols::bar::NINE_LEVELS);

    f.render_widget(sparkline, chunks[0])
}

// second block
pub fn draw_system_stats<B>(f: &mut Frame<B>, sys: &mut SystemResources, area: Rect)
where
    B: Backend,
{
}

// network and process
pub fn draw_network_and_processes<B>(f: &mut Frame<B>, sys: &mut SystemResources, area: Rect)
where
    B: Backend,
{
}

// second block
pub fn draw_disk_usage() {}
pub fn draw_memory_usage() {}
pub fn draw_components_temps() {}

// network and process
pub fn draw_network_usage() {}
pub fn draw_list_process() {}

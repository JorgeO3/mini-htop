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
    draw_system_stats(f, sys, chunks[1]);
    draw_network_and_processes(f, sys, chunks[2]);
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
    let chuncks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(area);

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 5), Constraint::Ratio(1, 5)])
        .split(chuncks[0]);

    draw_memory_usage(f, chuncks[1]);
    draw_disk_usage(f, vertical_chunks[0]);
    draw_components_temps(f, vertical_chunks[1]);
}

// second block
pub fn draw_memory_usage<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let items = [1, 2, 3]
        .iter()
        .map(|c| {
            let cells = vec![
                Cell::from(Span::raw(format!("{c:?}: "))),
                Cell::from(Span::styled(
                    "Foreground",
                    Style::default().fg(Color::Rgb(255, 255, 255)),
                )),
            ];
            Row::new(cells)
        })
        .collect::<Vec<_>>();

    let table = Table::new(items)
        .block(Block::default().title("Colors").borders(Borders::ALL))
        .widths(&[
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);
    f.render_widget(table, area);
}
pub fn draw_disk_usage<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
}
pub fn draw_components_temps<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
}

// network and process
pub fn draw_network_usage() {}
pub fn draw_list_process() {}

// network and process
pub fn draw_network_and_processes<B>(f: &mut Frame<B>, sys: &mut SystemResources, area: Rect)
where
    B: Backend,
{
}

use criterion::{criterion_group, criterion_main, BatchSize, Bencher, BenchmarkId, Criterion};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Cell, Row, StatefulWidget, Table, TableState, Widget},
};

//
fn table(c: &mut Criterion) {
    let mut group = c.benchmark_group("table");

    for row_count in [64, 2048, 16384] {
        for cell_count in [5, 15, 30] {
            let bench_id = format!("row={row_count}/cell={cell_count}");
            let mut rows = Vec::with_capacity(row_count);
            (0..row_count)
                .map(|_| {
                    let mut cells: Vec<Cell> = Vec::with_capacity(cell_count);
                    (0..cell_count)
                        .map(|_| Cell::new("Hello World!"))
                        .for_each(|cell| cells.push(cell));

                    Row::new(cells)
                })
                .for_each(|row| rows.push(row));

            let table = Table::from_iter(rows.clone());

            // Render default table.
            group.bench_with_input(
                BenchmarkId::new("render", bench_id.as_str()),
                &table,
                render,
            );

            // Render stateful with an offset to the middle of the table, and select a row.
            group.bench_with_input(
                BenchmarkId::new("render_scroll_half", bench_id.as_str()),
                &table,
                |b, table| {
                    let mut state = TableState::new()
                        .with_offset(row_count / 2)
                        .with_selected(row_count / 2);
                    render_stateful(b, table, &mut state);
                },
            );

            group.bench_with_input(
                BenchmarkId::new("new", bench_id.as_str()),
                &rows,
                |b, rows| {
                    new(b, rows, cell_count);
                },
            );

            group.bench_with_input(
                BenchmarkId::new("from_iter", bench_id.as_str()),
                &rows,
                from_iter,
            );
        }
    }

    group.finish();
}

fn render(b: &mut Bencher, table: &Table) {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));
    b.iter_batched(
        || table.to_owned(),
        |table| Widget::render(table, buffer.area, &mut buffer),
        BatchSize::LargeInput,
    );
}

fn render_stateful(b: &mut Bencher, table: &Table, state: &mut TableState) {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));
    b.iter_batched(
        || table.to_owned(),
        |table| StatefulWidget::render(table, buffer.area, &mut buffer, state),
        BatchSize::LargeInput,
    );
}

fn new(b: &mut Bencher, rows: &Vec<Row>, cell_count: usize) {
    b.iter_batched(
        || rows.to_owned(),
        |rows| Table::new(rows, vec![5; cell_count]),
        BatchSize::LargeInput,
    );
}

fn from_iter(b: &mut Bencher, rows: &Vec<Row>) {
    b.iter_batched(|| rows.to_owned(), Table::from_iter, BatchSize::LargeInput);
}

criterion_group!(benches, table);
criterion_main!(benches);

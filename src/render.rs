use slint::{ModelRc, SharedString, VecModel};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use crate::board::{BoardState, get_icon};

// !!!!!!!!!!!AI!!!!!!!!!!! //   
pub fn render_board_tui(frame: &mut Frame, state: &BoardState, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    // row 7 (rank 8) is black's back rank — draw top to bottom
    for row in (0..8usize).rev() {
        let mut spans = vec![Span::raw(format!("{} ", row + 1))];
        for col in 0..8usize {
            spans.push(Span::raw(format!("{} ", get_icon(&state.board[row][col]))));
        }
        lines.push(Line::from(spans));
    }
    lines.push(Line::from("  a b c d e f g h"));

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(Block::default().borders(Borders::ALL).title("cherish")),
        area,
    );
}

// !!!!!!!!!!!AI!!!!!!!!!!! //   
pub fn render_board_gui(state: &BoardState) {
    slint::slint! {
        export component ChessBoard inherits Window {
            title: "cherish";
            in property <[string]> rows;
            VerticalLayout {
                padding: 16px;
                for row in rows: Text {
                    text: row;
                    font-family: "monospace";
                    font-size: 28px;
                }
            }
        }
    }

    let window = ChessBoard::new().unwrap();

    let rows: Vec<SharedString> = (0..8usize)
        .rev()
        .map(|row| {
            let mut s = format!("{} ", row + 1);
            for col in 0..8usize {
                s.push(get_icon(&state.board[row][col]));
                s.push(' ');
            }
            s.into()
        })
        .chain(std::iter::once("  a b c d e f g h".into()))
        .collect();

    window.set_rows(ModelRc::new(VecModel::from(rows)));
    window.run().unwrap();
}

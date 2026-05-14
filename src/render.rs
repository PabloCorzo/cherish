use std::cell::RefCell;
use std::collections::HashMap;
use slint::{ModelRc, SharedString, VecModel};
use image::{DynamicImage, RgbaImage, imageops::FilterType};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use crate::board::{BoardState, PieceColor, PieceType, get_icon};

pub struct PieceImages {
    images: HashMap<(PieceType, PieceColor), DynamicImage>,
    // Scaled RGBA cache keyed by (type, color, target_w, target_h).
    // Avoids re-running Lanczos3 on every draw call.
    cache: RefCell<HashMap<(PieceType, PieceColor, u32, u32), RgbaImage>>,
}

impl PieceImages {
    // Load piece images from assets/pieces/ at runtime.
    // Drop replacement images there and restart — no recompile needed.
    pub fn load() -> Self {
        let from_disk = |name: &str| -> DynamicImage {
            image::open(format!("assets/pieces/{}", name))
                .unwrap_or_else(|_| panic!("failed to load assets/pieces/{name}"))
        };
        let mut images = HashMap::new();
        images.insert((PieceType::King,   PieceColor::White), from_disk("wK.png"));
        images.insert((PieceType::Queen,  PieceColor::White), from_disk("wQ.png"));
        images.insert((PieceType::Rook,   PieceColor::White), from_disk("wR.png"));
        images.insert((PieceType::Bishop, PieceColor::White), from_disk("wB.png"));
        images.insert((PieceType::Knight, PieceColor::White), from_disk("wN.png"));
        images.insert((PieceType::Pawn,   PieceColor::White), from_disk("wP.png"));
        images.insert((PieceType::King,   PieceColor::Black), from_disk("bK.png"));
        images.insert((PieceType::Queen,  PieceColor::Black), from_disk("bQ.png"));
        images.insert((PieceType::Rook,   PieceColor::Black), from_disk("bR.png"));
        images.insert((PieceType::Bishop, PieceColor::Black), from_disk("bB.png"));
        images.insert((PieceType::Knight, PieceColor::Black), from_disk("bN.png"));
        images.insert((PieceType::Pawn,   PieceColor::Black), from_disk("bP.png"));
        PieceImages { images, cache: RefCell::new(HashMap::new()) }
    }

    // Ensure all 12 pieces are scaled to (w, h) and stored in the cache.
    // No-ops for entries that are already cached at this size.
    fn precompute(&self, w: u32, h: u32) {
        let mut cache = self.cache.borrow_mut();
        for (&(pt, pc), img) in &self.images {
            cache.entry((pt, pc, w, h)).or_insert_with(|| {
                img.resize_exact(w, h, FilterType::Lanczos3).into_rgba8()
            });
        }
    }

    fn get_scaled(&self, t: PieceType, c: PieceColor, w: u32, h: u32) -> Option<std::cell::Ref<RgbaImage>> {
        let cache = self.cache.borrow();
        if cache.contains_key(&(t, c, w, h)) {
            Some(std::cell::Ref::map(cache, |m| m.get(&(t, c, w, h)).unwrap()))
        } else {
            None
        }
    }
}

// Blend an RGBA pixel onto a solid background, honouring the alpha channel.
fn blend(px: [u8; 4], bg: (u8, u8, u8)) -> (u8, u8, u8) {
    let a = px[3] as f32 / 255.0;
    (
        (px[0] as f32 * a + bg.0 as f32 * (1.0 - a)) as u8,
        (px[1] as f32 * a + bg.1 as f32 * (1.0 - a)) as u8,
        (px[2] as f32 * a + bg.2 as f32 * (1.0 - a)) as u8,
    )
}

// Build halfblock spans for a single cell. Takes a pre-scaled RgbaImage (cell_w × cell_h*2 px).
fn cell_spans(
    buf: Option<&RgbaImage>,
    cell_w: usize,
    cell_h: usize,
    bg: (u8, u8, u8),
) -> Vec<Vec<Span<'static>>> {
    (0..cell_h).map(|row| {
        (0..cell_w).map(|col| {
            let (top, bot) = match buf {
                Some(buf) => {
                    let t = buf.get_pixel(col as u32, (row * 2) as u32).0;
                    let b = buf.get_pixel(col as u32, (row * 2 + 1) as u32).0;
                    (blend(t, bg), blend(b, bg))
                }
                None => (bg, bg),
            };
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(top.0, top.1, top.2))
                    .bg(Color::Rgb(bot.0, bot.1, bot.2)),
            )
        }).collect()
    }).collect()
}

// !!!!!!!!!!!AI!!!!!!!!!!! //
pub fn render_board_tui(frame: &mut Frame, state: &BoardState, area: Rect, images: &PieceImages) {
    let light_bg: (u8, u8, u8) = (240, 217, 181);
    let dark_bg:  (u8, u8, u8) = (181, 136, 99);
    let sq_bg = |row: usize, col: usize| if (row + col) % 2 == 0 { dark_bg } else { light_bg };

    let inner_w = area.width.saturating_sub(2) as usize;
    let inner_h = area.height.saturating_sub(2) as usize;
    let cell_w = (inner_w.saturating_sub(2)) / 8;
    let cell_h = ((inner_h.saturating_sub(1)) / 8).max(1);

    // Populate the scaled-image cache for this cell size (no-op if already cached).
    let (w, h) = (cell_w as u32, (cell_h * 2) as u32);
    images.precompute(w, h);

    let grid: Vec<Vec<Vec<Vec<Span>>>> = (0..8).map(|row| {
        (0..8).map(|col| {
            let piece = &state.board[row][col];
            let scaled = images.get_scaled(piece.t, piece.c, w, h);
            cell_spans(scaled.as_deref(), cell_w, cell_h, sq_bg(row, col))
        }).collect()
    }).collect();

    let mid = cell_h / 2;
    let mut lines: Vec<Line> = Vec::new();

    for rank_row in (0..8usize).rev() {
        for sub in 0..cell_h {
            let label = if sub == mid { format!("{} ", rank_row + 1) } else { "  ".into() };
            let mut spans: Vec<Span> = vec![Span::raw(label)];
            for col in 0..8usize {
                spans.extend(grid[rank_row][col][sub].iter().cloned());
            }
            lines.push(Line::from(spans));
        }
    }

    let mut file_row = String::from("  ");
    for col in 0..8 {
        file_row.push_str(&format!("{:^width$}", (b'a' + col as u8) as char, width = cell_w));
    }
    lines.push(Line::from(file_row));

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

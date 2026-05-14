use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use slint::{ModelRc, VecModel};
use crate::board::{BoardState, get_icon};

pub fn input_tui() -> String{                                                                                                                                                                            
    print!("Enter your move: ");                                                                                                                                                                     
    io::stdout().flush().unwrap();                                                                                                                                                                   
                                                                                                                                                                                                     
    let mut input = String::new();                                                                                                                                                                   
    io::stdin().read_line(&mut input).unwrap();                                                                                                                                                      
    let play = input.trim().clone();

    play.into()                                                                                                                                                                                      
                                                                                                                                                                                                       
  }            

// !!!!!!!!!!!AI!!!!!!!!!!! //
pub fn input_gui(state: &BoardState) -> String {
    slint::slint! {
        struct SquareData {
            piece: string,
            bg: color,
            row: int,
            col: int,
            vrow: int,
        }

        export component InputBoard inherits Window {
            title: "cherish";
            preferred-width: 400px;
            preferred-height: 400px;

            in property <[SquareData]> squares;
            callback board_clicked(int, int);

            for sq in squares: Rectangle {
                x: sq.col * 50px;
                y: sq.vrow * 50px;
                width: 50px;
                height: 50px;
                background: sq.bg;

                Text {
                    text: sq.piece;
                    font-size: 32px;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
                TouchArea {
                    clicked => { root.board_clicked(sq.row, sq.col); }
                }
            }
        }
    }

    let result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let window = InputBoard::new().unwrap();

    let squares: Vec<SquareData> = (0..8usize)
        .flat_map(|row| {
            (0..8usize).map(move |col| SquareData {
                piece: get_icon(&state.board[row][col]).to_string().into(),
                bg: if (row + col) % 2 != 0 {
                    slint::Color::from_rgb_u8(240, 217, 181)
                } else {
                    slint::Color::from_rgb_u8(181, 136, 99)
                },
                row: row as i32,
                col: col as i32,
                vrow: (7 - row) as i32,
            })
        })
        .collect();

    window.set_squares(ModelRc::new(VecModel::from(squares)));

    let pending: Arc<Mutex<Option<(i32, i32)>>> = Arc::new(Mutex::new(None));

    {
        let pending = pending.clone();
        let result = result.clone();
        let window_weak = window.as_weak();
        window.on_board_clicked(move |row, col| {
            let mut p = pending.lock().unwrap();
            match p.take() {
                None => { *p = Some((row, col)); }
                Some((r1, c1)) => {
                    let mv = format!("{} {}", sq_str(r1, c1), sq_str(row, col));
                    *result.lock().unwrap() = Some(mv);
                    window_weak.unwrap().hide().unwrap();
                }
            }
        });
    }

    window.run().unwrap();
    result.lock().unwrap().take().unwrap_or_default()
}

// !!!!!!!!!!!AI!!!!!!!!!!! //   
fn sq_str(row: i32, col: i32) -> String {
    format!("{}{}", (b'a' + col as u8) as char, row + 1)
}

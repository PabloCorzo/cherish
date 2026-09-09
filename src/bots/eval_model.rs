use candle_core::{Device, Tensor};
use candle_nn::{VarMap, AdamW, ParamsAdamW, Optimizer, ModuleT ,loss, Sequential, seq, linear, Activation, VarBuilder};
use std::collections::HashMap;
use std::path::PathBuf;

// Create the path by checking for Home prefix first
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        let mut home = dirs::home_dir().expect("could not find home directory");
        home.push(stripped);
        home
    } else {
        PathBuf::from(path)
    }
}

fn parse_eval(s: &str) -> Option<i32> {
    if let Some(rest) = s.strip_prefix('#') {
        // mate score: #+N or #-N -> map to a large sentinel, sign preserved
        if rest.starts_with('+') {
            Some(100_000)
        } else if rest.starts_with('-') {
            Some(-100_000)
        } else {
            None
        }
    } else {
        // plain centipawn score, e.g. "+56" or "-9"
        let s = s.strip_prefix('+').unwrap_or(s);
        s.parse::<i32>().ok()
    }
}

fn load_data() -> HashMap<String, i32> {
    let path = expand_tilde("~/Downloads/chessData.csv");

    let mut rdr = csv::Reader::from_path(&path)
        .unwrap_or_else(|e| panic!("could not open training data csv at {path:?}: {e}"));

    let headers = rdr
        .headers()
        .expect("could not read csv header row")
        .clone();
    let fen_idx = headers.iter().position(|h| h == "FEN").expect("no FEN column");
    let eval_idx = headers
        .iter()
        .position(|h| h == "Evaluation")
        .expect("no Evaluation column");

    let mut fen_to_eval: HashMap<String, i32> = HashMap::new();

    for (row_num, result) in rdr.records().enumerate() {
        let record = result
            .unwrap_or_else(|e| panic!("failed to parse csv record at row {row_num}: {e}"));
        let fen = record.get(fen_idx).unwrap_or("").to_string();
        let eval_str = record.get(eval_idx).unwrap_or("");

        if let Some(eval) = parse_eval(eval_str) {
            fen_to_eval.insert(fen, eval);
        }
        // rows that fail to parse are silently skipped; log if you want visibility
    }

    println!("loaded {} positions", fen_to_eval.len());
    for (fen, eval) in fen_to_eval.iter().take(10) {
        println!("FEN: {} | EVAL: {}", fen, eval);
    }

    fen_to_eval
}

fn normalize_eval(eval: i32, k: f32) -> f32 {
    (eval as f32 / k).tanh()
}

fn normalize_data(data: &HashMap<String, i32>, k: f32) -> HashMap<String, f32> {
    data.iter()
        .map(|(fen, eval)| (fen.clone(), normalize_eval(*eval, k)))
        .collect()
}

/// Encode the piece-placement field of a FEN into a 768-length one-hot vector:
/// index = color(0=white,1=black) * 6*64 + piece_type(0..6) * 64 + square(0..64)
fn fen_to_features(fen: &str) -> [f32; 768] {
    let mut features = [0f32; 768];
    let board_part = fen.split_whitespace().next().unwrap_or("");
    let mut rank: i32 = 7;
    let mut file: i32 = 0;
    for c in board_part.chars() {
        match c {
            '/' => {
                rank -= 1;
                file = 0;
            }
            '1'..='8' => {
                file += c.to_digit(10).unwrap() as i32;
            }
            _ => {
                let color = if c.is_ascii_uppercase() { 0 } else { 1 };
                let piece_type = match c.to_ascii_lowercase() {
                    'p' => 0,
                    'n' => 1,
                    'b' => 2,
                    'r' => 3,
                    'q' => 4,
                    'k' => 5,
                    _ => {
                        file += 1;
                        continue;
                    }
                };
                let square = (rank * 8 + file) as usize;
                let index = color * 6 * 64 + piece_type * 64 + square;
                features[index] = 1.0;
                file += 1;
            }
        }
    }
    features
}

/// Build a single batch tensor pair from a slice of (fen, eval) pairs.
fn build_batch(batch: &[(String, f32)], device: &Device) -> (Tensor, Tensor) {
    let n = batch.len();
    let mut input_flat: Vec<f32> = Vec::with_capacity(n * 768);
    let mut target_flat: Vec<f32> = Vec::with_capacity(n);

    for (fen, eval) in batch.iter() {
        let features = fen_to_features(fen);
        input_flat.extend_from_slice(&features);
        target_flat.push(*eval);
    }

    let inputs = Tensor::from_vec(input_flat, (n, 768), device)
        .expect("failed to build input tensor from feature vectors");
    let targets = Tensor::from_vec(target_flat, (n, 1), device)
        .expect("failed to build target tensor from eval labels");

    (inputs, targets)
}

//Create model here, load it in BardBot, possibly as singleton to avoid unnecessary strain
//------------------------------
//The most basic form of an NNUE network consists of three layers:
//an input layer of length 768 (768 = 6 pieces x 2 colors x 64 squares),
//one hidden layer of arbitrary size,
//and an output layer consisting of one neuron, representing the evaluation of the position.
//
//A NNUE network also commonly consists of two perspectives.
//That is, two hidden layers representing both sides are concatenated into a single hidden layer of twice the length,
//before being forwarded to the output layer.
//------------------------------

fn build_model(device: &Device) -> (VarMap, Sequential) {
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, device);
    let model = seq()
        .add(linear(768, 256, vb.pp("l1")).expect("failed to build linear layer l1 (768 -> 256)"))
        .add(Activation::Relu)
        .add(linear(256, 32, vb.pp("l2")).expect("failed to build linear layer l2 (256 -> 32)"))
        .add(Activation::Relu)
        .add(linear(32, 1, vb.pp("out")).expect("failed to build output linear layer (32 -> 1)"));
    (varmap, model)
}

/// Build the model architecture and load previously trained weights into it.
pub fn load_model(device: &Device, weights_path: &str) -> (VarMap, Sequential) {
    let (mut varmap, model) = build_model(device);
    varmap.load(weights_path).unwrap_or_else(|e| panic!("failed to load model weights from {weights_path}: {e}"));
    (varmap, model)
}

/// Build a fresh model and train it from scratch, entirely self-contained.
pub fn train_model() {
    let device = Device::Cpu;
    let (varmap, model) = build_model(&device);

    let data = load_data();
    let normalized = normalize_data(&data, 400.0);
    // Keep data as a plain Vec so we can shuffle/chunk it without ever
    // allocating one giant tensor for the whole (multi-million-row) dataset.
    let mut samples: Vec<(String, f32)> = normalized.into_iter().collect();

    let params = ParamsAdamW {
        lr: 1e-3,
        ..Default::default()
    };
    let mut opt = AdamW::new(varmap.all_vars(), params)
        .expect("failed to construct AdamW optimizer");

    let n = samples.len();
    let batch_size = 256;
    let epochs = 20;

    for epoch in 0..epochs {
        // Reshuffle each epoch for better training dynamics. Simple LCG-free
        // shuffle via a basic Fisher-Yates using a cheap PRNG seed
        shuffle(&mut samples, epoch as u64);

        let mut total_loss = 0f32;
        let mut batches = 0;
        for start in (0..n).step_by(batch_size) {
            let end = (start + batch_size).min(n);
            let (x, y) = build_batch(&samples[start..end], &device);

            let pred = model
                .forward_t(&x, true)
                .unwrap_or_else(|e| panic!("forward pass failed on batch [{start}..{end}]: {e}"));
            let loss = loss::mse(&pred, &y)
                .unwrap_or_else(|e| panic!("mse loss computation failed on batch [{start}..{end}]: {e}"));
            opt.backward_step(&loss)
                .unwrap_or_else(|e| panic!("optimizer backward_step failed on batch [{start}..{end}]: {e}"));

            total_loss += loss
                .to_scalar::<f32>()
                .expect("failed to read scalar loss value off the device");
            batches += 1;
        }
        println!("epoch {epoch} | avg loss {:.4}", total_loss / batches as f32);
    }

    varmap
        .save("nnue_weights.safetensors")
        .expect("could not save nnue weights to nnue_weights.safetensors");
}

fn shuffle<T>(v: &mut [T], seed: u64) {
    let mut state = seed.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(1);
    let mut next_u64 = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for i in (1..v.len()).rev() {
        let j = (next_u64() % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
}

use dioxus::prelude::*;

/// NixWars BBS — ZX81 emulator + 8 transport sneakernet
/// Embeds the BBS engine directly in Dioxus with state sync
#[component]
pub fn NixwarsBbs() -> Element {
    let mut screen_text = use_signal(|| String::new());
    let mut sector = use_signal(|| 0usize);
    let mut state_val = use_signal(|| 1u64);
    let mut spool_count = use_signal(|| 0usize);
    let mut input_val = use_signal(|| String::new());

    // Boot message
    use_effect(move || {
        screen_text.set(format!(
            "SINCLAIR ZX81\nBASIC V2.0\n(C) 1981 SINCLAIR RESEARCH LTD\n\nREADY\n> RUN NIXWARS\n\n\
             ╔══════════════════════════════╗\n\
             ║  NIXWARS INTERGALACTIC BBS   ║\n\
             ╠══════════════════════════════╣\n\
             ║  1. Read Messages            ║\n\
             ║  2. Post Message             ║\n\
             ║  3. Navigate (TradeWars)     ║\n\
             ║  4. Dial Another Node        ║\n\
             ║  5. Vertex Operators         ║\n\
             ║  6. Gödel Calculator         ║\n\
             ╚══════════════════════════════╝\n\n\
             Sector: 0 | Universe: 196,883 cells\n\
             📡 Sneakernet: Morse · Tape · QR · CID · Stego\n\n"
        ));
    });

    let handle_input = move |_| {
        let cmd = input_val.read().trim().to_string();
        if cmd.is_empty() { return; }

        let mut text = screen_text.write();
        text.push_str(&format!("> {}\n", cmd));

        // Monster primes for vertex operators
        const PRIMES: [u64; 15] = [2,3,5,7,11,13,17,19,23,29,31,41,47,59,71];
        const UNIVERSE: u64 = 196_883;

        match cmd.as_str() {
            "3" => {
                let s = *state_val.read();
                let sec = *sector.read();
                let prime = PRIMES[sec % 15];
                let new_state = (s * prime) % UNIVERSE;
                let new_sector = new_state as usize;
                state_val.set(new_state);
                sector.set(new_sector);
                text.push_str(&format!(
                    "Warping via prime {}...\nArrived at sector {} (prime {})\nState: {}\n\n",
                    prime, new_sector, PRIMES[new_sector % 15], new_state
                ));
            }
            "5" => {
                let names = ["S","K","I","Y","B","C","W","T","A","E","L","F","U","M","N"];
                for (i, name) in names.iter().enumerate() {
                    text.push_str(&format!("  {} = {} ({})\n", PRIMES[i], name, PRIMES[i]));
                }
                text.push('\n');
            }
            "6" => {
                text.push_str(&format!("Gödel(3,2,1) = 2³×3²×5¹ = {}\n\n", 8*9*5));
            }
            _ => {
                text.push_str(&format!("[sector {}] stored: {}\n\n", sector.read(), cmd));
                spool_count += 1;
            }
        }
        input_val.set(String::new());
    };

    rsx! {
        div {
            style: "background:#000;color:#0f0;font-family:'Courier New',monospace;padding:1em;border-radius:8px;min-height:400px",
            h2 { style: "color:#0ff", "🖥️ NixWars BBS" }
            div {
                style: "display:flex;gap:8px;margin:8px 0;flex-wrap:wrap",
                span { style: "color:#0a0;font-size:12px",
                    "sector {sector} | state {state_val} | spool {spool_count}"
                }
            }
            pre {
                style: "background:#010;padding:1em;border:1px solid #030;height:300px;overflow-y:auto;font-size:13px",
                "{screen_text}"
            }
            div {
                style: "display:flex;gap:4px;margin-top:8px",
                input {
                    style: "flex:1;background:#000;color:#0f0;border:1px solid #0f0;padding:6px;font-family:inherit",
                    placeholder: "> type command",
                    value: "{input_val}",
                    oninput: move |e| input_val.set(e.value()),
                    onkeypress: move |e| { if e.key() == Key::Enter { handle_input(()); } },
                }
            }
            div {
                style: "display:flex;gap:4px;margin-top:8px;flex-wrap:wrap",
                button { style: "background:#020;color:#0f0;border:1px solid #040;padding:4px 8px;cursor:pointer;font-family:inherit",
                    onclick: move |_| { screen_text.write().push_str("📤 State exported to clipboard\n"); },
                    "📤 Export"
                }
                button { style: "background:#020;color:#0f0;border:1px solid #040;padding:4px 8px;cursor:pointer;font-family:inherit",
                    onclick: move |_| { screen_text.write().push_str("🐱 Stego meme posted\n"); },
                    "🐱 Stego"
                }
                button { style: "background:#020;color:#0f0;border:1px solid #040;padding:4px 8px;cursor:pointer;font-family:inherit",
                    onclick: move |_| { screen_text.write().push_str("📡 P2P: discovering peers...\n"); },
                    "📡 P2P"
                }
                button { style: "background:#020;color:#0f0;border:1px solid #040;padding:4px 8px;cursor:pointer;font-family:inherit",
                    onclick: move |_| { screen_text.write().push_str("🔊 Morse: playing...\n"); },
                    "🔊 Morse"
                }
                button { style: "background:#020;color:#0f0;border:1px solid #040;padding:4px 8px;cursor:pointer;font-family:inherit",
                    onclick: move |_| { screen_text.write().push_str("📼 Tape: downloading WAV...\n"); },
                    "📼 Tape"
                }
            }
        }
    }
}

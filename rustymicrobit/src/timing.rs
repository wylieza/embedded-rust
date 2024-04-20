use cortex_m::asm::nop;

pub fn delay_ms(period_ms: i32) {
    let nops_per_ms = 505;
    for _ in 0..(nops_per_ms*period_ms) {
        nop();
    }
}
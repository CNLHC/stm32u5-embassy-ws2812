# stm32u5-embassy-ws2812

A small rust program to control a WS2812 LED Array using the STM32U5 microcontroller.

Implement with `async` based concurrent supported by `embassy`

```rust

#[embassy_executor::task]
async fn update_vram_thread() {
    // update VRAM buffer
}

#[embassy_executor::task]
async fn render_thread() {
    // render VRAM buffer to WS2812 array, using pwm driver
}

```

To run the program:

1. please change the pwm pin and dma channel in `main.rs` to match your configuration. By default, `PA5`, `TIM2` and `GPDMA1_CH2` are used.

2. to use a different stm32 chip, please change the `embassy-stm32` feature flag and `.cargo/config.toml` run settings. By default, `stm32u545re`(for embassy feature flag) and `STM32U545RETx` (for probe-rs target) are used.

3. after the configuration, connect your board via `st-link` and run the program with `cargo run --release`.

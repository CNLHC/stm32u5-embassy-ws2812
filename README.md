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

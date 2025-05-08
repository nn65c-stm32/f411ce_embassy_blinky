# Embedded Rust with STM32 and Embassy

Blinky Rust project for BlackPill.

Board used is [STM32F411CEU6](https://github.com/WeActStudio/WeActStudio.MiniSTM32F4x1).

## Base setup for STM32 and ST-link programmer
[Embedded Rust with STM32 and Embassy](https://github.com/nn65c-stm32/.github/blob/main/profile/README.md)

## Expected results
Blinking LED.

Button press:
```
0.000000 [DEBUG] flash: latency=0 (embassy_stm32 src/rcc/f247.rs:258)
0.000000 [DEBUG] rcc: Clocks { hclk1: MaybeHertz(16000000), hclk2: MaybeHertz(16000000), hse: MaybeHertz(0), lse: MaybeHertz(0), lsi: MaybeHertz(0), pclk1: MaybeHertz(16000000), pclk1_tim: MaybeHertz(16000000), pclk2: MaybeHertz(16000000), pclk2_tim: MaybeHertz(16000000), pll1_q: MaybeHertz(0), plli2s1_p: MaybeHertz(0), plli2s1_q: MaybeHertz(0), plli2s1_r: MaybeHertz(0), pllsai1_q: MaybeHertz(0), rtc: MaybeHertz(32000), sys: MaybeHertz(16000000) } (embassy_stm32 src/rcc/mod.rs:71)
0.000000 [INFO ] Start (f411ce_embassy_blinky f411ce_embassy_blinky/src/main.rs:29)
3.663726 [INFO ] Button change! (f411ce_embassy_blinky f411ce_embassy_blinky/src/main.rs:39)
3.901245 [INFO ] Button change! (f411ce_embassy_blinky f411ce_embassy_blinky/src/main.rs:39)
4.466033 [INFO ] Button change! (f411ce_embassy_blinky f411ce_embassy_blinky/src/main.rs:39)
4.636413 [INFO ] Button change! (f411ce_embassy_blinky f411ce_embassy_blinky/src/main.rs:39)
```
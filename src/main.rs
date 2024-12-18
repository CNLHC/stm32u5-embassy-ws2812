#![no_std]
#![no_main]

mod constant;
mod graphic;
mod grid_library;
use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::rcc::mux::ClockMux;
use embassy_stm32::rcc::{self, Pll, Sysclk};
use embassy_stm32::time::khz;
use embassy_stm32::timer::{simple_pwm, Channel1Pin, GeneralInstance4Channel};
use embassy_stm32::{Config, Peripheral, Peripherals};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embassy_time::{Duration, Ticker, Timer};
use graphic::write_8x8_bitmap;
use grid_library::LETTERS;
use {defmt_rtt as _, panic_probe as _};

fn rcc_config() -> rcc::Config {
    let mut rcc_cfg = rcc::Config::default();
    rcc_cfg.mux = ClockMux::default();
    rcc_cfg.sys = Sysclk::PLL1_R;
    rcc_cfg.hsi = true;
    let pcfg = Pll {
        source: rcc::PllSource::HSI,
        prediv: rcc::PllPreDiv::DIV1,
        mul: rcc::PllMul::MUL10,
        divp: Some(rcc::PllDiv::DIV1),
        divq: Some(rcc::PllDiv::DIV1),
        divr: Some(rcc::PllDiv::DIV1),
    };
    rcc_cfg.pll1 = Some(pcfg);
    rcc_cfg.msis = Some(rcc::MSIRange::RANGE_48MHZ);
    rcc_cfg.apb1_pre = rcc::APBPrescaler::DIV1;
    rcc_cfg.apb2_pre = rcc::APBPrescaler::DIV1;
    rcc_cfg.apb3_pre = rcc::APBPrescaler::DIV1;
    rcc_cfg
}

fn setup_pwm<'d, T, P>(tim: T, pin: P) -> simple_pwm::SimplePwm<'d, T>
where
    T: Peripheral + GeneralInstance4Channel + 'd,
    P: Peripheral + Channel1Pin<T> + 'd,
{
    let pin = simple_pwm::PwmPin::new_ch1(pin, OutputType::PushPull);
    let mut pwm_driver = simple_pwm::SimplePwm::new(
        tim,
        Some(pin),
        None,
        None,
        None,
        khz(800),
        Default::default(),
    );
    // let max_duty = pwm_driver.max_duty_cycle();
    let mut ch1 = pwm_driver.ch1();
    ch1.enable();
    ch1.set_duty_cycle(0);
    pwm_driver
}

static DISPLAY_BUF: mutex::Mutex<ThreadModeRawMutex, RefCell<[u16; 1537]>> =
    mutex::Mutex::new(RefCell::new([0; 1537]));

#[embassy_executor::task]
async fn update_vram_thread() {
    let mut i = 0;
    loop {
        Timer::after(Duration::from_millis(1500)).await;
        let mut buf = DISPLAY_BUF.lock().await;
        write_8x8_bitmap(LETTERS[i % 26], buf.get_mut(), &[4, 16, 32]);
        i += 1;
    }
}

#[embassy_executor::task]
async fn render_thread(p: Peripherals) {
    let mut buf1: [u16; 1537] = [0; 1537];
    let mut ticker = Ticker::every(Duration::from_millis(200));
    let mut p = p;
    let mut pd = setup_pwm(p.TIM2, p.PA5);
    loop {
        pd.waveform_ch1(&mut p.GPDMA1_CH2, &buf1).await;
        Timer::after_micros(50).await;
        buf1.copy_from_slice(DISPLAY_BUF.lock().await.borrow().as_slice());
        ticker.next().await;
    }
}

#[embassy_executor::main]
async fn main(s: Spawner) -> ! {
    let mut c: Config = Default::default();
    c.rcc = rcc_config();
    let p = embassy_stm32::init(c);
    s.spawn(update_vram_thread()).unwrap();
    s.spawn(render_thread(p)).unwrap();
    loop {
        Timer::after(Duration::from_millis(10000)).await;
    }
}

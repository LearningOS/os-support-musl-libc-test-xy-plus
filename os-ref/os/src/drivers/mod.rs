pub mod block;
pub mod chardev;
#[cfg(feature = "board_qemu")]
pub mod gpu;
#[cfg(feature = "board_qemu")]
pub mod input;
pub mod plic;
pub use block::BLOCK_DEVICE;
#[cfg(feature = "board_qemu")]
pub use chardev::UART;
#[cfg(feature = "board_qemu")]
pub use gpu::*;
#[cfg(feature = "board_qemu")]
pub use input::*;
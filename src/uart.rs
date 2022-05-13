#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit configuration register"]
    pub transmit_config: crate::Reg<transmit_config::TRANSMIT_CONFIG_SPEC>,
    #[doc = "0x04 - Receive configuration register"]
    pub receive_config: crate::Reg<receive_config::RECEIVE_CONFIG_SPEC>,
    #[doc = "0x08 - Bit period control register"]
    pub bit_period: crate::Reg<bit_period::BIT_PERIOD_SPEC>,
    #[doc = "0x0c - Data configuration register"]
    pub data_config: crate::Reg<data_config::DATA_CONFIG_SPEC>,
    #[doc = "0x10 - IR-mode transmit position control"]
    pub transmit_position: crate::Reg<transmit_position::TRANSMIT_POSITION_SPEC>,
    #[doc = "0x14 - IR-mode receive position control"]
    pub receive_position: crate::Reg<receive_position::RECEIVE_POSITION_SPEC>,
    #[doc = "0x18 - Receive Time-Out interrupt control"]
    pub receive_timeout: crate::Reg<receive_timeout::RECEIVE_TIMEOUT_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Interrupt state register"]
    pub interrupt_state: crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>,
    #[doc = "0x24 - Interrupt mask register"]
    pub interrupt_mask: crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>,
    #[doc = "0x28 - Clear interrupt register"]
    pub interrupt_clear: crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>,
    #[doc = "0x2c - Interrupt enable register"]
    pub interrupt_enable: crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>,
    #[doc = "0x30 - Bus status register"]
    pub bus_status: crate::Reg<bus_status::BUS_STATUS_SPEC>,
    #[doc = "0x34 - Auto baudrate detection register"]
    pub auto_baudrate: crate::Reg<auto_baudrate::AUTO_BAUDRATE_SPEC>,
    _reserved13: [u8; 0x48],
    #[doc = "0x80 - FIFO configuration register 0"]
    pub fifo_config_0: crate::Reg<fifo_config_0::FIFO_CONFIG_0_SPEC>,
    #[doc = "0x84 - FIFO configuration register 1"]
    pub fifo_config_1: crate::Reg<fifo_config_1::FIFO_CONFIG_1_SPEC>,
    #[doc = "0x88 - FIFO write data register"]
    pub data_write: crate::Reg<data_write::DATA_WRITE_SPEC>,
    #[doc = "0x8c - FIFO read data register"]
    pub data_read: crate::Reg<data_read::DATA_READ_SPEC>,
}
#[doc = "transmit_config register accessor: an alias for `Reg<TRANSMIT_CONFIG_SPEC>`"]
pub type TRANSMIT_CONFIG = crate::Reg<transmit_config::TRANSMIT_CONFIG_SPEC>;
#[doc = "Transmit configuration register"]
pub mod transmit_config;
#[doc = "receive_config register accessor: an alias for `Reg<RECEIVE_CONFIG_SPEC>`"]
pub type RECEIVE_CONFIG = crate::Reg<receive_config::RECEIVE_CONFIG_SPEC>;
#[doc = "Receive configuration register"]
pub mod receive_config;
#[doc = "bit_period register accessor: an alias for `Reg<BIT_PERIOD_SPEC>`"]
pub type BIT_PERIOD = crate::Reg<bit_period::BIT_PERIOD_SPEC>;
#[doc = "Bit period control register"]
pub mod bit_period;
#[doc = "data_config register accessor: an alias for `Reg<DATA_CONFIG_SPEC>`"]
pub type DATA_CONFIG = crate::Reg<data_config::DATA_CONFIG_SPEC>;
#[doc = "Data configuration register"]
pub mod data_config;
#[doc = "transmit_position register accessor: an alias for `Reg<TRANSMIT_POSITION_SPEC>`"]
pub type TRANSMIT_POSITION = crate::Reg<transmit_position::TRANSMIT_POSITION_SPEC>;
#[doc = "IR-mode transmit position control"]
pub mod transmit_position;
#[doc = "receive_position register accessor: an alias for `Reg<RECEIVE_POSITION_SPEC>`"]
pub type RECEIVE_POSITION = crate::Reg<receive_position::RECEIVE_POSITION_SPEC>;
#[doc = "IR-mode receive position control"]
pub mod receive_position;
#[doc = "receive_timeout register accessor: an alias for `Reg<RECEIVE_TIMEOUT_SPEC>`"]
pub type RECEIVE_TIMEOUT = crate::Reg<receive_timeout::RECEIVE_TIMEOUT_SPEC>;
#[doc = "Receive Time-Out interrupt control"]
pub mod receive_timeout;
#[doc = "interrupt_state register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Interrupt state register"]
pub mod interrupt_state;
#[doc = "interrupt_mask register accessor: an alias for `Reg<INTERRUPT_MASK_SPEC>`"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod interrupt_mask;
#[doc = "interrupt_clear register accessor: an alias for `Reg<INTERRUPT_CLEAR_SPEC>`"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Clear interrupt register"]
pub mod interrupt_clear;
#[doc = "interrupt_enable register accessor: an alias for `Reg<INTERRUPT_ENABLE_SPEC>`"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod interrupt_enable;
#[doc = "bus_status register accessor: an alias for `Reg<BUS_STATUS_SPEC>`"]
pub type BUS_STATUS = crate::Reg<bus_status::BUS_STATUS_SPEC>;
#[doc = "Bus status register"]
pub mod bus_status;
#[doc = "auto_baudrate register accessor: an alias for `Reg<AUTO_BAUDRATE_SPEC>`"]
pub type AUTO_BAUDRATE = crate::Reg<auto_baudrate::AUTO_BAUDRATE_SPEC>;
#[doc = "Auto baudrate detection register"]
pub mod auto_baudrate;
#[doc = "fifo_config_0 register accessor: an alias for `Reg<FIFO_CONFIG_0_SPEC>`"]
pub type FIFO_CONFIG_0 = crate::Reg<fifo_config_0::FIFO_CONFIG_0_SPEC>;
#[doc = "FIFO configuration register 0"]
pub mod fifo_config_0;
#[doc = "fifo_config_1 register accessor: an alias for `Reg<FIFO_CONFIG_1_SPEC>`"]
pub type FIFO_CONFIG_1 = crate::Reg<fifo_config_1::FIFO_CONFIG_1_SPEC>;
#[doc = "FIFO configuration register 1"]
pub mod fifo_config_1;
#[doc = "data_write register accessor: an alias for `Reg<DATA_WRITE_SPEC>`"]
pub type DATA_WRITE = crate::Reg<data_write::DATA_WRITE_SPEC>;
#[doc = "FIFO write data register"]
pub mod data_write;
#[doc = "data_read register accessor: an alias for `Reg<DATA_READ_SPEC>`"]
pub type DATA_READ = crate::Reg<data_read::DATA_READ_SPEC>;
#[doc = "FIFO read data register"]
pub mod data_read;

//! Register-name overlay for the ESP32-P4 TWAI PAC.
//!
//! The peripheral matches the ESP32-C6. The `esp32p4` PAC spells the same
//! fields out in full, so this presents the C6 names as extension traits and
//! the driver body needs no `cfg` of its own. Same purpose as
//! `peripherals/overlay_rmt.rs`, one level down.
//!
//! An inherent method always wins over a trait method, so importing these on a
//! chip that already has the C6 names would change nothing; the module is
//! nevertheless built for the ESP32-P4 alone.

use crate::pac::twai0;

/// `STATUS` field names.
pub(super) trait StatusR {
    fn bus_off_st(&self) -> crate::pac::generic::BitReader;
    fn err_st(&self) -> crate::pac::generic::BitReader;
    fn miss_st(&self) -> crate::pac::generic::BitReader;
    fn rx_buf_st(&self) -> crate::pac::generic::BitReader;
    fn tx_buf_st(&self) -> crate::pac::generic::BitReader;
}

impl StatusR for twai0::status::R {
    #[inline(always)]
    fn bus_off_st(&self) -> crate::pac::generic::BitReader {
        self.node_bus_off()
    }
    #[inline(always)]
    fn err_st(&self) -> crate::pac::generic::BitReader {
        self.err()
    }
    #[inline(always)]
    fn miss_st(&self) -> crate::pac::generic::BitReader {
        self.miss()
    }
    #[inline(always)]
    fn rx_buf_st(&self) -> crate::pac::generic::BitReader {
        self.receive_buffer()
    }
    #[inline(always)]
    fn tx_buf_st(&self) -> crate::pac::generic::BitReader {
        self.transmit_buffer()
    }
}

/// `BUS_TIMING_1` field names.
pub(super) trait BusTiming1W {
    fn time_seg1(&mut self) -> twai0::bus_timing_1::TIME_SEGMENT1_W<'_, twai0::bus_timing_1::BUS_TIMING_1_SPEC>;
    fn time_seg2(&mut self) -> twai0::bus_timing_1::TIME_SEGMENT2_W<'_, twai0::bus_timing_1::BUS_TIMING_1_SPEC>;
    fn time_samp(&mut self) -> twai0::bus_timing_1::TIME_SAMPLING_W<'_, twai0::bus_timing_1::BUS_TIMING_1_SPEC>;
}

impl BusTiming1W for twai0::bus_timing_1::W {
    #[inline(always)]
    fn time_seg1(
        &mut self,
    ) -> twai0::bus_timing_1::TIME_SEGMENT1_W<'_, twai0::bus_timing_1::BUS_TIMING_1_SPEC> {
        self.time_segment1()
    }
    #[inline(always)]
    fn time_seg2(
        &mut self,
    ) -> twai0::bus_timing_1::TIME_SEGMENT2_W<'_, twai0::bus_timing_1::BUS_TIMING_1_SPEC> {
        self.time_segment2()
    }
    #[inline(always)]
    fn time_samp(
        &mut self,
    ) -> twai0::bus_timing_1::TIME_SAMPLING_W<'_, twai0::bus_timing_1::BUS_TIMING_1_SPEC> {
        self.time_sampling()
    }
}

/// `CMD` field names.
pub(super) trait CmdW {
    fn tx_req(&mut self) -> twai0::cmd::TX_REQUEST_W<'_, twai0::cmd::CMD_SPEC>;
    fn release_buf(&mut self) -> twai0::cmd::RELEASE_BUFFER_W<'_, twai0::cmd::CMD_SPEC>;
    fn self_rx_req(&mut self) -> twai0::cmd::SELF_RX_REQUEST_W<'_, twai0::cmd::CMD_SPEC>;
}

impl CmdW for twai0::cmd::W {
    #[inline(always)]
    fn tx_req(&mut self) -> twai0::cmd::TX_REQUEST_W<'_, twai0::cmd::CMD_SPEC> {
        self.tx_request()
    }
    #[inline(always)]
    fn release_buf(&mut self) -> twai0::cmd::RELEASE_BUFFER_W<'_, twai0::cmd::CMD_SPEC> {
        self.release_buffer()
    }
    #[inline(always)]
    fn self_rx_req(&mut self) -> twai0::cmd::SELF_RX_REQUEST_W<'_, twai0::cmd::CMD_SPEC> {
        self.self_rx_request()
    }
}

/// `MODE` field names.
pub(super) trait ModeW {
    fn rx_filter_mode(&mut self) -> twai0::mode::ACCEPTANCE_FILTER_MODE_W<'_, twai0::mode::MODE_SPEC>;
}

impl ModeW for twai0::mode::W {
    #[inline(always)]
    fn rx_filter_mode(
        &mut self,
    ) -> twai0::mode::ACCEPTANCE_FILTER_MODE_W<'_, twai0::mode::MODE_SPEC> {
        self.acceptance_filter_mode()
    }
}

/// `INTERRUPT` (raw status) field names.
pub(super) trait IntRawR {
    fn rx_int_st(&self) -> crate::pac::generic::BitReader;
    fn tx_int_st(&self) -> crate::pac::generic::BitReader;
    fn err_warn_int_st(&self) -> crate::pac::generic::BitReader;
    fn overrun_int_st(&self) -> crate::pac::generic::BitReader;
    fn arb_lost_int_st(&self) -> crate::pac::generic::BitReader;
}

impl IntRawR for twai0::interrupt::R {
    #[inline(always)]
    fn rx_int_st(&self) -> crate::pac::generic::BitReader {
        self.receive_int_st()
    }
    #[inline(always)]
    fn tx_int_st(&self) -> crate::pac::generic::BitReader {
        self.transmit_int_st()
    }
    #[inline(always)]
    fn err_warn_int_st(&self) -> crate::pac::generic::BitReader {
        self.err_warning_int_st()
    }
    #[inline(always)]
    fn overrun_int_st(&self) -> crate::pac::generic::BitReader {
        self.data_overrun_int_st()
    }
    #[inline(always)]
    fn arb_lost_int_st(&self) -> crate::pac::generic::BitReader {
        self.arbitration_lost_int_st()
    }
}

/// `INTERRUPT_ENABLE` field names. The P4 prefixes the four maskable sources
/// that the C6 leaves bare with `ext_`.
pub(super) trait IntEnaW {
    fn rx_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::EXT_RECEIVE_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>;
    fn tx_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::EXT_TRANSMIT_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>;
    fn err_warn_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::EXT_ERR_WARNING_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>;
    fn arb_lost_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::ARBITRATION_LOST_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>;
}

impl IntEnaW for twai0::interrupt_enable::W {
    #[inline(always)]
    fn rx_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::EXT_RECEIVE_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>
    {
        self.ext_receive_int_ena()
    }
    #[inline(always)]
    fn tx_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::EXT_TRANSMIT_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>
    {
        self.ext_transmit_int_ena()
    }
    #[inline(always)]
    fn err_warn_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::EXT_ERR_WARNING_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>
    {
        self.ext_err_warning_int_ena()
    }
    #[inline(always)]
    fn arb_lost_int_ena(
        &mut self,
    ) -> twai0::interrupt_enable::ARBITRATION_LOST_INT_ENA_W<'_, twai0::interrupt_enable::INTERRUPT_ENABLE_SPEC>
    {
        self.arbitration_lost_int_ena()
    }
}

/// Register names on the block itself.
pub(super) trait RegisterBlockExt {
    fn int_raw(&self) -> &twai0::INTERRUPT;
    fn int_ena(&self) -> &twai0::INTERRUPT_ENABLE;
    fn rx_message_cnt(&self) -> &twai0::RX_MESSAGE_COUNTER;
}

impl RegisterBlockExt for twai0::RegisterBlock {
    #[inline(always)]
    fn int_raw(&self) -> &twai0::INTERRUPT {
        self.interrupt()
    }
    #[inline(always)]
    fn int_ena(&self) -> &twai0::INTERRUPT_ENABLE {
        self.interrupt_enable()
    }
    #[inline(always)]
    fn rx_message_cnt(&self) -> &twai0::RX_MESSAGE_COUNTER {
        self.rx_message_counter()
    }
}

#[doc = "Register `I2CG0C` reader"]
pub type R = crate::R<I2cg0cSpec>;
#[doc = "Register `I2CG0C` writer"]
pub type W = crate::W<I2cg0cSpec>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "Field `Reserved02` writer - Reserved (0)"]
pub type Reserved02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClkDividerModeSel` reader - newverClock divider mode selection"]
pub type ClkDividerModeSelR = crate::BitReader;
#[doc = "Field `ClkDividerModeSel` writer - newverClock divider mode selection"]
pub type ClkDividerModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RegDefinitionSel` reader - newverRegister definition selection"]
pub type RegDefinitionSelR = crate::BitReader;
#[doc = "Field `RegDefinitionSel` writer - newverRegister definition selection"]
pub type RegDefinitionSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSeparatingMasterAndSlaveModeINTs` reader - newverEnable separating master and slave mode interrupts"]
pub type EnblSeparatingMasterAndSlaveModeIntsR = crate::BitReader;
#[doc = "Field `EnblSeparatingMasterAndSlaveModeINTs` writer - newverEnable separating master and slave mode interrupts"]
pub type EnblSeparatingMasterAndSlaveModeIntsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SelectTheActionWhenSlavePktModeRXBufEmpty` reader - newverSelect the action when Slave packet mode RX buffer empty"]
pub type SelectTheActionWhenSlavePktModeRxbufEmptyR = crate::BitReader;
#[doc = "Field `SelectTheActionWhenSlavePktModeRXBufEmpty` writer - newverSelect the action when Slave packet mode RX buffer empty"]
pub type SelectTheActionWhenSlavePktModeRxbufEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `MasterTxToRxTurnaroundDelay` reader - newverMaster transmit to receive turn-around delay"]
pub type MasterTxToRxTurnaroundDelayR = crate::FieldReader;
#[doc = "Field `MasterTxToRxTurnaroundDelay` writer - newverMaster transmit to receive turn-around delay"]
pub type MasterTxToRxTurnaroundDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - newverClock divider mode selection"]
    #[inline(always)]
    pub fn clk_divider_mode_sel(&self) -> ClkDividerModeSelR {
        ClkDividerModeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - newverRegister definition selection"]
    #[inline(always)]
    pub fn reg_definition_sel(&self) -> RegDefinitionSelR {
        RegDefinitionSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - newverEnable separating master and slave mode interrupts"]
    #[inline(always)]
    pub fn enbl_separating_master_and_slave_mode_ints(
        &self,
    ) -> EnblSeparatingMasterAndSlaveModeIntsR {
        EnblSeparatingMasterAndSlaveModeIntsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - newverSelect the action when Slave packet mode RX buffer empty"]
    #[inline(always)]
    pub fn select_the_action_when_slave_pkt_mode_rxbuf_empty(
        &self,
    ) -> SelectTheActionWhenSlavePktModeRxbufEmptyR {
        SelectTheActionWhenSlavePktModeRxbufEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - newverMaster transmit to receive turn-around delay"]
    #[inline(always)]
    pub fn master_tx_to_rx_turnaround_delay(&self) -> MasterTxToRxTurnaroundDelayR {
        MasterTxToRxTurnaroundDelayR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&mut self) -> Reserved02W<I2cg0cSpec> {
        Reserved02W::new(self, 0)
    }
    #[doc = "Bit 1 - newverClock divider mode selection"]
    #[inline(always)]
    pub fn clk_divider_mode_sel(&mut self) -> ClkDividerModeSelW<I2cg0cSpec> {
        ClkDividerModeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - newverRegister definition selection"]
    #[inline(always)]
    pub fn reg_definition_sel(&mut self) -> RegDefinitionSelW<I2cg0cSpec> {
        RegDefinitionSelW::new(self, 2)
    }
    #[doc = "Bit 3 - newverEnable separating master and slave mode interrupts"]
    #[inline(always)]
    pub fn enbl_separating_master_and_slave_mode_ints(
        &mut self,
    ) -> EnblSeparatingMasterAndSlaveModeIntsW<I2cg0cSpec> {
        EnblSeparatingMasterAndSlaveModeIntsW::new(self, 3)
    }
    #[doc = "Bit 4 - newverSelect the action when Slave packet mode RX buffer empty"]
    #[inline(always)]
    pub fn select_the_action_when_slave_pkt_mode_rxbuf_empty(
        &mut self,
    ) -> SelectTheActionWhenSlavePktModeRxbufEmptyW<I2cg0cSpec> {
        SelectTheActionWhenSlavePktModeRxbufEmptyW::new(self, 4)
    }
    #[doc = "Bits 8:11 - newverMaster transmit to receive turn-around delay"]
    #[inline(always)]
    pub fn master_tx_to_rx_turnaround_delay(&mut self) -> MasterTxToRxTurnaroundDelayW<I2cg0cSpec> {
        MasterTxToRxTurnaroundDelayW::new(self, 8)
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cg0cSpec;
impl crate::RegisterSpec for I2cg0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cg0c::R`](R) reader structure"]
impl crate::Readable for I2cg0cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cg0c::W`](W) writer structure"]
impl crate::Writable for I2cg0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CG0C to value 0"]
impl crate::Resettable for I2cg0cSpec {}

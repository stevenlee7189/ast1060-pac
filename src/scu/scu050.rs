#[doc = "Register `SCU050` reader"]
pub type R = crate::R<Scu050Spec>;
#[doc = "Register `SCU050` writer"]
pub type W = crate::W<Scu050Spec>;
#[doc = "Field `Reserved5` reader - Reserved, must keep at value \"1\""]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved, must keep at value \"1\""]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved, must keep at value \"0\""]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved, must keep at value \"0\""]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstI2CSMBusCtrl` reader - Reset I2C/SMBus controller"]
pub type RstI2csmbusCtrlR = crate::BitReader;
#[doc = "Field `RstI2CSMBusCtrl` writer - Reset I2C/SMBus controller"]
pub type RstI2csmbusCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved, must keep at value \"111\""]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved, must keep at value \"111\""]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - Reserved, must keep at value \"1\""]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved, must keep at value \"1\""]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstI3CREGDMACtrl` reader - Reset I3C REG/DMA controller"]
pub type RstI3cregdmactrlR = crate::BitReader;
#[doc = "Field `RstI3CREGDMACtrl` writer - Reset I3C REG/DMA controller"]
pub type RstI3cregdmactrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstI3C0Ctrl` reader - Reset I3C0 controller"]
pub type RstI3c0ctrlR = crate::BitReader;
#[doc = "Field `RstI3C0Ctrl` writer - Reset I3C0 controller"]
pub type RstI3c0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstI3C1Ctrl` reader - Reset I3C1 controller"]
pub type RstI3c1ctrlR = crate::BitReader;
#[doc = "Field `RstI3C1Ctrl` writer - Reset I3C1 controller"]
pub type RstI3c1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstI3C2Ctrl` reader - Reset I3C2 controller"]
pub type RstI3c2ctrlR = crate::BitReader;
#[doc = "Field `RstI3C2Ctrl` writer - Reset I3C2 controller"]
pub type RstI3c2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstI3C3Ctrl` reader - Reset I3C3 controller"]
pub type RstI3c3ctrlR = crate::BitReader;
#[doc = "Field `RstI3C3Ctrl` writer - Reset I3C3 controller"]
pub type RstI3c3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved, must keep at value all \"1\""]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - Reserved, must keep at value all \"1\""]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RstJTAGMaster2Ctrl` reader - Reset JTAG Master 2 controller"]
pub type RstJtagmaster2ctrlR = crate::BitReader;
#[doc = "Field `RstJTAGMaster2Ctrl` writer - Reset JTAG Master 2 controller"]
pub type RstJtagmaster2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstADCCtrl` reader - Reset ADC controller"]
pub type RstAdcctrlR = crate::BitReader;
#[doc = "Field `RstADCCtrl` writer - Reset ADC controller"]
pub type RstAdcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstJTAGMaster1Ctrl` reader - Reset JTAG Master 1 controller"]
pub type RstJtagmaster1ctrlR = crate::BitReader;
#[doc = "Field `RstJTAGMaster1Ctrl` writer - Reset JTAG Master 1 controller"]
pub type RstJtagmaster1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstUART1Ctrl` reader - Reset UART1 controller"]
pub type RstUart1ctrlR = crate::BitReader;
#[doc = "Field `RstUART1Ctrl` writer - Reset UART1 controller"]
pub type RstUart1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstUART2Ctrl` reader - Reset UART2 controller"]
pub type RstUart2ctrlR = crate::BitReader;
#[doc = "Field `RstUART2Ctrl` writer - Reset UART2 controller"]
pub type RstUart2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstUART3Ctrl` reader - Reset UART3 controller"]
pub type RstUart3ctrlR = crate::BitReader;
#[doc = "Field `RstUART3Ctrl` writer - Reset UART3 controller"]
pub type RstUart3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstUART4Ctrl` reader - Reset UART4 controller"]
pub type RstUart4ctrlR = crate::BitReader;
#[doc = "Field `RstUART4Ctrl` writer - Reset UART4 controller"]
pub type RstUart4ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved, must keep at value \"1\""]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved, must keep at value \"0\""]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset I2C/SMBus controller"]
    #[inline(always)]
    pub fn rst_i2csmbus_ctrl(&self) -> RstI2csmbusCtrlR {
        RstI2csmbusCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Reserved, must keep at value \"111\""]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Reserved, must keep at value \"1\""]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset I3C REG/DMA controller"]
    #[inline(always)]
    pub fn rst_i3cregdmactrl(&self) -> RstI3cregdmactrlR {
        RstI3cregdmactrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset I3C0 controller"]
    #[inline(always)]
    pub fn rst_i3c0ctrl(&self) -> RstI3c0ctrlR {
        RstI3c0ctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset I3C1 controller"]
    #[inline(always)]
    pub fn rst_i3c1ctrl(&self) -> RstI3c1ctrlR {
        RstI3c1ctrlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset I3C2 controller"]
    #[inline(always)]
    pub fn rst_i3c2ctrl(&self) -> RstI3c2ctrlR {
        RstI3c2ctrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset I3C3 controller"]
    #[inline(always)]
    pub fn rst_i3c3ctrl(&self) -> RstI3c3ctrlR {
        RstI3c3ctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:21 - Reserved, must keep at value all \"1\""]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bit 22 - Reset JTAG Master 2 controller"]
    #[inline(always)]
    pub fn rst_jtagmaster2ctrl(&self) -> RstJtagmaster2ctrlR {
        RstJtagmaster2ctrlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset ADC controller"]
    #[inline(always)]
    pub fn rst_adcctrl(&self) -> RstAdcctrlR {
        RstAdcctrlR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset JTAG Master 1 controller"]
    #[inline(always)]
    pub fn rst_jtagmaster1ctrl(&self) -> RstJtagmaster1ctrlR {
        RstJtagmaster1ctrlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset UART1 controller"]
    #[inline(always)]
    pub fn rst_uart1ctrl(&self) -> RstUart1ctrlR {
        RstUart1ctrlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset UART2 controller"]
    #[inline(always)]
    pub fn rst_uart2ctrl(&self) -> RstUart2ctrlR {
        RstUart2ctrlR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset UART3 controller"]
    #[inline(always)]
    pub fn rst_uart3ctrl(&self) -> RstUart3ctrlR {
        RstUart3ctrlR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reset UART4 controller"]
    #[inline(always)]
    pub fn rst_uart4ctrl(&self) -> RstUart4ctrlR {
        RstUart4ctrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved, must keep at value \"1\""]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Scu050Spec> {
        Reserved5W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved, must keep at value \"0\""]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu050Spec> {
        Reserved4W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset I2C/SMBus controller"]
    #[inline(always)]
    pub fn rst_i2csmbus_ctrl(&mut self) -> RstI2csmbusCtrlW<Scu050Spec> {
        RstI2csmbusCtrlW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Reserved, must keep at value \"111\""]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu050Spec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 6 - Reserved, must keep at value \"1\""]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu050Spec> {
        Reserved2W::new(self, 6)
    }
    #[doc = "Bit 7 - Reset I3C REG/DMA controller"]
    #[inline(always)]
    pub fn rst_i3cregdmactrl(&mut self) -> RstI3cregdmactrlW<Scu050Spec> {
        RstI3cregdmactrlW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset I3C0 controller"]
    #[inline(always)]
    pub fn rst_i3c0ctrl(&mut self) -> RstI3c0ctrlW<Scu050Spec> {
        RstI3c0ctrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset I3C1 controller"]
    #[inline(always)]
    pub fn rst_i3c1ctrl(&mut self) -> RstI3c1ctrlW<Scu050Spec> {
        RstI3c1ctrlW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset I3C2 controller"]
    #[inline(always)]
    pub fn rst_i3c2ctrl(&mut self) -> RstI3c2ctrlW<Scu050Spec> {
        RstI3c2ctrlW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset I3C3 controller"]
    #[inline(always)]
    pub fn rst_i3c3ctrl(&mut self) -> RstI3c3ctrlW<Scu050Spec> {
        RstI3c3ctrlW::new(self, 11)
    }
    #[doc = "Bits 12:21 - Reserved, must keep at value all \"1\""]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu050Spec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bit 22 - Reset JTAG Master 2 controller"]
    #[inline(always)]
    pub fn rst_jtagmaster2ctrl(&mut self) -> RstJtagmaster2ctrlW<Scu050Spec> {
        RstJtagmaster2ctrlW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset ADC controller"]
    #[inline(always)]
    pub fn rst_adcctrl(&mut self) -> RstAdcctrlW<Scu050Spec> {
        RstAdcctrlW::new(self, 23)
    }
    #[doc = "Bit 26 - Reset JTAG Master 1 controller"]
    #[inline(always)]
    pub fn rst_jtagmaster1ctrl(&mut self) -> RstJtagmaster1ctrlW<Scu050Spec> {
        RstJtagmaster1ctrlW::new(self, 26)
    }
    #[doc = "Bit 28 - Reset UART1 controller"]
    #[inline(always)]
    pub fn rst_uart1ctrl(&mut self) -> RstUart1ctrlW<Scu050Spec> {
        RstUart1ctrlW::new(self, 28)
    }
    #[doc = "Bit 29 - Reset UART2 controller"]
    #[inline(always)]
    pub fn rst_uart2ctrl(&mut self) -> RstUart2ctrlW<Scu050Spec> {
        RstUart2ctrlW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset UART3 controller"]
    #[inline(always)]
    pub fn rst_uart3ctrl(&mut self) -> RstUart3ctrlW<Scu050Spec> {
        RstUart3ctrlW::new(self, 30)
    }
    #[doc = "Bit 31 - Reset UART4 controller"]
    #[inline(always)]
    pub fn rst_uart4ctrl(&mut self) -> RstUart4ctrlW<Scu050Spec> {
        RstUart4ctrlW::new(self, 31)
    }
}
#[doc = "System Reset Control Register Set 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu050Spec;
impl crate::RegisterSpec for Scu050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu050::R`](R) reader structure"]
impl crate::Readable for Scu050Spec {}
#[doc = "`write(|w| ..)` method takes [`scu050::W`](W) writer structure"]
impl crate::Writable for Scu050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU050 to value 0x09ff_fffc"]
impl crate::Resettable for Scu050Spec {
    const RESET_VALUE: u32 = 0x09ff_fffc;
}

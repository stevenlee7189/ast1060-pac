#[doc = "Register `SCU070` reader"]
pub type R = crate::R<Scu070Spec>;
#[doc = "Register `SCU070` writer"]
pub type W = crate::W<Scu070Spec>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `EnblRstJTAG1MasterCtrl` reader - Enable reset JTAG #1 master controller"]
pub type EnblRstJtag1masterCtrlR = crate::BitReader;
#[doc = "Field `EnblRstJTAG1MasterCtrl` writer - Enable reset JTAG #1 master controller"]
pub type EnblRstJtag1masterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstJTAG2MasterCtrl` reader - Enable reset JTAG #2 master controller"]
pub type EnblRstJtag2masterCtrlR = crate::BitReader;
#[doc = "Field `EnblRstJTAG2MasterCtrl` writer - Enable reset JTAG #2 master controller"]
pub type EnblRstJtag2masterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstGPIOCtrl` reader - Enable reset GPIO controller"]
pub type EnblRstGpioctrlR = crate::BitReader;
#[doc = "Field `EnblRstGPIOCtrl` writer - Enable reset GPIO controller"]
pub type EnblRstGpioctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EnblRstADCCtrl` reader - Enable reset ADC controller"]
pub type EnblRstAdcctrlR = crate::BitReader;
#[doc = "Field `EnblRstADCCtrl` writer - Enable reset ADC controller"]
pub type EnblRstAdcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `EnblRstI2CI2CFLTCtrl` reader - Enable reset I2C/I2CFLT controller"]
pub type EnblRstI2ci2cfltctrlR = crate::BitReader;
#[doc = "Field `EnblRstI2CI2CFLTCtrl` writer - Enable reset I2C/I2CFLT controller"]
pub type EnblRstI2ci2cfltctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CGlobalCtrl` reader - Enable reset I3C Global controller"]
pub type EnblRstI3cglobalCtrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CGlobalCtrl` writer - Enable reset I3C Global controller"]
pub type EnblRstI3cglobalCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus1Ctrl` reader - Enable reset I3C bus1 controller"]
pub type EnblRstI3cbus1ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus1Ctrl` writer - Enable reset I3C bus1 controller"]
pub type EnblRstI3cbus1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus2Ctrl` reader - Enable reset I3C bus2 controller"]
pub type EnblRstI3cbus2ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus2Ctrl` writer - Enable reset I3C bus2 controller"]
pub type EnblRstI3cbus2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus3Ctrl` reader - Enable reset I3C bus3 controller"]
pub type EnblRstI3cbus3ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus3Ctrl` writer - Enable reset I3C bus3 controller"]
pub type EnblRstI3cbus3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus4Ctrl` reader - Enable reset I3C bus4 controller"]
pub type EnblRstI3cbus4ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus4Ctrl` writer - Enable reset I3C bus4 controller"]
pub type EnblRstI3cbus4ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSPI1SPI2Ctrl` reader - Enable reset SPI1/SPI2 controller"]
pub type EnblRstSpi1spi2ctrlR = crate::BitReader;
#[doc = "Field `EnblRstSPI1SPI2Ctrl` writer - Enable reset SPI1/SPI2 controller"]
pub type EnblRstSpi1spi2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved(0)"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable reset JTAG #1 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag1master_ctrl(&self) -> EnblRstJtag1masterCtrlR {
        EnblRstJtag1masterCtrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable reset JTAG #2 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag2master_ctrl(&self) -> EnblRstJtag2masterCtrlR {
        EnblRstJtag2masterCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_rst_gpioctrl(&self) -> EnblRstGpioctrlR {
        EnblRstGpioctrlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Enable reset ADC controller"]
    #[inline(always)]
    pub fn enbl_rst_adcctrl(&self) -> EnblRstAdcctrlR {
        EnblRstAdcctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable reset I2C/I2CFLT controller"]
    #[inline(always)]
    pub fn enbl_rst_i2ci2cfltctrl(&self) -> EnblRstI2ci2cfltctrlR {
        EnblRstI2ci2cfltctrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable reset I3C Global controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cglobal_ctrl(&self) -> EnblRstI3cglobalCtrlR {
        EnblRstI3cglobalCtrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus1ctrl(&self) -> EnblRstI3cbus1ctrlR {
        EnblRstI3cbus1ctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus2ctrl(&self) -> EnblRstI3cbus2ctrlR {
        EnblRstI3cbus2ctrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus3ctrl(&self) -> EnblRstI3cbus3ctrlR {
        EnblRstI3cbus3ctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus4ctrl(&self) -> EnblRstI3cbus4ctrlR {
        EnblRstI3cbus4ctrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable reset SPI1/SPI2 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi1spi2ctrl(&self) -> EnblRstSpi1spi2ctrlR {
        EnblRstSpi1spi2ctrlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Scu070Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 7 - Enable reset JTAG #1 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag1master_ctrl(&mut self) -> EnblRstJtag1masterCtrlW<Scu070Spec> {
        EnblRstJtag1masterCtrlW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable reset JTAG #2 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag2master_ctrl(&mut self) -> EnblRstJtag2masterCtrlW<Scu070Spec> {
        EnblRstJtag2masterCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_rst_gpioctrl(&mut self) -> EnblRstGpioctrlW<Scu070Spec> {
        EnblRstGpioctrlW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu070Spec> {
        Reserved4W::new(self, 10)
    }
    #[doc = "Bit 14 - Enable reset ADC controller"]
    #[inline(always)]
    pub fn enbl_rst_adcctrl(&mut self) -> EnblRstAdcctrlW<Scu070Spec> {
        EnblRstAdcctrlW::new(self, 14)
    }
    #[doc = "Bit 16 - Enable reset I2C/I2CFLT controller"]
    #[inline(always)]
    pub fn enbl_rst_i2ci2cfltctrl(&mut self) -> EnblRstI2ci2cfltctrlW<Scu070Spec> {
        EnblRstI2ci2cfltctrlW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable reset I3C Global controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cglobal_ctrl(&mut self) -> EnblRstI3cglobalCtrlW<Scu070Spec> {
        EnblRstI3cglobalCtrlW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus1ctrl(&mut self) -> EnblRstI3cbus1ctrlW<Scu070Spec> {
        EnblRstI3cbus1ctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus2ctrl(&mut self) -> EnblRstI3cbus2ctrlW<Scu070Spec> {
        EnblRstI3cbus2ctrlW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus3ctrl(&mut self) -> EnblRstI3cbus3ctrlW<Scu070Spec> {
        EnblRstI3cbus3ctrlW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus4ctrl(&mut self) -> EnblRstI3cbus4ctrlW<Scu070Spec> {
        EnblRstI3cbus4ctrlW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable reset SPI1/SPI2 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi1spi2ctrl(&mut self) -> EnblRstSpi1spi2ctrlW<Scu070Spec> {
        EnblRstSpi1spi2ctrlW::new(self, 22)
    }
    #[doc = "Bits 23:25 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu070Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu070Spec> {
        Reserved1W::new(self, 26)
    }
}
#[doc = "EXTRST\\# Reset Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`scu070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu070Spec;
impl crate::RegisterSpec for Scu070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu070::R`](R) reader structure"]
impl crate::Readable for Scu070Spec {}
#[doc = "`write(|w| ..)` method takes [`scu070::W`](W) writer structure"]
impl crate::Writable for Scu070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU070 to value 0x03ff_fff1"]
impl crate::Resettable for Scu070Spec {
    const RESET_VALUE: u32 = 0x03ff_fff1;
}

#[doc = "Register `GPIO128` reader"]
pub type R = crate::R<Gpio128Spec>;
#[doc = "Register `GPIO128` writer"]
pub type W = crate::W<Gpio128Spec>;
#[doc = "Field `PortGPIOQ70INTStsReg` reader - Port GPIOQ\\[7:0\\] interrupt status register"]
pub type PortGpioq70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70INTStsReg` writer - Port GPIOQ\\[7:0\\] interrupt status register"]
pub type PortGpioq70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70INTStsReg` reader - Port GPIOR\\[7:0\\] interrupt status register"]
pub type PortGpior70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOR70INTStsReg` writer - Port GPIOR\\[7:0\\] interrupt status register"]
pub type PortGpior70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70INTStsReg` reader - Port GPIOS\\[7:0\\] interrupt status register"]
pub type PortGpios70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOS70INTStsReg` writer - Port GPIOS\\[7:0\\] interrupt status register"]
pub type PortGpios70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70INTStsReg` reader - Port GPIT\\[7:0\\] interrupt status register"]
pub type PortGpit70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIT70INTStsReg` writer - Port GPIT\\[7:0\\] interrupt status register"]
pub type PortGpit70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioq70intsts_reg(&self) -> PortGpioq70intstsRegR {
        PortGpioq70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpior70intsts_reg(&self) -> PortGpior70intstsRegR {
        PortGpior70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpios70intsts_reg(&self) -> PortGpios70intstsRegR {
        PortGpios70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpit70intsts_reg(&self) -> PortGpit70intstsRegR {
        PortGpit70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioq70intsts_reg(&mut self) -> PortGpioq70intstsRegW<Gpio128Spec> {
        PortGpioq70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpior70intsts_reg(&mut self) -> PortGpior70intstsRegW<Gpio128Spec> {
        PortGpior70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpios70intsts_reg(&mut self) -> PortGpios70intstsRegW<Gpio128Spec> {
        PortGpios70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpit70intsts_reg(&mut self) -> PortGpit70intstsRegW<Gpio128Spec> {
        PortGpit70intstsRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio128Spec;
impl crate::RegisterSpec for Gpio128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio128::R`](R) reader structure"]
impl crate::Readable for Gpio128Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio128::W`](W) writer structure"]
impl crate::Writable for Gpio128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO128 to value 0"]
impl crate::Resettable for Gpio128Spec {}

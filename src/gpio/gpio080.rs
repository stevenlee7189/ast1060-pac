#[doc = "Register `GPIO080` reader"]
pub type R = crate::R<Gpio080Spec>;
#[doc = "Register `GPIO080` writer"]
pub type W = crate::W<Gpio080Spec>;
#[doc = "Field `PortGPIOQ70DataReg` reader - Port GPIOQ\\[7:0\\] data register"]
pub type PortGpioq70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70DataReg` writer - Port GPIOQ\\[7:0\\] data register"]
pub type PortGpioq70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70DataReg` reader - Port GPIOR\\[7:0\\] data register"]
pub type PortGpior70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOR70DataReg` writer - Port GPIOR\\[7:0\\] data register"]
pub type PortGpior70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70DataReg` reader - Port GPIOS\\[7:0\\] data register"]
pub type PortGpios70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOS70DataReg` writer - Port GPIOS\\[7:0\\] data register"]
pub type PortGpios70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOT70DataReg` reader - Port GPIOT\\[7:0\\] data register"]
pub type PortGpiot70dataRegR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioq70data_reg(&self) -> PortGpioq70dataRegR {
        PortGpioq70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpior70data_reg(&self) -> PortGpior70dataRegR {
        PortGpior70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpios70data_reg(&self) -> PortGpios70dataRegR {
        PortGpios70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOT\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiot70data_reg(&self) -> PortGpiot70dataRegR {
        PortGpiot70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioq70data_reg(&mut self) -> PortGpioq70dataRegW<Gpio080Spec> {
        PortGpioq70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpior70data_reg(&mut self) -> PortGpior70dataRegW<Gpio080Spec> {
        PortGpior70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpios70data_reg(&mut self) -> PortGpios70dataRegW<Gpio080Spec> {
        PortGpios70dataRegW::new(self, 16)
    }
}
#[doc = "GPIO\\_Q/R/S/T Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio080Spec;
impl crate::RegisterSpec for Gpio080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio080::R`](R) reader structure"]
impl crate::Readable for Gpio080Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio080::W`](W) writer structure"]
impl crate::Writable for Gpio080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO080 to value 0"]
impl crate::Resettable for Gpio080Spec {}

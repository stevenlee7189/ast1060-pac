#[doc = "Register `GPIO538` reader"]
pub type R = crate::R<Gpio538Spec>;
#[doc = "Register `GPIO538` writer"]
pub type W = crate::W<Gpio538Spec>;
#[doc = "Field `PortSerialGPIOI70DataReg` reader - Port Serial GPIOI\\[7:0\\] data register"]
pub type PortSerialGpioi70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOI70DataReg` writer - Port Serial GPIOI\\[7:0\\] data register"]
pub type PortSerialGpioi70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOJ70DataReg` reader - Port Serial GPIOJ\\[7:0\\] data register"]
pub type PortSerialGpioj70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOJ70DataReg` writer - Port Serial GPIOJ\\[7:0\\] data register"]
pub type PortSerialGpioj70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOK70DataReg` reader - Port Serial GPIOK\\[7:0\\] data register"]
pub type PortSerialGpiok70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOK70DataReg` writer - Port Serial GPIOK\\[7:0\\] data register"]
pub type PortSerialGpiok70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOL70DataReg` reader - Port Serial GPIOL\\[7:0\\] data register"]
pub type PortSerialGpiol70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOL70DataReg` writer - Port Serial GPIOL\\[7:0\\] data register"]
pub type PortSerialGpiol70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioi70data_reg(&self) -> PortSerialGpioi70dataRegR {
        PortSerialGpioi70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioj70data_reg(&self) -> PortSerialGpioj70dataRegR {
        PortSerialGpioj70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiok70data_reg(&self) -> PortSerialGpiok70dataRegR {
        PortSerialGpiok70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiol70data_reg(&self) -> PortSerialGpiol70dataRegR {
        PortSerialGpiol70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioi70data_reg(&mut self) -> PortSerialGpioi70dataRegW<Gpio538Spec> {
        PortSerialGpioi70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioj70data_reg(&mut self) -> PortSerialGpioj70dataRegW<Gpio538Spec> {
        PortSerialGpioj70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiok70data_reg(&mut self) -> PortSerialGpiok70dataRegW<Gpio538Spec> {
        PortSerialGpiok70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiol70data_reg(&mut self) -> PortSerialGpiol70dataRegW<Gpio538Spec> {
        PortSerialGpiol70dataRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_I/J/K/L 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio538::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio538::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio538Spec;
impl crate::RegisterSpec for Gpio538Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio538::R`](R) reader structure"]
impl crate::Readable for Gpio538Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio538::W`](W) writer structure"]
impl crate::Writable for Gpio538Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO538 to value 0"]
impl crate::Resettable for Gpio538Spec {}

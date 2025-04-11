#[doc = "Register `GPIO54C` reader"]
pub type R = crate::R<Gpio54cSpec>;
#[doc = "Register `GPIO54C` writer"]
pub type W = crate::W<Gpio54cSpec>;
#[doc = "Field `PortSerialGPIOI70INTStsReg` reader - Port Serial GPIOI\\[7:0\\] interrupt status register"]
pub type PortSerialGpioi70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOI70INTStsReg` writer - Port Serial GPIOI\\[7:0\\] interrupt status register"]
pub type PortSerialGpioi70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOJ70INTStsReg` reader - Port Serial GPIOJ\\[7:0\\] interrupt status register"]
pub type PortSerialGpioj70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOJ70INTStsReg` writer - Port Serial GPIOJ\\[7:0\\] interrupt status register"]
pub type PortSerialGpioj70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOK70INTStsReg` reader - Port Serial GPIOK\\[7:0\\] interrupt status register"]
pub type PortSerialGpiok70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOK70INTStsReg` writer - Port Serial GPIOK\\[7:0\\] interrupt status register"]
pub type PortSerialGpiok70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOL70INTStsReg` reader - Port Serial GPIOL\\[7:0\\] interrupt status register"]
pub type PortSerialGpiol70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOL70INTStsReg` writer - Port Serial GPIOL\\[7:0\\] interrupt status register"]
pub type PortSerialGpiol70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsts_reg(&self) -> PortSerialGpioi70intstsRegR {
        PortSerialGpioi70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsts_reg(&self) -> PortSerialGpioj70intstsRegR {
        PortSerialGpioj70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsts_reg(&self) -> PortSerialGpiok70intstsRegR {
        PortSerialGpiok70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsts_reg(&self) -> PortSerialGpiol70intstsRegR {
        PortSerialGpiol70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsts_reg(&mut self) -> PortSerialGpioi70intstsRegW<Gpio54cSpec> {
        PortSerialGpioi70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsts_reg(&mut self) -> PortSerialGpioj70intstsRegW<Gpio54cSpec> {
        PortSerialGpioj70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsts_reg(&mut self) -> PortSerialGpiok70intstsRegW<Gpio54cSpec> {
        PortSerialGpiok70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsts_reg(&mut self) -> PortSerialGpiol70intstsRegW<Gpio54cSpec> {
        PortSerialGpiol70intstsRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio54c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio54c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio54cSpec;
impl crate::RegisterSpec for Gpio54cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio54c::R`](R) reader structure"]
impl crate::Readable for Gpio54cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio54c::W`](W) writer structure"]
impl crate::Writable for Gpio54cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO54C to value 0"]
impl crate::Resettable for Gpio54cSpec {}

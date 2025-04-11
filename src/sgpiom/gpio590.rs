#[doc = "Register `GPIO590` reader"]
pub type R = crate::R<Gpio590Spec>;
#[doc = "Register `GPIO590` writer"]
pub type W = crate::W<Gpio590Spec>;
#[doc = "Field `PortSerialGPIOM70DataReg` reader - Port Serial GPIOM\\[7:0\\] data register"]
pub type PortSerialGpiom70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOM70DataReg` writer - Port Serial GPIOM\\[7:0\\] data register"]
pub type PortSerialGpiom70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPION70DataReg` reader - Port Serial GPION\\[7:0\\] data register"]
pub type PortSerialGpion70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPION70DataReg` writer - Port Serial GPION\\[7:0\\] data register"]
pub type PortSerialGpion70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOO70DataReg` reader - Port Serial GPIOO\\[7:0\\] data register"]
pub type PortSerialGpioo70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOO70DataReg` writer - Port Serial GPIOO\\[7:0\\] data register"]
pub type PortSerialGpioo70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOP70DataReg` reader - Port Serial GPIOP\\[7:0\\] data register"]
pub type PortSerialGpiop70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOP70DataReg` writer - Port Serial GPIOP\\[7:0\\] data register"]
pub type PortSerialGpiop70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiom70data_reg(&self) -> PortSerialGpiom70dataRegR {
        PortSerialGpiom70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpion70data_reg(&self) -> PortSerialGpion70dataRegR {
        PortSerialGpion70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioo70data_reg(&self) -> PortSerialGpioo70dataRegR {
        PortSerialGpioo70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiop70data_reg(&self) -> PortSerialGpiop70dataRegR {
        PortSerialGpiop70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiom70data_reg(&mut self) -> PortSerialGpiom70dataRegW<Gpio590Spec> {
        PortSerialGpiom70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpion70data_reg(&mut self) -> PortSerialGpion70dataRegW<Gpio590Spec> {
        PortSerialGpion70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioo70data_reg(&mut self) -> PortSerialGpioo70dataRegW<Gpio590Spec> {
        PortSerialGpioo70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiop70data_reg(&mut self) -> PortSerialGpiop70dataRegW<Gpio590Spec> {
        PortSerialGpiop70dataRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio590::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio590::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio590Spec;
impl crate::RegisterSpec for Gpio590Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio590::R`](R) reader structure"]
impl crate::Readable for Gpio590Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio590::W`](W) writer structure"]
impl crate::Writable for Gpio590Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO590 to value 0"]
impl crate::Resettable for Gpio590Spec {}

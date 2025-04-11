#[doc = "Register `GPIO070` reader"]
pub type R = crate::R<Gpio070Spec>;
#[doc = "Register `GPIO070` writer"]
pub type W = crate::W<Gpio070Spec>;
#[doc = "Field `PortGPIOI70DataReg` reader - Port GPIOI\\[7:0\\] data register"]
pub type PortGpioi70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOI70DataReg` writer - Port GPIOI\\[7:0\\] data register"]
pub type PortGpioi70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70DataReg` reader - Port GPIOJ\\[7:0\\] data register"]
pub type PortGpioj70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70DataReg` writer - Port GPIOJ\\[7:0\\] data register"]
pub type PortGpioj70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70DataReg` reader - Port GPIOK\\[7:0\\] data register"]
pub type PortGpiok70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOK70DataReg` writer - Port GPIOK\\[7:0\\] data register"]
pub type PortGpiok70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70DataReg` reader - Port GPIOL\\[7:0\\] data register"]
pub type PortGpiol70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOL70DataReg` writer - Port GPIOL\\[7:0\\] data register"]
pub type PortGpiol70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioi70data_reg(&self) -> PortGpioi70dataRegR {
        PortGpioi70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioj70data_reg(&self) -> PortGpioj70dataRegR {
        PortGpioj70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiok70data_reg(&self) -> PortGpiok70dataRegR {
        PortGpiok70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiol70data_reg(&self) -> PortGpiol70dataRegR {
        PortGpiol70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioi70data_reg(&mut self) -> PortGpioi70dataRegW<Gpio070Spec> {
        PortGpioi70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioj70data_reg(&mut self) -> PortGpioj70dataRegW<Gpio070Spec> {
        PortGpioj70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiok70data_reg(&mut self) -> PortGpiok70dataRegW<Gpio070Spec> {
        PortGpiok70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiol70data_reg(&mut self) -> PortGpiol70dataRegW<Gpio070Spec> {
        PortGpiol70dataRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio070Spec;
impl crate::RegisterSpec for Gpio070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio070::R`](R) reader structure"]
impl crate::Readable for Gpio070Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio070::W`](W) writer structure"]
impl crate::Writable for Gpio070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO070 to value 0"]
impl crate::Resettable for Gpio070Spec {}

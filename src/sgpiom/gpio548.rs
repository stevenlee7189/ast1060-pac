#[doc = "Register `GPIO548` reader"]
pub type R = crate::R<Gpio548Spec>;
#[doc = "Register `GPIO548` writer"]
pub type W = crate::W<Gpio548Spec>;
#[doc = "Field `PortSerialGPIOI70INTSensitivityType2Sel` reader - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioi70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOI70INTSensitivityType2Sel` writer - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioi70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOJ70INTSensitivityType2Sel` reader - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioj70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOJ70INTSensitivityType2Sel` writer - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioj70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOK70INTSensitivityType2Sel` reader - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiok70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOK70INTSensitivityType2Sel` writer - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiok70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOL70INTSensitivityType2Sel` reader - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiol70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOL70INTSensitivityType2Sel` writer - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiol70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpioi70intsensitivityType2selR {
        PortSerialGpioi70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpioj70intsensitivityType2selR {
        PortSerialGpioj70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiok70intsensitivityType2selR {
        PortSerialGpiok70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiol70intsensitivityType2selR {
        PortSerialGpiol70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpioi70intsensitivityType2selW<Gpio548Spec> {
        PortSerialGpioi70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpioj70intsensitivityType2selW<Gpio548Spec> {
        PortSerialGpioj70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiok70intsensitivityType2selW<Gpio548Spec> {
        PortSerialGpiok70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiol70intsensitivityType2selW<Gpio548Spec> {
        PortSerialGpiol70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio548::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio548::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio548Spec;
impl crate::RegisterSpec for Gpio548Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio548::R`](R) reader structure"]
impl crate::Readable for Gpio548Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio548::W`](W) writer structure"]
impl crate::Writable for Gpio548Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO548 to value 0"]
impl crate::Resettable for Gpio548Spec {}

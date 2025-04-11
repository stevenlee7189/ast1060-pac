#[doc = "Register `GPIO544` reader"]
pub type R = crate::R<Gpio544Spec>;
#[doc = "Register `GPIO544` writer"]
pub type W = crate::W<Gpio544Spec>;
#[doc = "Field `PortSerialGPIOI70INTSensitivityType1Sel` reader - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioi70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOI70INTSensitivityType1Sel` writer - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioi70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOJ70INTSensitivityType1Sel` reader - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioj70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOJ70INTSensitivityType1Sel` writer - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpioj70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOK70INTSensitivityType1Sel` reader - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiok70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOK70INTSensitivityType1Sel` writer - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiok70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOL70INTSensitivityType1Sel` reader - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiol70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOL70INTSensitivityType1Sel` writer - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortSerialGpiol70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpioi70intsensitivityType1selR {
        PortSerialGpioi70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpioj70intsensitivityType1selR {
        PortSerialGpioj70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiok70intsensitivityType1selR {
        PortSerialGpiok70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsensitivity_type1sel(
        &self,
    ) -> PortSerialGpiol70intsensitivityType1selR {
        PortSerialGpiol70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpioi70intsensitivityType1selW<Gpio544Spec> {
        PortSerialGpioi70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpioj70intsensitivityType1selW<Gpio544Spec> {
        PortSerialGpioj70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiok70intsensitivityType1selW<Gpio544Spec> {
        PortSerialGpiok70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsensitivity_type1sel(
        &mut self,
    ) -> PortSerialGpiol70intsensitivityType1selW<Gpio544Spec> {
        PortSerialGpiol70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio544::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio544::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio544Spec;
impl crate::RegisterSpec for Gpio544Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio544::R`](R) reader structure"]
impl crate::Readable for Gpio544Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio544::W`](W) writer structure"]
impl crate::Writable for Gpio544Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO544 to value 0"]
impl crate::Resettable for Gpio544Spec {}

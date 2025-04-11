#[doc = "Register `GPIO540` reader"]
pub type R = crate::R<Gpio540Spec>;
#[doc = "Register `GPIO540` writer"]
pub type W = crate::W<Gpio540Spec>;
#[doc = "Field `PortSerialGPIOI70INTSensitivityType0Sel` reader - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioi70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOI70INTSensitivityType0Sel` writer - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioi70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOJ70INTSensitivityType0Sel` reader - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioj70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOJ70INTSensitivityType0Sel` writer - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioj70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOK70INTSensitivityType0Sel` reader - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiok70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOK70INTSensitivityType0Sel` writer - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiok70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOL70INTSensitivityType0Sel` reader - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiol70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOL70INTSensitivityType0Sel` writer - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiol70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpioi70intsensitivityType0selR {
        PortSerialGpioi70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpioj70intsensitivityType0selR {
        PortSerialGpioj70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiok70intsensitivityType0selR {
        PortSerialGpiok70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiol70intsensitivityType0selR {
        PortSerialGpiol70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioi70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpioi70intsensitivityType0selW<Gpio540Spec> {
        PortSerialGpioi70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioj70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpioj70intsensitivityType0selW<Gpio540Spec> {
        PortSerialGpioj70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiok70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiok70intsensitivityType0selW<Gpio540Spec> {
        PortSerialGpiok70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiol70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiol70intsensitivityType0selW<Gpio540Spec> {
        PortSerialGpiol70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio540::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio540::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio540Spec;
impl crate::RegisterSpec for Gpio540Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio540::R`](R) reader structure"]
impl crate::Readable for Gpio540Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio540::W`](W) writer structure"]
impl crate::Writable for Gpio540Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO540 to value 0"]
impl crate::Resettable for Gpio540Spec {}

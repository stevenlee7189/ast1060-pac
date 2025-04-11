#[doc = "Register `GPIO09C` reader"]
pub type R = crate::R<Gpio09cSpec>;
#[doc = "Register `GPIO09C` writer"]
pub type W = crate::W<Gpio09cSpec>;
#[doc = "Field `PortGPIOI70INTSensitivityType0Sel` reader - Port GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioi70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOI70INTSensitivityType0Sel` writer - Port GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioi70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70INTSensitivityType0Sel` reader - Port GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioj70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70INTSensitivityType0Sel` writer - Port GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioj70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70INTSensitivityType0Sel` reader - Port GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiok70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOK70INTSensitivityType0Sel` writer - Port GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiok70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70INTSensitivityType0Sel` reader - Port GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiol70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOL70INTSensitivityType0Sel` writer - Port GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiol70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioi70intsensitivity_type0sel(&self) -> PortGpioi70intsensitivityType0selR {
        PortGpioi70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioj70intsensitivity_type0sel(&self) -> PortGpioj70intsensitivityType0selR {
        PortGpioj70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiok70intsensitivity_type0sel(&self) -> PortGpiok70intsensitivityType0selR {
        PortGpiok70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiol70intsensitivity_type0sel(&self) -> PortGpiol70intsensitivityType0selR {
        PortGpiol70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioi70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioi70intsensitivityType0selW<Gpio09cSpec> {
        PortGpioi70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioj70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioj70intsensitivityType0selW<Gpio09cSpec> {
        PortGpioj70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiok70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiok70intsensitivityType0selW<Gpio09cSpec> {
        PortGpiok70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiol70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiol70intsensitivityType0selW<Gpio09cSpec> {
        PortGpiol70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio09cSpec;
impl crate::RegisterSpec for Gpio09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio09c::R`](R) reader structure"]
impl crate::Readable for Gpio09cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio09c::W`](W) writer structure"]
impl crate::Writable for Gpio09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO09C to value 0"]
impl crate::Resettable for Gpio09cSpec {}

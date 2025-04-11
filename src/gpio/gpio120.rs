#[doc = "Register `GPIO120` reader"]
pub type R = crate::R<Gpio120Spec>;
#[doc = "Register `GPIO120` writer"]
pub type W = crate::W<Gpio120Spec>;
#[doc = "Field `PortGPIOQ70INTSensitivityType1Sel` reader - Port GPIOQ\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioq70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70INTSensitivityType1Sel` writer - Port GPIOQ\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioq70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70INTSensitivityType1Sel` reader - Port GPIOR\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpior70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOR70INTSensitivityType1Sel` writer - Port GPIOR\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpior70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70INTSensitivityType1Sel` reader - Port GPIOS\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpios70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOS70INTSensitivityType1Sel` writer - Port GPIOS\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpios70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70INTSensitivityType1Sel` reader - Port GPIT\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpit70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIT70INTSensitivityType1Sel` writer - Port GPIT\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpit70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioq70intsensitivity_type1sel(&self) -> PortGpioq70intsensitivityType1selR {
        PortGpioq70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpior70intsensitivity_type1sel(&self) -> PortGpior70intsensitivityType1selR {
        PortGpior70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpios70intsensitivity_type1sel(&self) -> PortGpios70intsensitivityType1selR {
        PortGpios70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpit70intsensitivity_type1sel(&self) -> PortGpit70intsensitivityType1selR {
        PortGpit70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioq70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpioq70intsensitivityType1selW<Gpio120Spec> {
        PortGpioq70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpior70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpior70intsensitivityType1selW<Gpio120Spec> {
        PortGpior70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpios70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpios70intsensitivityType1selW<Gpio120Spec> {
        PortGpios70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpit70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpit70intsensitivityType1selW<Gpio120Spec> {
        PortGpit70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio120Spec;
impl crate::RegisterSpec for Gpio120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio120::R`](R) reader structure"]
impl crate::Readable for Gpio120Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio120::W`](W) writer structure"]
impl crate::Writable for Gpio120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO120 to value 0"]
impl crate::Resettable for Gpio120Spec {}

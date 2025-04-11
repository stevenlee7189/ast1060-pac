#[doc = "Register `GPIO124` reader"]
pub type R = crate::R<Gpio124Spec>;
#[doc = "Register `GPIO124` writer"]
pub type W = crate::W<Gpio124Spec>;
#[doc = "Field `PortGPIOQ70INTSensitivityType2Sel` reader - Port GPIOQ\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioq70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70INTSensitivityType2Sel` writer - Port GPIOQ\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioq70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70INTSensitivityType2Sel` reader - Port GPIOR\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpior70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOR70INTSensitivityType2Sel` writer - Port GPIOR\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpior70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70INTSensitivityType2Sel` reader - Port GPIOS\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpios70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOS70INTSensitivityType2Sel` writer - Port GPIOS\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpios70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70INTSensitivityType2Sel` reader - Port GPIT\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpit70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIT70INTSensitivityType2Sel` writer - Port GPIT\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpit70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioq70intsensitivity_type2sel(&self) -> PortGpioq70intsensitivityType2selR {
        PortGpioq70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpior70intsensitivity_type2sel(&self) -> PortGpior70intsensitivityType2selR {
        PortGpior70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpios70intsensitivity_type2sel(&self) -> PortGpios70intsensitivityType2selR {
        PortGpios70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpit70intsensitivity_type2sel(&self) -> PortGpit70intsensitivityType2selR {
        PortGpit70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioq70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioq70intsensitivityType2selW<Gpio124Spec> {
        PortGpioq70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpior70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpior70intsensitivityType2selW<Gpio124Spec> {
        PortGpior70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpios70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpios70intsensitivityType2selW<Gpio124Spec> {
        PortGpios70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpit70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpit70intsensitivityType2selW<Gpio124Spec> {
        PortGpit70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio124Spec;
impl crate::RegisterSpec for Gpio124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio124::R`](R) reader structure"]
impl crate::Readable for Gpio124Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio124::W`](W) writer structure"]
impl crate::Writable for Gpio124Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO124 to value 0"]
impl crate::Resettable for Gpio124Spec {}

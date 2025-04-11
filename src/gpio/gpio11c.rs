#[doc = "Register `GPIO11C` reader"]
pub type R = crate::R<Gpio11cSpec>;
#[doc = "Register `GPIO11C` writer"]
pub type W = crate::W<Gpio11cSpec>;
#[doc = "Field `PortGPIOQ70INTSensitivityType0Sel` reader - Port GPIOQ\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioq70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70INTSensitivityType0Sel` writer - Port GPIOQ\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioq70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70INTSensitivityType0Sel` reader - Port GPIOR\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpior70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOR70INTSensitivityType0Sel` writer - Port GPIOR\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpior70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70INTSensitivityType0Sel` reader - Port GPIOS\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpios70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOS70INTSensitivityType0Sel` writer - Port GPIOS\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpios70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70INTSensitivityType0Sel` reader - Port GPIT\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpit70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIT70INTSensitivityType0Sel` writer - Port GPIT\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpit70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioq70intsensitivity_type0sel(&self) -> PortGpioq70intsensitivityType0selR {
        PortGpioq70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpior70intsensitivity_type0sel(&self) -> PortGpior70intsensitivityType0selR {
        PortGpior70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpios70intsensitivity_type0sel(&self) -> PortGpios70intsensitivityType0selR {
        PortGpios70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpit70intsensitivity_type0sel(&self) -> PortGpit70intsensitivityType0selR {
        PortGpit70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioq70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioq70intsensitivityType0selW<Gpio11cSpec> {
        PortGpioq70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpior70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpior70intsensitivityType0selW<Gpio11cSpec> {
        PortGpior70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpios70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpios70intsensitivityType0selW<Gpio11cSpec> {
        PortGpios70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpit70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpit70intsensitivityType0selW<Gpio11cSpec> {
        PortGpit70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio11c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio11c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio11cSpec;
impl crate::RegisterSpec for Gpio11cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio11c::R`](R) reader structure"]
impl crate::Readable for Gpio11cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio11c::W`](W) writer structure"]
impl crate::Writable for Gpio11cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO11C to value 0"]
impl crate::Resettable for Gpio11cSpec {}

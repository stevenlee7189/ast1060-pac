#[doc = "Register `GPIO0F0` reader"]
pub type R = crate::R<Gpio0f0Spec>;
#[doc = "Register `GPIO0F0` writer"]
pub type W = crate::W<Gpio0f0Spec>;
#[doc = "Field `PortGPIOM70INTSensitivityType1Sel` reader - Port GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiom70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOM70INTSensitivityType1Sel` writer - Port GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiom70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70INTSensitivityType1Sel` reader - Port GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpion70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPION70INTSensitivityType1Sel` writer - Port GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpion70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70INTSensitivityType1Sel` reader - Port GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioo70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOO70INTSensitivityType1Sel` writer - Port GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioo70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70INTSensitivityType1Sel` reader - Port GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiop70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOP70INTSensitivityType1Sel` writer - Port GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiop70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiom70intsensitivity_type1sel(&self) -> PortGpiom70intsensitivityType1selR {
        PortGpiom70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpion70intsensitivity_type1sel(&self) -> PortGpion70intsensitivityType1selR {
        PortGpion70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioo70intsensitivity_type1sel(&self) -> PortGpioo70intsensitivityType1selR {
        PortGpioo70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiop70intsensitivity_type1sel(&self) -> PortGpiop70intsensitivityType1selR {
        PortGpiop70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiom70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpiom70intsensitivityType1selW<Gpio0f0Spec> {
        PortGpiom70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpion70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpion70intsensitivityType1selW<Gpio0f0Spec> {
        PortGpion70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioo70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpioo70intsensitivityType1selW<Gpio0f0Spec> {
        PortGpioo70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiop70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpiop70intsensitivityType1selW<Gpio0f0Spec> {
        PortGpiop70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0f0Spec;
impl crate::RegisterSpec for Gpio0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0f0::R`](R) reader structure"]
impl crate::Readable for Gpio0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0f0::W`](W) writer structure"]
impl crate::Writable for Gpio0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0F0 to value 0"]
impl crate::Resettable for Gpio0f0Spec {}

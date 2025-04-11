#[doc = "Register `GPIO010` reader"]
pub type R = crate::R<Gpio010Spec>;
#[doc = "Register `GPIO010` writer"]
pub type W = crate::W<Gpio010Spec>;
#[doc = "Field `PortGPIOA70INTSensitivityType1Sel` reader - Port GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioa70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOA70INTSensitivityType1Sel` writer - Port GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioa70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70INTSensitivityType1Sel` reader - Port GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiob70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOB70INTSensitivityType1Sel` writer - Port GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiob70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70INTSensitivityType1Sel` reader - Port GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioc70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOC70INTSensitivityType1Sel` writer - Port GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioc70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70INTSensitivityType1Sel` reader - Port GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiod70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOD70INTSensitivityType1Sel` writer - Port GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiod70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioa70intsensitivity_type1sel(&self) -> PortGpioa70intsensitivityType1selR {
        PortGpioa70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiob70intsensitivity_type1sel(&self) -> PortGpiob70intsensitivityType1selR {
        PortGpiob70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioc70intsensitivity_type1sel(&self) -> PortGpioc70intsensitivityType1selR {
        PortGpioc70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiod70intsensitivity_type1sel(&self) -> PortGpiod70intsensitivityType1selR {
        PortGpiod70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioa70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpioa70intsensitivityType1selW<Gpio010Spec> {
        PortGpioa70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiob70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpiob70intsensitivityType1selW<Gpio010Spec> {
        PortGpiob70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioc70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpioc70intsensitivityType1selW<Gpio010Spec> {
        PortGpioc70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiod70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpiod70intsensitivityType1selW<Gpio010Spec> {
        PortGpiod70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio010Spec;
impl crate::RegisterSpec for Gpio010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio010::R`](R) reader structure"]
impl crate::Readable for Gpio010Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio010::W`](W) writer structure"]
impl crate::Writable for Gpio010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO010 to value 0"]
impl crate::Resettable for Gpio010Spec {}

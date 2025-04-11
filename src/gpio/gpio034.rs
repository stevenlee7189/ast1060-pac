#[doc = "Register `GPIO034` reader"]
pub type R = crate::R<Gpio034Spec>;
#[doc = "Register `GPIO034` writer"]
pub type W = crate::W<Gpio034Spec>;
#[doc = "Field `PortGPIOE70INTSensitivityType2Sel` reader - Port GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioe70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOE70INTSensitivityType2Sel` writer - Port GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioe70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70INTSensitivityType2Sel` reader - Port GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiof70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOF70INTSensitivityType2Sel` writer - Port GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiof70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70INTSensitivityType2Sel` reader - Port GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiog70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOG70INTSensitivityType2Sel` writer - Port GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiog70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70INTSensitivityType2Sel` reader - Port GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioh70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOH70INTSensitivityType2Sel` writer - Port GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioh70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioe70intsensitivity_type2sel(&self) -> PortGpioe70intsensitivityType2selR {
        PortGpioe70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiof70intsensitivity_type2sel(&self) -> PortGpiof70intsensitivityType2selR {
        PortGpiof70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiog70intsensitivity_type2sel(&self) -> PortGpiog70intsensitivityType2selR {
        PortGpiog70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioh70intsensitivity_type2sel(&self) -> PortGpioh70intsensitivityType2selR {
        PortGpioh70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioe70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioe70intsensitivityType2selW<Gpio034Spec> {
        PortGpioe70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiof70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiof70intsensitivityType2selW<Gpio034Spec> {
        PortGpiof70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiog70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiog70intsensitivityType2selW<Gpio034Spec> {
        PortGpiog70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioh70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioh70intsensitivityType2selW<Gpio034Spec> {
        PortGpioh70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio034Spec;
impl crate::RegisterSpec for Gpio034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio034::R`](R) reader structure"]
impl crate::Readable for Gpio034Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio034::W`](W) writer structure"]
impl crate::Writable for Gpio034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO034 to value 0"]
impl crate::Resettable for Gpio034Spec {}

#[doc = "Register `GPIO0EC` reader"]
pub type R = crate::R<Gpio0ecSpec>;
#[doc = "Register `GPIO0EC` writer"]
pub type W = crate::W<Gpio0ecSpec>;
#[doc = "Field `PortGPIOM70INTSensitivityType0Sel` reader - Port GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiom70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOM70INTSensitivityType0Sel` writer - Port GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiom70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70INTSensitivityType0Sel` reader - Port GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpion70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPION70INTSensitivityType0Sel` writer - Port GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpion70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70INTSensitivityType0Sel` reader - Port GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioo70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOO70INTSensitivityType0Sel` writer - Port GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioo70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70INTSensitivityType0Sel` reader - Port GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiop70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOP70INTSensitivityType0Sel` writer - Port GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiop70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiom70intsensitivity_type0sel(&self) -> PortGpiom70intsensitivityType0selR {
        PortGpiom70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpion70intsensitivity_type0sel(&self) -> PortGpion70intsensitivityType0selR {
        PortGpion70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioo70intsensitivity_type0sel(&self) -> PortGpioo70intsensitivityType0selR {
        PortGpioo70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiop70intsensitivity_type0sel(&self) -> PortGpiop70intsensitivityType0selR {
        PortGpiop70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiom70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiom70intsensitivityType0selW<Gpio0ecSpec> {
        PortGpiom70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpion70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpion70intsensitivityType0selW<Gpio0ecSpec> {
        PortGpion70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioo70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioo70intsensitivityType0selW<Gpio0ecSpec> {
        PortGpioo70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiop70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiop70intsensitivityType0selW<Gpio0ecSpec> {
        PortGpiop70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0ecSpec;
impl crate::RegisterSpec for Gpio0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0ec::R`](R) reader structure"]
impl crate::Readable for Gpio0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0ec::W`](W) writer structure"]
impl crate::Writable for Gpio0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0EC to value 0"]
impl crate::Resettable for Gpio0ecSpec {}

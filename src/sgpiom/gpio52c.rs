#[doc = "Register `GPIO52C` reader"]
pub type R = crate::R<Gpio52cSpec>;
#[doc = "Register `GPIO52C` writer"]
pub type W = crate::W<Gpio52cSpec>;
#[doc = "Field `PortSerialGPIOE70INTSensitivityType2Sel` reader - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioe70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOE70INTSensitivityType2Sel` writer - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioe70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOF70INTSensitivityType2Sel` reader - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiof70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOF70INTSensitivityType2Sel` writer - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiof70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOG70INTSensitivityType2Sel` reader - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiog70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOG70INTSensitivityType2Sel` writer - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiog70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOH70INTSensitivityType2Sel` reader - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioh70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOH70INTSensitivityType2Sel` writer - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioh70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpioe70intsensitivityType2selR {
        PortSerialGpioe70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiof70intsensitivityType2selR {
        PortSerialGpiof70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiog70intsensitivityType2selR {
        PortSerialGpiog70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpioh70intsensitivityType2selR {
        PortSerialGpioh70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpioe70intsensitivityType2selW<Gpio52cSpec> {
        PortSerialGpioe70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiof70intsensitivityType2selW<Gpio52cSpec> {
        PortSerialGpiof70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiog70intsensitivityType2selW<Gpio52cSpec> {
        PortSerialGpiog70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpioh70intsensitivityType2selW<Gpio52cSpec> {
        PortSerialGpioh70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio52c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio52c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio52cSpec;
impl crate::RegisterSpec for Gpio52cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio52c::R`](R) reader structure"]
impl crate::Readable for Gpio52cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio52c::W`](W) writer structure"]
impl crate::Writable for Gpio52cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO52C to value 0"]
impl crate::Resettable for Gpio52cSpec {}

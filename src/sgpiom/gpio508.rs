#[doc = "Register `GPIO508` reader"]
pub type R = crate::R<Gpio508Spec>;
#[doc = "Register `GPIO508` writer"]
pub type W = crate::W<Gpio508Spec>;
#[doc = "Field `PortSerialGPIOA70INTSensitivityType0Sel` reader - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioa70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOA70INTSensitivityType0Sel` writer - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioa70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOB70INTSensitivityType0Sel` reader - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiob70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOB70INTSensitivityType0Sel` writer - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiob70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOC70INTSensitivityType0Sel` reader - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioc70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOC70INTSensitivityType0Sel` writer - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioc70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOD70INTSensitivityType0Sel` reader - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiod70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOD70INTSensitivityType0Sel` writer - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiod70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpioa70intsensitivityType0selR {
        PortSerialGpioa70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiob70intsensitivityType0selR {
        PortSerialGpiob70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpioc70intsensitivityType0selR {
        PortSerialGpioc70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiod70intsensitivityType0selR {
        PortSerialGpiod70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioa70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpioa70intsensitivityType0selW<Gpio508Spec> {
        PortSerialGpioa70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiob70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiob70intsensitivityType0selW<Gpio508Spec> {
        PortSerialGpiob70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioc70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpioc70intsensitivityType0selW<Gpio508Spec> {
        PortSerialGpioc70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiod70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiod70intsensitivityType0selW<Gpio508Spec> {
        PortSerialGpiod70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio508::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio508::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio508Spec;
impl crate::RegisterSpec for Gpio508Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio508::R`](R) reader structure"]
impl crate::Readable for Gpio508Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio508::W`](W) writer structure"]
impl crate::Writable for Gpio508Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO508 to value 0"]
impl crate::Resettable for Gpio508Spec {}

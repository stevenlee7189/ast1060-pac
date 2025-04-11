#[doc = "Register `GPIO0F8` reader"]
pub type R = crate::R<Gpio0f8Spec>;
#[doc = "Register `GPIO0F8` writer"]
pub type W = crate::W<Gpio0f8Spec>;
#[doc = "Field `PortGPIOM70INTStsReg` reader - Port GPIOM\\[7:0\\] interrupt status register"]
pub type PortGpiom70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOM70INTStsReg` writer - Port GPIOM\\[7:0\\] interrupt status register"]
pub type PortGpiom70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70INTStsReg` reader - Port GPION\\[7:0\\] interrupt status register"]
pub type PortGpion70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPION70INTStsReg` writer - Port GPION\\[7:0\\] interrupt status register"]
pub type PortGpion70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70INTStsReg` reader - Port GPIOO\\[7:0\\] interrupt status register"]
pub type PortGpioo70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOO70INTStsReg` writer - Port GPIOO\\[7:0\\] interrupt status register"]
pub type PortGpioo70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70INTStsReg` reader - Port GPIOP\\[7:0\\] interrupt status register"]
pub type PortGpiop70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOP70INTStsReg` writer - Port GPIOP\\[7:0\\] interrupt status register"]
pub type PortGpiop70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiom70intsts_reg(&self) -> PortGpiom70intstsRegR {
        PortGpiom70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpion70intsts_reg(&self) -> PortGpion70intstsRegR {
        PortGpion70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioo70intsts_reg(&self) -> PortGpioo70intstsRegR {
        PortGpioo70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiop70intsts_reg(&self) -> PortGpiop70intstsRegR {
        PortGpiop70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiom70intsts_reg(&mut self) -> PortGpiom70intstsRegW<Gpio0f8Spec> {
        PortGpiom70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpion70intsts_reg(&mut self) -> PortGpion70intstsRegW<Gpio0f8Spec> {
        PortGpion70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioo70intsts_reg(&mut self) -> PortGpioo70intstsRegW<Gpio0f8Spec> {
        PortGpioo70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiop70intsts_reg(&mut self) -> PortGpiop70intstsRegW<Gpio0f8Spec> {
        PortGpiop70intstsRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0f8Spec;
impl crate::RegisterSpec for Gpio0f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0f8::R`](R) reader structure"]
impl crate::Readable for Gpio0f8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0f8::W`](W) writer structure"]
impl crate::Writable for Gpio0f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0F8 to value 0"]
impl crate::Resettable for Gpio0f8Spec {}

#[doc = "Register `GPIO000` reader"]
pub type R = crate::R<Gpio000Spec>;
#[doc = "Register `GPIO000` writer"]
pub type W = crate::W<Gpio000Spec>;
#[doc = "Field `PortGPIOA70DataReg` reader - Port GPIOA\\[7:0\\] data register"]
pub type PortGpioa70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOA70DataReg` writer - Port GPIOA\\[7:0\\] data register"]
pub type PortGpioa70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70DataReg` reader - Port GPIOB\\[7:0\\] data register"]
pub type PortGpiob70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOB70DataReg` writer - Port GPIOB\\[7:0\\] data register"]
pub type PortGpiob70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70DataReg` reader - Port GPIOC\\[7:0\\] data register"]
pub type PortGpioc70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOC70DataReg` writer - Port GPIOC\\[7:0\\] data register"]
pub type PortGpioc70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70DataReg` reader - Port GPIOD\\[7:0\\] data register"]
pub type PortGpiod70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOD70DataReg` writer - Port GPIOD\\[7:0\\] data register"]
pub type PortGpiod70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioa70data_reg(&self) -> PortGpioa70dataRegR {
        PortGpioa70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiob70data_reg(&self) -> PortGpiob70dataRegR {
        PortGpiob70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioc70data_reg(&self) -> PortGpioc70dataRegR {
        PortGpioc70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiod70data_reg(&self) -> PortGpiod70dataRegR {
        PortGpiod70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioa70data_reg(&mut self) -> PortGpioa70dataRegW<Gpio000Spec> {
        PortGpioa70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiob70data_reg(&mut self) -> PortGpiob70dataRegW<Gpio000Spec> {
        PortGpiob70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioc70data_reg(&mut self) -> PortGpioc70dataRegW<Gpio000Spec> {
        PortGpioc70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiod70data_reg(&mut self) -> PortGpiod70dataRegW<Gpio000Spec> {
        PortGpiod70dataRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio000Spec;
impl crate::RegisterSpec for Gpio000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio000::R`](R) reader structure"]
impl crate::Readable for Gpio000Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio000::W`](W) writer structure"]
impl crate::Writable for Gpio000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO000 to value 0"]
impl crate::Resettable for Gpio000Spec {}

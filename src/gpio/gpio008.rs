#[doc = "Register `GPIO008` reader"]
pub type R = crate::R<Gpio008Spec>;
#[doc = "Register `GPIO008` writer"]
pub type W = crate::W<Gpio008Spec>;
#[doc = "Field `PortGPIOA70INTEnbl` reader - Port GPIOA\\[7:0\\] interrupt enable"]
pub type PortGpioa70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOA70INTEnbl` writer - Port GPIOA\\[7:0\\] interrupt enable"]
pub type PortGpioa70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70INTEnbl` reader - Port GPIOB\\[7:0\\] interrupt enable"]
pub type PortGpiob70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOB70INTEnbl` writer - Port GPIOB\\[7:0\\] interrupt enable"]
pub type PortGpiob70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70INTEnbl` reader - Port GPIOC\\[7:0\\] interrupt enable"]
pub type PortGpioc70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOC70INTEnbl` writer - Port GPIOC\\[7:0\\] interrupt enable"]
pub type PortGpioc70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70INTEnbl` reader - Port GPIOD\\[7:0\\] interrupt enable"]
pub type PortGpiod70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOD70INTEnbl` writer - Port GPIOD\\[7:0\\] interrupt enable"]
pub type PortGpiod70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioa70intenbl(&self) -> PortGpioa70intenblR {
        PortGpioa70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiob70intenbl(&self) -> PortGpiob70intenblR {
        PortGpiob70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioc70intenbl(&self) -> PortGpioc70intenblR {
        PortGpioc70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiod70intenbl(&self) -> PortGpiod70intenblR {
        PortGpiod70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioa70intenbl(&mut self) -> PortGpioa70intenblW<Gpio008Spec> {
        PortGpioa70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiob70intenbl(&mut self) -> PortGpiob70intenblW<Gpio008Spec> {
        PortGpiob70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioc70intenbl(&mut self) -> PortGpioc70intenblW<Gpio008Spec> {
        PortGpioc70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiod70intenbl(&mut self) -> PortGpiod70intenblW<Gpio008Spec> {
        PortGpiod70intenblW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio008Spec;
impl crate::RegisterSpec for Gpio008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio008::R`](R) reader structure"]
impl crate::Readable for Gpio008Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio008::W`](W) writer structure"]
impl crate::Writable for Gpio008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO008 to value 0"]
impl crate::Resettable for Gpio008Spec {}

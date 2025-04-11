#[doc = "Register `GPIO504` reader"]
pub type R = crate::R<Gpio504Spec>;
#[doc = "Register `GPIO504` writer"]
pub type W = crate::W<Gpio504Spec>;
#[doc = "Field `PortSerialGPIOA70INTEnbl` reader - Port Serial GPIOA\\[7:0\\] interrupt enable"]
pub type PortSerialGpioa70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOA70INTEnbl` writer - Port Serial GPIOA\\[7:0\\] interrupt enable"]
pub type PortSerialGpioa70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOB70INTEnbl` reader - Port Serial GPIOB\\[7:0\\] interrupt enable"]
pub type PortSerialGpiob70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOB70INTEnbl` writer - Port Serial GPIOB\\[7:0\\] interrupt enable"]
pub type PortSerialGpiob70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOC70INTEnbl` reader - Port Serial GPIOC\\[7:0\\] interrupt enable"]
pub type PortSerialGpioc70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOC70INTEnbl` writer - Port Serial GPIOC\\[7:0\\] interrupt enable"]
pub type PortSerialGpioc70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOD70INTEnbl` reader - Port Serial GPIOD\\[7:0\\] interrupt enable"]
pub type PortSerialGpiod70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOD70INTEnbl` writer - Port Serial GPIOD\\[7:0\\] interrupt enable"]
pub type PortSerialGpiod70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioa70intenbl(&self) -> PortSerialGpioa70intenblR {
        PortSerialGpioa70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiob70intenbl(&self) -> PortSerialGpiob70intenblR {
        PortSerialGpiob70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioc70intenbl(&self) -> PortSerialGpioc70intenblR {
        PortSerialGpioc70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiod70intenbl(&self) -> PortSerialGpiod70intenblR {
        PortSerialGpiod70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioa70intenbl(&mut self) -> PortSerialGpioa70intenblW<Gpio504Spec> {
        PortSerialGpioa70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiob70intenbl(&mut self) -> PortSerialGpiob70intenblW<Gpio504Spec> {
        PortSerialGpiob70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioc70intenbl(&mut self) -> PortSerialGpioc70intenblW<Gpio504Spec> {
        PortSerialGpioc70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiod70intenbl(&mut self) -> PortSerialGpiod70intenblW<Gpio504Spec> {
        PortSerialGpiod70intenblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio504::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio504::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio504Spec;
impl crate::RegisterSpec for Gpio504Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio504::R`](R) reader structure"]
impl crate::Readable for Gpio504Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio504::W`](W) writer structure"]
impl crate::Writable for Gpio504Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO504 to value 0"]
impl crate::Resettable for Gpio504Spec {}

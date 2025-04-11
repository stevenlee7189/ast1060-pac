#[doc = "Register `GPIO028` reader"]
pub type R = crate::R<Gpio028Spec>;
#[doc = "Register `GPIO028` writer"]
pub type W = crate::W<Gpio028Spec>;
#[doc = "Field `PortGPIOE70INTEnbl` reader - Port GPIOE\\[7:0\\] interrupt enable"]
pub type PortGpioe70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOE70INTEnbl` writer - Port GPIOE\\[7:0\\] interrupt enable"]
pub type PortGpioe70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70INTEnbl` reader - Port GPIOF\\[7:0\\] interrupt enable"]
pub type PortGpiof70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOF70INTEnbl` writer - Port GPIOF\\[7:0\\] interrupt enable"]
pub type PortGpiof70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70INTEnbl` reader - Port GPIOG\\[7:0\\] interrupt enable"]
pub type PortGpiog70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOG70INTEnbl` writer - Port GPIOG\\[7:0\\] interrupt enable"]
pub type PortGpiog70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70INTEnbl` reader - Port GPIOH\\[7:0\\] interrupt enable"]
pub type PortGpioh70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOH70INTEnbl` writer - Port GPIOH\\[7:0\\] interrupt enable"]
pub type PortGpioh70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioe70intenbl(&self) -> PortGpioe70intenblR {
        PortGpioe70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiof70intenbl(&self) -> PortGpiof70intenblR {
        PortGpiof70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiog70intenbl(&self) -> PortGpiog70intenblR {
        PortGpiog70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioh70intenbl(&self) -> PortGpioh70intenblR {
        PortGpioh70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioe70intenbl(&mut self) -> PortGpioe70intenblW<Gpio028Spec> {
        PortGpioe70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiof70intenbl(&mut self) -> PortGpiof70intenblW<Gpio028Spec> {
        PortGpiof70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiog70intenbl(&mut self) -> PortGpiog70intenblW<Gpio028Spec> {
        PortGpiog70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioh70intenbl(&mut self) -> PortGpioh70intenblW<Gpio028Spec> {
        PortGpioh70intenblW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio028Spec;
impl crate::RegisterSpec for Gpio028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio028::R`](R) reader structure"]
impl crate::Readable for Gpio028Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio028::W`](W) writer structure"]
impl crate::Writable for Gpio028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO028 to value 0"]
impl crate::Resettable for Gpio028Spec {}

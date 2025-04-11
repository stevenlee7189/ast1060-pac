#[doc = "Register `I2CFILTERTHR08` reader"]
pub type R = crate::R<I2cfilterthr08Spec>;
#[doc = "Register `I2CFILTERTHR08` writer"]
pub type W = crate::W<I2cfilterthr08Spec>;
#[doc = "Field `ADDR` reader - ADDR"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDR"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<I2cfilterthr08Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THRN\\_ADR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr08Spec;
impl crate::RegisterSpec for I2cfilterthr08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr08::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr08Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr08::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR08 to value 0"]
impl crate::Resettable for I2cfilterthr08Spec {}

#[doc = "Register `I2CC08` reader"]
pub type R = crate::R<I2cc08Spec>;
#[doc = "Register `I2CC08` writer"]
pub type W = crate::W<I2cc08Spec>;
#[doc = "Field `SameAsI2CD20` reader - Same as hlinkI2CD20"]
pub type SameAsI2cd20R = crate::FieldReader<u16>;
#[doc = "Field `SameAsI2CD20` writer - Same as hlinkI2CD20"]
pub type SameAsI2cd20W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SameAsI2CD14` reader - Same as hlinkI2CD14"]
pub type SameAsI2cd14R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Same as hlinkI2CD20"]
    #[inline(always)]
    pub fn same_as_i2cd20(&self) -> SameAsI2cd20R {
        SameAsI2cd20R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Same as hlinkI2CD14"]
    #[inline(always)]
    pub fn same_as_i2cd14(&self) -> SameAsI2cd14R {
        SameAsI2cd14R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Same as hlinkI2CD20"]
    #[inline(always)]
    pub fn same_as_i2cd20(&mut self) -> SameAsI2cd20W<I2cc08Spec> {
        SameAsI2cd20W::new(self, 0)
    }
}
#[doc = "Master/Slave Transmit/Receive Byte Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc08Spec;
impl crate::RegisterSpec for I2cc08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc08::R`](R) reader structure"]
impl crate::Readable for I2cc08Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cc08::W`](W) writer structure"]
impl crate::Writable for I2cc08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC08 to value 0x0a06_0000"]
impl crate::Resettable for I2cc08Spec {
    const RESET_VALUE: u32 = 0x0a06_0000;
}

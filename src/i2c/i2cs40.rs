#[doc = "Register `I2CS40` reader"]
pub type R = crate::R<I2cs40Spec>;
#[doc = "Register `I2CS40` writer"]
pub type W = crate::W<I2cs40Spec>;
#[doc = "Field `SameAsI2CD18` reader - Same as hlinkI2CD18"]
pub type SameAsI2cd18R = crate::FieldReader<u32>;
#[doc = "Field `SameAsI2CD18` writer - Same as hlinkI2CD18"]
pub type SameAsI2cd18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Same as hlinkI2CD18"]
    #[inline(always)]
    pub fn same_as_i2cd18(&self) -> SameAsI2cd18R {
        SameAsI2cd18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Same as hlinkI2CD18"]
    #[inline(always)]
    pub fn same_as_i2cd18(&mut self) -> SameAsI2cd18W<I2cs40Spec> {
        SameAsI2cd18W::new(self, 0)
    }
}
#[doc = "Slave Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs40Spec;
impl crate::RegisterSpec for I2cs40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs40::R`](R) reader structure"]
impl crate::Readable for I2cs40Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cs40::W`](W) writer structure"]
impl crate::Writable for I2cs40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS40 to value 0"]
impl crate::Resettable for I2cs40Spec {}

#[doc = "Register `SCU5D0` reader"]
pub type R = crate::R<Scu5d0Spec>;
#[doc = "Register `SCU5D0` writer"]
pub type W = crate::W<Scu5d0Spec>;
#[doc = "Field `Reserved2` reader - 0] read back"]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 0] read back"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(self.bits)
    }
}
impl W {}
#[doc = "Reserved Read Only ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5d0Spec;
impl crate::RegisterSpec for Scu5d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5d0::R`](R) reader structure"]
impl crate::Readable for Scu5d0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5d0::W`](W) writer structure"]
impl crate::Writable for Scu5d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5D0 to value 0"]
impl crate::Resettable for Scu5d0Spec {}

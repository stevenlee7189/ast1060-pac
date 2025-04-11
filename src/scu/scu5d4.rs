#[doc = "Register `SCU5D4` reader"]
pub type R = crate::R<Scu5d4Spec>;
#[doc = "Register `SCU5D4` writer"]
pub type W = crate::W<Scu5d4Spec>;
#[doc = "Field `Reserved3` reader - 0] read back"]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 0] read back"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(self.bits)
    }
}
impl W {}
#[doc = "Reserved Read Only ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5d4Spec;
impl crate::RegisterSpec for Scu5d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5d4::R`](R) reader structure"]
impl crate::Readable for Scu5d4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5d4::W`](W) writer structure"]
impl crate::Writable for Scu5d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5D4 to value 0"]
impl crate::Resettable for Scu5d4Spec {}

#[doc = "Register `SECURE000` reader"]
pub type R = crate::R<Secure000Spec>;
#[doc = "Register `SECURE000` writer"]
pub type W = crate::W<Secure000Spec>;
#[doc = "Field `ProtKey` reader - Protection Key"]
pub type ProtKeyR = crate::FieldReader<u32>;
#[doc = "Field `ProtKey` writer - Protection Key"]
pub type ProtKeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Protection Key"]
    #[inline(always)]
    pub fn prot_key(&self) -> ProtKeyR {
        ProtKeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Protection Key"]
    #[inline(always)]
    pub fn prot_key(&mut self) -> ProtKeyW<Secure000Spec> {
        ProtKeyW::new(self, 0)
    }
}
#[doc = "Protection Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure000Spec;
impl crate::RegisterSpec for Secure000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure000::R`](R) reader structure"]
impl crate::Readable for Secure000Spec {}
#[doc = "`write(|w| ..)` method takes [`secure000::W`](W) writer structure"]
impl crate::Writable for Secure000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE000 to value 0"]
impl crate::Resettable for Secure000Spec {}

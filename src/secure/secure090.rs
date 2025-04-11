#[doc = "Register `SECURE090` reader"]
pub type R = crate::R<Secure090Spec>;
#[doc = "Register `SECURE090` writer"]
pub type W = crate::W<Secure090Spec>;
#[doc = "Field `SecBootCounterReg` reader - Secure Boot Counter Register"]
pub type SecBootCounterRegR = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Secure Boot Counter Register"]
    #[inline(always)]
    pub fn sec_boot_counter_reg(&self) -> SecBootCounterRegR {
        SecBootCounterRegR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Secure Boot Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure090Spec;
impl crate::RegisterSpec for Secure090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure090::R`](R) reader structure"]
impl crate::Readable for Secure090Spec {}
#[doc = "`write(|w| ..)` method takes [`secure090::W`](W) writer structure"]
impl crate::Writable for Secure090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE090 to value 0"]
impl crate::Resettable for Secure090Spec {}

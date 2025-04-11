#[doc = "Register `SECURE030` reader"]
pub type R = crate::R<Secure030Spec>;
#[doc = "Register `SECURE030` writer"]
pub type W = crate::W<Secure030Spec>;
#[doc = "Field `OTPTRAPDataReadBack1` reader - OTPTRAP data read back 1"]
pub type OtptrapdataReadBack1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OTPTRAP data read back 1"]
    #[inline(always)]
    pub fn otptrapdata_read_back1(&self) -> OtptrapdataReadBack1R {
        OtptrapdataReadBack1R::new(self.bits)
    }
}
impl W {}
#[doc = "OTPTRAP data read back 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure030Spec;
impl crate::RegisterSpec for Secure030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure030::R`](R) reader structure"]
impl crate::Readable for Secure030Spec {}
#[doc = "`write(|w| ..)` method takes [`secure030::W`](W) writer structure"]
impl crate::Writable for Secure030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE030 to value 0"]
impl crate::Resettable for Secure030Spec {}

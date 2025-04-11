#[doc = "Register `SECURE048` reader"]
pub type R = crate::R<Secure048Spec>;
#[doc = "Register `SECURE048` writer"]
pub type W = crate::W<Secure048Spec>;
#[doc = "Field `OTPQMRADataReadBack` reader - OTP QMRA data read back"]
pub type OtpqmradataReadBackR = crate::FieldReader<u16>;
#[doc = "Field `OTPQMRBDataReadBack` reader - OTP QMRB data read back"]
pub type OtpqmrbdataReadBackR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - OTP QMRA data read back"]
    #[inline(always)]
    pub fn otpqmradata_read_back(&self) -> OtpqmradataReadBackR {
        OtpqmradataReadBackR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OTP QMRB data read back"]
    #[inline(always)]
    pub fn otpqmrbdata_read_back(&self) -> OtpqmrbdataReadBackR {
        OtpqmrbdataReadBackR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "OTP QMRA and QMRB data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure048Spec;
impl crate::RegisterSpec for Secure048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure048::R`](R) reader structure"]
impl crate::Readable for Secure048Spec {}
#[doc = "`write(|w| ..)` method takes [`secure048::W`](W) writer structure"]
impl crate::Writable for Secure048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE048 to value 0"]
impl crate::Resettable for Secure048Spec {}

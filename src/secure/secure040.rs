#[doc = "Register `SECURE040` reader"]
pub type R = crate::R<Secure040Spec>;
#[doc = "Register `SECURE040` writer"]
pub type W = crate::W<Secure040Spec>;
#[doc = "Field `OTPQSRDataReadBack` reader - OTP QSR data read back"]
pub type OtpqsrdataReadBackR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OTP QSR data read back"]
    #[inline(always)]
    pub fn otpqsrdata_read_back(&self) -> OtpqsrdataReadBackR {
        OtpqsrdataReadBackR::new(self.bits)
    }
}
impl W {}
#[doc = "OTP QSR data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure040Spec;
impl crate::RegisterSpec for Secure040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure040::R`](R) reader structure"]
impl crate::Readable for Secure040Spec {}
#[doc = "`write(|w| ..)` method takes [`secure040::W`](W) writer structure"]
impl crate::Writable for Secure040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE040 to value 0"]
impl crate::Resettable for Secure040Spec {}

#[doc = "Register `SECURE024` reader"]
pub type R = crate::R<Secure024Spec>;
#[doc = "Register `SECURE024` writer"]
pub type W = crate::W<Secure024Spec>;
#[doc = "Field `OTPDataCompareReg2` reader - OTP Data Compare Register 2"]
pub type OtpdataCompareReg2R = crate::FieldReader<u32>;
#[doc = "Field `OTPDataCompareReg2` writer - OTP Data Compare Register 2"]
pub type OtpdataCompareReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP Data Compare Register 2"]
    #[inline(always)]
    pub fn otpdata_compare_reg2(&self) -> OtpdataCompareReg2R {
        OtpdataCompareReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP Data Compare Register 2"]
    #[inline(always)]
    pub fn otpdata_compare_reg2(&mut self) -> OtpdataCompareReg2W<Secure024Spec> {
        OtpdataCompareReg2W::new(self, 0)
    }
}
#[doc = "OTP Data Compare Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure024Spec;
impl crate::RegisterSpec for Secure024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure024::R`](R) reader structure"]
impl crate::Readable for Secure024Spec {}
#[doc = "`write(|w| ..)` method takes [`secure024::W`](W) writer structure"]
impl crate::Writable for Secure024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE024 to value 0"]
impl crate::Resettable for Secure024Spec {}

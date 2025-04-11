#[doc = "Register `SECURE020` reader"]
pub type R = crate::R<Secure020Spec>;
#[doc = "Register `SECURE020` writer"]
pub type W = crate::W<Secure020Spec>;
#[doc = "Field `OTPDataCompareReg1` reader - OTP Data Compare Register 1"]
pub type OtpdataCompareReg1R = crate::FieldReader<u32>;
#[doc = "Field `OTPDataCompareReg1` writer - OTP Data Compare Register 1"]
pub type OtpdataCompareReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP Data Compare Register 1"]
    #[inline(always)]
    pub fn otpdata_compare_reg1(&self) -> OtpdataCompareReg1R {
        OtpdataCompareReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP Data Compare Register 1"]
    #[inline(always)]
    pub fn otpdata_compare_reg1(&mut self) -> OtpdataCompareReg1W<Secure020Spec> {
        OtpdataCompareReg1W::new(self, 0)
    }
}
#[doc = "OTP Data Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure020Spec;
impl crate::RegisterSpec for Secure020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure020::R`](R) reader structure"]
impl crate::Readable for Secure020Spec {}
#[doc = "`write(|w| ..)` method takes [`secure020::W`](W) writer structure"]
impl crate::Writable for Secure020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE020 to value 0"]
impl crate::Resettable for Secure020Spec {}

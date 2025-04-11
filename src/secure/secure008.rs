#[doc = "Register `SECURE008` reader"]
pub type R = crate::R<Secure008Spec>;
#[doc = "Register `SECURE008` writer"]
pub type W = crate::W<Secure008Spec>;
#[doc = "Field `OTPWrCycleTime` reader - OTP write cycle time"]
pub type OtpwrCycleTimeR = crate::FieldReader<u16>;
#[doc = "Field `OTPWrCycleTime` writer - OTP write cycle time"]
pub type OtpwrCycleTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OTPWrWaitTime` reader - OTP write wait time"]
pub type OtpwrWaitTimeR = crate::FieldReader;
#[doc = "Field `OTPWrWaitTime` writer - OTP write wait time"]
pub type OtpwrWaitTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OTPReadCycleTime` reader - OTP read cycle time"]
pub type OtpreadCycleTimeR = crate::FieldReader;
#[doc = "Field `OTPReadCycleTime` writer - OTP read cycle time"]
pub type OtpreadCycleTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - OTP write cycle time"]
    #[inline(always)]
    pub fn otpwr_cycle_time(&self) -> OtpwrCycleTimeR {
        OtpwrCycleTimeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - OTP write wait time"]
    #[inline(always)]
    pub fn otpwr_wait_time(&self) -> OtpwrWaitTimeR {
        OtpwrWaitTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - OTP read cycle time"]
    #[inline(always)]
    pub fn otpread_cycle_time(&self) -> OtpreadCycleTimeR {
        OtpreadCycleTimeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - OTP write cycle time"]
    #[inline(always)]
    pub fn otpwr_cycle_time(&mut self) -> OtpwrCycleTimeW<Secure008Spec> {
        OtpwrCycleTimeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - OTP write wait time"]
    #[inline(always)]
    pub fn otpwr_wait_time(&mut self) -> OtpwrWaitTimeW<Secure008Spec> {
        OtpwrWaitTimeW::new(self, 16)
    }
    #[doc = "Bits 24:27 - OTP read cycle time"]
    #[inline(always)]
    pub fn otpread_cycle_time(&mut self) -> OtpreadCycleTimeW<Secure008Spec> {
        OtpreadCycleTimeW::new(self, 24)
    }
}
#[doc = "OTP Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure008Spec;
impl crate::RegisterSpec for Secure008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure008::R`](R) reader structure"]
impl crate::Readable for Secure008Spec {}
#[doc = "`write(|w| ..)` method takes [`secure008::W`](W) writer structure"]
impl crate::Writable for Secure008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE008 to value 0x0419_0760"]
impl crate::Resettable for Secure008Spec {
    const RESET_VALUE: u32 = 0x0419_0760;
}

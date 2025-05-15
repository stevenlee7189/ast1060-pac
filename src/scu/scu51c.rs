#[doc = "Register `SCU51C` reader"]
pub type R = crate::R<Scu51cSpec>;
#[doc = "Register `SCU51C` writer"]
pub type W = crate::W<Scu51cSpec>;
#[doc = "Field `EnblLowSecuritySecBootKeyByPinStrapFWSPIMISO` reader - Enable Low Security Secure Boot Key by Pin Strap (FWSPIMISO)"]
pub type EnblLowSecuritySecBootKeyByPinStrapFwspimisoR = crate::BitReader;
#[doc = "Field `EnblLowSecuritySecBootKeyByPinStrapFWSPIMISO` writer - Enable Low Security Secure Boot Key by Pin Strap (FWSPIMISO)"]
pub type EnblLowSecuritySecBootKeyByPinStrapFwspimisoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSecBootByOTPStrapOrPinStrapOTPSTRAP0` reader - Enable Secure Boot by OTP Strap or Pin Strap (hlinkOTPSTRAP\\[0\\])"]
pub type EnblSecBootByOtpstrapOrPinStrapOtpstrap0R = crate::BitReader;
#[doc = "Field `EnblSecBootByOTPStrapOrPinStrapOTPSTRAP0` writer - Enable Secure Boot by OTP Strap or Pin Strap (hlinkOTPSTRAP\\[0\\])"]
pub type EnblSecBootByOtpstrapOrPinStrapOtpstrap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MirrorBitOfORPSTRAP22` reader - Mirror Bit of ORPSTRAP\\[22\\]"]
pub type MirrorBitOfOrpstrap22R = crate::BitReader;
#[doc = "Field `MirrorBitOfORPSTRAP22` writer - Mirror Bit of ORPSTRAP\\[22\\]"]
pub type MirrorBitOfOrpstrap22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MirrorBitOfORPSTRAP24` reader - Mirror Bit of ORPSTRAP\\[24\\]"]
pub type MirrorBitOfOrpstrap24R = crate::BitReader;
#[doc = "Field `MirrorBitOfORPSTRAP24` writer - Mirror Bit of ORPSTRAP\\[24\\]"]
pub type MirrorBitOfOrpstrap24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MirrorBitOfORPSTRAP25` reader - Mirror Bit of ORPSTRAP\\[25\\]"]
pub type MirrorBitOfOrpstrap25R = crate::BitReader;
#[doc = "Field `MirrorBitOfORPSTRAP25` writer - Mirror Bit of ORPSTRAP\\[25\\]"]
pub type MirrorBitOfOrpstrap25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MirrorBitOfORPSTRAP26` reader - Mirror Bit of ORPSTRAP\\[26\\]"]
pub type MirrorBitOfOrpstrap26R = crate::BitReader;
#[doc = "Field `MirrorBitOfORPSTRAP26` writer - Mirror Bit of ORPSTRAP\\[26\\]"]
pub type MirrorBitOfOrpstrap26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved (SPI1CK)"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved (SPI1CK)"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved (SPI1DQ0)"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved (SPI1DQ0)"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved (SPI1DQ1)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved (SPI1DQ1)"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved (SGPMCK)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved (SGPMCK)"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved (SGPMO)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved (SGPMO)"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Low Security Secure Boot Key by Pin Strap (FWSPIMISO)"]
    #[inline(always)]
    pub fn enbl_low_security_sec_boot_key_by_pin_strap_fwspimiso(
        &self,
    ) -> EnblLowSecuritySecBootKeyByPinStrapFwspimisoR {
        EnblLowSecuritySecBootKeyByPinStrapFwspimisoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Secure Boot by OTP Strap or Pin Strap (hlinkOTPSTRAP\\[0\\])"]
    #[inline(always)]
    pub fn enbl_sec_boot_by_otpstrap_or_pin_strap_otpstrap0(
        &self,
    ) -> EnblSecBootByOtpstrapOrPinStrapOtpstrap0R {
        EnblSecBootByOtpstrapOrPinStrapOtpstrap0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mirror Bit of ORPSTRAP\\[22\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap22(&self) -> MirrorBitOfOrpstrap22R {
        MirrorBitOfOrpstrap22R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mirror Bit of ORPSTRAP\\[24\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap24(&self) -> MirrorBitOfOrpstrap24R {
        MirrorBitOfOrpstrap24R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mirror Bit of ORPSTRAP\\[25\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap25(&self) -> MirrorBitOfOrpstrap25R {
        MirrorBitOfOrpstrap25R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mirror Bit of ORPSTRAP\\[26\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap26(&self) -> MirrorBitOfOrpstrap26R {
        MirrorBitOfOrpstrap26R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved (SPI1CK)"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved (SPI1DQ0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved (SPI1DQ1)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved (SGPMCK)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved (SGPMO)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Low Security Secure Boot Key by Pin Strap (FWSPIMISO)"]
    #[inline(always)]
    pub fn enbl_low_security_sec_boot_key_by_pin_strap_fwspimiso(
        &mut self,
    ) -> EnblLowSecuritySecBootKeyByPinStrapFwspimisoW<Scu51cSpec> {
        EnblLowSecuritySecBootKeyByPinStrapFwspimisoW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Secure Boot by OTP Strap or Pin Strap (hlinkOTPSTRAP\\[0\\])"]
    #[inline(always)]
    pub fn enbl_sec_boot_by_otpstrap_or_pin_strap_otpstrap0(
        &mut self,
    ) -> EnblSecBootByOtpstrapOrPinStrapOtpstrap0W<Scu51cSpec> {
        EnblSecBootByOtpstrapOrPinStrapOtpstrap0W::new(self, 1)
    }
    #[doc = "Bit 2 - Mirror Bit of ORPSTRAP\\[22\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap22(&mut self) -> MirrorBitOfOrpstrap22W<Scu51cSpec> {
        MirrorBitOfOrpstrap22W::new(self, 2)
    }
    #[doc = "Bit 3 - Mirror Bit of ORPSTRAP\\[24\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap24(&mut self) -> MirrorBitOfOrpstrap24W<Scu51cSpec> {
        MirrorBitOfOrpstrap24W::new(self, 3)
    }
    #[doc = "Bit 4 - Mirror Bit of ORPSTRAP\\[25\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap25(&mut self) -> MirrorBitOfOrpstrap25W<Scu51cSpec> {
        MirrorBitOfOrpstrap25W::new(self, 4)
    }
    #[doc = "Bit 5 - Mirror Bit of ORPSTRAP\\[26\\]"]
    #[inline(always)]
    pub fn mirror_bit_of_orpstrap26(&mut self) -> MirrorBitOfOrpstrap26W<Scu51cSpec> {
        MirrorBitOfOrpstrap26W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved (SPI1CK)"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Scu51cSpec> {
        Reserved5W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved (SPI1DQ0)"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu51cSpec> {
        Reserved4W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved (SPI1DQ1)"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu51cSpec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved (SGPMCK)"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu51cSpec> {
        Reserved2W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved (SGPMO)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu51cSpec> {
        Reserved1W::new(self, 10)
    }
}
#[doc = "Hardware Pin Strap Register\\label{SCUREG:HWTRAPHRO\n\nYou can [`read`](crate::Reg::read) this register and get [`scu51c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu51c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu51cSpec;
impl crate::RegisterSpec for Scu51cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu51c::R`](R) reader structure"]
impl crate::Readable for Scu51cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu51c::W`](W) writer structure"]
impl crate::Writable for Scu51cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU51C to value 0"]
impl crate::Resettable for Scu51cSpec {}

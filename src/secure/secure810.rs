#[doc = "Register `SECURE810` reader"]
pub type R = crate::R<Secure810Spec>;
#[doc = "Register `SECURE810` writer"]
pub type W = crate::W<Secure810Spec>;
#[doc = "Secure Boot Digest Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecBootDigestStatus {
    #[doc = "0: Secure boot digest check fail when SEC814\\[0\\]=1"]
    SecureBootDigestCheckFailWhenSec81401 = 0,
    #[doc = "1: Secure boot digest check pass when SEC814\\[0\\]=1"]
    SecureBootDigestCheckPassWhenSec81401 = 1,
}
impl From<SecBootDigestStatus> for bool {
    #[inline(always)]
    fn from(variant: SecBootDigestStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecBootDigestStatus` reader - Secure Boot Digest Status"]
pub type SecBootDigestStatusR = crate::BitReader<SecBootDigestStatus>;
impl SecBootDigestStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecBootDigestStatus {
        match self.bits {
            false => SecBootDigestStatus::SecureBootDigestCheckFailWhenSec81401,
            true => SecBootDigestStatus::SecureBootDigestCheckPassWhenSec81401,
        }
    }
    #[doc = "Secure boot digest check fail when SEC814\\[0\\]=1"]
    #[inline(always)]
    pub fn is_secure_boot_digest_check_fail_when_sec81401(&self) -> bool {
        *self == SecBootDigestStatus::SecureBootDigestCheckFailWhenSec81401
    }
    #[doc = "Secure boot digest check pass when SEC814\\[0\\]=1"]
    #[inline(always)]
    pub fn is_secure_boot_digest_check_pass_when_sec81401(&self) -> bool {
        *self == SecBootDigestStatus::SecureBootDigestCheckPassWhenSec81401
    }
}
#[doc = "Field `SecBootDigestStatus` writer - Secure Boot Digest Status"]
pub type SecBootDigestStatusW<'a, REG> = crate::BitWriter<'a, REG, SecBootDigestStatus>;
impl<'a, REG> SecBootDigestStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure boot digest check fail when SEC814\\[0\\]=1"]
    #[inline(always)]
    pub fn secure_boot_digest_check_fail_when_sec81401(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootDigestStatus::SecureBootDigestCheckFailWhenSec81401)
    }
    #[doc = "Secure boot digest check pass when SEC814\\[0\\]=1"]
    #[inline(always)]
    pub fn secure_boot_digest_check_pass_when_sec81401(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootDigestStatus::SecureBootDigestCheckPassWhenSec81401)
    }
}
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Secure Boot Digest Status"]
    #[inline(always)]
    pub fn sec_boot_digest_status(&self) -> SecBootDigestStatusR {
        SecBootDigestStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot Digest Status"]
    #[inline(always)]
    pub fn sec_boot_digest_status(&mut self) -> SecBootDigestStatusW<Secure810Spec> {
        SecBootDigestStatusW::new(self, 0)
    }
}
#[doc = "Secure Boot Digest Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secure810::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure810::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure810Spec;
impl crate::RegisterSpec for Secure810Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure810::R`](R) reader structure"]
impl crate::Readable for Secure810Spec {}
#[doc = "`write(|w| ..)` method takes [`secure810::W`](W) writer structure"]
impl crate::Writable for Secure810Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE810 to value 0"]
impl crate::Resettable for Secure810Spec {}

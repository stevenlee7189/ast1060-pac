#[doc = "Register `SECURE078` reader"]
pub type R = crate::R<Secure078Spec>;
#[doc = "Register `SECURE078` writer"]
pub type W = crate::W<Secure078Spec>;
#[doc = "Field `SecBootKeyNumberRegs` reader - Secure Boot Key Number Registers"]
pub type SecBootKeyNumberRegsR = crate::FieldReader;
#[doc = "Field `SecBootKeyNumberRegs` writer - Secure Boot Key Number Registers"]
pub type SecBootKeyNumberRegsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SecBootImageEncryptionEnbldRegs` reader - Secure Boot Image Encryption Enabled Registers"]
pub type SecBootImageEncryptionEnbldRegsR = crate::BitReader;
#[doc = "Field `SecBootImageEncryptionEnbldRegs` writer - Secure Boot Image Encryption Enabled Registers"]
pub type SecBootImageEncryptionEnbldRegsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `WrProtOfThisRegSEC78` reader - Write Protection of this register SEC78"]
pub type WrProtOfThisRegSec78R = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC78` writer - Write Protection of this register SEC78"]
pub type WrProtOfThisRegSec78W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Secure Boot Key Number Registers"]
    #[inline(always)]
    pub fn sec_boot_key_number_regs(&self) -> SecBootKeyNumberRegsR {
        SecBootKeyNumberRegsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Secure Boot Image Encryption Enabled Registers"]
    #[inline(always)]
    pub fn sec_boot_image_encryption_enbld_regs(&self) -> SecBootImageEncryptionEnbldRegsR {
        SecBootImageEncryptionEnbldRegsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Write Protection of this register SEC78"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec78(&self) -> WrProtOfThisRegSec78R {
        WrProtOfThisRegSec78R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - Secure Boot Key Number Registers"]
    #[inline(always)]
    pub fn sec_boot_key_number_regs(&mut self) -> SecBootKeyNumberRegsW<Secure078Spec> {
        SecBootKeyNumberRegsW::new(self, 0)
    }
    #[doc = "Bit 3 - Secure Boot Image Encryption Enabled Registers"]
    #[inline(always)]
    pub fn sec_boot_image_encryption_enbld_regs(
        &mut self,
    ) -> SecBootImageEncryptionEnbldRegsW<Secure078Spec> {
        SecBootImageEncryptionEnbldRegsW::new(self, 3)
    }
    #[doc = "Bit 8 - Write Protection of this register SEC78"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec78(&mut self) -> WrProtOfThisRegSec78W<Secure078Spec> {
        WrProtOfThisRegSec78W::new(self, 8)
    }
}
#[doc = "Secure Boot Key Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure078Spec;
impl crate::RegisterSpec for Secure078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure078::R`](R) reader structure"]
impl crate::Readable for Secure078Spec {}
#[doc = "`write(|w| ..)` method takes [`secure078::W`](W) writer structure"]
impl crate::Writable for Secure078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE078 to value 0"]
impl crate::Resettable for Secure078Spec {}

#[doc = "Register `SECURE05C` reader"]
pub type R = crate::R<Secure05cSpec>;
#[doc = "Register `SECURE05C` writer"]
pub type W = crate::W<Secure05cSpec>;
#[doc = "Field `SecBootEngIntCtrlRegs` reader - Secure Boot Engine Internal Controller Registers"]
pub type SecBootEngIntCtrlRegsR = crate::FieldReader;
#[doc = "Field `SecBootEngIntCtrlRegs` writer - Secure Boot Engine Internal Controller Registers"]
pub type SecBootEngIntCtrlRegsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `WrProtOfThisRegSEC5C` reader - Write Protection of this register SEC5C"]
pub type WrProtOfThisRegSec5cR = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC5C` writer - Write Protection of this register SEC5C"]
pub type WrProtOfThisRegSec5cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Secure Boot Engine Internal Controller Registers"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs(&self) -> SecBootEngIntCtrlRegsR {
        SecBootEngIntCtrlRegsR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 1:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
    #[doc = "Bit 6 - Write Protection of this register SEC5C"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec5c(&self) -> WrProtOfThisRegSec5cR {
        WrProtOfThisRegSec5cR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Secure Boot Engine Internal Controller Registers"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs(&mut self) -> SecBootEngIntCtrlRegsW<Secure05cSpec> {
        SecBootEngIntCtrlRegsW::new(self, 0)
    }
    #[doc = "Bit 6 - Write Protection of this register SEC5C"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec5c(&mut self) -> WrProtOfThisRegSec5cW<Secure05cSpec> {
        WrProtOfThisRegSec5cW::new(self, 6)
    }
}
#[doc = "Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure05cSpec;
impl crate::RegisterSpec for Secure05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure05c::R`](R) reader structure"]
impl crate::Readable for Secure05cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure05c::W`](W) writer structure"]
impl crate::Writable for Secure05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE05C to value 0"]
impl crate::Resettable for Secure05cSpec {}

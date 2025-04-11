#[doc = "Register `SECURE068` reader"]
pub type R = crate::R<Secure068Spec>;
#[doc = "Register `SECURE068` writer"]
pub type W = crate::W<Secure068Spec>;
#[doc = "Field `SecBootSwRevReg1` reader - Secure Boot Software Revision Register 1"]
pub type SecBootSwRevReg1R = crate::FieldReader<u32>;
#[doc = "Field `SecBootSwRevReg1` writer - Secure Boot Software Revision Register 1"]
pub type SecBootSwRevReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Software Revision Register 1"]
    #[inline(always)]
    pub fn sec_boot_sw_rev_reg1(&self) -> SecBootSwRevReg1R {
        SecBootSwRevReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Software Revision Register 1"]
    #[inline(always)]
    pub fn sec_boot_sw_rev_reg1(&mut self) -> SecBootSwRevReg1W<Secure068Spec> {
        SecBootSwRevReg1W::new(self, 0)
    }
}
#[doc = "Secure Boot Software Revision Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure068Spec;
impl crate::RegisterSpec for Secure068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure068::R`](R) reader structure"]
impl crate::Readable for Secure068Spec {}
#[doc = "`write(|w| ..)` method takes [`secure068::W`](W) writer structure"]
impl crate::Writable for Secure068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE068 to value 0"]
impl crate::Resettable for Secure068Spec {}

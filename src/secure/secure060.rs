#[doc = "Register `SECURE060` reader"]
pub type R = crate::R<Secure060Spec>;
#[doc = "Register `SECURE060` writer"]
pub type W = crate::W<Secure060Spec>;
#[doc = "Field `SecBootHwRevReg1` reader - Secure Boot Hardware Revision Register 1"]
pub type SecBootHwRevReg1R = crate::FieldReader<u32>;
#[doc = "Field `SecBootHwRevReg1` writer - Secure Boot Hardware Revision Register 1"]
pub type SecBootHwRevReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Hardware Revision Register 1"]
    #[inline(always)]
    pub fn sec_boot_hw_rev_reg1(&self) -> SecBootHwRevReg1R {
        SecBootHwRevReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Hardware Revision Register 1"]
    #[inline(always)]
    pub fn sec_boot_hw_rev_reg1(&mut self) -> SecBootHwRevReg1W<Secure060Spec> {
        SecBootHwRevReg1W::new(self, 0)
    }
}
#[doc = "Secure Boot Hardware Revision Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure060Spec;
impl crate::RegisterSpec for Secure060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure060::R`](R) reader structure"]
impl crate::Readable for Secure060Spec {}
#[doc = "`write(|w| ..)` method takes [`secure060::W`](W) writer structure"]
impl crate::Writable for Secure060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE060 to value 0"]
impl crate::Resettable for Secure060Spec {}

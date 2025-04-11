#[doc = "Register `SECURE064` reader"]
pub type R = crate::R<Secure064Spec>;
#[doc = "Register `SECURE064` writer"]
pub type W = crate::W<Secure064Spec>;
#[doc = "Field `SecBootHwRevReg2` reader - Secure Boot Hardware Revision Register 2"]
pub type SecBootHwRevReg2R = crate::FieldReader<u32>;
#[doc = "Field `SecBootHwRevReg2` writer - Secure Boot Hardware Revision Register 2"]
pub type SecBootHwRevReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Hardware Revision Register 2"]
    #[inline(always)]
    pub fn sec_boot_hw_rev_reg2(&self) -> SecBootHwRevReg2R {
        SecBootHwRevReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Hardware Revision Register 2"]
    #[inline(always)]
    pub fn sec_boot_hw_rev_reg2(&mut self) -> SecBootHwRevReg2W<Secure064Spec> {
        SecBootHwRevReg2W::new(self, 0)
    }
}
#[doc = "Secure Boot Hardware Revision Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure064Spec;
impl crate::RegisterSpec for Secure064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure064::R`](R) reader structure"]
impl crate::Readable for Secure064Spec {}
#[doc = "`write(|w| ..)` method takes [`secure064::W`](W) writer structure"]
impl crate::Writable for Secure064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE064 to value 0"]
impl crate::Resettable for Secure064Spec {}

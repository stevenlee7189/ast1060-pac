#[doc = "Register `SCU4F4` reader"]
pub type R = crate::R<Scu4f4Spec>;
#[doc = "Register `SCU4F4` writer"]
pub type W = crate::W<Scu4f4Spec>;
#[doc = "Field `BaudRateDivisorOfNormalPhase` reader - Baud Rate divisor of normal phase"]
pub type BaudRateDivisorOfNormalPhaseR = crate::FieldReader<u16>;
#[doc = "Field `BaudRateDivisorOfNormalPhase` writer - Baud Rate divisor of normal phase"]
pub type BaudRateDivisorOfNormalPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BaudRateDivisorOfPasswordPhase` reader - Baud Rate divisor of password phase"]
pub type BaudRateDivisorOfPasswordPhaseR = crate::FieldReader<u16>;
#[doc = "Field `BaudRateDivisorOfPasswordPhase` writer - Baud Rate divisor of password phase"]
pub type BaudRateDivisorOfPasswordPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Baud Rate divisor of normal phase"]
    #[inline(always)]
    pub fn baud_rate_divisor_of_normal_phase(&self) -> BaudRateDivisorOfNormalPhaseR {
        BaudRateDivisorOfNormalPhaseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Baud Rate divisor of password phase"]
    #[inline(always)]
    pub fn baud_rate_divisor_of_password_phase(&self) -> BaudRateDivisorOfPasswordPhaseR {
        BaudRateDivisorOfPasswordPhaseR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate divisor of normal phase"]
    #[inline(always)]
    pub fn baud_rate_divisor_of_normal_phase(
        &mut self,
    ) -> BaudRateDivisorOfNormalPhaseW<Scu4f4Spec> {
        BaudRateDivisorOfNormalPhaseW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Baud Rate divisor of password phase"]
    #[inline(always)]
    pub fn baud_rate_divisor_of_password_phase(
        &mut self,
    ) -> BaudRateDivisorOfPasswordPhaseW<Scu4f4Spec> {
        BaudRateDivisorOfPasswordPhaseW::new(self, 16)
    }
}
#[doc = "UART Debug interface Baud Rate Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4f4Spec;
impl crate::RegisterSpec for Scu4f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4f4::R`](R) reader structure"]
impl crate::Readable for Scu4f4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4f4::W`](W) writer structure"]
impl crate::Writable for Scu4f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4F4 to value 0x0060_0001"]
impl crate::Resettable for Scu4f4Spec {
    const RESET_VALUE: u32 = 0x0060_0001;
}

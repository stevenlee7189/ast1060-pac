#[doc = "Register `SCU518` reader"]
pub type R = crate::R<Scu518Spec>;
#[doc = "Register `SCU518` writer"]
pub type W = crate::W<Scu518Spec>;
#[doc = "Field `SCU510WrProtBitCtrl` reader - SCU510 Write Protection Bit Control"]
pub type Scu510wrProtBitCtrlR = crate::FieldReader<u32>;
#[doc = "Field `SCU510WrProtBitCtrl` writer - SCU510 Write Protection Bit Control"]
pub type Scu510wrProtBitCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU510 Write Protection Bit Control"]
    #[inline(always)]
    pub fn scu510wr_prot_bit_ctrl(&self) -> Scu510wrProtBitCtrlR {
        Scu510wrProtBitCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU510 Write Protection Bit Control"]
    #[inline(always)]
    pub fn scu510wr_prot_bit_ctrl(&mut self) -> Scu510wrProtBitCtrlW<Scu518Spec> {
        Scu510wrProtBitCtrlW::new(self, 0)
    }
}
#[doc = "Hardware Strap2 Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu518::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu518::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu518Spec;
impl crate::RegisterSpec for Scu518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu518::R`](R) reader structure"]
impl crate::Readable for Scu518Spec {}
#[doc = "`write(|w| ..)` method takes [`scu518::W`](W) writer structure"]
impl crate::Writable for Scu518Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU518 to value 0x10"]
impl crate::Resettable for Scu518Spec {
    const RESET_VALUE: u32 = 0x10;
}

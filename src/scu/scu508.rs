#[doc = "Register `SCU508` reader"]
pub type R = crate::R<Scu508Spec>;
#[doc = "Register `SCU508` writer"]
pub type W = crate::W<Scu508Spec>;
#[doc = "Field `SCU500WrProtBitCtrl` reader - SCU500 Write Protection Bit Control"]
pub type Scu500wrProtBitCtrlR = crate::FieldReader<u32>;
#[doc = "Field `SCU500WrProtBitCtrl` writer - SCU500 Write Protection Bit Control"]
pub type Scu500wrProtBitCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU500 Write Protection Bit Control"]
    #[inline(always)]
    pub fn scu500wr_prot_bit_ctrl(&self) -> Scu500wrProtBitCtrlR {
        Scu500wrProtBitCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU500 Write Protection Bit Control"]
    #[inline(always)]
    pub fn scu500wr_prot_bit_ctrl(&mut self) -> Scu500wrProtBitCtrlW<Scu508Spec> {
        Scu500wrProtBitCtrlW::new(self, 0)
    }
}
#[doc = "Hardware Strap1 Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu508::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu508::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu508Spec;
impl crate::RegisterSpec for Scu508Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu508::R`](R) reader structure"]
impl crate::Readable for Scu508Spec {}
#[doc = "`write(|w| ..)` method takes [`scu508::W`](W) writer structure"]
impl crate::Writable for Scu508Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU508 to value 0x4008_0000"]
impl crate::Resettable for Scu508Spec {
    const RESET_VALUE: u32 = 0x4008_0000;
}

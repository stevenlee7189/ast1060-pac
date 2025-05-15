#[doc = "Register `SCUF18` reader"]
pub type R = crate::R<Scuf18Spec>;
#[doc = "Register `SCUF18` writer"]
pub type W = crate::W<Scuf18Spec>;
#[doc = "Field `EnblWrProtOfSCU210` reader - Enable Write Protection of hlinkSCU210"]
pub type EnblWrProtOfScu210R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU210` writer - Enable Write Protection of hlinkSCU210"]
pub type EnblWrProtOfScu210W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU214` reader - Enable Write Protection of hlinkSCU214"]
pub type EnblWrProtOfScu214R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU214` writer - Enable Write Protection of hlinkSCU214"]
pub type EnblWrProtOfScu214W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU210"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu210(&self) -> EnblWrProtOfScu210R {
        EnblWrProtOfScu210R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU214"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu214(&self) -> EnblWrProtOfScu214R {
        EnblWrProtOfScu214R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU210"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu210(&mut self) -> EnblWrProtOfScu210W<Scuf18Spec> {
        EnblWrProtOfScu210W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU214"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu214(&mut self) -> EnblWrProtOfScu214W<Scuf18Spec> {
        EnblWrProtOfScu214W::new(self, 1)
    }
}
#[doc = "Write Protect Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf18Spec;
impl crate::RegisterSpec for Scuf18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf18::R`](R) reader structure"]
impl crate::Readable for Scuf18Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf18::W`](W) writer structure"]
impl crate::Writable for Scuf18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF18 to value 0"]
impl crate::Resettable for Scuf18Spec {}

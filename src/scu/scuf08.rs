#[doc = "Register `SCUF08` reader"]
pub type R = crate::R<Scuf08Spec>;
#[doc = "Register `SCUF08` writer"]
pub type W = crate::W<Scuf08Spec>;
#[doc = "Field `EnblWrProtOfSCU200` reader - Enable Write Protection of hlinkSCU200"]
pub type EnblWrProtOfScu200R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU200` writer - Enable Write Protection of hlinkSCU200"]
pub type EnblWrProtOfScu200W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU204` reader - Enable Write Protection of hlinkSCU204"]
pub type EnblWrProtOfScu204R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU204` writer - Enable Write Protection of hlinkSCU204"]
pub type EnblWrProtOfScu204W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU200"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu200(&self) -> EnblWrProtOfScu200R {
        EnblWrProtOfScu200R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU204"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu204(&self) -> EnblWrProtOfScu204R {
        EnblWrProtOfScu204R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU200"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu200(&mut self) -> EnblWrProtOfScu200W<Scuf08Spec> {
        EnblWrProtOfScu200W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU204"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu204(&mut self) -> EnblWrProtOfScu204W<Scuf08Spec> {
        EnblWrProtOfScu204W::new(self, 1)
    }
}
#[doc = "Write Protect Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf08Spec;
impl crate::RegisterSpec for Scuf08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf08::R`](R) reader structure"]
impl crate::Readable for Scuf08Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf08::W`](W) writer structure"]
impl crate::Writable for Scuf08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF08 to value 0"]
impl crate::Resettable for Scuf08Spec {}

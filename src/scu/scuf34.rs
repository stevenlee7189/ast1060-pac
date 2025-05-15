#[doc = "Register `SCUF34` reader"]
pub type R = crate::R<Scuf34Spec>;
#[doc = "Register `SCUF34` writer"]
pub type W = crate::W<Scuf34Spec>;
#[doc = "Field `EnblWrProtOfSCU510` reader - Enable Write Protection of hlinkSCU510"]
pub type EnblWrProtOfScu510R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU510` writer - Enable Write Protection of hlinkSCU510"]
pub type EnblWrProtOfScu510W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU514` reader - Enable Write Protection of hlinkSCU514"]
pub type EnblWrProtOfScu514R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU514` writer - Enable Write Protection of hlinkSCU514"]
pub type EnblWrProtOfScu514W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU518` reader - Enable Write Protection of hlinkSCU518"]
pub type EnblWrProtOfScu518R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU518` writer - Enable Write Protection of hlinkSCU518"]
pub type EnblWrProtOfScu518W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU530` reader - Enable Write Protection of hlinkSCU530"]
pub type EnblWrProtOfScu530R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU530` writer - Enable Write Protection of hlinkSCU530"]
pub type EnblWrProtOfScu530W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU550` reader - Enable Write Protection of hlinkSCU550"]
pub type EnblWrProtOfScu550R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU550` writer - Enable Write Protection of hlinkSCU550"]
pub type EnblWrProtOfScu550W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU590` reader - Enable Write Protection of hlinkSCU590"]
pub type EnblWrProtOfScu590R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU590` writer - Enable Write Protection of hlinkSCU590"]
pub type EnblWrProtOfScu590W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU510"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu510(&self) -> EnblWrProtOfScu510R {
        EnblWrProtOfScu510R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU514"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu514(&self) -> EnblWrProtOfScu514R {
        EnblWrProtOfScu514R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCU518"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu518(&self) -> EnblWrProtOfScu518R {
        EnblWrProtOfScu518R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU530"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu530(&self) -> EnblWrProtOfScu530R {
        EnblWrProtOfScu530R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU550"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu550(&self) -> EnblWrProtOfScu550R {
        EnblWrProtOfScu550R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU590"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu590(&self) -> EnblWrProtOfScu590R {
        EnblWrProtOfScu590R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU510"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu510(&mut self) -> EnblWrProtOfScu510W<Scuf34Spec> {
        EnblWrProtOfScu510W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU514"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu514(&mut self) -> EnblWrProtOfScu514W<Scuf34Spec> {
        EnblWrProtOfScu514W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCU518"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu518(&mut self) -> EnblWrProtOfScu518W<Scuf34Spec> {
        EnblWrProtOfScu518W::new(self, 2)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU530"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu530(&mut self) -> EnblWrProtOfScu530W<Scuf34Spec> {
        EnblWrProtOfScu530W::new(self, 4)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU550"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu550(&mut self) -> EnblWrProtOfScu550W<Scuf34Spec> {
        EnblWrProtOfScu550W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU590"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu590(&mut self) -> EnblWrProtOfScu590W<Scuf34Spec> {
        EnblWrProtOfScu590W::new(self, 16)
    }
}
#[doc = "Write Protect Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf34Spec;
impl crate::RegisterSpec for Scuf34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf34::R`](R) reader structure"]
impl crate::Readable for Scuf34Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf34::W`](W) writer structure"]
impl crate::Writable for Scuf34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF34 to value 0"]
impl crate::Resettable for Scuf34Spec {}

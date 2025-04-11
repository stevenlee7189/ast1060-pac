#[doc = "Register `SCUF34` reader"]
pub type R = crate::R<Scuf34Spec>;
#[doc = "Register `SCUF34` writer"]
pub type W = crate::W<Scuf34Spec>;
#[doc = "Field `EnblWrProtOfHlinkSCU510` reader - Enable Write Protection of hlinkSCU510"]
pub type EnblWrProtOfHlinkScu510R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU510` writer - Enable Write Protection of hlinkSCU510"]
pub type EnblWrProtOfHlinkScu510W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU514` reader - Enable Write Protection of hlinkSCU514"]
pub type EnblWrProtOfHlinkScu514R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU514` writer - Enable Write Protection of hlinkSCU514"]
pub type EnblWrProtOfHlinkScu514W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU518` reader - Enable Write Protection of hlinkSCU518"]
pub type EnblWrProtOfHlinkScu518R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU518` writer - Enable Write Protection of hlinkSCU518"]
pub type EnblWrProtOfHlinkScu518W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU530` reader - Enable Write Protection of hlinkSCU530"]
pub type EnblWrProtOfHlinkScu530R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU530` writer - Enable Write Protection of hlinkSCU530"]
pub type EnblWrProtOfHlinkScu530W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCU550` reader - Enable Write Protection of hlinkSCU550"]
pub type EnblWrProtOfHlinkScu550R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU550` writer - Enable Write Protection of hlinkSCU550"]
pub type EnblWrProtOfHlinkScu550W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCU590` reader - Enable Write Protection of hlinkSCU590"]
pub type EnblWrProtOfHlinkScu590R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU590` writer - Enable Write Protection of hlinkSCU590"]
pub type EnblWrProtOfHlinkScu590W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU510"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu510(&self) -> EnblWrProtOfHlinkScu510R {
        EnblWrProtOfHlinkScu510R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU514"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu514(&self) -> EnblWrProtOfHlinkScu514R {
        EnblWrProtOfHlinkScu514R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCU518"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu518(&self) -> EnblWrProtOfHlinkScu518R {
        EnblWrProtOfHlinkScu518R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU530"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu530(&self) -> EnblWrProtOfHlinkScu530R {
        EnblWrProtOfHlinkScu530R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU550"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu550(&self) -> EnblWrProtOfHlinkScu550R {
        EnblWrProtOfHlinkScu550R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU590"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu590(&self) -> EnblWrProtOfHlinkScu590R {
        EnblWrProtOfHlinkScu590R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU510"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu510(&mut self) -> EnblWrProtOfHlinkScu510W<Scuf34Spec> {
        EnblWrProtOfHlinkScu510W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU514"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu514(&mut self) -> EnblWrProtOfHlinkScu514W<Scuf34Spec> {
        EnblWrProtOfHlinkScu514W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCU518"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu518(&mut self) -> EnblWrProtOfHlinkScu518W<Scuf34Spec> {
        EnblWrProtOfHlinkScu518W::new(self, 2)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU530"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu530(&mut self) -> EnblWrProtOfHlinkScu530W<Scuf34Spec> {
        EnblWrProtOfHlinkScu530W::new(self, 4)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU550"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu550(&mut self) -> EnblWrProtOfHlinkScu550W<Scuf34Spec> {
        EnblWrProtOfHlinkScu550W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU590"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu590(&mut self) -> EnblWrProtOfHlinkScu590W<Scuf34Spec> {
        EnblWrProtOfHlinkScu590W::new(self, 16)
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

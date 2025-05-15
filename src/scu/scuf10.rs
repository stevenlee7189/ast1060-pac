#[doc = "Register `SCUF10` reader"]
pub type R = crate::R<Scuf10Spec>;
#[doc = "Register `SCUF10` writer"]
pub type W = crate::W<Scuf10Spec>;
#[doc = "Field `EnblWrProtOfSCU010HlinkSCU000` reader - Enable Write Protection of hlinkSCU010 hlinkSCU000"]
pub type EnblWrProtOfScu010hlinkScu000R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU010HlinkSCU000` writer - Enable Write Protection of hlinkSCU010 hlinkSCU000"]
pub type EnblWrProtOfScu010hlinkScu000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved(0)"]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU050` reader - Enable Write Protection of hlinkSCU050"]
pub type EnblWrProtOfScu050R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU050` writer - Enable Write Protection of hlinkSCU050"]
pub type EnblWrProtOfScu050W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved(0)"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU058` reader - Enable Write Protection of hlinkSCU058"]
pub type EnblWrProtOfScu058R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU058` writer - Enable Write Protection of hlinkSCU058"]
pub type EnblWrProtOfScu058W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved(0)"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU070` reader - Enable Write Protection of hlinkSCU070"]
pub type EnblWrProtOfScu070R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU070` writer - Enable Write Protection of hlinkSCU070"]
pub type EnblWrProtOfScu070W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU074` reader - Enable Write Protection of hlinkSCU074"]
pub type EnblWrProtOfScu074R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU074` writer - Enable Write Protection of hlinkSCU074"]
pub type EnblWrProtOfScu074W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU090` reader - Enable Write Protection of hlinkSCU090"]
pub type EnblWrProtOfScu090R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU090` writer - Enable Write Protection of hlinkSCU090"]
pub type EnblWrProtOfScu090W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU098` reader - Enable Write Protection of hlinkSCU098"]
pub type EnblWrProtOfScu098R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU098` writer - Enable Write Protection of hlinkSCU098"]
pub type EnblWrProtOfScu098W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU0D0` reader - Enable Write Protection of hlinkSCU0D0"]
pub type EnblWrProtOfScu0d0R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU0D0` writer - Enable Write Protection of hlinkSCU0D0"]
pub type EnblWrProtOfScu0d0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU0D4` reader - Enable Write Protection of hlinkSCU0D4"]
pub type EnblWrProtOfScu0d4R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU0D4` writer - Enable Write Protection of hlinkSCU0D4"]
pub type EnblWrProtOfScu0d4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU0D8` reader - Enable Write Protection of hlinkSCU0D8"]
pub type EnblWrProtOfScu0d8R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU0D8` writer - Enable Write Protection of hlinkSCU0D8"]
pub type EnblWrProtOfScu0d8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU0F0` reader - Enable Write Protection of hlinkSCU0F0"]
pub type EnblWrProtOfScu0f0R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU0F0` writer - Enable Write Protection of hlinkSCU0F0"]
pub type EnblWrProtOfScu0f0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU0F8` reader - Enable Write Protection of hlinkSCU0F8"]
pub type EnblWrProtOfScu0f8R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU0F8` writer - Enable Write Protection of hlinkSCU0F8"]
pub type EnblWrProtOfScu0f8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU0FC` reader - Enable Write Protection of hlinkSCU0FC"]
pub type EnblWrProtOfScu0fcR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU0FC` writer - Enable Write Protection of hlinkSCU0FC"]
pub type EnblWrProtOfScu0fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU010 hlinkSCU000"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu010hlink_scu000(&self) -> EnblWrProtOfScu010hlinkScu000R {
        EnblWrProtOfScu010hlinkScu000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU050"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu050(&self) -> EnblWrProtOfScu050R {
        EnblWrProtOfScu050R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU058"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu058(&self) -> EnblWrProtOfScu058R {
        EnblWrProtOfScu058R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU070"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu070(&self) -> EnblWrProtOfScu070R {
        EnblWrProtOfScu070R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Protection of hlinkSCU074"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu074(&self) -> EnblWrProtOfScu074R {
        EnblWrProtOfScu074R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU090"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu090(&self) -> EnblWrProtOfScu090R {
        EnblWrProtOfScu090R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCU098"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu098(&self) -> EnblWrProtOfScu098R {
        EnblWrProtOfScu098R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enable Write Protection of hlinkSCU0D0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0d0(&self) -> EnblWrProtOfScu0d0R {
        EnblWrProtOfScu0d0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Protection of hlinkSCU0D4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0d4(&self) -> EnblWrProtOfScu0d4R {
        EnblWrProtOfScu0d4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Protection of hlinkSCU0D8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0d8(&self) -> EnblWrProtOfScu0d8R {
        EnblWrProtOfScu0d8R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Protection of hlinkSCU0F0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0f0(&self) -> EnblWrProtOfScu0f0R {
        EnblWrProtOfScu0f0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Write Protection of hlinkSCU0F8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0f8(&self) -> EnblWrProtOfScu0f8R {
        EnblWrProtOfScu0f8R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCU0FC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0fc(&self) -> EnblWrProtOfScu0fcR {
        EnblWrProtOfScu0fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU010 hlinkSCU000"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu010hlink_scu000(
        &mut self,
    ) -> EnblWrProtOfScu010hlinkScu000W<Scuf10Spec> {
        EnblWrProtOfScu010hlinkScu000W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU050"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu050(&mut self) -> EnblWrProtOfScu050W<Scuf10Spec> {
        EnblWrProtOfScu050W::new(self, 8)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU058"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu058(&mut self) -> EnblWrProtOfScu058W<Scuf10Spec> {
        EnblWrProtOfScu058W::new(self, 10)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU070"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu070(&mut self) -> EnblWrProtOfScu070W<Scuf10Spec> {
        EnblWrProtOfScu070W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Protection of hlinkSCU074"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu074(&mut self) -> EnblWrProtOfScu074W<Scuf10Spec> {
        EnblWrProtOfScu074W::new(self, 13)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU090"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu090(&mut self) -> EnblWrProtOfScu090W<Scuf10Spec> {
        EnblWrProtOfScu090W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCU098"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu098(&mut self) -> EnblWrProtOfScu098W<Scuf10Spec> {
        EnblWrProtOfScu098W::new(self, 18)
    }
    #[doc = "Bit 24 - Enable Write Protection of hlinkSCU0D0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0d0(&mut self) -> EnblWrProtOfScu0d0W<Scuf10Spec> {
        EnblWrProtOfScu0d0W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Protection of hlinkSCU0D4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0d4(&mut self) -> EnblWrProtOfScu0d4W<Scuf10Spec> {
        EnblWrProtOfScu0d4W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Protection of hlinkSCU0D8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0d8(&mut self) -> EnblWrProtOfScu0d8W<Scuf10Spec> {
        EnblWrProtOfScu0d8W::new(self, 26)
    }
    #[doc = "Bit 28 - Enable Write Protection of hlinkSCU0F0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0f0(&mut self) -> EnblWrProtOfScu0f0W<Scuf10Spec> {
        EnblWrProtOfScu0f0W::new(self, 28)
    }
    #[doc = "Bit 30 - Enable Write Protection of hlinkSCU0F8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0f8(&mut self) -> EnblWrProtOfScu0f8W<Scuf10Spec> {
        EnblWrProtOfScu0f8W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCU0FC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu0fc(&mut self) -> EnblWrProtOfScu0fcW<Scuf10Spec> {
        EnblWrProtOfScu0fcW::new(self, 31)
    }
}
#[doc = "Write Protect Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf10Spec;
impl crate::RegisterSpec for Scuf10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf10::R`](R) reader structure"]
impl crate::Readable for Scuf10Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf10::W`](W) writer structure"]
impl crate::Writable for Scuf10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF10 to value 0"]
impl crate::Resettable for Scuf10Spec {}

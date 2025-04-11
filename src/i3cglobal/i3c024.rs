#[doc = "Register `I3C024` reader"]
pub type R = crate::R<I3c024Spec>;
#[doc = "Register `I3C024` writer"]
pub type W = crate::W<I3c024Spec>;
#[doc = "Field `Modei2c` reader - mode_i2c"]
pub type Modei2cR = crate::BitReader;
#[doc = "Field `Modei2c` writer - mode_i2c"]
pub type Modei2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Slvtestmode` reader - slv_test_mode"]
pub type SlvtestmodeR = crate::BitReader;
#[doc = "Field `Slvtestmode` writer - slv_test_mode"]
pub type SlvtestmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Actmode` reader - act_mode"]
pub type ActmodeR = crate::FieldReader;
#[doc = "Field `Actmode` writer - act_mode"]
pub type ActmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Pendingint` reader - pending_int"]
pub type PendingintR = crate::FieldReader;
#[doc = "Field `Pendingint` writer - pending_int"]
pub type PendingintW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Staticaddr` reader - static_addr"]
pub type StaticaddrR = crate::FieldReader;
#[doc = "Field `Staticaddr` writer - static_addr"]
pub type StaticaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Staticaddren` reader - static_addr_en"]
pub type StaticaddrenR = crate::BitReader;
#[doc = "Field `Staticaddren` writer - static_addr_en"]
pub type StaticaddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Instid` reader - inst_id"]
pub type InstidR = crate::FieldReader;
#[doc = "Field `Instid` writer - inst_id"]
pub type InstidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - mode_i2c"]
    #[inline(always)]
    pub fn modei2c(&self) -> Modei2cR {
        Modei2cR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - slv_test_mode"]
    #[inline(always)]
    pub fn slvtestmode(&self) -> SlvtestmodeR {
        SlvtestmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - act_mode"]
    #[inline(always)]
    pub fn actmode(&self) -> ActmodeR {
        ActmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - pending_int"]
    #[inline(always)]
    pub fn pendingint(&self) -> PendingintR {
        PendingintR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - static_addr"]
    #[inline(always)]
    pub fn staticaddr(&self) -> StaticaddrR {
        StaticaddrR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - static_addr_en"]
    #[inline(always)]
    pub fn staticaddren(&self) -> StaticaddrenR {
        StaticaddrenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - inst_id"]
    #[inline(always)]
    pub fn instid(&self) -> InstidR {
        InstidR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - mode_i2c"]
    #[inline(always)]
    pub fn modei2c(&mut self) -> Modei2cW<I3c024Spec> {
        Modei2cW::new(self, 0)
    }
    #[doc = "Bit 1 - slv_test_mode"]
    #[inline(always)]
    pub fn slvtestmode(&mut self) -> SlvtestmodeW<I3c024Spec> {
        SlvtestmodeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - act_mode"]
    #[inline(always)]
    pub fn actmode(&mut self) -> ActmodeW<I3c024Spec> {
        ActmodeW::new(self, 2)
    }
    #[doc = "Bits 4:7 - pending_int"]
    #[inline(always)]
    pub fn pendingint(&mut self) -> PendingintW<I3c024Spec> {
        PendingintW::new(self, 4)
    }
    #[doc = "Bits 8:14 - static_addr"]
    #[inline(always)]
    pub fn staticaddr(&mut self) -> StaticaddrW<I3c024Spec> {
        StaticaddrW::new(self, 8)
    }
    #[doc = "Bit 15 - static_addr_en"]
    #[inline(always)]
    pub fn staticaddren(&mut self) -> StaticaddrenW<I3c024Spec> {
        StaticaddrenW::new(self, 15)
    }
    #[doc = "Bits 16:19 - inst_id"]
    #[inline(always)]
    pub fn instid(&mut self) -> InstidW<I3c024Spec> {
        InstidW::new(self, 16)
    }
}
#[doc = "I3C2Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3c024Spec;
impl crate::RegisterSpec for I3c024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c024::R`](R) reader structure"]
impl crate::Readable for I3c024Spec {}
#[doc = "`write(|w| ..)` method takes [`i3c024::W`](W) writer structure"]
impl crate::Writable for I3c024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3C024 to value 0"]
impl crate::Resettable for I3c024Spec {}

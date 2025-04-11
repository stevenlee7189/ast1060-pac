#[doc = "Register `SPI16C` reader"]
pub type R = crate::R<Spi16cSpec>;
#[doc = "Register `SPI16C` writer"]
pub type W = crate::W<Spi16cSpec>;
#[doc = "Field `Cmd07` reader - Command 0"]
pub type Cmd07R = crate::FieldReader;
#[doc = "Field `Cmd07` writer - Command 0"]
pub type Cmd07W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd17` reader - Command 1"]
pub type Cmd17R = crate::FieldReader;
#[doc = "Field `Cmd17` writer - Command 1"]
pub type Cmd17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd27` reader - Command 2"]
pub type Cmd27R = crate::FieldReader;
#[doc = "Field `Cmd27` writer - Command 2"]
pub type Cmd27W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd7 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd7> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd7` reader - 3B/4B Command"]
pub type _3b4bcmd7R = crate::BitReader<_3b4bcmd7>;
impl _3b4bcmd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd7 {
        match self.bits {
            false => _3b4bcmd7::Command210AreFor3bMode,
            true => _3b4bcmd7::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd7::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd7::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd7` writer - 3B/4B Command"]
pub type _3b4bcmd7W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd7>;
impl<'a, REG> _3b4bcmd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd7::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd7::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead7` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead7R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead7` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr7` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr7R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr7` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead7` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead7R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead7` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr7` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr7R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr7` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead7` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead7R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead7` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr7` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr7R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr7` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd07(&self) -> Cmd07R {
        Cmd07R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd17(&self) -> Cmd17R {
        Cmd17R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd27(&self) -> Cmd27R {
        Cmd27R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd7(&self) -> _3b4bcmd7R {
        _3b4bcmd7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read7(&self) -> EnblCmd0forRead7R {
        EnblCmd0forRead7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr7(&self) -> EnblCmd0forWr7R {
        EnblCmd0forWr7R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read7(&self) -> EnblCmd1forRead7R {
        EnblCmd1forRead7R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr7(&self) -> EnblCmd1forWr7R {
        EnblCmd1forWr7R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read7(&self) -> EnblCmd2forRead7R {
        EnblCmd2forRead7R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr7(&self) -> EnblCmd2forWr7R {
        EnblCmd2forWr7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd07(&mut self) -> Cmd07W<Spi16cSpec> {
        Cmd07W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd17(&mut self) -> Cmd17W<Spi16cSpec> {
        Cmd17W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd27(&mut self) -> Cmd27W<Spi16cSpec> {
        Cmd27W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd7(&mut self) -> _3b4bcmd7W<Spi16cSpec> {
        _3b4bcmd7W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read7(&mut self) -> EnblCmd0forRead7W<Spi16cSpec> {
        EnblCmd0forRead7W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr7(&mut self) -> EnblCmd0forWr7W<Spi16cSpec> {
        EnblCmd0forWr7W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read7(&mut self) -> EnblCmd1forRead7W<Spi16cSpec> {
        EnblCmd1forRead7W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr7(&mut self) -> EnblCmd1forWr7W<Spi16cSpec> {
        EnblCmd1forWr7W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read7(&mut self) -> EnblCmd2forRead7W<Spi16cSpec> {
        EnblCmd2forRead7W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr7(&mut self) -> EnblCmd2forWr7W<Spi16cSpec> {
        EnblCmd2forWr7W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi16c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi16c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi16cSpec;
impl crate::RegisterSpec for Spi16cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi16c::R`](R) reader structure"]
impl crate::Readable for Spi16cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi16c::W`](W) writer structure"]
impl crate::Writable for Spi16cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI16C to value 0"]
impl crate::Resettable for Spi16cSpec {}

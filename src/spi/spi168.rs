#[doc = "Register `SPI168` reader"]
pub type R = crate::R<Spi168Spec>;
#[doc = "Register `SPI168` writer"]
pub type W = crate::W<Spi168Spec>;
#[doc = "Field `Cmd06` reader - Command 0"]
pub type Cmd06R = crate::FieldReader;
#[doc = "Field `Cmd06` writer - Command 0"]
pub type Cmd06W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd16` reader - Command 1"]
pub type Cmd16R = crate::FieldReader;
#[doc = "Field `Cmd16` writer - Command 1"]
pub type Cmd16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd26` reader - Command 2"]
pub type Cmd26R = crate::FieldReader;
#[doc = "Field `Cmd26` writer - Command 2"]
pub type Cmd26W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd6 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd6> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd6` reader - 3B/4B Command"]
pub type _3b4bcmd6R = crate::BitReader<_3b4bcmd6>;
impl _3b4bcmd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd6 {
        match self.bits {
            false => _3b4bcmd6::Command210AreFor3bMode,
            true => _3b4bcmd6::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd6::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd6::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd6` writer - 3B/4B Command"]
pub type _3b4bcmd6W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd6>;
impl<'a, REG> _3b4bcmd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd6::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd6::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead6` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead6R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead6` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr6` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr6R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr6` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead6` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead6R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead6` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr6` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr6R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr6` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead6` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead6R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead6` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr6` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr6R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr6` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd06(&self) -> Cmd06R {
        Cmd06R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd16(&self) -> Cmd16R {
        Cmd16R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd26(&self) -> Cmd26R {
        Cmd26R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd6(&self) -> _3b4bcmd6R {
        _3b4bcmd6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read6(&self) -> EnblCmd0forRead6R {
        EnblCmd0forRead6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr6(&self) -> EnblCmd0forWr6R {
        EnblCmd0forWr6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read6(&self) -> EnblCmd1forRead6R {
        EnblCmd1forRead6R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr6(&self) -> EnblCmd1forWr6R {
        EnblCmd1forWr6R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read6(&self) -> EnblCmd2forRead6R {
        EnblCmd2forRead6R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr6(&self) -> EnblCmd2forWr6R {
        EnblCmd2forWr6R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd06(&mut self) -> Cmd06W<Spi168Spec> {
        Cmd06W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd16(&mut self) -> Cmd16W<Spi168Spec> {
        Cmd16W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd26(&mut self) -> Cmd26W<Spi168Spec> {
        Cmd26W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd6(&mut self) -> _3b4bcmd6W<Spi168Spec> {
        _3b4bcmd6W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read6(&mut self) -> EnblCmd0forRead6W<Spi168Spec> {
        EnblCmd0forRead6W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr6(&mut self) -> EnblCmd0forWr6W<Spi168Spec> {
        EnblCmd0forWr6W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read6(&mut self) -> EnblCmd1forRead6W<Spi168Spec> {
        EnblCmd1forRead6W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr6(&mut self) -> EnblCmd1forWr6W<Spi168Spec> {
        EnblCmd1forWr6W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read6(&mut self) -> EnblCmd2forRead6W<Spi168Spec> {
        EnblCmd2forRead6W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr6(&mut self) -> EnblCmd2forWr6W<Spi168Spec> {
        EnblCmd2forWr6W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi168Spec;
impl crate::RegisterSpec for Spi168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi168::R`](R) reader structure"]
impl crate::Readable for Spi168Spec {}
#[doc = "`write(|w| ..)` method takes [`spi168::W`](W) writer structure"]
impl crate::Writable for Spi168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI168 to value 0"]
impl crate::Resettable for Spi168Spec {}

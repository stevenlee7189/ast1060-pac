#[doc = "Register `SPI158` reader"]
pub type R = crate::R<Spi158Spec>;
#[doc = "Register `SPI158` writer"]
pub type W = crate::W<Spi158Spec>;
#[doc = "Field `Cmd02` reader - Command 0"]
pub type Cmd02R = crate::FieldReader;
#[doc = "Field `Cmd02` writer - Command 0"]
pub type Cmd02W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd12` reader - Command 1"]
pub type Cmd12R = crate::FieldReader;
#[doc = "Field `Cmd12` writer - Command 1"]
pub type Cmd12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd22` reader - Command 2"]
pub type Cmd22R = crate::FieldReader;
#[doc = "Field `Cmd22` writer - Command 2"]
pub type Cmd22W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd2 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd2> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd2` reader - 3B/4B Command"]
pub type _3b4bcmd2R = crate::BitReader<_3b4bcmd2>;
impl _3b4bcmd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd2 {
        match self.bits {
            false => _3b4bcmd2::Command210AreFor3bMode,
            true => _3b4bcmd2::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd2::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd2::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd2` writer - 3B/4B Command"]
pub type _3b4bcmd2W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd2>;
impl<'a, REG> _3b4bcmd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd2::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd2::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead2` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead2R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead2` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr2` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr2R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr2` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead2` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead2R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead2` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr2` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr2R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr2` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead2` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead2R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead2` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr2` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr2R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr2` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd02(&self) -> Cmd02R {
        Cmd02R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd12(&self) -> Cmd12R {
        Cmd12R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd22(&self) -> Cmd22R {
        Cmd22R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd2(&self) -> _3b4bcmd2R {
        _3b4bcmd2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read2(&self) -> EnblCmd0forRead2R {
        EnblCmd0forRead2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr2(&self) -> EnblCmd0forWr2R {
        EnblCmd0forWr2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read2(&self) -> EnblCmd1forRead2R {
        EnblCmd1forRead2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr2(&self) -> EnblCmd1forWr2R {
        EnblCmd1forWr2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read2(&self) -> EnblCmd2forRead2R {
        EnblCmd2forRead2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr2(&self) -> EnblCmd2forWr2R {
        EnblCmd2forWr2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd02(&mut self) -> Cmd02W<Spi158Spec> {
        Cmd02W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd12(&mut self) -> Cmd12W<Spi158Spec> {
        Cmd12W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd22(&mut self) -> Cmd22W<Spi158Spec> {
        Cmd22W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd2(&mut self) -> _3b4bcmd2W<Spi158Spec> {
        _3b4bcmd2W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read2(&mut self) -> EnblCmd0forRead2W<Spi158Spec> {
        EnblCmd0forRead2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr2(&mut self) -> EnblCmd0forWr2W<Spi158Spec> {
        EnblCmd0forWr2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read2(&mut self) -> EnblCmd1forRead2W<Spi158Spec> {
        EnblCmd1forRead2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr2(&mut self) -> EnblCmd1forWr2W<Spi158Spec> {
        EnblCmd1forWr2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read2(&mut self) -> EnblCmd2forRead2W<Spi158Spec> {
        EnblCmd2forRead2W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr2(&mut self) -> EnblCmd2forWr2W<Spi158Spec> {
        EnblCmd2forWr2W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi158Spec;
impl crate::RegisterSpec for Spi158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi158::R`](R) reader structure"]
impl crate::Readable for Spi158Spec {}
#[doc = "`write(|w| ..)` method takes [`spi158::W`](W) writer structure"]
impl crate::Writable for Spi158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI158 to value 0x1400_1303"]
impl crate::Resettable for Spi158Spec {
    const RESET_VALUE: u32 = 0x1400_1303;
}

#[doc = "Register `FMC158` reader"]
pub type R = crate::R<Fmc158Spec>;
#[doc = "Register `FMC158` writer"]
pub type W = crate::W<Fmc158Spec>;
#[doc = "Field `Cmd12` reader - Command #1"]
pub type Cmd12R = crate::FieldReader;
#[doc = "Field `Cmd12` writer - Command #1"]
pub type Cmd12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd22` reader - Command #2"]
pub type Cmd22R = crate::FieldReader;
#[doc = "Field `Cmd22` writer - Command #2"]
pub type Cmd22W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd32` reader - Command #3"]
pub type Cmd32R = crate::FieldReader;
#[doc = "Field `Cmd32` writer - Command #3"]
pub type Cmd32W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd2 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
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
            false => _3b4bcmd2::Command321AreFor3bMode,
            true => _3b4bcmd2::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd2::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd2::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd2` writer - 3B/4B Command"]
pub type _3b4bcmd2W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd2>;
impl<'a, REG> _3b4bcmd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd2::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd2::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting2 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting2> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting2 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting2 {}
#[doc = "Field `Cmd1Setting2` reader - Command #1 setting"]
pub type Cmd1setting2R = crate::FieldReader<Cmd1setting2>;
impl Cmd1setting2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting2 {
        match self.bits {
            0 => Cmd1setting2::Disable,
            1 => Cmd1setting2::ForRead,
            2 => Cmd1setting2::ForWrite,
            3 => Cmd1setting2::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting2::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting2::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting2::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting2::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting2` writer - Command #1 setting"]
pub type Cmd1setting2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting2, crate::Safe>;
impl<'a, REG> Cmd1setting2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting2::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting2::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting2::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting2::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting2 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting2> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting2 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting2 {}
#[doc = "Field `Cmd2Setting2` reader - Command #2 setting"]
pub type Cmd2setting2R = crate::FieldReader<Cmd2setting2>;
impl Cmd2setting2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting2 {
        match self.bits {
            0 => Cmd2setting2::Disable,
            1 => Cmd2setting2::ForRead,
            2 => Cmd2setting2::ForWrite,
            3 => Cmd2setting2::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting2::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting2::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting2::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting2::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting2` writer - Command #2 setting"]
pub type Cmd2setting2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting2, crate::Safe>;
impl<'a, REG> Cmd2setting2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting2::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting2::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting2::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting2::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting2 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting2> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting2 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting2 {}
#[doc = "Field `Cmd3Setting2` reader - Command #3 setting"]
pub type Cmd3setting2R = crate::FieldReader<Cmd3setting2>;
impl Cmd3setting2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting2 {
        match self.bits {
            0 => Cmd3setting2::Disable,
            1 => Cmd3setting2::ForRead,
            2 => Cmd3setting2::ForWrite,
            3 => Cmd3setting2::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting2::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting2::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting2::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting2::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting2` writer - Command #3 setting"]
pub type Cmd3setting2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting2, crate::Safe>;
impl<'a, REG> Cmd3setting2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting2::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting2::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting2::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting2::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd12(&self) -> Cmd12R {
        Cmd12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd22(&self) -> Cmd22R {
        Cmd22R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd32(&self) -> Cmd32R {
        Cmd32R::new(((self.bits >> 16) & 0xff) as u8)
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
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting2(&self) -> Cmd1setting2R {
        Cmd1setting2R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting2(&self) -> Cmd2setting2R {
        Cmd2setting2R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting2(&self) -> Cmd3setting2R {
        Cmd3setting2R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd12(&mut self) -> Cmd12W<Fmc158Spec> {
        Cmd12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd22(&mut self) -> Cmd22W<Fmc158Spec> {
        Cmd22W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd32(&mut self) -> Cmd32W<Fmc158Spec> {
        Cmd32W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd2(&mut self) -> _3b4bcmd2W<Fmc158Spec> {
        _3b4bcmd2W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting2(&mut self) -> Cmd1setting2W<Fmc158Spec> {
        Cmd1setting2W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting2(&mut self) -> Cmd2setting2W<Fmc158Spec> {
        Cmd2setting2W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting2(&mut self) -> Cmd3setting2W<Fmc158Spec> {
        Cmd3setting2W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc158Spec;
impl crate::RegisterSpec for Fmc158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc158::R`](R) reader structure"]
impl crate::Readable for Fmc158Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc158::W`](W) writer structure"]
impl crate::Writable for Fmc158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC158 to value 0"]
impl crate::Resettable for Fmc158Spec {}

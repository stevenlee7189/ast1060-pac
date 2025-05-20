#[doc = "Register `FMC17C` reader"]
pub type R = crate::R<Fmc17cSpec>;
#[doc = "Register `FMC17C` writer"]
pub type W = crate::W<Fmc17cSpec>;
#[doc = "Field `Cmd111` reader - Command #1"]
pub type Cmd111R = crate::FieldReader;
#[doc = "Field `Cmd111` writer - Command #1"]
pub type Cmd111W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd211` reader - Command #2"]
pub type Cmd211R = crate::FieldReader;
#[doc = "Field `Cmd211` writer - Command #2"]
pub type Cmd211W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd311` reader - Command #3"]
pub type Cmd311R = crate::FieldReader;
#[doc = "Field `Cmd311` writer - Command #3"]
pub type Cmd311W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd11 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd11> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd11` reader - 3B/4B Command"]
pub type _3b4bcmd11R = crate::BitReader<_3b4bcmd11>;
impl _3b4bcmd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd11 {
        match self.bits {
            false => _3b4bcmd11::Command321AreFor3bMode,
            true => _3b4bcmd11::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd11::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd11::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd11` writer - 3B/4B Command"]
pub type _3b4bcmd11W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd11>;
impl<'a, REG> _3b4bcmd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd11::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd11::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting11 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting11> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting11 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting11 {}
#[doc = "Field `Cmd1Setting11` reader - Command #1 setting"]
pub type Cmd1setting11R = crate::FieldReader<Cmd1setting11>;
impl Cmd1setting11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting11 {
        match self.bits {
            0 => Cmd1setting11::Disable,
            1 => Cmd1setting11::ForRead,
            2 => Cmd1setting11::ForWrite,
            3 => Cmd1setting11::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting11::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting11::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting11::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting11::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting11` writer - Command #1 setting"]
pub type Cmd1setting11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting11, crate::Safe>;
impl<'a, REG> Cmd1setting11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting11::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting11::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting11::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting11::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting11 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting11> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting11 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting11 {}
#[doc = "Field `Cmd2Setting11` reader - Command #2 setting"]
pub type Cmd2setting11R = crate::FieldReader<Cmd2setting11>;
impl Cmd2setting11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting11 {
        match self.bits {
            0 => Cmd2setting11::Disable,
            1 => Cmd2setting11::ForRead,
            2 => Cmd2setting11::ForWrite,
            3 => Cmd2setting11::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting11::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting11::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting11::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting11::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting11` writer - Command #2 setting"]
pub type Cmd2setting11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting11, crate::Safe>;
impl<'a, REG> Cmd2setting11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting11::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting11::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting11::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting11::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting11 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting11> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting11 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting11 {}
#[doc = "Field `Cmd3Setting11` reader - Command #3 setting"]
pub type Cmd3setting11R = crate::FieldReader<Cmd3setting11>;
impl Cmd3setting11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting11 {
        match self.bits {
            0 => Cmd3setting11::Disable,
            1 => Cmd3setting11::ForRead,
            2 => Cmd3setting11::ForWrite,
            3 => Cmd3setting11::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting11::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting11::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting11::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting11::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting11` writer - Command #3 setting"]
pub type Cmd3setting11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting11, crate::Safe>;
impl<'a, REG> Cmd3setting11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting11::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting11::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting11::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting11::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd111(&self) -> Cmd111R {
        Cmd111R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd211(&self) -> Cmd211R {
        Cmd211R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd311(&self) -> Cmd311R {
        Cmd311R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd11(&self) -> _3b4bcmd11R {
        _3b4bcmd11R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting11(&self) -> Cmd1setting11R {
        Cmd1setting11R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting11(&self) -> Cmd2setting11R {
        Cmd2setting11R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting11(&self) -> Cmd3setting11R {
        Cmd3setting11R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd111(&mut self) -> Cmd111W<Fmc17cSpec> {
        Cmd111W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd211(&mut self) -> Cmd211W<Fmc17cSpec> {
        Cmd211W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd311(&mut self) -> Cmd311W<Fmc17cSpec> {
        Cmd311W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd11(&mut self) -> _3b4bcmd11W<Fmc17cSpec> {
        _3b4bcmd11W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting11(&mut self) -> Cmd1setting11W<Fmc17cSpec> {
        Cmd1setting11W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting11(&mut self) -> Cmd2setting11W<Fmc17cSpec> {
        Cmd2setting11W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting11(&mut self) -> Cmd3setting11W<Fmc17cSpec> {
        Cmd3setting11W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc17c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc17c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc17cSpec;
impl crate::RegisterSpec for Fmc17cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc17c::R`](R) reader structure"]
impl crate::Readable for Fmc17cSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc17c::W`](W) writer structure"]
impl crate::Writable for Fmc17cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC17C to value 0"]
impl crate::Resettable for Fmc17cSpec {}

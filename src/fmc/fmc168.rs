#[doc = "Register `FMC168` reader"]
pub type R = crate::R<Fmc168Spec>;
#[doc = "Register `FMC168` writer"]
pub type W = crate::W<Fmc168Spec>;
#[doc = "Field `Cmd16` reader - Command #1"]
pub type Cmd16R = crate::FieldReader;
#[doc = "Field `Cmd16` writer - Command #1"]
pub type Cmd16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd26` reader - Command #2"]
pub type Cmd26R = crate::FieldReader;
#[doc = "Field `Cmd26` writer - Command #2"]
pub type Cmd26W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd36` reader - Command #3"]
pub type Cmd36R = crate::FieldReader;
#[doc = "Field `Cmd36` writer - Command #3"]
pub type Cmd36W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd6 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
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
            false => _3b4bcmd6::Command321AreFor3bMode,
            true => _3b4bcmd6::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd6::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd6::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd6` writer - 3B/4B Command"]
pub type _3b4bcmd6W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd6>;
impl<'a, REG> _3b4bcmd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd6::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd6::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting6 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting6> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting6 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting6 {}
#[doc = "Field `Cmd1Setting6` reader - Command #1 setting"]
pub type Cmd1setting6R = crate::FieldReader<Cmd1setting6>;
impl Cmd1setting6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting6 {
        match self.bits {
            0 => Cmd1setting6::Disable,
            1 => Cmd1setting6::ForRead,
            2 => Cmd1setting6::ForWrite,
            3 => Cmd1setting6::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting6::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting6::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting6::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting6::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting6` writer - Command #1 setting"]
pub type Cmd1setting6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting6, crate::Safe>;
impl<'a, REG> Cmd1setting6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting6::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting6::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting6::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting6::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting6 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting6> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting6 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting6 {}
#[doc = "Field `Cmd2Setting6` reader - Command #2 setting"]
pub type Cmd2setting6R = crate::FieldReader<Cmd2setting6>;
impl Cmd2setting6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting6 {
        match self.bits {
            0 => Cmd2setting6::Disable,
            1 => Cmd2setting6::ForRead,
            2 => Cmd2setting6::ForWrite,
            3 => Cmd2setting6::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting6::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting6::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting6::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting6::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting6` writer - Command #2 setting"]
pub type Cmd2setting6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting6, crate::Safe>;
impl<'a, REG> Cmd2setting6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting6::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting6::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting6::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting6::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting6 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting6> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting6 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting6 {}
#[doc = "Field `Cmd3Setting6` reader - Command #3 setting"]
pub type Cmd3setting6R = crate::FieldReader<Cmd3setting6>;
impl Cmd3setting6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting6 {
        match self.bits {
            0 => Cmd3setting6::Disable,
            1 => Cmd3setting6::ForRead,
            2 => Cmd3setting6::ForWrite,
            3 => Cmd3setting6::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting6::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting6::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting6::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting6::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting6` writer - Command #3 setting"]
pub type Cmd3setting6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting6, crate::Safe>;
impl<'a, REG> Cmd3setting6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting6::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting6::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting6::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting6::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd16(&self) -> Cmd16R {
        Cmd16R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd26(&self) -> Cmd26R {
        Cmd26R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd36(&self) -> Cmd36R {
        Cmd36R::new(((self.bits >> 16) & 0xff) as u8)
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
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting6(&self) -> Cmd1setting6R {
        Cmd1setting6R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting6(&self) -> Cmd2setting6R {
        Cmd2setting6R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting6(&self) -> Cmd3setting6R {
        Cmd3setting6R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd16(&mut self) -> Cmd16W<Fmc168Spec> {
        Cmd16W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd26(&mut self) -> Cmd26W<Fmc168Spec> {
        Cmd26W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd36(&mut self) -> Cmd36W<Fmc168Spec> {
        Cmd36W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd6(&mut self) -> _3b4bcmd6W<Fmc168Spec> {
        _3b4bcmd6W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting6(&mut self) -> Cmd1setting6W<Fmc168Spec> {
        Cmd1setting6W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting6(&mut self) -> Cmd2setting6W<Fmc168Spec> {
        Cmd2setting6W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting6(&mut self) -> Cmd3setting6W<Fmc168Spec> {
        Cmd3setting6W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc168Spec;
impl crate::RegisterSpec for Fmc168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc168::R`](R) reader structure"]
impl crate::Readable for Fmc168Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc168::W`](W) writer structure"]
impl crate::Writable for Fmc168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC168 to value 0"]
impl crate::Resettable for Fmc168Spec {}

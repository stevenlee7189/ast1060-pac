#[doc = "Register `FMC164` reader"]
pub type R = crate::R<Fmc164Spec>;
#[doc = "Register `FMC164` writer"]
pub type W = crate::W<Fmc164Spec>;
#[doc = "Field `Cmd15` reader - Command #1"]
pub type Cmd15R = crate::FieldReader;
#[doc = "Field `Cmd15` writer - Command #1"]
pub type Cmd15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd25` reader - Command #2"]
pub type Cmd25R = crate::FieldReader;
#[doc = "Field `Cmd25` writer - Command #2"]
pub type Cmd25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd35` reader - Command #3"]
pub type Cmd35R = crate::FieldReader;
#[doc = "Field `Cmd35` writer - Command #3"]
pub type Cmd35W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd5 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd5> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd5` reader - 3B/4B Command"]
pub type _3b4bcmd5R = crate::BitReader<_3b4bcmd5>;
impl _3b4bcmd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd5 {
        match self.bits {
            false => _3b4bcmd5::Command321AreFor3bMode,
            true => _3b4bcmd5::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd5::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd5::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd5` writer - 3B/4B Command"]
pub type _3b4bcmd5W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd5>;
impl<'a, REG> _3b4bcmd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd5::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd5::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting5 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting5> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting5 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting5 {}
#[doc = "Field `Cmd1Setting5` reader - Command #1 setting"]
pub type Cmd1setting5R = crate::FieldReader<Cmd1setting5>;
impl Cmd1setting5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting5 {
        match self.bits {
            0 => Cmd1setting5::Disable,
            1 => Cmd1setting5::ForRead,
            2 => Cmd1setting5::ForWrite,
            3 => Cmd1setting5::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting5::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting5::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting5::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting5::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting5` writer - Command #1 setting"]
pub type Cmd1setting5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting5, crate::Safe>;
impl<'a, REG> Cmd1setting5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting5::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting5::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting5::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting5::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting5 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting5> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting5 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting5 {}
#[doc = "Field `Cmd2Setting5` reader - Command #2 setting"]
pub type Cmd2setting5R = crate::FieldReader<Cmd2setting5>;
impl Cmd2setting5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting5 {
        match self.bits {
            0 => Cmd2setting5::Disable,
            1 => Cmd2setting5::ForRead,
            2 => Cmd2setting5::ForWrite,
            3 => Cmd2setting5::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting5::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting5::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting5::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting5::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting5` writer - Command #2 setting"]
pub type Cmd2setting5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting5, crate::Safe>;
impl<'a, REG> Cmd2setting5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting5::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting5::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting5::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting5::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting5 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting5> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting5 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting5 {}
#[doc = "Field `Cmd3Setting5` reader - Command #3 setting"]
pub type Cmd3setting5R = crate::FieldReader<Cmd3setting5>;
impl Cmd3setting5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting5 {
        match self.bits {
            0 => Cmd3setting5::Disable,
            1 => Cmd3setting5::ForRead,
            2 => Cmd3setting5::ForWrite,
            3 => Cmd3setting5::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting5::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting5::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting5::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting5::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting5` writer - Command #3 setting"]
pub type Cmd3setting5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting5, crate::Safe>;
impl<'a, REG> Cmd3setting5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting5::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting5::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting5::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting5::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd15(&self) -> Cmd15R {
        Cmd15R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd25(&self) -> Cmd25R {
        Cmd25R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd35(&self) -> Cmd35R {
        Cmd35R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd5(&self) -> _3b4bcmd5R {
        _3b4bcmd5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting5(&self) -> Cmd1setting5R {
        Cmd1setting5R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting5(&self) -> Cmd2setting5R {
        Cmd2setting5R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting5(&self) -> Cmd3setting5R {
        Cmd3setting5R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd15(&mut self) -> Cmd15W<Fmc164Spec> {
        Cmd15W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd25(&mut self) -> Cmd25W<Fmc164Spec> {
        Cmd25W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd35(&mut self) -> Cmd35W<Fmc164Spec> {
        Cmd35W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd5(&mut self) -> _3b4bcmd5W<Fmc164Spec> {
        _3b4bcmd5W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting5(&mut self) -> Cmd1setting5W<Fmc164Spec> {
        Cmd1setting5W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting5(&mut self) -> Cmd2setting5W<Fmc164Spec> {
        Cmd2setting5W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting5(&mut self) -> Cmd3setting5W<Fmc164Spec> {
        Cmd3setting5W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc164Spec;
impl crate::RegisterSpec for Fmc164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc164::R`](R) reader structure"]
impl crate::Readable for Fmc164Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc164::W`](W) writer structure"]
impl crate::Writable for Fmc164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC164 to value 0"]
impl crate::Resettable for Fmc164Spec {}

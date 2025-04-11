#[doc = "Register `HACE50` reader"]
pub type R = crate::R<Hace50Spec>;
#[doc = "Register `HACE50` writer"]
pub type W = crate::W<Hace50Spec>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `CmdQueueBaseAddr304` reader - Command queue base address\\[30:4\\] (16-byte aligned)"]
pub type CmdQueueBaseAddr304R = crate::FieldReader<u32>;
#[doc = "Field `CmdQueueBaseAddr304` writer - Command queue base address\\[30:4\\] (16-byte aligned)"]
pub type CmdQueueBaseAddr304W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:30 - Command queue base address\\[30:4\\] (16-byte aligned)"]
    #[inline(always)]
    pub fn cmd_queue_base_addr304(&self) -> CmdQueueBaseAddr304R {
        CmdQueueBaseAddr304R::new((self.bits >> 4) & 0x07ff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:30 - Command queue base address\\[30:4\\] (16-byte aligned)"]
    #[inline(always)]
    pub fn cmd_queue_base_addr304(&mut self) -> CmdQueueBaseAddr304W<Hace50Spec> {
        CmdQueueBaseAddr304W::new(self, 4)
    }
}
#[doc = "Command Queue Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`hace50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace50Spec;
impl crate::RegisterSpec for Hace50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace50::R`](R) reader structure"]
impl crate::Readable for Hace50Spec {}
#[doc = "`write(|w| ..)` method takes [`hace50::W`](W) writer structure"]
impl crate::Writable for Hace50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE50 to value 0"]
impl crate::Resettable for Hace50Spec {}

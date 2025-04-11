#[doc = "Register `HACE58` reader"]
pub type R = crate::R<Hace58Spec>;
#[doc = "Register `HACE58` writer"]
pub type W = crate::W<Hace58Spec>;
#[doc = "Field `CmdQueueWrPointer110` reader - Command queue write pointer\\[11:0\\] (8-byte aligned)"]
pub type CmdQueueWrPointer110R = crate::FieldReader<u16>;
#[doc = "Field `CmdQueueWrPointer110` writer - Command queue write pointer\\[11:0\\] (8-byte aligned)"]
pub type CmdQueueWrPointer110W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:11 - Command queue write pointer\\[11:0\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn cmd_queue_wr_pointer110(&self) -> CmdQueueWrPointer110R {
        CmdQueueWrPointer110R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Command queue write pointer\\[11:0\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn cmd_queue_wr_pointer110(&mut self) -> CmdQueueWrPointer110W<Hace58Spec> {
        CmdQueueWrPointer110W::new(self, 0)
    }
}
#[doc = "Command Queue Write Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`hace58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace58Spec;
impl crate::RegisterSpec for Hace58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace58::R`](R) reader structure"]
impl crate::Readable for Hace58Spec {}
#[doc = "`write(|w| ..)` method takes [`hace58::W`](W) writer structure"]
impl crate::Writable for Hace58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE58 to value 0"]
impl crate::Resettable for Hace58Spec {}

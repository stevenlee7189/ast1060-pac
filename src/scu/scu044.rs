#[doc = "Register `SCU044` reader"]
pub type R = crate::R<Scu044Spec>;
#[doc = "Register `SCU044` writer"]
pub type W = crate::W<Scu044Spec>;
#[doc = "Field `SCU040SysRstCtrlClearReg` reader - SCU040 System Reset Control Clear Register"]
pub type Scu040sysRstCtrlClearRegR = crate::FieldReader<u32>;
#[doc = "Field `SCU040SysRstCtrlClearReg` writer - SCU040 System Reset Control Clear Register"]
pub type Scu040sysRstCtrlClearRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU040 System Reset Control Clear Register"]
    #[inline(always)]
    pub fn scu040sys_rst_ctrl_clear_reg(&self) -> Scu040sysRstCtrlClearRegR {
        Scu040sysRstCtrlClearRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU040 System Reset Control Clear Register"]
    #[inline(always)]
    pub fn scu040sys_rst_ctrl_clear_reg(&mut self) -> Scu040sysRstCtrlClearRegW<Scu044Spec> {
        Scu040sysRstCtrlClearRegW::new(self, 0)
    }
}
#[doc = "System Reset Control Clear Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu044Spec;
impl crate::RegisterSpec for Scu044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu044::R`](R) reader structure"]
impl crate::Readable for Scu044Spec {}
#[doc = "`write(|w| ..)` method takes [`scu044::W`](W) writer structure"]
impl crate::Writable for Scu044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU044 to value 0"]
impl crate::Resettable for Scu044Spec {}

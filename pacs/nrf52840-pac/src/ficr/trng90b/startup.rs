#[doc = "Register `STARTUP` reader"]
pub type R = crate::R<StartupSpec>;
#[doc = "Field `STARTUP` reader - Amount of bytes for the startup tests"]
pub type StartupR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Amount of bytes for the startup tests"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(self.bits)
    }
}
#[doc = "Amount of bytes for the startup tests\n\nYou can [`read`](crate::Reg::read) this register and get [`startup::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartupSpec;
impl crate::RegisterSpec for StartupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startup::R`](R) reader structure"]
impl crate::Readable for StartupSpec {}
#[doc = "`reset()` method sets STARTUP to value 0xffff_ffff"]
impl crate::Resettable for StartupSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

#[doc = "Register `HOST_IOT_KPRTL_LOCK` reader"]
pub type R = crate::R<HostIotKprtlLockSpec>;
#[doc = "Register `HOST_IOT_KPRTL_LOCK` writer"]
pub type W = crate::W<HostIotKprtlLockSpec>;
#[doc = "This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostIotKprtlLock {
    #[doc = "0: K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    Disabled = 0,
    #[doc = "1: K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    Enabled = 1,
}
impl From<HostIotKprtlLock> for bool {
    #[inline(always)]
    fn from(variant: HostIotKprtlLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_IOT_KPRTL_LOCK` reader - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub type HostIotKprtlLockR = crate::BitReader<HostIotKprtlLock>;
impl HostIotKprtlLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostIotKprtlLock {
        match self.bits {
            false => HostIotKprtlLock::Disabled,
            true => HostIotKprtlLock::Enabled,
        }
    }
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HostIotKprtlLock::Disabled
    }
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HostIotKprtlLock::Enabled
    }
}
#[doc = "Field `HOST_IOT_KPRTL_LOCK` writer - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub type HostIotKprtlLockW<'a, REG> = crate::BitWriter<'a, REG, HostIotKprtlLock>;
impl<'a, REG> HostIotKprtlLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HostIotKprtlLock::Disabled)
    }
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HostIotKprtlLock::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kprtl_lock(&self) -> HostIotKprtlLockR {
        HostIotKprtlLockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    #[must_use]
    pub fn host_iot_kprtl_lock(&mut self) -> HostIotKprtlLockW<HostIotKprtlLockSpec> {
        HostIotKprtlLockW::new(self, 0)
    }
}
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_iot_kprtl_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kprtl_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostIotKprtlLockSpec;
impl crate::RegisterSpec for HostIotKprtlLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_iot_kprtl_lock::R`](R) reader structure"]
impl crate::Readable for HostIotKprtlLockSpec {}
#[doc = "`write(|w| ..)` method takes [`host_iot_kprtl_lock::W`](W) writer structure"]
impl crate::Writable for HostIotKprtlLockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_IOT_KPRTL_LOCK to value 0"]
impl crate::Resettable for HostIotKprtlLockSpec {
    const RESET_VALUE: u32 = 0;
}

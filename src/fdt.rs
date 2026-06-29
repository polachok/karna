use ::fdt::Fdt;

pub unsafe fn init(addr: *const u8) -> HwMap<'static> {
    let fdt = unsafe { Fdt::from_ptr(addr).unwrap() };
    HwMap { inner: fdt }
}

pub struct HwMap<'a> {
    inner: Fdt<'a>,
}

pub struct CpuInfo<'a> {
    pub id: usize,
    pub compatible: &'a str,
}

impl<'a> HwMap<'a> {
    pub fn model(&'a self) -> &'a str {
        self.inner.root().model()
    }

    pub fn cpus(&'a self) -> impl Iterator<Item = CpuInfo<'a>> {
        self.inner.cpus().map(|cpu| CpuInfo {
            id: cpu.ids().first(),
            compatible: cpu.property("compatible").unwrap().as_str().unwrap(),
        })
    }
}

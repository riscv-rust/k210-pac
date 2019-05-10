#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 1st-4th word of key"]
    pub key: [KEY; 4],
    #[doc = "0x10 - Encryption or decryption select"]
    pub encrypt_sel: ENCRYPT_SEL,
    #[doc = "0x14 - AES mode register"]
    pub mode_ctl: MODE_CTL,
    #[doc = "0x18 - Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
    pub iv: [IV; 4],
    #[doc = "0x28 - Endian control"]
    pub endian: ENDIAN,
    #[doc = "0x2c - Finished status"]
    pub finish: FINISH,
    #[doc = "0x30 - DMA select"]
    pub dma_sel: DMA_SEL,
    #[doc = "0x34 - GCM additional authenticated data count in bytes, minus one"]
    pub aad_num: AAD_NUM,
    _reserved0: [u8; 4usize],
    #[doc = "0x3c - Plaintext/ciphertext input data count in bytes, minus one"]
    pub pc_num: PC_NUM,
    #[doc = "0x40 - Plaintext/ciphertext input data"]
    pub text_data: TEXT_DATA,
    #[doc = "0x44 - Additional authenticated data"]
    pub aad_data: AAD_DATA,
    #[doc = "0x48 - Tag check status"]
    pub tag_chk: TAG_CHK,
    #[doc = "0x4c - Data can input flag"]
    pub data_in_flag: DATA_IN_FLAG,
    #[doc = "0x50 - GCM input tag for comparison with the calculated tag"]
    pub gcm_in_tag: [GCM_IN_TAG; 4],
    #[doc = "0x60 - Plaintext/ciphertext output data"]
    pub out_data: OUT_DATA,
    #[doc = "0x64 - AES module enable"]
    pub en: EN,
    #[doc = "0x68 - Data can output flag"]
    pub data_out_flag: DATA_OUT_FLAG,
    #[doc = "0x6c - Can input tag (when using GCM)"]
    pub tag_in_flag: TAG_IN_FLAG,
    #[doc = "0x70 - Tag clear (a write to this register clears the tag_chk status)"]
    pub tag_clear: TAG_CLEAR,
    #[doc = "0x74 - Computed GCM output tag"]
    pub gcm_out_tag: [GCM_OUT_TAG; 4],
    #[doc = "0x84 - 5th-8th word of key"]
    pub key_ext: [KEY_EXT; 4],
}
#[doc = "1st-4th word of key"]
pub struct KEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1st-4th word of key"]
pub mod key;
#[doc = "Encryption or decryption select"]
pub struct ENCRYPT_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Encryption or decryption select"]
pub mod encrypt_sel;
#[doc = "AES mode register"]
pub struct MODE_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES mode register"]
pub mod mode_ctl;
#[doc = "Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
pub struct IV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
pub mod iv;
#[doc = "Endian control"]
pub struct ENDIAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endian control"]
pub mod endian;
#[doc = "Finished status"]
pub struct FINISH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Finished status"]
pub mod finish;
#[doc = "DMA select"]
pub struct DMA_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA select"]
pub mod dma_sel;
#[doc = "GCM additional authenticated data count in bytes, minus one"]
pub struct AAD_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCM additional authenticated data count in bytes, minus one"]
pub mod aad_num;
#[doc = "Plaintext/ciphertext input data count in bytes, minus one"]
pub struct PC_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Plaintext/ciphertext input data count in bytes, minus one"]
pub mod pc_num;
#[doc = "Plaintext/ciphertext input data"]
pub struct TEXT_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Plaintext/ciphertext input data"]
pub mod text_data;
#[doc = "Additional authenticated data"]
pub struct AAD_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Additional authenticated data"]
pub mod aad_data;
#[doc = "Tag check status"]
pub struct TAG_CHK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tag check status"]
pub mod tag_chk;
#[doc = "Data can input flag"]
pub struct DATA_IN_FLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data can input flag"]
pub mod data_in_flag;
#[doc = "GCM input tag for comparison with the calculated tag"]
pub struct GCM_IN_TAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCM input tag for comparison with the calculated tag"]
pub mod gcm_in_tag;
#[doc = "Plaintext/ciphertext output data"]
pub struct OUT_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Plaintext/ciphertext output data"]
pub mod out_data;
#[doc = "AES module enable"]
pub struct EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES module enable"]
pub mod en;
#[doc = "Data can output flag"]
pub struct DATA_OUT_FLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data can output flag"]
pub mod data_out_flag;
#[doc = "Can input tag (when using GCM)"]
pub struct TAG_IN_FLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Can input tag (when using GCM)"]
pub mod tag_in_flag;
#[doc = "Tag clear (a write to this register clears the tag_chk status)"]
pub struct TAG_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tag clear (a write to this register clears the tag_chk status)"]
pub mod tag_clear;
#[doc = "Computed GCM output tag"]
pub struct GCM_OUT_TAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Computed GCM output tag"]
pub mod gcm_out_tag;
#[doc = "5th-8th word of key"]
pub struct KEY_EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "5th-8th word of key"]
pub mod key_ext;

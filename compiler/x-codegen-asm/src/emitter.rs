//! 二进制输出发射器
//!
//! 支持多种二进制输出格式：ELF, Mach-O, PE, Wasm

use std::collections::HashMap;
use std::io::{self, Write};

/// 辅助扩展：对齐计算
trait Align {
    fn round_next_multiple_of(self, align: Self) -> Self;
}

impl Align for u32 {
    #[inline]
    fn round_next_multiple_of(self, align: Self) -> Self {
        ((self + align - 1) / align) * align
    }
}

impl Align for usize {
    #[inline]
    fn round_next_multiple_of(self, align: Self) -> Self {
        ((self + align - 1) / align) * align
    }
}

// ============================================================================
// 二进制格式
// ============================================================================

/// 支持的二进制输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryFormat {
    /// ELF (Linux, BSD)
    Elf,
    /// Mach-O (macOS, iOS)
    MachO,
    /// PE/COFF (Windows)
    PE,
    /// WebAssembly
    Wasm,
    /// 原始二进制
    Raw,
}

impl BinaryFormat {
    /// 获取格式的魔数
    pub fn magic(&self) -> &'static [u8] {
        match self {
            BinaryFormat::Elf => &[0x7f, b'E', b'L', b'F'],
            BinaryFormat::MachO => &[0xfe, 0xed, 0xfa, 0xce],
            BinaryFormat::PE => &[b'M', b'Z'],
            BinaryFormat::Wasm => &[0x00, 0x61, 0x73, 0x6d],
            BinaryFormat::Raw => &[],
        }
    }

    /// 获取默认文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            BinaryFormat::Elf => "o",
            BinaryFormat::MachO => "o",
            BinaryFormat::PE => "obj",
            BinaryFormat::Wasm => "wasm",
            BinaryFormat::Raw => "bin",
        }
    }
}

// ============================================================================
// 二进制发射器
// ============================================================================

/// 二进制文件发射器
///
/// 用于构建和输出可执行文件或目标文件
pub struct BinaryEmitter {
    /// 二进制格式
    format: BinaryFormat,
    /// 代码段
    text_section: Vec<u8>,
    /// 数据段
    data_section: Vec<u8>,
    /// 只读数据段
    rodata_section: Vec<u8>,
    /// BSS 段（未初始化数据）
    bss_size: usize,
    /// 符号表
    symbols: HashMap<String, SymbolInfo>,
    /// 重定位表
    relocations: Vec<Relocation>,
    /// 当前段偏移
    current_offset: usize,
    /// 导入的外部函数列表: (DLL名称, 函数名称)
    imported_functions: Vec<(String, String)>,
}

/// 符号信息
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct SymbolInfo {
    /// 符号名称
    name: String,
    /// 符号类型
    sym_type: SymbolType,
    /// 绑定类型
    binding: SymbolBinding,
    /// 所在段
    section: Section,
    /// 偏移量
    offset: usize,
    /// 大小
    size: usize,
}

/// 符号类型
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolType {
    /// 未定义
    Notype,
    /// 函数
    Func,
    /// 对象（变量）
    Object,
    /// 段
    Section,
    /// 文件
    File,
}

/// 符号绑定类型
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolBinding {
    /// 局部符号
    Local,
    /// 全局符号
    Global,
    /// 弱符号
    Weak,
}

/// 段类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    /// 代码段
    Text,
    /// 数据段
    Data,
    /// 只读数据段
    Rodata,
    /// BSS 段
    Bss,
}

/// 重定位条目
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Relocation {
    /// 重定位偏移
    offset: usize,
    /// 重定位类型
    rel_type: RelocationType,
    /// 关联符号
    symbol: String,
    /// 添加值
    addend: i64,
}

/// 重定位类型
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelocationType {
    /// 64 位绝对地址
    Absolute64,
    /// 32 位相对地址
    Relative32,
    /// PLT 跳转
    Plt32,
    /// GOT 引用
    Got64,
}

impl BinaryEmitter {
    /// 创建新的二进制发射器
    pub fn new(format: BinaryFormat) -> Self {
        Self {
            format,
            text_section: Vec::new(),
            data_section: Vec::new(),
            rodata_section: Vec::new(),
            bss_size: 0,
            symbols: HashMap::new(),
            relocations: Vec::new(),
            current_offset: 0,
            imported_functions: Vec::new(),
        }
    }

    /// 添加导入函数
    pub fn add_imported_function(&mut self, dll: String, func: String) {
        self.imported_functions.push((dll, func));
    }

    /// 获取二进制格式
    pub fn format(&self) -> BinaryFormat {
        self.format
    }

    /// 向代码段写入字节
    pub fn emit_code(&mut self, bytes: &[u8]) {
        self.text_section.extend_from_slice(bytes);
        self.current_offset = self.text_section.len();
    }

    /// 写入单条指令
    pub fn emit_instruction(&mut self, opcode: u8, operands: &[u8]) {
        self.text_section.push(opcode);
        self.text_section.extend_from_slice(operands);
        self.current_offset = self.text_section.len();
    }

    /// 向数据段写入字节
    pub fn emit_data(&mut self, bytes: &[u8]) {
        self.data_section.extend_from_slice(bytes);
    }

    /// 向只读数据段写入字节
    pub fn emit_rodata(&mut self, bytes: &[u8]) {
        self.rodata_section.extend_from_slice(bytes);
    }

    /// 添加全局符号
    pub fn add_global_symbol(&mut self, name: &str, section: Section, offset: usize, size: usize) {
        self.symbols.insert(
            name.to_string(),
            SymbolInfo {
                name: name.to_string(),
                sym_type: SymbolType::Func,
                binding: SymbolBinding::Global,
                section,
                offset,
                size,
            },
        );
    }

    /// 添加局部符号
    pub fn add_local_symbol(&mut self, name: &str, section: Section, offset: usize, size: usize) {
        self.symbols.insert(
            name.to_string(),
            SymbolInfo {
                name: name.to_string(),
                sym_type: SymbolType::Object,
                binding: SymbolBinding::Local,
                section,
                offset,
                size,
            },
        );
    }

    /// 添加外部符号引用
    pub fn add_external_symbol(&mut self, name: &str) {
        self.symbols.insert(
            name.to_string(),
            SymbolInfo {
                name: name.to_string(),
                sym_type: SymbolType::Notype,
                binding: SymbolBinding::Global,
                section: Section::Text, // 未定义符号
                offset: 0,
                size: 0,
            },
        );
    }

    /// 添加重定位条目
    pub fn add_relocation(
        &mut self,
        offset: usize,
        rel_type: RelocationType,
        symbol: &str,
        addend: i64,
    ) {
        self.relocations.push(Relocation {
            offset,
            rel_type,
            symbol: symbol.to_string(),
            addend,
        });
    }

    /// 分配 BSS 空间
    pub fn allocate_bss(&mut self, size: usize) {
        self.bss_size += size;
    }

    /// 获取当前代码段偏移
    pub fn current_offset(&self) -> usize {
        self.current_offset
    }

    /// 获取代码段大小
    pub fn text_size(&self) -> usize {
        self.text_section.len()
    }

    /// 获取数据段大小
    pub fn data_size(&self) -> usize {
        self.data_section.len()
    }

    /// 生成完整的二进制输出
    pub fn emit(&self) -> io::Result<Vec<u8>> {
        match self.format {
            BinaryFormat::Elf => self.emit_elf(),
            BinaryFormat::MachO => self.emit_macho(),
            BinaryFormat::PE => self.emit_pe(),
            BinaryFormat::Wasm => self.emit_wasm(),
            BinaryFormat::Raw => self.emit_raw(),
        }
    }

    /// 生成 ELF 文件
    fn emit_elf(&self) -> io::Result<Vec<u8>> {
        let mut output = Vec::new();

        // ELF 头部 (64-bit)
        output.write_all(&[0x7f, b'E', b'L', b'F'])?; // 魔数
        output.write_all(&[2])?; // 64-bit
        output.write_all(&[1])?; // 小端序
        output.write_all(&[1])?; // ELF 版本
        output.write_all(&[0; 9])?; // 填充

        // e_type: ET_EXEC = 2
        output.write_all(&2u16.to_le_bytes())?;
        // e_machine: EM_X86_64 = 62
        output.write_all(&62u16.to_le_bytes())?;
        // e_version
        output.write_all(&1u32.to_le_bytes())?;
        // e_entry: 入口点地址
        output.write_all(&0x400000u64.to_le_bytes())?;
        // e_phoff: 程序头偏移
        output.write_all(&64u64.to_le_bytes())?;
        // e_shoff: 段头偏移（后面填充）
        let _shoff_offset = output.len();
        output.write_all(&0u64.to_le_bytes())?;
        // e_flags
        output.write_all(&0u32.to_le_bytes())?;
        // e_ehsize: ELF 头大小
        output.write_all(&64u16.to_le_bytes())?;
        // e_phentsize: 程序头条目大小
        output.write_all(&56u16.to_le_bytes())?;
        // e_phnum: 程序头数量
        output.write_all(&2u16.to_le_bytes())?;
        // e_shentsize: 段头条目大小
        output.write_all(&64u16.to_le_bytes())?;
        // e_shnum: 段头数量
        output.write_all(&4u16.to_le_bytes())?;
        // e_shstrndx: 段名字符串表索引
        output.write_all(&3u16.to_le_bytes())?;

        // 程序头 - 代码段
        // p_type: PT_LOAD = 1
        output.write_all(&1u32.to_le_bytes())?;
        // p_flags: PF_R | PF_X = 5
        output.write_all(&5u32.to_le_bytes())?;
        // p_offset
        output.write_all(&0u64.to_le_bytes())?;
        // p_vaddr
        output.write_all(&0x400000u64.to_le_bytes())?;
        // p_paddr
        output.write_all(&0x400000u64.to_le_bytes())?;
        // p_filesz
        output.write_all(&(self.text_section.len() as u64).to_le_bytes())?;
        // p_memsz
        output.write_all(&(self.text_section.len() as u64).to_le_bytes())?;
        // p_align
        output.write_all(&0x1000u64.to_le_bytes())?;

        // 程序头 - 数据段
        output.write_all(&1u32.to_le_bytes())?; // p_type
        output.write_all(&6u32.to_le_bytes())?; // p_flags: PF_R | PF_W
        output.write_all(&0u64.to_le_bytes())?; // p_offset（待修正）
        output.write_all(&0x600000u64.to_le_bytes())?; // p_vaddr
        output.write_all(&0x600000u64.to_le_bytes())?; // p_paddr
        output.write_all(&(self.data_section.len() as u64).to_le_bytes())?;
        output.write_all(&((self.data_section.len() + self.bss_size) as u64).to_le_bytes())?;
        output.write_all(&0x1000u64.to_le_bytes())?;

        // 代码段内容
        output.write_all(&self.text_section)?;

        // 数据段内容
        output.write_all(&self.data_section)?;

        Ok(output)
    }

    /// 生成 Mach-O 文件
    fn emit_macho(&self) -> io::Result<Vec<u8>> {
        let mut output = Vec::new();

        // Mach-O 头部 (64-bit)
        output.write_all(&0xfeedfacfu32.to_le_bytes())?; // 魔数
        output.write_all(&0x01000007u32.to_le_bytes())?; // cputype: CPU_TYPE_X86_64
        output.write_all(&3u32.to_le_bytes())?; // cpusubtype: CPU_SUBTYPE_X86_64_ALL
        output.write_all(&2u32.to_le_bytes())?; // filetype: MH_EXECUTE
        output.write_all(&0u32.to_le_bytes())?; // ncmds（待填充）
        output.write_all(&0u32.to_le_bytes())?; // sizeofcmds（待填充）
        output.write_all(&0x0085u32.to_le_bytes())?; // flags
        output.write_all(&0u32.to_le_bytes())?; // reserved

        // LC_SEGMENT_64 命令
        // ...（简化实现）

        // 代码段
        output.write_all(&self.text_section)?;

        // 数据段
        output.write_all(&self.data_section)?;

        Ok(output)
    }

    /// 生成 PE/COFF 文件
    pub fn emit_pe(&self) -> io::Result<Vec<u8>> {
        let mut output = Vec::new();

        // --------------------------------------------------------------------
        // DOS 头部
        // --------------------------------------------------------------------
        output.write_all(b"MZ")?; // DOS 签名
        output.write_all(&[0u8; 58])?; // DOS 头部其余部分填充
        output.write_all(&0x80u32.to_le_bytes())?; // e_lfanew: PE 头偏移量 (0x80)

        // 填充 DOS 存根程序到 0x80 偏移
        let dos_stub_size = 0x80 - (2 + 58 + 4);
        output.write_all(&vec![0u8; dos_stub_size])?;

        // --------------------------------------------------------------------
        // PE 签名
        // --------------------------------------------------------------------
        output.write_all(b"PE\x00\x00")?;

        // --------------------------------------------------------------------
        // COFF 文件头
        // --------------------------------------------------------------------
        let number_of_sections = if self.bss_size > 0 { 3 } else { 2 }
            + if !self.relocations.is_empty() { 1 } else { 0 };
        output.write_all(&0x8664u16.to_le_bytes())?; // Machine: AMD64
        output.write_all(&(number_of_sections as u16).to_le_bytes())?; // NumberOfSections
        output.write_all(&0u32.to_le_bytes())?; // TimeDateStamp
        output.write_all(&0u32.to_le_bytes())?; // PointerToSymbolTable
        output.write_all(&0u32.to_le_bytes())?; // NumberOfSymbols
                                                // SizeOfOptionalHeader: PE32+ 可选头大小是 112 (0x70) + 数据目录
        let size_of_optional_header = 112 + 16 * 2; // 标准字段 + 2 个数据目录（IAT 和 Import）
        output.write_all(&(size_of_optional_header as u16).to_le_bytes())?;
        // Characteristics: 可执行文件 = 0x0022 (IMAGE_FILE_EXECUTABLE_IMAGE | IMAGE_FILE_32BIT_MACHINE)
        // 对于 AMD64 需要: IMAGE_FILE_LARGE_ADDRESS_AWARE = 0x0020 → 实际 0x0022 对
        output.write_all(&0x0022u16.to_le_bytes())?;

        // --------------------------------------------------------------------
        // 可选头 (PE32+ 格式，64位)
        // --------------------------------------------------------------------
        // Magic: PE32+ = 0x20B
        output.write_all(&0x20Bu16.to_le_bytes())?;
        output.write_all(&[0x02u8])?; // Linker major version
        output.write_all(&[0x00u8])?; // Linker minor version
                                      // SizeOfCode
        let size_of_code = self.text_section.len() as u32;
        output.write_all(&size_of_code.to_le_bytes())?;
        // SizeOfInitializedData
        let size_of_initialized_data = (self.data_section.len() + self.rodata_section.len()) as u32;
        output.write_all(&size_of_initialized_data.to_le_bytes())?;
        // SizeOfUninitializedData
        output.write_all(&(self.bss_size as u32).to_le_bytes())?;
        // AddressOfEntryPoint
        let entry_point_rva: u32 = 0x1000; // .text 段基地址
        output.write_all(&entry_point_rva.to_le_bytes())?;
        // BaseOfCode
        output.write_all(&0x1000u32.to_le_bytes())?;

        // PE32+ 没有 BaseOfData，直接是 ImageBase
        let image_base = 0x140000000u64; // 默认首选加载地址
        output.write_all(&image_base.to_le_bytes())?;

        // SectionAlignment (内存对齐) 和 FileAlignment (文件对齐)
        let section_alignment = 0x1000u32; // 4KB
        let file_alignment = 0x200u32; // 512 字节
        output.write_all(&section_alignment.to_le_bytes())?;
        output.write_all(&file_alignment.to_le_bytes())?;

        // OS 版本
        output.write_all(&0x0006u16.to_le_bytes())?; // MajorOperatingSystemVersion
        output.write_all(&0x0000u16.to_le_bytes())?; // MinorOperatingSystemVersion
        output.write_all(&0x0000u16.to_le_bytes())?; // MajorImageVersion
        output.write_all(&0x0000u16.to_le_bytes())?; // MinorImageVersion
        output.write_all(&0x0004u16.to_le_bytes())?; // MajorSubsystemVersion
        output.write_all(&0x0000u16.to_le_bytes())?; // MinorSubsystemVersion

        // Win32VersionValue
        output.write_all(&0u32.to_le_bytes())?;

        // SizeOfImage
        let mut size_of_image = 0u32;
        size_of_image += section_alignment; // .text 从 0x1000 开始
        size_of_image = size_of_image.round_next_multiple_of(section_alignment);
        size_of_image += (size_of_code as u32).round_next_multiple_of(section_alignment);
        size_of_image += size_of_initialized_data.round_next_multiple_of(section_alignment);
        if self.bss_size > 0 {
            size_of_image += (self.bss_size as u32).round_next_multiple_of(section_alignment);
        }
        output.write_all(&size_of_image.to_le_bytes())?;

        // SizeOfHeaders
        let header_size = 0x80 + // DOS 头
            4 + // PE 签名
            20 + // COFF 头
            size_of_optional_header + // 可选头
            40 * number_of_sections; // 每个节头 40 字节
        let size_of_headers = (header_size as u32).round_next_multiple_of(file_alignment);
        output.write_all(&size_of_headers.to_le_bytes())?;

        // CheckSum
        output.write_all(&0u32.to_le_bytes())?;
        // Subsystem: Windows GUI = 2, CUI (控制台) = 3
        // 我们做控制台程序
        output.write_all(&3u16.to_le_bytes())?; // Subsystem
                                                // DllCharacteristics
        output.write_all(&0x8140u16.to_le_bytes())?;
        // SizeOfStackReserve
        output.write_all(&0x100000u64.to_le_bytes())?;
        // SizeOfStackCommit
        output.write_all(&0x1000u64.to_le_bytes())?;
        // SizeOfHeapReserve
        output.write_all(&0x100000u64.to_le_bytes())?;
        // SizeOfHeapCommit
        output.write_all(&0x1000u64.to_le_bytes())?;
        // LoaderFlags
        output.write_all(&0u32.to_le_bytes())?;
        // NumberOfRvaAndSizes
        let number_of_rva_and_sizes = 2u32; // 我们只需要 Import Table 和 IAT
        output.write_all(&number_of_rva_and_sizes.to_le_bytes())?;

        // 数据目录
        // 我们需要导入表和导入地址表 (IAT)
        let num_imports = self.imported_functions.len();
        if num_imports > 0 {
            // 按 DLL 分组导入
            use std::collections::HashMap;
            let mut imports_by_dll: HashMap<String, Vec<String>> = HashMap::new();
            for (dll, func) in &self.imported_functions {
                imports_by_dll
                    .entry(dll.clone())
                    .or_default()
                    .push(func.clone());
            }

            // 计算导入表大小：
            // - 每个 DLL: 一个 IMAGE_IMPORT_DESCRIPTOR (20 字节)
            // - 以空描述符结束 (+20)
            // - ILT: 每个导入一个 IMAGE_THUNK_DATA (8 字节) + 空结束 (+8)
            // - IAT: 每个导入一个地址 (8 字节)
            // - DLL 名称字符串：每个名称 +1 (null 终止)
            // - hint/name: 每个导入 2 字节 hint + 名称 + null 终止
            let import_desc_size = (imports_by_dll.len() + 1) * 20;
            let ilt_size = (num_imports + imports_by_dll.len()) * 8;
            let iat_size = num_imports * 8;
            let import_table_size = (import_desc_size + ilt_size + iat_size) as u32;
            // 名称存储在导入表末尾之后，这里简化计算，我们将导入表放在 .data 段末尾
            let import_table_rva = 0x1000
                + (size_of_code.round_next_multiple_of(section_alignment))
                + (size_of_initialized_data).round_next_multiple_of(section_alignment);
            let iat_rva = import_table_rva + import_desc_size as u32 + ilt_size as u32;
            let iat_size = num_imports * 8;

            output.write_all(&import_table_rva.to_le_bytes())?;
            output.write_all(&import_table_size.to_le_bytes())?;
            // IAT 数据目录
            output.write_all(&iat_rva.to_le_bytes())?;
            output.write_all(&(iat_size as u32).to_le_bytes())?;
        } else {
            // 没有导入
            output.write_all(&0u32.to_le_bytes())?; // import table rva
            output.write_all(&0u32.to_le_bytes())?; // import table size
            output.write_all(&0u32.to_le_bytes())?; // IAT rva
            output.write_all(&0u32.to_le_bytes())?; // IAT size
        }
        // 其余数据目录不需要，留空
        // 如果需要更多数据目录，这里继续添加
        // 其余数据目录不需要，留空
        // 我们已经只声明了 2 个，所以这里结束

        // --------------------------------------------------------------------
        // 节表
        // --------------------------------------------------------------------
        let header_end = output.len();
        let mut current_rva = 0x1000u32; // 第一个段从 0x1000 开始
        let mut current_raw = size_of_headers;

        // .text 段
        output.write_all(b".text\x00\x00\x00")?; // Name
        let virtual_size = size_of_code;
        output.write_all(&virtual_size.to_le_bytes())?; // VirtualSize
        output.write_all(&current_rva.to_le_bytes())?; // VirtualAddress
        let raw_size = size_of_code.round_next_multiple_of(file_alignment);
        output.write_all(&raw_size.to_le_bytes())?; // SizeOfRawData
        output.write_all(&current_raw.to_le_bytes())?; // PointerToRawData
        output.write_all(&0u32.to_le_bytes())?; // PointerToRelocations
        output.write_all(&0u32.to_le_bytes())?; // PointerToLineNumbers
        output.write_all(&0u16.to_le_bytes())?; // NumberOfRelocations
        output.write_all(&0u16.to_le_bytes())?; // NumberOfLineNumbers
                                                // Characteristics: CODE | EXECUTE | READ → 0x60000020
        output.write_all(&0x60000020u32.to_le_bytes())?;

        current_rva += size_of_code.round_next_multiple_of(section_alignment);
        current_raw += raw_size;

        // .data 段（包含导入表和只读数据）
        output.write_all(b".data\x00\x00\x00")?;
        let data_virtual_size = size_of_initialized_data + (self.bss_size as u32) + 100;
        output.write_all(&data_virtual_size.to_le_bytes())?;
        output.write_all(&current_rva.to_le_bytes())?;
        let data_raw_size = size_of_initialized_data.round_next_multiple_of(file_alignment);
        output.write_all(&data_raw_size.to_le_bytes())?;
        output.write_all(&current_raw.to_le_bytes())?;
        output.write_all(&0u32.to_le_bytes())?;
        output.write_all(&0u32.to_le_bytes())?;
        output.write_all(&0u16.to_le_bytes())?;
        output.write_all(&0u16.to_le_bytes())?;
        // Characteristics: DATA | READ | WRITE → 0xC0000040
        output.write_all(&0xC0000040u32.to_le_bytes())?;

        current_raw += data_raw_size;

        // 填充头部对齐
        let header_padding = size_of_headers as usize - header_end;
        if header_padding > 0 {
            output.write_all(&vec![0u8; header_padding])?;
        }

        // --------------------------------------------------------------------
        // 段内容
        // --------------------------------------------------------------------
        // .text 段内容
        output.write_all(&self.text_section)?;
        // 填充对齐
        let text_padding = raw_size as usize - self.text_section.len();
        if text_padding > 0 {
            output.write_all(&vec![0u8; text_padding])?;
        }

        // .data 段内容
        output.write_all(&self.data_section)?;
        output.write_all(&self.rodata_section)?;

        // 生成导入表
        if !self.imported_functions.is_empty() {
            use std::collections::HashMap;
            let mut imports_by_dll: HashMap<String, Vec<String>> = HashMap::new();
            for (dll, func) in &self.imported_functions {
                imports_by_dll
                    .entry(dll.clone())
                    .or_default()
                    .push(func.clone());
            }

            // 计算导入表 RVA
            let import_table_rva = 0x1000
                + (size_of_code.round_next_multiple_of(section_alignment))
                + (size_of_initialized_data).round_next_multiple_of(section_alignment);

            // 计算各部分偏移：
            // 1. 导入描述符数组
            // 2. ILT（导入查找表）
            // 3. IAT（导入地址表）
            // 4. DLL 名称和函数名称字符串

            let num_dlls = imports_by_dll.len();
            let num_imports = self.imported_functions.len();

            // 生成导入描述符
            for (_dll_name, _funcs) in &imports_by_dll {
                // 每个 IMAGE_IMPORT_DESCRIPTOR 是 20 字节
                // OriginalFirstThunk (4) - RVA of ILT
                let ilt_rva = import_table_rva + ((num_dlls + 1) * 20) as u32;
                output.write_all(&ilt_rva.to_le_bytes())?;
                // TimeDateStamp (4) = 0
                output.write_all(&0u32.to_le_bytes())?;
                // ForwarderChain (4) = 0
                output.write_all(&0u32.to_le_bytes())?;
                // Name (4) - RVA of DLL name
                let name_rva =
                    import_table_rva + ((num_dlls + 1) * 20 + (num_imports + num_dlls) * 8) as u32;
                output.write_all(&name_rva.to_le_bytes())?;
                // FirstThunk (4) - RVA of IAT
                let iat_start_rva =
                    import_table_rva + ((num_dlls + 1) * 20 + (num_imports + num_dlls) * 8) as u32;
                output.write_all(&iat_start_rva.to_le_bytes())?;
            }
            // 空描述符结束
            output.write_all(&[0u8; 20])?;

            // 生成 ILT
            let mut current_string_offset = 0;
            // Calculate string table start offset after all descriptors and ILT
            let string_table_start =
                ((num_dlls + 1) * 20 + (num_imports + num_dlls) * 8 + num_imports * 8) as u32;

            for (dll_name, funcs) in &imports_by_dll {
                // After descriptors + ILT + all strings before this DLL
                for func in funcs {
                    // IMAGE_THUNK_DATA: 最高位为 1 表示按序号导入，这里我们按名称导入
                    // bit 31 = 0，低 31 位 is RVA 指向 hint/name
                    let hint_name_rva =
                        import_table_rva + string_table_start + current_string_offset as u32;
                    output.write_all(&(hint_name_rva as u64).to_le_bytes())?;
                    current_string_offset += func.len() + 1 + 2;
                }
                // Add DLL name length to current offset after processing functions
                current_string_offset += dll_name.len() + 1;
                // 空 thunk 结束
                output.write_all(&0u64.to_le_bytes())?;
            }

            // IAT 已经预留了空间，现在填充（一开始都为 0，加载器会修复）
            for _ in &self.imported_functions {
                output.write_all(&0u64.to_le_bytes())?;
            }

            // 写入 DLL 名称和函数名称
            for (dll_name, funcs) in &imports_by_dll {
                // DLL name 以 null 结尾
                output.write_all(dll_name.as_bytes())?;
                output.write_all(&[0u8])?;

                // 每个函数的 hint/name
                for func in funcs {
                    // hint 是 2 字节，我们使用 0
                    output.write_all(&0u16.to_le_bytes())?;
                    // 函数名以 null 结尾
                    output.write_all(func.as_bytes())?;
                    output.write_all(&[0u8])?;
                }
            }
        }

        // 填充对齐
        let data_padding = data_raw_size as usize - (output.len() - current_raw as usize);
        if data_padding > 0 {
            output.write_all(&vec![0u8; data_padding])?;
        }

        Ok(output)
    }

    /// 生成 WebAssembly 模块
    fn emit_wasm(&self) -> io::Result<Vec<u8>> {
        let mut output = Vec::new();

        // Wasm 魔数
        output.write_all(&[0x00, 0x61, 0x73, 0x6d])?;

        // 版本号
        output.write_all(&[0x01, 0x00, 0x00, 0x00])?;

        // 类型段（简化）
        output.write_all(&[0x01])?; // Section ID: Type
        output.write_all(&[0x05])?; // Section size
        output.write_all(&[0x01])?; // Number of types
        output.write_all(&[0x60])?; // Function type
        output.write_all(&[0x00])?; // No params
        output.write_all(&[0x01, 0x7f])?; // Result: i32

        // 函数段
        output.write_all(&[0x03])?; // Section ID: Function
        output.write_all(&[0x02])?; // Section size
        output.write_all(&[0x01])?; // Number of functions
        output.write_all(&[0x00])?; // Type index

        // 代码段
        output.write_all(&[0x0a])?; // Section ID: Code
        let code_size = self.text_section.len() + 2;
        output.write_all(&[code_size as u8])?; // Section size
        output.write_all(&[0x01])?; // Number of functions
        output.write_all(&[self.text_section.len() as u8])?; // Function body size
        output.write_all(&self.text_section)?;

        Ok(output)
    }

    /// 生成原始二进制
    fn emit_raw(&self) -> io::Result<Vec<u8>> {
        let mut output = Vec::new();
        output.extend_from_slice(&self.text_section);
        output.extend_from_slice(&self.data_section);
        Ok(output)
    }
}

// ============================================================================
// 辅助结构
// ============================================================================

/// ELF 段头构建器
pub struct SectionHeaderBuilder {
    pub name: String,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl SectionHeaderBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 1,
            sh_entsize: 0,
        }
    }

    pub fn sh_type(mut self, t: u32) -> Self {
        self.sh_type = t;
        self
    }

    pub fn sh_flags(mut self, f: u64) -> Self {
        self.sh_flags = f;
        self
    }

    pub fn sh_addr(mut self, a: u64) -> Self {
        self.sh_addr = a;
        self
    }

    pub fn sh_offset(mut self, o: u64) -> Self {
        self.sh_offset = o;
        self
    }

    pub fn sh_size(mut self, s: u64) -> Self {
        self.sh_size = s;
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&0u32.to_le_bytes()); // sh_name (index into string table)
        buf.extend_from_slice(&self.sh_type.to_le_bytes());
        buf.extend_from_slice(&self.sh_flags.to_le_bytes());
        buf.extend_from_slice(&self.sh_addr.to_le_bytes());
        buf.extend_from_slice(&self.sh_offset.to_le_bytes());
        buf.extend_from_slice(&self.sh_size.to_le_bytes());
        buf.extend_from_slice(&self.sh_link.to_le_bytes());
        buf.extend_from_slice(&self.sh_info.to_le_bytes());
        buf.extend_from_slice(&self.sh_addralign.to_le_bytes());
        buf.extend_from_slice(&self.sh_entsize.to_le_bytes());
        buf
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_format_magic() {
        assert_eq!(BinaryFormat::Elf.magic(), &[0x7f, b'E', b'L', b'F']);
        assert_eq!(BinaryFormat::Wasm.magic(), &[0x00, 0x61, 0x73, 0x6d]);
        assert_eq!(BinaryFormat::Raw.magic(), &[]);
    }

    #[test]
    fn test_binary_format_extension() {
        assert_eq!(BinaryFormat::Elf.extension(), "o");
        assert_eq!(BinaryFormat::MachO.extension(), "o");
        assert_eq!(BinaryFormat::PE.extension(), "obj");
        assert_eq!(BinaryFormat::Wasm.extension(), "wasm");
        assert_eq!(BinaryFormat::Raw.extension(), "bin");
    }

    #[test]
    fn test_binary_emitter_creation() {
        let emitter = BinaryEmitter::new(BinaryFormat::Elf);
        assert_eq!(emitter.format(), BinaryFormat::Elf);
        assert_eq!(emitter.text_size(), 0);
        assert_eq!(emitter.data_size(), 0);
    }

    #[test]
    fn test_emit_code() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Raw);
        emitter.emit_code(&[0x48, 0x89, 0xe5]); // mov rbp, rsp
        assert_eq!(emitter.text_size(), 3);
        assert_eq!(emitter.current_offset(), 3);
    }

    #[test]
    fn test_emit_instruction() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Raw);
        emitter.emit_instruction(0xc3, &[]); // ret
        assert_eq!(emitter.text_size(), 1);
    }

    #[test]
    fn test_emit_data() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Raw);
        emitter.emit_data(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(emitter.data_size(), 4);
    }

    #[test]
    fn test_add_global_symbol() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Elf);
        emitter.add_global_symbol("main", Section::Text, 0, 100);
        assert!(emitter.symbols.contains_key("main"));
    }

    #[test]
    fn test_add_local_symbol() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Elf);
        emitter.add_local_symbol("local_var", Section::Data, 0, 8);
        assert!(emitter.symbols.contains_key("local_var"));
    }

    #[test]
    fn test_add_relocation() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Elf);
        emitter.add_relocation(0x10, RelocationType::Absolute64, "printf", 0);
        assert_eq!(emitter.relocations.len(), 1);
    }

    #[test]
    fn test_allocate_bss() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Elf);
        emitter.allocate_bss(1024);
        assert_eq!(emitter.bss_size, 1024);
    }

    #[test]
    fn test_emit_raw() {
        let mut emitter = BinaryEmitter::new(BinaryFormat::Raw);
        emitter.emit_code(&[0x48, 0x31, 0xc0]); // xor rax, rax
        emitter.emit_data(&[0x42]); // data

        let output = emitter.emit().unwrap();
        assert_eq!(output.len(), 4);
        assert_eq!(&output[..3], &[0x48, 0x31, 0xc0]);
    }

    #[test]
    fn test_emit_elf() {
        let emitter = BinaryEmitter::new(BinaryFormat::Elf);
        let output = emitter.emit().unwrap();

        // 验证 ELF 魔数
        assert_eq!(&output[0..4], &[0x7f, b'E', b'L', b'F']);
    }

    #[test]
    fn test_emit_wasm() {
        let emitter = BinaryEmitter::new(BinaryFormat::Wasm);
        let output = emitter.emit().unwrap();

        // 验证 Wasm 魔数
        assert_eq!(&output[0..4], &[0x00, 0x61, 0x73, 0x6d]);
        // 验证版本号
        assert_eq!(&output[4..8], &[0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_section_header_builder() {
        let sh = SectionHeaderBuilder::new(".text")
            .sh_type(1) // SHT_PROGBITS
            .sh_flags(6) // SHF_ALLOC | SHF_EXECINSTR
            .sh_size(100)
            .build();

        assert_eq!(sh.len(), 64); // ELF64 section header size
    }
}

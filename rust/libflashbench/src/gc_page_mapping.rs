#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]

use linux_types as libc;

extern "C" {
    pub type flash_page;
    pub type fb_context_t;
    pub type fb_bio_t;
    pub type page_mapping_context_t;
    pub type ssd_info;
    pub type fb_bus_controller_t;
    pub type flash_chip;
    //
// Lightweight Linux header. It is designed to be included instead of the actual
// Linux header. Made for c2rust transpilation.
//
    // linux/printk.h
    #[no_mangle]
    fn printk(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    // linux/vmalloc.h
    #[no_mangle]
    fn vmalloc(size: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn vfree(addr: *const libc::c_void);
    #[no_mangle]
    fn get_gcm(ftl: *mut page_mapping_context_t) -> *mut fb_gc_mngr_t;
    #[no_mangle]
    fn print_blk_mgmt(fb: *mut fb_context_t);
    #[no_mangle]
    fn get_ssd_inf(fb: *mut fb_context_t) -> *mut ssd_info;
    #[no_mangle]
    fn get_vdev(fb: *mut fb_context_t) -> *mut vdevice_t;
    #[no_mangle]
    fn get_ftl(fb: *mut fb_context_t) -> *mut libc::c_void;
    #[no_mangle]
    fn set_prev_bus_chip(ptr_fb_context: *mut fb_context_t, bus: u8_0,
                         chip: u8_0);
    #[no_mangle]
    fn get_next_bus_chip(ptr_fb_context: *mut fb_context_t,
                         ptr_bus: *mut u8_0, ptr_chip: *mut u8_0);
    #[no_mangle]
    fn get_curr_gc_block(ptr_fb_context: *mut fb_context_t, bus: u32_0,
                         chip: u32_0) -> *mut flash_block;
    #[no_mangle]
    fn set_curr_gc_block(ptr_fb_context: *mut fb_context_t, bus: u32_0,
                         chip: u32_0, ptr_new_block: *mut flash_block);
    #[no_mangle]
    fn get_curr_active_block(ptr_fb_context: *mut fb_context_t, bus: u32_0,
                             chip: u32_0) -> *mut flash_block;
    #[no_mangle]
    fn set_curr_active_block(ptr_fb_context: *mut fb_context_t, bus: u32_0,
                             chip: u32_0, ptr_new_block: *mut flash_block);
    #[no_mangle]
    fn alloc_new_page(ptr_fb_context: *mut fb_context_t, bus: u8_0,
                      chip: u8_0, ptr_block: *mut u32_0, ptr_page: *mut u32_0)
     -> libc::c_int;
    #[no_mangle]
    fn map_logical_to_physical(ptr_fb_context: *mut fb_context_t,
                               logical_page_address: *mut u32_0, bus: u32_0,
                               chip: u32_0, block: u32_0, page: u32_0)
     -> libc::c_int;
    #[no_mangle]
    fn update_act_blk(fb: *mut fb_context_t, bus: u8_0, chip: u8_0);
    // page info interface
    #[no_mangle]
    fn get_mapped_lpa(pgi: *mut flash_page, ofs: u8_0) -> u32_0;
    #[no_mangle]
    fn get_pg_status(pgi: *mut flash_page, ofs: u8_0) -> fb_pg_status_t;
    #[no_mangle]
    fn get_nr_invalid_lps(pgi: *mut flash_page) -> u32_0;
    // block info interface
    #[no_mangle]
    fn init_blk_inf(blki: *mut flash_block);
    #[no_mangle]
    fn get_blk_idx(blki: *mut flash_block) -> u32_0;
    #[no_mangle]
    fn get_pgi_from_blki(blki: *mut flash_block, pg: u32_0)
     -> *mut flash_page;
    #[no_mangle]
    fn get_nr_valid_lps_in_blk(blki: *mut flash_block) -> u32_0;
    #[no_mangle]
    fn get_nr_invalid_lps_in_blk(blki: *mut flash_block) -> u32_0;
    #[no_mangle]
    fn inc_bers_cnt(blki: *mut flash_block) -> u32_0;
    #[no_mangle]
    fn set_rsv_blk_flag(blki: *mut flash_block, flag: libc::c_int);
    // chip info interface
    #[no_mangle]
    fn set_free_blk(ssdi: *mut ssd_info, bi: *mut flash_block);
    #[no_mangle]
    fn reset_free_blk(ssdi: *mut ssd_info, bi: *mut flash_block);
    #[no_mangle]
    fn get_free_block(ssdi: *mut ssd_info, bus: u32_0, chip: u32_0)
     -> *mut flash_block;
    #[no_mangle]
    fn get_nr_free_blks_in_chip(ci: *mut flash_chip) -> u32_0;
    #[no_mangle]
    fn get_used_block(ssdi: *mut ssd_info, bus: u32_0, chip: u32_0)
     -> *mut flash_block;
    #[no_mangle]
    fn reset_dirt_blk(ssdi: *mut ssd_info, bi: *mut flash_block);
    #[no_mangle]
    fn get_dirt_block(ssdi: *mut ssd_info, bus: u32_0, chip: u32_0)
     -> *mut flash_block;
    #[no_mangle]
    fn get_nr_dirt_blks_in_chip(ci: *mut flash_chip) -> u32_0;
    #[no_mangle]
    fn get_chip_info(ptr_ssd_info: *mut ssd_info, bus: u32_0, chip: u32_0)
     -> *mut flash_chip;
    #[no_mangle]
    fn perf_inc_nr_wordline_prog_bg();
    #[no_mangle]
    fn perf_inc_nr_page_reads();
    #[no_mangle]
    fn perf_inc_nr_blk_erasures();
    #[no_mangle]
    fn vdevice_read(ptr_vdevice: *mut vdevice_t, bus: u8_0, chip: u8_0,
                    block: u32_0, page: u32_0, page_bitmap: *mut u8_0,
                    ptr_dest: *mut u8_0, ptr_fb_bio: *mut fb_bio_t);
    #[no_mangle]
    fn vdevice_write(ptr_vdevice: *mut vdevice_t, bus: u8_0, chip: u8_0,
                     block: u32_0, page: u32_0, ptr_src: *const u8_0,
                     ptr_fb_bio: *mut fb_bio_t);
    #[no_mangle]
    fn vdevice_erase(ptr_vdevice: *mut vdevice_t, bus: u8_0, chip: u8_0,
                     block: u32_0, ptr_fb_bio: *mut fb_bio_t);
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_int;
pub const PAGE_UNMAPPED: C2RustUnnamed_0 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flash_block {
    pub no_block: u32_0,
    pub no_chip: u32_0,
    pub no_bus: u32_0,
    pub nr_valid_pages: u32_0,
    pub nr_invalid_pages: u32_0,
    pub nr_free_pages: u32_0,
    pub nr_valid_log_pages: u32_0,
    pub nr_invalid_log_pages: u32_0,
    pub nr_erase_cnt: u32_0,
    pub is_bad_block: libc::c_int,
    pub last_modified_time: u32_0,
    pub is_reserved_block: libc::c_int,
    pub is_active_block: libc::c_int,
    pub list_pages: *mut flash_page,
    pub prev: *mut flash_block,
    pub next: *mut flash_block,
    pub del_flag: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fb_gc_mngr_t {
    pub gc_blks: *mut *mut flash_block,
    pub vic_blks: *mut *mut flash_block,
    pub first_valid_pg: *mut u32_0,
    pub lpas_to_copy: *mut u32_0,
    pub data_to_copy: *mut u8_0,
    pub nr_pgs_to_copy: u32_0,
}
pub const NUM_CHIPS_PER_BUS: C2RustUnnamed_1 = 2;
//
  // Hardware configuration
  //
pub const NUM_BUSES: C2RustUnnamed_1 = 1;
pub const NUM_CHIPS: C2RustUnnamed_1 = 2;
pub const LOGICAL_PAGE_SIZE: C2RustUnnamed_1 = 4096;
pub const NR_LP_IN_PP: C2RustUnnamed_1 = 4;
pub const NUM_PAGES_PER_BLOCK: C2RustUnnamed_1 = 192;
pub const PHYSICAL_PAGE_SIZE: C2RustUnnamed_1 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdevice_t {
    pub device_capacity: u64_0,
    pub logical_capacity: u64_0,
    pub ptr_vdisk: [*mut u8_0; 1],
    pub buses: [vdevice_bus_t; 1],
    pub ptr_bus_controller: *mut *mut fb_bus_controller_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdevice_bus_t {
    pub chips: [vdevice_chip_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdevice_chip_t {
    pub blocks: [vdevice_block_t; 171],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdevice_block_t {
    pub pages: [vdevice_page_t; 192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdevice_page_t {
    pub ptr_data: *mut u8_0,
}
pub const PAGE_STATUS_VALID: fb_pg_status_t = 1;
pub type fb_pg_status_t = libc::c_uint;
pub const PAGE_STATUS_DEL: fb_pg_status_t = 3;
pub const PAGE_STATUS_INVALID: fb_pg_status_t = 2;
pub const PAGE_STATUS_FREE: fb_pg_status_t = 0;
pub const BGC_TH_NR_BLKS: C2RustUnnamed_1 = 14;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const BGC_TH_WB_UTIL: C2RustUnnamed_1 = 5;
pub const BGC_TH_INTV: C2RustUnnamed_1 = 5000;
pub const NUM_MAX_ENTRIES_OPR_QUEUE: C2RustUnnamed_1 = 4;
pub const TBLOCK: C2RustUnnamed_1 = 100;
pub const TPLOCK: C2RustUnnamed_1 = 100;
pub const TBERS: C2RustUnnamed_1 = 5000;
pub const TPROG: C2RustUnnamed_1 = 800;
pub const TREAD: C2RustUnnamed_1 = 80;
pub const SECTOR_SIZE: C2RustUnnamed_1 = 512;
pub const NUM_LOG_PAGES: C2RustUnnamed_1 = 236390;
pub const NUM_PAGES_IN_WRITE_BUFFER: C2RustUnnamed_1 = 8;
pub const NUM_PAGES: C2RustUnnamed_1 = 65664;
pub const NUM_BLOCKS: C2RustUnnamed_1 = 342;
pub const LP_PAGE_SHIFT: C2RustUnnamed_1 = 2;
pub const LP_PAGE_MASK: C2RustUnnamed_1 = 3;
// v-layer * h-layer * multi-level degree
pub const CFACTOR_PERCENT: C2RustUnnamed_1 = 90;
pub const NUM_BLOCKS_PER_CHIP: C2RustUnnamed_1 = 171;
pub const NR_MAX_LPGS_COPY: C2RustUnnamed_1 = 6144;
pub const NUM_BTODS: C2RustUnnamed_1 = 2048;
pub const NUM_WTODS: C2RustUnnamed_1 = 2048;
//
  // Parameterf for DEL manager
  //
pub const NR_MAX_LPAS_DISCARD: C2RustUnnamed_1 = 2048;
#[no_mangle]
pub unsafe extern "C" fn init_gcm(mut gcm: *mut fb_gc_mngr_t) {
    (*gcm).nr_pgs_to_copy = 0 as libc::c_int as u32_0;
}
unsafe extern "C" fn get_vic_blk(mut gcm: *mut fb_gc_mngr_t, mut bus: u32_0,
                                 mut chip: u32_0) -> *mut flash_block {
    return *(*gcm).vic_blks.offset(bus.wrapping_mul(NUM_CHIPS_PER_BUS as
                                                        libc::c_int as
                                                        libc::c_uint).wrapping_add(chip)
                                       as isize);
}
unsafe extern "C" fn set_vic_blk(mut gcm: *mut fb_gc_mngr_t, mut bus: u32_0,
                                 mut chip: u32_0,
                                 mut blki: *mut flash_block) {
    let ref mut fresh0 =
        *(*gcm).vic_blks.offset(bus.wrapping_mul(NUM_CHIPS_PER_BUS as
                                                     libc::c_int as
                                                     libc::c_uint).wrapping_add(chip)
                                    as isize);
    *fresh0 = blki;
}
unsafe extern "C" fn find_first_valid_pg(mut blki: *mut flash_block,
                                         mut start_pg: u32_0) -> u32_0 {
    let mut pgi: *mut flash_page = 0 as *mut flash_page;
    let mut pg: u32_0 = 0;
    pg = start_pg;
    while pg < NUM_PAGES_PER_BLOCK as libc::c_int as libc::c_uint {
        pgi = get_pgi_from_blki(blki, pg);
        if get_nr_invalid_lps(pgi) <
               NR_LP_IN_PP as libc::c_int as libc::c_uint {
            break ;
        }
        pg = pg.wrapping_add(1)
    }
    return pg;
}
unsafe extern "C" fn set_first_valid_pg(mut gcm: *mut fb_gc_mngr_t,
                                        mut bus: u32_0, mut chip: u32_0,
                                        mut pg: u32_0) {
    *(*gcm).first_valid_pg.offset(bus.wrapping_mul(NUM_CHIPS_PER_BUS as
                                                       libc::c_int as
                                                       libc::c_uint).wrapping_add(chip)
                                      as isize) = pg;
}
unsafe extern "C" fn get_first_valid_pg(mut gcm: *mut fb_gc_mngr_t,
                                        mut bus: u32_0, mut chip: u32_0)
 -> u32_0 {
    return *(*gcm).first_valid_pg.offset(bus.wrapping_mul(NUM_CHIPS_PER_BUS as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(chip)
                                             as isize);
}
unsafe extern "C" fn select_vic_blk_from_used(mut ssdi: *mut ssd_info,
                                              mut bus: u32_0, mut chip: u32_0)
 -> *mut flash_block {
    let mut vic_blki: *mut flash_block = 0 as *mut flash_block;
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut nr_max_invalid_lpgs: u32_0 = 0 as libc::c_int as u32_0;
    let mut nr_invalid_lpgs: u32_0 = 0;
    blki = get_used_block(ssdi, bus, chip);
    while !blki.is_null() {
        nr_invalid_lpgs = get_nr_invalid_lps_in_blk(blki);
        if nr_invalid_lpgs > nr_max_invalid_lpgs {
            nr_max_invalid_lpgs = nr_invalid_lpgs;
            vic_blki = blki
        }
        blki = (*blki).next
    }
    return vic_blki;
}
unsafe extern "C" fn select_vic_blk_greedy(mut ssdi: *mut ssd_info,
                                           mut bus: u32_0, mut chip: u32_0)
 -> *mut flash_block {
    let mut vic_blki: *mut flash_block = 0 as *mut flash_block;
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut nr_max_invalid_lpgs: u32_0 = 0 as libc::c_int as u32_0;
    let mut nr_invalid_lpgs: u32_0 = 0;
    vic_blki = get_dirt_block(ssdi, bus, chip);
    if !vic_blki.is_null() { return vic_blki }
    blki = get_used_block(ssdi, bus, chip);
    while !blki.is_null() {
        nr_invalid_lpgs = get_nr_invalid_lps_in_blk(blki);
        if nr_invalid_lpgs > nr_max_invalid_lpgs {
            nr_max_invalid_lpgs = nr_invalid_lpgs;
            vic_blki = blki
        }
        blki = (*blki).next
    }
    return vic_blki;
}
unsafe extern "C" fn set_vic_blks(mut fb: *mut fb_context_t) -> libc::c_int {
    let mut ftl: *mut page_mapping_context_t =
        get_ftl(fb) as *mut page_mapping_context_t;
    let mut gcm: *mut fb_gc_mngr_t = get_gcm(ftl);
    let mut ssdi: *mut ssd_info = get_ssd_inf(fb);
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut bus: u32_0 = 0;
    let mut chip: u32_0 = 0;
    bus = 0 as libc::c_int as u32_0;
    while bus < NUM_BUSES as libc::c_int as libc::c_uint {
        chip = 0 as libc::c_int as u32_0;
        while chip < NUM_CHIPS_PER_BUS as libc::c_int as libc::c_uint {
            if !get_curr_gc_block(fb, bus, chip).is_null() &&
                   !get_curr_active_block(fb, bus, chip).is_null() {
                blki = 0 as *mut flash_block
            } else {
                blki = select_vic_blk_greedy(ssdi, bus, chip);
                if blki.is_null() {
                    printk(b"\x013flashbench: gc_page_mapping: There is no avaiable victim block.\n\x00"
                               as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                if get_nr_valid_lps_in_blk(blki) ==
                       0 as libc::c_int as libc::c_uint {
                    blki = 0 as *mut flash_block
                } else {
                    (*gcm).nr_pgs_to_copy =
                        ((*gcm).nr_pgs_to_copy as
                             libc::c_uint).wrapping_add(get_nr_valid_lps_in_blk(blki))
                            as u32_0 as u32_0;
                    set_first_valid_pg(gcm, bus, chip,
                                       find_first_valid_pg(blki,
                                                           0 as libc::c_int as
                                                               u32_0));
                }
            }
            set_vic_blk(gcm, bus, chip, blki);
            chip = chip.wrapping_add(1)
        }
        bus = bus.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_valid_pgs_in_vic_blks(mut fb: *mut fb_context_t) {
    let mut ftl: *mut page_mapping_context_t =
        get_ftl(fb) as *mut page_mapping_context_t;
    let mut gcm: *mut fb_gc_mngr_t = get_gcm(ftl);
    let mut vdev: *mut vdevice_t = get_vdev(fb);
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut pgi: *mut flash_page = 0 as *mut flash_page;
    let mut nr_read_pgs: u32_0 = 0 as libc::c_int as u32_0;
    let mut ptr_lpa: *mut u32_0 = (*gcm).lpas_to_copy;
    let mut ptr_data: *mut u8_0 = (*gcm).data_to_copy;
    let mut lp_bitmap: [u8_0; 4] = [0; 4];
    let mut bus: u8_0 = 0;
    let mut chip: u8_0 = 0;
    let mut pg: u32_0 = 0;
    let mut lp: u32_0 = 0;
    let mut nr_pgs_to_read: u32_0 = 0;
    while (*gcm).nr_pgs_to_copy > nr_read_pgs {
        pg = 0 as libc::c_int as u32_0;
        while pg < NUM_PAGES_PER_BLOCK as libc::c_int as libc::c_uint {
            chip = 0 as libc::c_int as u8_0;
            while (chip as libc::c_int) < NUM_CHIPS_PER_BUS as libc::c_int {
                bus = 0 as libc::c_int as u8_0;
                while (bus as libc::c_int) < NUM_BUSES as libc::c_int {
                    blki = get_vic_blk(gcm, bus as u32_0, chip as u32_0);
                    if !blki.is_null() {
                        pgi = get_pgi_from_blki(blki, pg);
                        nr_pgs_to_read =
                            (NR_LP_IN_PP as libc::c_int as
                                 libc::c_uint).wrapping_sub(get_nr_invalid_lps(pgi));
                        if nr_pgs_to_read > 0 as libc::c_int as libc::c_uint {
                            lp = 0 as libc::c_int as u32_0;
                            while lp <
                                      NR_LP_IN_PP as libc::c_int as
                                          libc::c_uint {
                                if get_pg_status(pgi, lp as u8_0) as
                                       libc::c_uint ==
                                       PAGE_STATUS_VALID as libc::c_int as
                                           libc::c_uint {
                                    *ptr_lpa =
                                        get_mapped_lpa(pgi, lp as u8_0);
                                    lp_bitmap[lp as usize] =
                                        1 as libc::c_int as u8_0;
                                    ptr_lpa = ptr_lpa.offset(1)
                                } else {
                                    lp_bitmap[lp as usize] =
                                        0 as libc::c_int as u8_0
                                }
                                lp = lp.wrapping_add(1)
                            }
                            perf_inc_nr_page_reads();
                            vdevice_read(vdev, bus, chip, get_blk_idx(blki),
                                         pg, lp_bitmap.as_mut_ptr(), ptr_data,
                                         0 as *mut fb_bio_t);
                            ptr_data =
                                ptr_data.offset(nr_pgs_to_read.wrapping_mul(LOGICAL_PAGE_SIZE
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                    as isize);
                            nr_read_pgs =
                                (nr_read_pgs as
                                     libc::c_uint).wrapping_add(nr_pgs_to_read)
                                    as u32_0 as u32_0
                        }
                    }
                    bus = bus.wrapping_add(1)
                }
                chip = chip.wrapping_add(1)
            }
            pg = pg.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn prog_valid_pgs_to_gc_blks(mut fb: *mut fb_context_t)
 -> libc::c_int {
    let mut ftl: *mut page_mapping_context_t =
        get_ftl(fb) as *mut page_mapping_context_t;
    let mut gcm: *mut fb_gc_mngr_t = get_gcm(ftl);
    let mut vdev: *mut vdevice_t = get_vdev(fb);
    let mut nr_pgs_to_prog: s32 = (*gcm).nr_pgs_to_copy as s32;
    let mut ptr_lpa: *mut u32_0 = (*gcm).lpas_to_copy;
    let mut ptr_data: *mut u8_0 = (*gcm).data_to_copy;
    let mut bus: u8_0 = 0;
    let mut chip: u8_0 = 0;
    let mut blk: u32_0 = 0;
    let mut pg: u32_0 = 0;
    let mut lp: u32_0 = 0;
    while nr_pgs_to_prog > 0 as libc::c_int {
        get_next_bus_chip(fb, &mut bus, &mut chip);
        if alloc_new_page(fb, bus, chip, &mut blk, &mut pg) ==
               -(1 as libc::c_int) {
            printk(b"\x013flashbench: Wrong active block handling\n\x00" as
                       *const u8 as *const libc::c_char);
            print_blk_mgmt(fb);
            return -(1 as libc::c_int)
        }
        if nr_pgs_to_prog < NR_LP_IN_PP as libc::c_int {
            lp = nr_pgs_to_prog as u32_0;
            while lp < NR_LP_IN_PP as libc::c_int as libc::c_uint {
                *ptr_lpa.offset(lp as isize) =
                    PAGE_UNMAPPED as libc::c_int as u32_0;
                lp = lp.wrapping_add(1)
            }
        }
        perf_inc_nr_wordline_prog_bg();
        vdevice_write(vdev, bus, chip, blk, pg, ptr_data, 0 as *mut fb_bio_t);
        if map_logical_to_physical(fb, ptr_lpa, bus as u32_0, chip as u32_0,
                                   blk, pg) == -(1 as libc::c_int) {
            printk(b"\x013flashbench: Mapping L2P in GC failed.\n\x00" as
                       *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        ptr_lpa = ptr_lpa.offset(NR_LP_IN_PP as libc::c_int as isize);
        ptr_data =
            ptr_data.offset(PHYSICAL_PAGE_SIZE as libc::c_int as isize);
        nr_pgs_to_prog -= NR_LP_IN_PP as libc::c_int;
        set_prev_bus_chip(fb, bus, chip);
        update_act_blk(fb, bus, chip);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn prepare_act_blks(mut fb: *mut fb_context_t)
 -> libc::c_int {
    let mut vdev: *mut vdevice_t = get_vdev(fb);
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut bus: u8_0 = 0;
    let mut chip: u8_0 = 0;
    chip = 0 as libc::c_int as u8_0;
    while (chip as libc::c_int) < NUM_CHIPS_PER_BUS as libc::c_int {
        bus = 0 as libc::c_int as u8_0;
        while (bus as libc::c_int) < NUM_BUSES as libc::c_int {
            if get_curr_active_block(fb, bus as u32_0,
                                     chip as u32_0).is_null() {
                blki = get_curr_gc_block(fb, bus as u32_0, chip as u32_0);
                if blki.is_null() {
                    printk(b"\x013flashbench: Wrong GC block handling\n\x00"
                               as *const u8 as *const libc::c_char);
                    print_blk_mgmt(fb);
                    return -(1 as libc::c_int)
                }
                vdevice_erase(vdev, bus, chip, get_blk_idx(blki),
                              0 as *mut fb_bio_t);
                perf_inc_nr_blk_erasures();
                init_blk_inf(blki);
                inc_bers_cnt(blki);
                set_curr_active_block(fb, bus as u32_0, chip as u32_0, blki);
                set_curr_gc_block(fb, bus as u32_0, chip as u32_0,
                                  0 as *mut flash_block);
            }
            bus = bus.wrapping_add(1)
        }
        chip = chip.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn update_gc_blks(mut fb: *mut fb_context_t)
 -> libc::c_int {
    let mut gc_blki: *mut flash_block = 0 as *mut flash_block;
    let mut bus: u32_0 = 0;
    let mut chip: u32_0 = 0;
    bus = 0 as libc::c_int as u32_0;
    while bus < NUM_BUSES as libc::c_int as libc::c_uint {
        chip = 0 as libc::c_int as u32_0;
        while chip < NUM_CHIPS_PER_BUS as libc::c_int as libc::c_uint {
            if get_curr_gc_block(fb, bus, chip).is_null() {
                gc_blki = get_dirt_block(get_ssd_inf(fb), bus, chip);
                if gc_blki.is_null() {
                    if get_curr_active_block(fb, bus, chip).is_null() {
                        printk(b"\x013flashbench: Wrong block management handling\n\x00"
                                   as *const u8 as *const libc::c_char);
                        print_blk_mgmt(fb);
                        return -(1 as libc::c_int)
                    }
                } else {
                    reset_dirt_blk(get_ssd_inf(fb), gc_blki);
                    set_curr_gc_block(fb, bus, chip, gc_blki);
                }
            }
            chip = chip.wrapping_add(1)
        }
        bus = bus.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn create_gc_mngr(mut fb: *mut fb_context_t)
 -> *mut fb_gc_mngr_t {
    let mut current_block: u64;
    let mut ssdi: *mut ssd_info = get_ssd_inf(fb);
    let mut gcm: *mut fb_gc_mngr_t = 0 as *mut fb_gc_mngr_t;
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut bus: u32_0 = 0;
    let mut chip: u32_0 = 0;
    gcm =
        vmalloc(::core::mem::size_of::<fb_gc_mngr_t>() as libc::c_ulong) as
            *mut fb_gc_mngr_t;
    if gcm.is_null() {
        printk(b"\x013flashbench: Allocating GC manager failed.\n\x00" as
                   *const u8 as *const libc::c_char);
    } else {
        (*gcm).gc_blks =
            vmalloc((::core::mem::size_of::<*mut flash_block>() as
                         libc::c_ulong).wrapping_mul(NUM_CHIPS as libc::c_int
                                                         as libc::c_ulong)) as
                *mut *mut flash_block;
        if (*gcm).gc_blks.is_null() {
            printk(b"\x013flashbench: Allocating GC block list failed.\n\x00"
                       as *const u8 as *const libc::c_char);
        } else {
            (*gcm).vic_blks =
                vmalloc((::core::mem::size_of::<*mut flash_block>() as
                             libc::c_ulong).wrapping_mul(NUM_CHIPS as
                                                             libc::c_int as
                                                             libc::c_ulong))
                    as *mut *mut flash_block;
            if (*gcm).vic_blks.is_null() {
                printk(b"\x013flashbench: Allocating victim block list failed.\n\x00"
                           as *const u8 as *const libc::c_char);
            } else {
                (*gcm).lpas_to_copy =
                    vmalloc((::core::mem::size_of::<u32_0>() as
                                 libc::c_ulong).wrapping_mul(NUM_CHIPS as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(NUM_PAGES_PER_BLOCK
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_mul(NR_LP_IN_PP
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong))
                        as *mut u32_0;
                if (*gcm).lpas_to_copy.is_null() {
                    printk(b"\x013flashbench: Allocating LPA list failed.\n\x00"
                               as *const u8 as *const libc::c_char);
                } else {
                    (*gcm).data_to_copy =
                        vmalloc((::core::mem::size_of::<u32_0>() as
                                     libc::c_ulong).wrapping_mul(NUM_CHIPS as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(NUM_PAGES_PER_BLOCK
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_mul(NR_LP_IN_PP
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong).wrapping_mul(LOGICAL_PAGE_SIZE
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_ulong))
                            as *mut u8_0;
                    if (*gcm).data_to_copy.is_null() {
                        printk(b"\x013flashbench: Allocating valid page buffer failed.\n\x00"
                                   as *const u8 as *const libc::c_char);
                    } else {
                        (*gcm).first_valid_pg =
                            vmalloc((::core::mem::size_of::<u32_0>() as
                                         libc::c_ulong).wrapping_mul(NUM_CHIPS
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong))
                                as *mut u32_0;
                        if (*gcm).first_valid_pg.is_null() {
                            printk(b"\x013flashbench: Allocating page_offset failed.\n\x00"
                                       as *const u8 as *const libc::c_char);
                        } else {
                            init_gcm(gcm);
                            bus = 0 as libc::c_int as u32_0;
                            's_97:
                                loop  {
                                    if !(bus <
                                             NUM_BUSES as libc::c_int as
                                                 libc::c_uint) {
                                        current_block = 4775909272756257391;
                                        break ;
                                    }
                                    chip = 0 as libc::c_int as u32_0;
                                    while chip <
                                              NUM_CHIPS_PER_BUS as libc::c_int
                                                  as libc::c_uint {
                                        blki =
                                            get_free_block(ssdi, bus, chip);
                                        if blki.is_null() {
                                            printk(b"\x013flashbench: Getting new gc block failed.\n\x00"
                                                       as *const u8 as
                                                       *const libc::c_char);
                                            current_block =
                                                8921050815587899969;
                                            break 's_97 ;
                                        } else {
                                            reset_free_blk(ssdi, blki);
                                            set_rsv_blk_flag(blki,
                                                             true_0 as
                                                                 libc::c_int);
                                            let ref mut fresh1 =
                                                *(*gcm).gc_blks.offset(bus.wrapping_mul(NUM_CHIPS_PER_BUS
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_add(chip)
                                                                           as
                                                                           isize);
                                            *fresh1 = blki;
                                            chip = chip.wrapping_add(1)
                                        }
                                    }
                                    bus = bus.wrapping_add(1)
                                }
                            match current_block {
                                8921050815587899969 => { }
                                _ => { return gcm }
                            }
                        }
                    }
                }
            }
        }
    }
    destroy_gc_mngr(gcm);
    return 0 as *mut fb_gc_mngr_t;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_gc_mngr(mut gcm: *mut fb_gc_mngr_t) {
    if !gcm.is_null() {
        if !(*gcm).gc_blks.is_null() {
            vfree((*gcm).gc_blks as *const libc::c_void);
        }
        if !(*gcm).vic_blks.is_null() {
            vfree((*gcm).vic_blks as *const libc::c_void);
        }
        if !(*gcm).lpas_to_copy.is_null() {
            vfree((*gcm).lpas_to_copy as *const libc::c_void);
        }
        if !(*gcm).data_to_copy.is_null() {
            vfree((*gcm).data_to_copy as *const libc::c_void);
        }
        vfree(gcm as *const libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn trigger_gc_page_mapping(mut fb: *mut fb_context_t)
 -> libc::c_int {
    let mut ftl: *mut page_mapping_context_t =
        get_ftl(fb) as *mut page_mapping_context_t;
    let mut gcm: *mut fb_gc_mngr_t = get_gcm(ftl);
    // initialize GC context
    init_gcm(gcm);
    // 1. erase GC blocks and set them as active blocks
    if prepare_act_blks(fb) != 0 as libc::c_int {
        printk(b"\x013flashbench: Preparing GC blocks failed.\n\x00" as
                   *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    // 2. select a victim block for every parallel unit
    if set_vic_blks(fb) != 0 as libc::c_int {
        printk(b"\x013flashbench: Setting victim blocks failed.\n\x00" as
                   *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    // 3. read valid pages from victim blocks
  // JS - here is the function
    get_valid_pgs_in_vic_blks(fb);
    // 4. write read data to other pages
    if prog_valid_pgs_to_gc_blks(fb) != 0 as libc::c_int {
        printk(b"\x013flashbench: Writing valid data failed.\n\x00" as
                   *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    // 5. set GC blocks with dirt blocks
    if update_gc_blks(fb) != 0 as libc::c_int {
        printk(b"\x013flashbench: Updating GC blocks failed.\n\x00" as
                   *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fb_bgc_prepare_act_blks(mut fb: *mut fb_context_t)
 -> libc::c_int {
    let mut ssdi: *mut ssd_info = get_ssd_inf(fb);
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut bus: u8_0 = 0;
    let mut chip: u8_0 = 0;
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < NUM_CHIPS as libc::c_int as libc::c_uint {
        get_next_bus_chip(fb, &mut bus, &mut chip);
        blki = get_curr_gc_block(fb, bus as u32_0, chip as u32_0);
        if blki.is_null() {
            printk(b"\x013flashbench: Wrong BGC block handling\n\x00" as
                       *const u8 as *const libc::c_char);
            print_blk_mgmt(fb);
            return -(1 as libc::c_int)
        }
        vdevice_erase(get_vdev(fb), bus, chip, get_blk_idx(blki),
                      0 as *mut fb_bio_t);
        perf_inc_nr_blk_erasures();
        init_blk_inf(blki);
        inc_bers_cnt(blki);
        if get_curr_active_block(fb, bus as u32_0, chip as u32_0).is_null() {
            set_curr_active_block(fb, bus as u32_0, chip as u32_0, blki);
        } else { set_free_blk(ssdi, blki); }
        blki = get_dirt_block(ssdi, bus as u32_0, chip as u32_0);
        if !blki.is_null() { reset_dirt_blk(ssdi, blki); }
        set_curr_gc_block(fb, bus as u32_0, chip as u32_0, blki);
        set_prev_bus_chip(fb, bus, chip);
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fb_bgc_set_vic_blks(mut fb: *mut fb_context_t)
 -> libc::c_int {
    let mut ftl: *mut page_mapping_context_t =
        get_ftl(fb) as *mut page_mapping_context_t;
    let mut ssdi: *mut ssd_info = get_ssd_inf(fb);
    let mut gcm: *mut fb_gc_mngr_t = get_gcm(ftl);
    let mut chipi: *mut flash_chip = 0 as *mut flash_chip;
    let mut blki: *mut flash_block = 0 as *mut flash_block;
    let mut bus: u8_0 = 0;
    let mut chip: u8_0 = 0;
    // if necesary, find a new victim block for each chip
    bus = 0 as libc::c_int as u8_0;
    while (bus as libc::c_int) < NUM_BUSES as libc::c_int {
        let mut current_block_15: u64;
        chip = 0 as libc::c_int as u8_0;
        while (chip as libc::c_int) < NUM_CHIPS_PER_BUS as libc::c_int {
            chipi = get_chip_info(ssdi, bus as u32_0, chip as u32_0);
            if get_nr_dirt_blks_in_chip(chipi).wrapping_add(get_nr_free_blks_in_chip(chipi))
                   < BGC_TH_NR_BLKS as libc::c_int as libc::c_uint {
                // victim block exists
                blki = get_vic_blk(gcm, bus as u32_0, chip as u32_0);
                if !blki.is_null() {
                    set_first_valid_pg(gcm, bus as u32_0, chip as u32_0,
                                       find_first_valid_pg(blki,
                                                           get_first_valid_pg(gcm,
                                                                              bus
                                                                                  as
                                                                                  u32_0,
                                                                              chip
                                                                                  as
                                                                                  u32_0)));
                    if get_first_valid_pg(gcm, bus as u32_0, chip as u32_0) ==
                           NUM_PAGES_PER_BLOCK as libc::c_int as libc::c_uint
                       {
                        set_vic_blk(gcm, bus as u32_0, chip as u32_0,
                                    0 as *mut flash_block);
                        current_block_15 = 7976072742316086414;
                    } else { current_block_15 = 6483416627284290920; }
                } else { current_block_15 = 7976072742316086414; }
                match current_block_15 {
                    6483416627284290920 => { }
                    _ => {
                        // find a new one
                        blki =
                            select_vic_blk_from_used(ssdi, bus as u32_0,
                                                     chip as u32_0);
                        if blki.is_null() {
                            printk(b"\x013flashbench: Wrong block managment\n\x00"
                                       as *const u8 as *const libc::c_char);
                            print_blk_mgmt(fb);
                            return -(1 as libc::c_int)
                        }
                        set_vic_blk(gcm, bus as u32_0, chip as u32_0, blki);
                        set_first_valid_pg(gcm, bus as u32_0, chip as u32_0,
                                           find_first_valid_pg(blki,
                                                               0 as
                                                                   libc::c_int
                                                                   as u32_0));
                    }
                }
            } else {
                set_vic_blk(gcm, bus as u32_0, chip as u32_0,
                            0 as *mut flash_block);
                set_first_valid_pg(gcm, bus as u32_0, chip as u32_0,
                                   0 as libc::c_int as u32_0);
            }
            chip = chip.wrapping_add(1)
        }
        bus = bus.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fb_bgc_read_valid_pgs(mut fb: *mut fb_context_t)
 -> libc::c_int {
    let mut ftl: *mut page_mapping_context_t =
        get_ftl(fb) as *mut page_mapping_context_t;
    let mut gcm: *mut fb_gc_mngr_t = get_gcm(ftl);
    let mut vic_blki: *mut flash_block = 0 as *mut flash_block;
    let mut pgi: *mut flash_page = 0 as *mut flash_page;
    let mut bus: u8_0 = 0;
    let mut chip: u8_0 = 0;
    let mut lp: u8_0 = 0;
    let mut pg: u32_0 = 0;
    let mut nr_pgs_to_read: u32_0 = 0;
    let mut lp_bitmap: [u8_0; 4] = [0; 4];
    let mut ptr_lpas: *mut u32_0 = (*gcm).lpas_to_copy;
    let mut ptr_data: *mut u8_0 = (*gcm).data_to_copy;
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < NUM_CHIPS as libc::c_int as libc::c_uint {
        get_next_bus_chip(fb, &mut bus, &mut chip);
        vic_blki = get_vic_blk(gcm, bus as u32_0, chip as u32_0);
        if vic_blki.is_null() {
            set_prev_bus_chip(fb, bus, chip);
        } else {
            pg = get_first_valid_pg(gcm, bus as u32_0, chip as u32_0);
            pgi = get_pgi_from_blki(vic_blki, pg);
            nr_pgs_to_read =
                (NR_LP_IN_PP as libc::c_int as
                     libc::c_uint).wrapping_sub(get_nr_invalid_lps(pgi));
            if nr_pgs_to_read == 0 as libc::c_int as libc::c_uint {
                printk(b"\x013flashbench: Wrong page offset in victim block\n\x00"
                           as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int)
            }
            (*gcm).nr_pgs_to_copy =
                ((*gcm).nr_pgs_to_copy as
                     libc::c_uint).wrapping_add(nr_pgs_to_read) as u32_0 as
                    u32_0;
            lp = 0 as libc::c_int as u8_0;
            while (lp as libc::c_int) < NR_LP_IN_PP as libc::c_int {
                if get_pg_status(pgi, lp) as libc::c_uint ==
                       PAGE_STATUS_VALID as libc::c_int as libc::c_uint {
                    *ptr_lpas = get_mapped_lpa(pgi, lp);
                    lp_bitmap[lp as usize] = 1 as libc::c_int as u8_0;
                    ptr_lpas = ptr_lpas.offset(1)
                } else { lp_bitmap[lp as usize] = 0 as libc::c_int as u8_0 }
                lp = lp.wrapping_add(1)
            }
            perf_inc_nr_page_reads();
            vdevice_read(get_vdev(fb), bus, chip, get_blk_idx(vic_blki), pg,
                         lp_bitmap.as_mut_ptr(), ptr_data,
                         0 as *mut fb_bio_t);
            ptr_data =
                ptr_data.offset(nr_pgs_to_read.wrapping_mul(LOGICAL_PAGE_SIZE
                                                                as libc::c_int
                                                                as
                                                                libc::c_uint)
                                    as isize);
            set_prev_bus_chip(fb, bus, chip);
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
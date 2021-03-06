#pragma once
#include <linux/completion.h>
#include <linux/types.h>

enum {
  PAGE_UNMAPPED = -1,
};

struct bio;
struct flash_block;
struct fb_del_mngr_t;
struct fb_context_t;
struct fb_bio_t;
struct page_mapping_context_t;

struct page_mapping_table_t {
  u32 nr_entries;
  u32 *mappings;
};

struct fb_act_blk_mngr_t {
  struct flash_block **act_blks;

  u32 mru_bus;
  u32 mru_chip;
};

struct fb_gc_mngr_t {
  struct flash_block **gc_blks;

  struct flash_block **vic_blks;
  u32 *first_valid_pg;

  u32 *lpas_to_copy;
  u8 *data_to_copy;

  u32 nr_pgs_to_copy;
};

struct page_mapping_context_t {
  struct completion mapping_context_lock;

  struct page_mapping_table_t *ptr_mapping_table;
  struct fb_act_blk_mngr_t *abm;
  struct fb_gc_mngr_t *gcm;
  struct fb_del_mngr_t *delm;

  u32 *lpas_to_discard;
};

void *create_pg_ftl(struct fb_context_t *fb);
void destroy_pg_ftl(struct page_mapping_context_t *ftl);
struct fb_gc_mngr_t *get_gcm(struct page_mapping_context_t *ftl);
struct fb_act_blk_mngr_t *get_abm(struct page_mapping_context_t *ftl);
void print_blk_mgmt(struct fb_context_t *fb);
void fb_del_invalid_data(struct fb_context_t *fb, struct fb_bio_t *fb_bio);

int make_read_request_page_mapping(struct fb_context_t *ptr_fb_context,
                                   u32 logical_page_address,
                                   u8 *ptr_page_buffer,
                                   struct fb_bio_t *ptr_fb_bio);
int make_flush_request_page_mapping(void);
int make_discard_request_page_mapping(struct fb_context_t *ptr_fb_context,
                                      struct bio *bio);
int fb_wb_flush(struct fb_context_t *fb);
